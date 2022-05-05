// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

use std::net::TcpListener;

// Launch our application in the background ~somehow~
fn spawn_app() -> String {
    let svr_addr = "127.0.0.1";
    let listener = TcpListener::bind(format!("{svr_addr}:0")).expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    format!("http://{svr_addr}:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let server_addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    println!("{server_addr}/health_check");
    let response = client
        .get(format!("{server_addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
