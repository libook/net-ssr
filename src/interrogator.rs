use clap::Parser;
use net_ssr::listen_on_port;
use pnet::datalink::{self, NetworkInterface};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;
use tokio::task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address to broadcast to
    #[arg(short, long)]
    broadcast_address: Option<Ipv4Addr>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Parses command-line arguments to get the broadcast address
    let args = Args::parse();
    let broadcast_address = args.broadcast_address.clone();

    // Spawns a listening task on port 1090 to handle incoming data
    let listener = task::spawn(async {
        listen_on_port(1090, |received_string, addr, _| {
            Box::pin(async move {
                // Prints received messages starting with "R "
                let partten = "R ";
                if received_string.starts_with(partten) {
                    // Get hostname from 3rd characters to end
                    let hostname = received_string[partten.len()..].to_string();
                    println!("Received from {}: {}", addr, hostname);
                }
            })
        })
        .await;
    });

    // Vector to hold tasks for broadcasting
    let mut broadcast_tasks = Vec::new();

    // Depending on whether a broadcast IP is provided via args, spawns tasks accordingly
    if let Some(bc_addr) = broadcast_address {
        // Spawns a single broadcast task with the specified address from args
        let task = task::spawn(broadcast_to(bc_addr, 1030));
        broadcast_tasks.push(task);
    } else {
        // Retrieves network interfaces and spawns a task for each with a valid broadcast IP
        let interfaces = datalink::interfaces();
        for interface in interfaces {
            if let Some(broadcast_ip) = get_broadcast_address(&interface) {
                let task = task::spawn(broadcast_to(broadcast_ip, 1030));
                broadcast_tasks.push(task);
            }
        }
    }

    // Awaits completion of all broadcast tasks
    for task in broadcast_tasks {
        task.await.unwrap();
    }

    listener.await.unwrap();

    Ok(())
}

/// Retrieves the broadcast IP address from a given network interface.
fn get_broadcast_address(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    interface.ips.iter().find_map(|ip| {
        if let pnet::ipnetwork::IpNetwork::V4(ipv4) = ip {
            Some(ipv4.broadcast())
        } else {
            None
        }
    })
}

/// Asynchronously sends a broadcast message to a specified IP address and port.
async fn broadcast_to(broadcast_ip: Ipv4Addr, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind socket");
    socket
        .set_broadcast(true)
        .expect("Failed to set broadcast mode");

    let addr = SocketAddrV4::new(broadcast_ip, port);
    let message = b"CQ";

    if let Err(e) = socket.send_to(message, &addr).await {
        eprintln!(
            "Failed to send broadcast to {}:{}. Error: {}",
            broadcast_ip, port, e
        );
    } else {
        println!("Sent broadcast to {}:{}", broadcast_ip, port);
    }
}
