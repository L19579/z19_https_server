use {
    env_logger::Env,
    z19_https_server::{
        run, get_configuration,
        Connection, PgPool
    },
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //decl is a bit bizarre, review.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); 

    let configuration = get_configuration()
        .expect("Failed to read config");
    let db_conn_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    return run(listener, db_conn_pool)?.await;
}
