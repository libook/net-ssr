use tokio::net::UdpSocket;
use tokio::task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start listening on port 1030
    let listener = task::spawn(async {
        listen_on_port(1030).await;
    });

    // Wait for the listener to finish
    listener.await.unwrap();

    Ok(())
}

async fn listen_on_port(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr).await.unwrap();
    println!("Listening on port {}", port);

    let mut buf = vec![0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        let received = &buf[..len];

        // Judge if message recieved is 'CQ'.
        if received == b"CQ" {
            println!("Received from {}", addr,);

            // Respond with 'R ' and hostname
            let response = format!("R {}", hostname::get().unwrap().to_str().unwrap());
            // Send to same ip with udp 1090 port
            let respond_addr = format!("{}:1090", addr.ip());
            socket
                .send_to(response.as_bytes(), respond_addr)
                .await
                .unwrap();
            println!("Sent response to {}", addr);
        }
    }
}
