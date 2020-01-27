// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use listenfd::ListenFd;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new()
        .service(index)
        .service(index3)
        );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}

#[get("/hello")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}

#[get("/")]
async fn index3(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
