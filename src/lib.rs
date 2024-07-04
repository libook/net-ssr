use std::net::{Ipv4Addr,SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

/// Listen on a specific port and call the provided custom code when a message is received.
pub async fn listen_on_port<F, Fut>(addr:SocketAddr, custom_code: F, verbose: bool)
where
    F: Fn(String, SocketAddr, Arc<Mutex<UdpSocket>>, bool) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    // Bind to the specified address and wrap the socket in an Arc and Mutex for thread-safe access.
    let socket = Arc::new(Mutex::new(
        UdpSocket::bind(&addr).await.expect("Failed to bind socket"),
    ));

    if verbose {
        println!("Listening on {}", addr);
    }

    // Buffer for receiving data.
    let mut buf = vec![0; 1024];
    loop {
        // Receive data from any sender and get the length of the message and the sender's address.
        let (len, addr) = socket
            .lock()
            .await
            .recv_from(&mut buf)
            .await
            .expect("Failed to receive data");
        // Convert the received bytes into an owned string, ignoring non-UTF8 data.
        let received_string = String::from_utf8_lossy(&buf[..len]).into_owned();

        // Invoke the user-provided custom function with the received data and the cloned socket Arc.
        custom_code(received_string, addr, Arc::clone(&socket), verbose).await;
    }
}

pub fn get_ip_range(start_ip: Ipv4Addr, end_ip: Ipv4Addr) -> Vec<Ipv4Addr> {
    // Convert the start and end IPv4 addresses to u32
    let mut current_u32 = ipv4_to_u32(start_ip);
    let end_u32 = ipv4_to_u32(end_ip);

    let mut ip_list = Vec::new();

    while current_u32 <= end_u32 {
        // Convert the current u32 to an IPv4 address
        let ip = u32_to_ipv4(current_u32);
        ip_list.push(ip);
        current_u32 += 1;
    }

    ip_list
}

/// Convert an IPv4 address to an u32
fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    let octets = ip.octets();
    (u32::from(octets[0]) << 24)
        | (u32::from(octets[1]) << 16)
        | (u32::from(octets[2]) << 8)
        | u32::from(octets[3])
}

/// Convert a u32 to an IPv4 address
fn u32_to_ipv4(n: u32) -> Ipv4Addr {
    Ipv4Addr::new(
        ((n >> 24) & 0xFF) as u8,
        ((n >> 16) & 0xFF) as u8,
        ((n >> 8) & 0xFF) as u8,
        (n & 0xFF) as u8,
    )
}
