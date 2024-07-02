use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use std::net::Ipv4Addr;

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

pub fn get_ip_range(start_ip:Ipv4Addr,end_ip:Ipv4Addr)->Vec<Ipv4Addr> {
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

/// Convert an IPv4 address to a u32
fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    let octets = ip.octets();
    (u32::from(octets[0]) << 24) |
    (u32::from(octets[1]) << 16) |
    (u32::from(octets[2]) << 8) |
    u32::from(octets[3])
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
