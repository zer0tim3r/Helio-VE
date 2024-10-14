use dotenvy::dotenv;
use helio_pg::{models, PGClient};

mod common;
mod qemu_disk;
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
            result: instances.iter().map(|i| i.uuid.clone()).collect(),
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

        let new_instance = request.into_inner().into();

        models::instance::Instance::_rpc_create(conn, new_instance)
            .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

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

        qemu_kvm::start_instance(conn, args.uuid, args.created_by)
            .map_err(|e| tonic::Status::from_error(e))?;

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
    dotenv()?;

    setup_ipt()?;

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

    println!("App listening on port {}", port);

    server.await?;

    Ok(())
}

use rand::Rng;

impl From<CreateInstanceArgs> for models::instance::NewInstance {
    fn from(args: CreateInstanceArgs) -> Self {
        let mut rng = rand::thread_rng();

        // 첫 번째 바이트의 첫 두 비트를 00으로 설정 (유니캐스트, 로컬)
        let mac = [
            (rng.gen_range(0..=255) & 0b11111100) | 0b00000001, // 첫 번째 바이트 | 유니캐스트 설정
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

        models::instance::NewInstance {
            uuid: args.uuid,
            label: args.label,
            itype: args.itype,
            image: args.image,
            mac,
            ipv4: None,
            created_by: args.created_by,
        }
    }
}
