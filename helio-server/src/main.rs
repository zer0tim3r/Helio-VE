use std::net::Ipv4Addr;

use helio_pg::{models, wrapper, PGClient};
use rand::Rng;

mod common;
mod qemu_kvm;

// !! IMPORTANT !!
// vscode -> settings.json
// "rust-analyzer.cargo.loadOutDirsFromCheck": true,
tonic::include_proto!("rpc");

use helio_server::{Helio, HelioServer};

pub struct RPC {
    client_pg: PGClient,
}

impl RPC {
    fn new(client_pg: PGClient) -> Self {
        RPC { client_pg }
    }
}

fn to_timestamp(t: chrono::DateTime<chrono::Utc>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    }
}

#[tonic::async_trait]
impl Helio for RPC {
    async fn list_instance(
        &self,
        request: tonic::Request<ListInstanceArgs>,
    ) -> Result<tonic::Response<ListInstanceResult>, tonic::Status> {
        println!("Received Instance list request: {:?}", request);

        let conn = &mut self
            .client_pg
            .0
            .get()
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        let args = request.into_inner();

        let instances = models::instance::Instance::_rpc_list(conn, args.created_by)
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        Ok(tonic::Response::new(ListInstanceResult {
            instances: instances
                .iter()
                .map(|i| InstanceModel {
                    uuid: i.uuid.clone(),
                    label: i.label.clone(),
                    itype: i.itype,
                    image: i.image,
                    mac: i.mac.clone(),
                    ipv4: i.ipv4.clone(),
                    created_by: i.created_by.clone(),
                    created_at: Some(to_timestamp(i.created_at)),
                    updated_at: Some(to_timestamp(i.updated_at)),
                })
                .collect(),
        }))
    }

    async fn create_instance(
        &self,
        request: tonic::Request<CreateInstanceArgs>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        println!("Received Instance create request: {:?}", request);

        let conn = &mut self
            .client_pg
            .0
            .get()
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        let mut rng = rand::thread_rng();

        let mut unique_mac;

        loop {
            unique_mac = [
                (rng.gen_range(0..=255) & 0b11111100) | 0b00000010, // 로컬, 멀티캐스트
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ]
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(":");

            match models::instance::Instance::_dhcp_get_by_mac(conn, unique_mac.clone()) {
                Ok(_) => {
                    continue;
                }
                Err(err) => {
                    if let wrapper::Error::NotFound = err {
                        break;
                    }

                    continue;
                }
            }
        }

        let mut unique_ipv4;

        loop {
            unique_ipv4 = Ipv4Addr::new(192, 168, 10, rng.gen_range(1..=250));

            match models::instance::Instance::_cloudinit_get_by_ipv4(conn, unique_ipv4.to_string())
            {
                Ok(_) => {
                    continue;
                }
                Err(err) => {
                    if let wrapper::Error::NotFound = err {
                        break;
                    }

                    continue;
                }
            }
        }

        let args = request.into_inner();

        let instance = models::instance::Instance::_rpc_create(
            conn,
            models::instance::NewInstance {
                uuid: args.uuid,
                label: args.label,
                itype: args.itype,
                image: args.image,
                mac: unique_mac,
                ipv4: unique_ipv4.to_string(),
                created_by: args.created_by,
            },
        )
        .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        qemu_kvm::create_instance(instance).map_err(|e| tonic::Status::from_error(e))?;

        Ok(tonic::Response::new(()))
    }

    async fn delete_instance(
        &self,
        request: tonic::Request<DeleteInstanceArgs>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        println!("Received Instance delete request: {:?}", request);

        let conn = &mut self
            .client_pg
            .0
            .get()
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        let args = request.into_inner();

        let instance = models::instance::Instance::_rpc_delete(conn, args.uuid, args.created_by)
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        qemu_kvm::delete_instance(instance).map_err(|e| tonic::Status::from_error(e))?;

        Ok(tonic::Response::new(()))
    }

    async fn start_instance(
        &self,
        request: tonic::Request<StartInstanceArgs>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        println!("Received Instance start request: {:?}", request);

        let conn = &mut self
            .client_pg
            .0
            .get()
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        let args = request.into_inner();

        let instance =
            models::instance::Instance::_default_get_by_uuid(conn, args.uuid, args.created_by)
                .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

        qemu_kvm::start_instance(instance).map_err(|e| tonic::Status::from_error(e))?;

        Ok(tonic::Response::new(()))
    }
}

fn setup_ipt() -> Result<(), Box<dyn std::error::Error>> {
    let ipt = iptables::new(false)?;

    let check_add =
        |table: &str, chain: &str, rule: &str| -> Result<(), Box<dyn std::error::Error>> {
            if !ipt.exists(table, chain, rule)? {
                ipt.append_unique(table, chain, rule)?;
            }

            Ok(())
        };

    ipt.set_policy("filter", "FORWARD", "DROP")?;
    check_add("filter", "FORWARD", "-d 192.168.10.0/24 -j ACCEPT")?;
    check_add(
        "nat",
        "PREROUTING",
        "-d 169.254.169.254 -p tcp --dport 80 -j DNAT --to-destination 192.168.10.254:8180",
    )?;
    check_add("nat", "POSTROUTING", "-o enp7s0 -j MASQUERADE")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_ipt()?;

    // dotenv()?;
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_pg = PGClient::new(database_url);

    let port = 8080;
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    let server = tokio::spawn(async move {
        let rpc_service = RPC::new(client_pg);

        tonic::transport::Server::builder()
            .add_service(HelioServer::new(rpc_service))
            .serve(addr)
            .await
            .unwrap()
    });

    println!("HVE server listening on port {}", port);

    server.await?;

    Ok(())
}

// impl From<CreateInstanceArgs> for models::instance::NewInstance {
//     fn from(args: CreateInstanceArgs) -> Self {
//         let generate_unique_mac = || {
//             loop {
//                 let mut rng = rand::thread_rng();

//                 let mac = [
//                     (rng.gen_range(0..=255) & 0b11111100) | 0b00000010, // 로컬, 멀티캐스트
//                     rng.gen_range(0..=255),
//                     rng.gen_range(0..=255),
//                     rng.gen_range(0..=255),
//                     rng.gen_range(0..=255),
//                     rng.gen_range(0..=255),
//                 ]
//                 .iter()
//                 .map(|b| format!("{:02x}", b))
//                 .collect::<Vec<_>>()
//                 .join(":");

//                 if models::instance::Instance::_dhcp_get_by_mac(conn, mac)
//             }
//         };

//         let ipv4 = { Ipv4Addr::new(192, 168, 10, rng.gen_range(1..=250)) };

//         models::instance::NewInstance {
//             uuid: args.uuid,
//             label: args.label,
//             itype: args.itype,
//             image: args.image,
//             mac,
//             ipv4,
//             created_by: args.created_by,
//         }
//     }
// }
