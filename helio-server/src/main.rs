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
}

fn setup_ipt() -> Result<(), Box<dyn std::error::Error>> {
    let ipt = iptables::new(false).unwrap();

    let check_add = |table: &str, chain: &str, rule: &str| -> Result<(), Box<dyn std::error::Error>>{
        if !ipt.exists(table, chain, rule)? {
            ipt.append_unique(table, chain, rule)?;
        }

        Ok(())
    };

    ipt.set_policy("filter", "FORWARD", "DROP")?;
    check_add("filter", "FORWARD", "! -s 192.168.10.0/24 -j ACCEPT")?;
    check_add("nat", "PREROUTING", "-d 169.254.169.254 -p tcp --dport 80 -j DNAT --to-destination 192.168.10.254:8180")?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

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

impl From<CreateInstanceArgs> for models::instance::NewInstance {
    fn from(args: CreateInstanceArgs) -> Self {
        models::instance::NewInstance {
            uuid: args.uuid,
            label: args.label,
            itype: args.itype,
            image: args.image,
            created_by: args.created_by,
        }
    }
}
