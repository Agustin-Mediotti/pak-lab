use std::net::TcpListener;

#[tokio::test]
async fn check_test_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    // retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = pak_lab::run(listener).expect("failed to bind address");
    tokio::spawn(server);
    // return app address to caller
    format!("http://127.0.0.1:{port}")
}
