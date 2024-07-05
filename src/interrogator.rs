use net_ssr::{get_ip_range, listen_on_port};
use pnet::datalink::{self, NetworkInterface};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use tokio::net::UdpSocket;
use tokio::task;
use net_ssr_shared::get_interrogator_command;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Parses command-line arguments
    let matches =get_interrogator_command()
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let start_ip = matches
        .get_one::<String>("start")
        .map(|s| Ipv4Addr::from_str(s).expect("Invalid --start IP address"));
    let to_ip = matches
        .get_one::<String>("to")
        .map(|s| Ipv4Addr::from_str(s).expect("Invalid --to IP address"));
    let bind_addr = matches
        .get_one::<String>("bind")
        .map(|s| {
            return if s.contains(":") {
                SocketAddr::from_str(s).expect("Invalid --bind IP address and port")
            } else {
                SocketAddr::from_str(&format!("{}:1090", s))
                    .expect("Invalid --bind IP address")
            }
        })
        .unwrap();
    let port = matches
        .get_one::<String>("port")
        .map(|s| u16::from_str(s).expect("Invalid --port number"))
        .unwrap();

    if let (Some(start), Some(to)) = (start_ip, to_ip) {
        if to <= start {
            panic!("Error: --to must be greater than --start");
        }
    } else if to_ip.is_some() && start_ip.is_none() {
        panic!("Error: --to specified without --start");
    }

    // Spawns a listening task on port 1090 to handle incoming data
    let listener = task::spawn(async move {
        listen_on_port(
            bind_addr,
            |received_string: String, addr, _, verbose| {
                Box::pin(async move {
                    // Prints received messages starting with "R "
                    let pattern = "R ";
                    if received_string.starts_with(pattern) {
                        // Get hostname from 3rd characters to end
                        let hostname = received_string[pattern.len()..].to_string();
                        if verbose {
                            println!("Received from IP: {}, hostname: {}", addr.ip(), hostname);
                        } else {
                            println!("{}\t{}", addr.ip(), hostname);
                        }
                    }
                })
            },
            verbose,
        )
        .await;
    });

    // Vector to hold tasks for broadcasting
    let mut broadcast_tasks = Vec::new();

    // Depending on whether a broadcast IP is provided via args, spawns tasks accordingly
    if let Some(broadcast_address_start) = start_ip {
        if let Some(broadcast_address_to) = to_ip {
            for broadcast_address in get_ip_range(broadcast_address_start, broadcast_address_to) {
                let task = task::spawn(broadcast_to(broadcast_address, port, verbose));
                broadcast_tasks.push(task);
            }
        } else {
            // Spawns a single broadcast task with the specified address from --start
            let task = task::spawn(broadcast_to(broadcast_address_start, port, verbose));
            broadcast_tasks.push(task);
        }
    } else {
        // Retrieves network interfaces and spawns a task for each with a valid broadcast IP
        let interfaces = datalink::interfaces();
        for interface in interfaces {
            if let Some(broadcast_ip) = get_broadcast_address(&interface) {
                let task = task::spawn(broadcast_to(broadcast_ip, 1030, verbose));
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
async fn broadcast_to(broadcast_ip: Ipv4Addr, port: u16, verbose: bool) {
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
        if verbose {
            println!("Sent broadcast to {}:{}", broadcast_ip, port);
        }
    }
}
