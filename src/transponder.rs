use net_ssr::listen_on_port;
use tokio::task;
use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = task::spawn(async {
        listen_on_port(1030, handle_request).await;
    });

    listener.await.unwrap();

    Ok(())
}

async fn handle_request(received_string: String, addr: std::net::SocketAddr, socket: Arc<Mutex<UdpSocket>>) {
    if received_string == "CQ" {
        println!("Received from {}", addr);

        let response = format!("R {}", hostname::get().expect("Hostname error").to_str().expect("UTF-8 error"));
        let respond_addr = format!("{}:1090", addr.ip());
        socket.lock().await.send_to(response.as_bytes(), &respond_addr).await.expect("Send error");
        println!("Sent response to {}", addr);
    }
}
