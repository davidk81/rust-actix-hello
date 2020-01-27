use actix_web::{App, HttpServer};
use listenfd::ListenFd;

mod index;
mod hello;
mod json;
mod params;
mod extractor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new()
        .service(index::get)
        .service(hello::get)
        .service(json::get)
        .service(params::get)
        .service(extractor::post)
        );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
