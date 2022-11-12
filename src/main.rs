use z19_https_server::{
    run, get_configuration,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration()
        .expect("Failed to read config");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    return run(listener)?.await;
}
