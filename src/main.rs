use pak_lab::configuration::get_configuration;
use pak_lab::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );
    let listener = TcpListener::bind(address).expect("failed to bind");

    println!(
        "App running on {}:{}",
        configuration.database.host, configuration.application_port
    );
    run(listener, connection_pool)?.await
}
