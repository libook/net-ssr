use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

/// Listen on a specific port and call the provided custom code when a message is received.
pub async fn listen_on_port<F, Fut>(port: u16, custom_code: F)
where
    F: Fn(String, std::net::SocketAddr, Arc<Mutex<UdpSocket>>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    let addr = format!("0.0.0.0:{}", port);
    // Bind to the specified address and wrap the socket in an Arc and Mutex for thread-safe access.
    let socket = Arc::new(Mutex::new(UdpSocket::bind(&addr).await.expect("Failed to bind socket")));
    println!("Listening on port {}", port);

    // Buffer for receiving data.
    let mut buf = vec![0; 1024];
    loop {
        // Receive data from any sender and get the length of the message and the sender's address.
        let (len, addr) = socket.lock().await.recv_from(&mut buf).await.expect("Failed to receive data");
        // Convert the received bytes into a owned string, ignoring non-UTF8 data.
        let received_string = String::from_utf8_lossy(&buf[..len]).into_owned();

        // Invoke the user-provided custom function with the received data and the cloned socket Arc.
        custom_code(received_string, addr, Arc::clone(&socket)).await;
    }
}
