#[tokio::test]
async fn check_test_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let res = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() {
    let server = pak_lab::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}
