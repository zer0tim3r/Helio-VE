use actix_web::*;

const NOT_FOUND_RESPONSE: &str = "<?xml version=\"1.0\" encoding=\"iso-8859-1\"?>
<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\"
	\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">
 <head>
  <title>404 - Not Found</title>
 </head>
 <body>
  <h1>404 - Not Found</h1>
 </body>
</html>";

const BAD_REQUEST_RESPONSE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">
   <head>
      <title>400 - Bad Request</title>
   </head>
   <body>
      <h1>400 - Bad Request</h1>
   </body>
</html>";

const UNAUTHORIZED_RESPONSE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\" lang=\"en\">
   <head>
      <title>401 - Unauthorized</title>
   </head>
   <body>
      <h1>401 - Unauthorized</h1>
   </body>
</html>";

async fn root_all() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("latest")
}

async fn latest_all() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("meta-data")
}

async fn meta_data_all() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("instance-id")
}

async fn meta_data_part(path: web::Path<String>) -> impl Responder {
    let data_name = path.into_inner();

    if data_name == "instance-id" {
        return HttpResponse::Ok().body("example-vm");
    }

    HttpResponse::NotFound().content_type("text/html").body(NOT_FOUND_RESPONSE)
}

async fn user_data_all() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").body("instance-id")
}

async fn user_data_part(path: web::Path<String>) -> impl Responder {
    let data_name = path.into_inner();

    if data_name == "instance-id" {
        return HttpResponse::Ok().body("example-vm");
    }

    HttpResponse::NotFound().content_type("text/html").body(NOT_FOUND_RESPONSE)
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

    let server = rt::spawn(async move {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::NormalizePath::trim())
                .route("/", web::get().to(root_all))
                .route("/latest", web::get().to(latest_all))
                .route("/latest/meta-data", web::get().to(meta_data_all))
                .route("/latest/meta-data/{data_name}", web::get().to(meta_data_part))
                .default_service(web::route().to(|| async { HttpResponse::NotFound().content_type("text/html").body(NOT_FOUND_RESPONSE) }))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    });

    println!("App listening on port {}", port);

    server.await?
}