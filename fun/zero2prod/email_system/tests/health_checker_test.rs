//! To integrate test the health checker
#[tokio::test]
async fn health_check_working() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_checker")
        .send()
        .await
        .expect("Failed to execute the given request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = email_system::run().expect("Failed to bind the address");
    let _ = tokio::spawn(server);
}
