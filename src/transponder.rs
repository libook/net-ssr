use net_ssr::listen_on_port;
use tokio::task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start listening on port 1030 with custom code
    let listener = task::spawn(async {
        listen_on_port(1030, |received_string, addr, socket| {
            Box::pin(async move {
                if received_string == "CQ" {
                    println!("Received from {}", addr);

                    // Respond with 'R ' and hostname
                    let response = format!("R {}", hostname::get().unwrap().to_str().unwrap());
                    // Send to same ip with udp 1090 port
                    let respond_addr = format!("{}:1090", addr.ip());
                    socket
                        .lock()
                        .await
                        .send_to(response.as_bytes(), respond_addr)
                        .await
                        .unwrap();
                    println!("Sent response to {}", addr);
                }
            })
        })
        .await;
    });

    // Wait for the listener to finish
    listener.await.unwrap();

    Ok(())
}
