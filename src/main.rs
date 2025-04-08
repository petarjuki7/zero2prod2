use std::io::stdout;
use std::{io::Error, net::TcpListener};

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod2::configuration::get_configuration;
use zero2prod2::startup::run;
use zero2prod2::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

    run(listener, connection_pool)?.await
}
