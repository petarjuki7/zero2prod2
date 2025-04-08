use std::io::stdout;
use std::{io::Error, net::TcpListener};

use sqlx::PgPool;

use zero2prod2::configuration::get_configuration;
use zero2prod2::startup::run;
use zero2prod2::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.connect_options());

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

    run(listener, connection_pool)?.await
}
