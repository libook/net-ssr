use net_ssr::command::get_transponder_command;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::task;
use net_ssr::listen_on_port;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Parses command-line arguments
    let matches =get_transponder_command()
        .get_matches();

    let verbose = matches.get_flag("verbose");

    let bind_addr = matches
        .get_one::<String>("bind")
        .map(|s| {
            return if s.contains(":") {
                SocketAddr::from_str(s).expect("Invalid --bind IP address and port")
            } else {
                SocketAddr::from_str(&format!("{}:1030", s))
                    .expect("Invalid --bind IP address")
            }
        })
        .unwrap();

    let listener = task::spawn(async move {
        listen_on_port(bind_addr, handle_request, verbose).await;
    });

    listener.await.unwrap();

    Ok(())
}

/// Asynchronously handles requests received through a UDP socket.
///
/// # Arguments
/// * `received_string` - The received string request.
/// * `addr` - The address from which the request was sent.
/// * `socket` - An Arc-wrapped Mutex-protected UDP socket for sending responses.
async fn handle_request(
    received_string: String,
    addr: SocketAddr,
    socket: Arc<Mutex<UdpSocket>>,
    verbose: bool,
) {
    // Check if the received string is "CQ"
    if received_string == "CQ" {
        if verbose {
            println!("Received from {}", addr.ip());
        } else {
            println!("{}", addr.ip());
        }

        // Construct the response string, including the current hostname
        let response = format!(
            "R {}",
            hostname::get()
                .expect("Hostname error")
                .to_str()
                .expect("UTF-8 error")
        );

        // Format the response address using the IP from the request address and a fixed port 1090
        let respond_addr = format!("{}:1090", addr.ip());

        // Attempt to send the response to the source address, logging the send operation result
        socket
            .lock()
            .await
            .send_to(response.as_bytes(), &respond_addr)
            .await
            .expect("Send error");

        if verbose {
            println!("Sent response to {}", addr);
        }
    }
}
