use pak_lab::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind");
    let port = listener.local_addr().unwrap().port();

    println!("App running on: 127.0.0.1:{port}");
    run(listener)?.await
}
