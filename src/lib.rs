use actix_web::{
    dev::Server,
    {App, HttpResponse, HttpServer, Responder, web},
};

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
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
