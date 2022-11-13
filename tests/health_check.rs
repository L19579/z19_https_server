use {
    uuid::Uuid,
    sqlx::{
        self,
        PgPool,
        Connection,
        PgConnection,
        Executor,
    },
};

pub struct TestAppConnections{
    pub address: String,
    pub db_conn_pool: PgPool,
}

/// Creates test DB
pub async fn configure_test_db(db_config: &z19_https_server::DatabaseSettings) 
-> PgPool{
    let mut db_conn = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    db_conn
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let db_conn_pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&db_conn_pool)
        .await
        .expect("Failed to migrate the database");
    return db_conn_pool;
}

async fn spawn_app()-> TestAppConnections{
    let mut configuration = z19_https_server::get_configuration()
        .expect("Failed to read config");
    configuration.database.database_name = 
        format!("test_{}", Uuid::new_v4().to_string());
    let db_conn_pool = configure_test_db(&configuration.database)
        .await;

    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random IP");
    let port = listener.local_addr().unwrap().port();

    let server = z19_https_server::run(listener, db_conn_pool.clone()) // creates obj w/o polling
        .expect("Failed to bind address"); 
    let _ = tokio::spawn(server); // start poll future here.
    
    return TestAppConnections{
        address: format!("http://127.0.0.1:{}", port),
        db_conn_pool: db_conn_pool,
    };
}

// run client
#[tokio::test] // <- starts & kills runtimes used by tokio::spawn & others
async fn health_check_works(){
    // start server
    let connections = spawn_app()
        .await;

    // make client request
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", connections.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid__form_data(){
    let connections = spawn_app()
        .await;
    let configuration = z19_https_server::get_configuration()
        .expect("Failed to get config"); 
    /*
    let db_connection_string = configuration.database.connection_string();
    let mut db_connection = sqlx::PgConnection::connect(&db_connection_string)
        .await
        .expect("Failed to connect to Postgres");
    */

    let client = reqwest::Client::new();
    let body = "name=junandre%20paul&email=juju%40emailprovider.com";
    let response = client
        .post(&format!("{}/subscribe", &connections.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&connections.db_conn_pool)
        .await
        .expect("Failed to fetch saved subscription");
    
    assert_eq!(saved.name, "junandre paul");
    assert_eq!(saved.email, "juju@emailprovider.com");
}

#[tokio::test]
async fn subscriber_returns_a_400_when_data_is_missing(){
    let connections = spawn_app()
        .await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=junandre%20paul", "missing the email"),
        ("email=juju%40emailprovider.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &connections.address))
            .header("Content-Type", "Application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400, 
            response.status().as_u16(),
            "The API did not fail with expected 400 Bad Request with payload: {}",
            error_message,
        );
    }
}
