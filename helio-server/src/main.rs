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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_pg = PGClient::new(database_url);

    let server = tokio::spawn(async move {
        let addr = "0.0.0.0:8080".parse().unwrap();
        let rpc_service = RPC::new(client_pg);

        tonic::transport::Server::builder()
            .add_service(HelioServer::new(rpc_service))
            .serve(addr)
            .await
            .unwrap()
    });

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
