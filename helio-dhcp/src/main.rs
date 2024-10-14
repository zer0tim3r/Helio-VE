use dhcproto::v4::{DhcpOption, Message, MessageType, Opcode, OptionCode};
use dhcproto::{Decodable, Decoder, Encodable, Encoder};
use dotenvy::dotenv;
use helio_pg::{models, PGClient};
use nix::sys::socket::{setsockopt, sockopt::BindToDevice};
use std::net::{Ipv4Addr, UdpSocket};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_pg = PGClient::new(database_url);

    let ipt = iptables::new(false)?;

    let check_add =
        |table: &str, chain: &str, rule: &str| -> Result<(), Box<dyn std::error::Error>> {
            if !ipt.exists(table, chain, rule)? {
                ipt.append_unique(table, chain, rule)?;
            }

            Ok(())
        };

    // UDP 소켓 생성 (DHCP 서버는 67번 포트를 사용)
    let socket = UdpSocket::bind("0.0.0.0:67")?;
    setsockopt(&socket, BindToDevice, &std::ffi::OsString::from("br0"))
        .expect("인터페이스 바인딩 실패");
    socket.set_broadcast(true)?;

    println!("HVE dhcp listening on port {}", 67);

    loop {
        let mut buf = [0u8; 1024];
        let (amt, _) = socket.recv_from(&mut buf)?;
        let broadcast =
            core::net::SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 68);

        // 수신한 DHCP 메시지를 파싱
        if let Ok(dhcp_message) = Message::decode(&mut Decoder::new(&buf[..amt])) {
            let client_mac = dhcp_message
                .chaddr()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(":");

            // DHCP Discover 메시지에만 응답
            if let Some(DhcpOption::MessageType(message_type)) =
                dhcp_message.opts().get(OptionCode::MessageType)
            {
                match message_type {
                    MessageType::Discover => {
                        println!("Received DHCP Discover from MAC: {}", client_mac);

                        let conn = &mut client_pg.0.get()?;

                        match models::instance::Instance::_dhcp_get_by_mac(conn, client_mac) {
                            Ok(instance) => {
                                println!(
                                    "{} : MAC 주소가 허용되었습니다. DHCP Offer 전송...",
                                    instance.uuid
                                );

                                // 할당할 IP 주소 선택 (임시로 고정 IP를 사용)
                                let offered_ip = Ipv4Addr::new(192, 168, 10, 252);

                                // DHCP Offer 메시지 생성
                                let mut offer_message = Message::default();
                                offer_message.set_opcode(Opcode::BootReply);
                                offer_message.set_xid(dhcp_message.xid());
                                offer_message.set_yiaddr(offered_ip);
                                offer_message.set_chaddr(dhcp_message.chaddr());

                                offer_message
                                    .opts_mut()
                                    .insert(DhcpOption::MessageType(MessageType::Offer));
                                offer_message
                                    .opts_mut()
                                    .insert(DhcpOption::ServerIdentifier(Ipv4Addr::new(
                                        192, 168, 10, 254,
                                    )));
                                offer_message.opts_mut().insert(DhcpOption::SubnetMask(
                                    Ipv4Addr::new(255, 255, 255, 0),
                                ));
                                offer_message
                                    .opts_mut()
                                    .insert(DhcpOption::AddressLeaseTime(3600u32));

                                // DHCP Offer 메시지를 클라이언트로 전송
                                let mut offer_buf = Vec::new();
                                offer_message
                                    .encode(&mut Encoder::new(&mut offer_buf))
                                    .unwrap();
                                socket.send_to(&offer_buf, broadcast)?;
                            }
                            Err(_) => {
                                println!("MAC 주소가 허용되지 않았습니다. 응답을 무시합니다.");
                            }
                        };
                    }
                    MessageType::Request => {
                        println!("Received DHCP Request from MAC: {}", client_mac);

                        let conn = &mut client_pg.0.get()?;

                        match models::instance::Instance::_dhcp_get_by_mac(conn, client_mac.clone())
                        {
                            Ok(instance) => {
                                println!(
                                    "{} : MAC 주소가 허용되었습니다. DHCP ACK 전송...",
                                    instance.uuid
                                );

                                let instance_ip = instance.ipv4.parse::<Ipv4Addr>()?;

                                check_add(
                                    "filter",
                                    "FORWARD",
                                    format!(
                                        "-s {} -m mac --mac-source {} -j ACCEPT",
                                        instance_ip.to_string(),
                                        client_mac.clone()
                                    )
                                    .as_str(),
                                )?;

                                // DHCP ACK 메시지 생성
                                let mut ack_message = Message::default();
                                ack_message.set_opcode(Opcode::BootReply);
                                ack_message.set_xid(dhcp_message.xid());
                                ack_message.set_yiaddr(instance_ip);
                                ack_message.set_chaddr(dhcp_message.chaddr());

                                ack_message
                                    .opts_mut()
                                    .insert(DhcpOption::MessageType(MessageType::Ack));
                                ack_message.opts_mut().insert(DhcpOption::ServerIdentifier(
                                    Ipv4Addr::new(192, 168, 10, 254),
                                ));
                                ack_message.opts_mut().insert(DhcpOption::SubnetMask(
                                    Ipv4Addr::new(255, 255, 255, 0),
                                ));
                                ack_message.opts_mut().insert(DhcpOption::Router(vec![
                                    Ipv4Addr::new(192, 168, 10, 254),
                                ]));
                                ack_message
                                    .opts_mut()
                                    .insert(DhcpOption::AddressLeaseTime(3600u32));

                                // DHCP ACK 메시지를 클라이언트로 전송
                                let mut ack_buf = Vec::new();
                                ack_message.encode(&mut Encoder::new(&mut ack_buf)).unwrap();
                                socket.send_to(&ack_buf, broadcast)?;
                            }
                            Err(_) => {
                                println!("MAC 주소가 허용되지 않았습니다. DHCP Request 응답을 무시합니다.");
                            }
                        };
                    }
                    _ => {}
                }
            }
        }
    }
}
