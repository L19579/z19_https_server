// run server
async fn spawn_app() -> std::io::Result<()>{
    z19_https_server::run().await
}

// run client
#[tokio::test]
async fn health_check_works(){
    spawn_app().await.expect("Failed to spawn the app");
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

