use pnet::datalink::{self, NetworkInterface};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;
use tokio::task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get all network interfaces
    let interfaces = datalink::interfaces();

    // Start listening on port 1090
    let listener = task::spawn(async {
        listen_on_port(1090).await;
    });

    // Broadcast to all addresses
    let mut broadcast_tasks = vec![];
    for interface in interfaces {
        if let Some(broadcast_ip) = get_broadcast_address(&interface) {
            let task = task::spawn(async move {
                broadcast_to(&broadcast_ip, 1030).await;
            });
            broadcast_tasks.push(task);
        }
    }

    for task in broadcast_tasks {
        task.await.unwrap();
    }

    // Wait for the listener to finish
    listener.await.unwrap();

    Ok(())
}

fn get_broadcast_address(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    for ip in &interface.ips {
        if let pnet::ipnetwork::IpNetwork::V4(ipv4) = ip {
            let broadcast = ipv4.broadcast();
            return Some(broadcast);
        }
    }
    None
}

async fn broadcast_to(broadcast_ip: &Ipv4Addr, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.set_broadcast(true).unwrap();

    let addr = SocketAddrV4::new(*broadcast_ip, port);
    let socket_addr: SocketAddr = addr.into();
    let message = b"CQ";

    match socket.send_to(message, &socket_addr).await {
        Ok(_) => println!("Sent broadcast to {}:{}", broadcast_ip, port),
        Err(e) => eprintln!(
            "Failed to send broadcast to {}:{}. Error: {}",
            broadcast_ip, port, e
        ),
    }
}

async fn listen_on_port(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr).await.unwrap();
    println!("Listening on port {}", port);

    let mut buf = vec![0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        let received_buf = &buf[..len];
        let received_str = std::str::from_utf8(received_buf).unwrap();

        // Check if the received data is message start with 'R '.
        if received_str.starts_with("R ") {
            // Print message after "R " and IP
            let message = received_str.split_at(2).1;
            println!("Received hostname: {} from {}", message, addr);
        }
    }
}
