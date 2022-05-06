mod setup;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let svr_addr = setup::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{svr_addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
