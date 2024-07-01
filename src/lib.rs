use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

pub async fn listen_on_port<F, Fut>(port: u16, custom_code: F)
where
    F: Fn(Vec<u8>, std::net::SocketAddr, Arc<Mutex<UdpSocket>>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    let addr = format!("0.0.0.0:{}", port);
    let socket = Arc::new(Mutex::new(UdpSocket::bind(&addr).await.unwrap()));
    println!("Listening on port {}", port);

    let mut buf = vec![0; 1024];
    loop {
        let (len, addr) = socket.lock().await.recv_from(&mut buf).await.unwrap();
        let received = buf[..len].to_vec();

        // Execute custom code
        custom_code(received, addr, Arc::clone(&socket)).await;
    }
}
