use actix_web::*;
use dotenvy::dotenv;
use helio_pg::{models, DBPool, PGClient};
use middleware::Logger;

async fn meta_data(
    pool: web::Data<DBPool>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let conn = &mut pool.get()?;

    let instance = models::instance::Instance::_cloudinit_get_by_ipv4(conn, req.connection_info().realip_remote_addr().unwrap_or_default().to_string())?;

    Ok(HttpResponse::Ok().content_type("application/xml").body(format!("instance-id: {}
local-hostname: ip-{}
", instance.uuid, instance.ipv4.replace(".", "-"))))
}

async fn user_data(
    pool: web::Data<DBPool>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let conn = &mut pool.get()?;

    let instance = models::instance::Instance::_cloudinit_get_by_ipv4(conn, req.connection_info().realip_remote_addr().unwrap_or_default().to_string())?;

    Ok(HttpResponse::Ok().content_type("application/xml").body(format!("#cloud-config
hostname: ip-{}

chpasswd:
  list: |
    root:root
  expire: False

runcmd:
  - sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
  - systemctl restart sshd
", instance.ipv4.replace(".", "-"))))
}

async fn vendor_data() -> impl Responder {
    HttpResponse::Ok().content_type("application/xml").body("#cloud-config
cloud-init:
  data_sources:
    - NoCloud
")
}

// async fn user_data_all() -> impl Responder {
//     HttpResponse::Ok().content_type("text/plain").body("instance-id")
// }

// async fn user_data_part(path: web::Path<String>) -> impl Responder {
//     let data_name = path.into_inner();

//     if data_name == "instance-id" {
//         return HttpResponse::Ok().body("example-vm");
//     }

//     HttpResponse::NotFound().content_type("text/html").body(NOT_FOUND_RESPONSE)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8180;

    // dotenv().except("dotenv error");
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client_pg = PGClient::new("postgres://postgres:6e2115148f4ba7e80ca0ce786d17c64f@localhost:5432/helio".to_string());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server = rt::spawn(async move {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new("%a %r %{User-Agent}i"))
                .wrap(middleware::NormalizePath::trim())
                .app_data(web::Data::new(client_pg.0.clone()))
                .route("/meta-data", web::get().to(meta_data))
                .route("/user-data", web::get().to(user_data))
                .route("/vendor-data", web::get().to(vendor_data))
                .default_service(web::route().to(|| async { HttpResponse::NotFound() }))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    });

    println!("HVE cloudinit listening on port {}", port);

    server.await?
}