use actix_web::HttpResponse;

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
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
