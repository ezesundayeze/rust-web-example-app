use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


use log::{ info, log_enabled, Level, warn, error, debug, };

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
    if log_enabled!(Level::Error) {
        debug!("Mary has a little lamb");
        error!("{}", "Its fleece was white as snow");
        info!("{:?}", "And every where that Mary went");
        warn!("{:#?}", "The lamb was sure to go");
    }
    println!("Running on: Host: 27.0.0.1, PORT: 8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
