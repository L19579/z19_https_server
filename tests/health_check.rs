/// Spawns runtime for server, with caller's lifetime; returns ip:port as string
fn spawn_app()-> String{
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random IP");
    let port = listener.local_addr().unwrap().port();
    let server = z19_https_server::run(listener) // creates obj w/o polling
        .expect("Failed to bind address"); 
    let _ = tokio::spawn(server); // start poll future here.
    
    return format!("http://127.0.0.1:{}", port);
}

// run client
#[tokio::test] // <- starts & kills runtimes used by tokio::spawn & others
async fn health_check_works(){
    // start server
    let address = spawn_app();

    // make client request
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid__form_data(){
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=junandre%20paul&email=juju%40emailprovider.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test] // 400 returns not set atm.
async fn subscriber_returns_a_400_when_data_is_missing(){
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=junandre%20paul", "missing the email"),
        ("email=juju%40emailprovider.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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
