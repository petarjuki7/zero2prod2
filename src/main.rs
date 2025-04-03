use std::{io::Error, net::TcpListener};

use zero2prod2::startup::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    println!(
        "Listening on port: {}",
        TcpListener::local_addr(&listener).unwrap().port()
    );
    run(listener)?.await
}
