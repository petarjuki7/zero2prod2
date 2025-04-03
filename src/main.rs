use std::{io::Error, net::TcpListener};

use zero2prod2::configuration::get_configuration;
use zero2prod2::startup::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    println!(
        "Listening on port: {}",
        TcpListener::local_addr(&listener).unwrap().port()
    );
    run(listener)?.await
}
