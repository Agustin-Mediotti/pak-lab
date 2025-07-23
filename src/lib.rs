use actix_web::{
    dev::Server,
    {App, HttpResponse, HttpServer, web},
};

use std::net::TcpListener;

/// Health check endpoint for monitoring the server's status.
///
/// This asynchronous handler responds with HTTP 200 OK, indicating that
/// the server is running and reachable. It is typically used by
/// load balancers, uptime monitoring tools, or orchestration systems
/// (like Kubernetes) to verify that the service is healthy.
///
/// # Returns
///
/// An HTTP 200 OK response with an empty body.
///
/// # Example
///
/// ```http
/// GET /health_check
/// HTTP/1.1 200 OK
/// ```
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
