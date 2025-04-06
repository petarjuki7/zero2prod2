use std::{io::Error, net::TcpListener};

use sqlx::PgPool;
use zero2prod2::configuration::get_configuration;
use zero2prod2::startup::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    println!(
        "Listening on port: {}",
        TcpListener::local_addr(&listener).unwrap().port()
    );
    run(listener, connection_pool)?.await
}
