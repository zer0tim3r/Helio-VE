use dhcproto::v4::{DhcpOption, Message, MessageType, Opcode, OptionCode};
use dhcproto::{Decodable, Decoder, Encodable, Encoder};
use std::collections::HashMap;
use std::ffi::OsString;
use std::net::{Ipv4Addr, UdpSocket};
use nix::sys::socket::{setsockopt, sockopt::BindToDevice};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let allowed_mac = vec!["52:54:00:11:22:33".to_string()]; // 허용된 MAC 주소 목록
    let mut ip_pool = HashMap::new(); // 클라이언트 IP 할당을 위한 IP 풀

    // UDP 소켓 생성 (DHCP 서버는 67번 포트를 사용)
    let socket = UdpSocket::bind("0.0.0.0:67")?;
    setsockopt(&socket, BindToDevice, &OsString::from("br0")).expect("인터페이스 바인딩 실패");
    socket.set_broadcast(true)?;

    loop {
        let mut buf = [0u8; 1024];
        let (amt, _) = socket.recv_from(&mut buf)?;
        let broadcast =  core::net::SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 68);

        // 수신한 DHCP 메시지를 파싱
        if let Ok(dhcp_message) = Message::decode(&mut Decoder::new(&buf[..amt])) {
            let client_mac = format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                dhcp_message.chaddr()[0],
                dhcp_message.chaddr()[1],
                dhcp_message.chaddr()[2],
                dhcp_message.chaddr()[3],
                dhcp_message.chaddr()[4],
                dhcp_message.chaddr()[5]
            );

            // DHCP Discover 메시지에만 응답
            if let Some(DhcpOption::MessageType(message_type)) =
                dhcp_message.opts().get(OptionCode::MessageType)
            {
                match message_type {
                    MessageType::Discover => {
                        println!("Received DHCP Discover from MAC: {}", client_mac);

                        // 허용된 MAC 주소인지 확인
                        if allowed_mac.contains(&client_mac) {
                            println!("MAC 주소가 허용되었습니다. DHCP Offer 전송...");

                            // 할당할 IP 주소 선택 (임시로 고정 IP를 사용)
                            let offered_ip = Ipv4Addr::new(192, 168, 10, 3);
                            ip_pool.insert(client_mac.clone(), offered_ip);

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
                            offer_message
                                .opts_mut()
                                .insert(DhcpOption::SubnetMask(Ipv4Addr::new(255, 255, 255, 0)));
                            offer_message
                                .opts_mut()
                                .insert(DhcpOption::AddressLeaseTime(3600u32));

                            // DHCP Offer 메시지를 클라이언트로 전송
                            let mut offer_buf = Vec::new();
                            offer_message
                                .encode(&mut Encoder::new(&mut offer_buf))
                                .unwrap();
                            socket.send_to(&offer_buf, broadcast)?;
                        } else {
                            println!("MAC 주소가 허용되지 않았습니다. 응답을 무시합니다.");
                        }
                    }
                    MessageType::Request => {
                        println!("Received DHCP Request from MAC: {}", client_mac);

                        if allowed_mac.contains(&client_mac) {
                            if let Some(allocated_ip) = ip_pool.get(&client_mac) {
                                println!("MAC 주소가 허용되었습니다. DHCP ACK 전송...");

                                // DHCP ACK 메시지 생성
                                let mut ack_message = Message::default();
                                ack_message.set_opcode(Opcode::BootReply);
                                ack_message.set_xid(dhcp_message.xid());
                                ack_message.set_yiaddr(*allocated_ip);
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
                                ack_message.opts_mut().insert(DhcpOption::Router(
                                    vec!(Ipv4Addr::new(192, 168, 10, 254)),
                                ));
                                ack_message
                                    .opts_mut()
                                    .insert(DhcpOption::AddressLeaseTime(3600u32));

                                // DHCP ACK 메시지를 클라이언트로 전송
                                let mut ack_buf = Vec::new();
                                ack_message.encode(&mut Encoder::new(&mut ack_buf)).unwrap();
                                socket.send_to(&ack_buf, broadcast)?;
                            }
                        } else {
                            println!(
                                "MAC 주소가 허용되지 않았습니다. DHCP Request 응답을 무시합니다."
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

/*
sudo firewall-cmd --permanent --zone=public --change-interface=br0


sudo firewall-cmd --permanent --zone=public --add-port=67/udp
sudo firewall-cmd --permanent --zone=public --add-masquerade
sudo firewall-cmd --permanent --zone=public --add-forward

sudo firewall-cmd --permanent --direct --add-rule ipv4 filter FORWARD 2 -j DROP
sudo firewall-cmd --permanent --direct --add-rule ipv4 filter FORWARD 1 ! -s 192.168.10.0/24 -j ACCEPT
sudo firewall-cmd --permanent --direct --add-rule ipv4 filter FORWARD 0 -s 192.168.10.3 -m mac --mac-source 52:54:00:11:22:33 -j ACCEPT

sudo firewall-cmd --permanent --direct --add-rule ipv4 nat PREROUTING 0 -d 169.254.169.254 -p tcp --dport 80 -j DNAT --to-destination 192.168.10.254:8180

sudo firewall-cmd --reload

*/
