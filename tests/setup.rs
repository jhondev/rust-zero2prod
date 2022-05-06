use std::net::TcpListener;

// Launch our application in the background
pub fn spawn_app() -> String {
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
