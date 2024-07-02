use net_ssr::listen_on_port;
use pnet::datalink::{self, NetworkInterface};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;
use tokio::task;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let broadcast_address = parse_args();

    let listener = task::spawn(async {
        listen_on_port(1090, |received_string, addr, _| {
            Box::pin(async move {
                if received_string.starts_with("R ") {
                    println!("Received from {}: {}", addr, received_string);
                }
            })
        }).await;
    });

    let mut broadcast_tasks = vec![];

    if let Some(bc_addr) = broadcast_address {
        // 使用命令行参数指定的广播地址
        let task = task::spawn(broadcast_to(bc_addr, 1030));
        broadcast_tasks.push(task);
    } else {
        // 获取所有网络接口的广播地址
        let interfaces = datalink::interfaces();
        for interface in interfaces {
            if let Some(broadcast_ip) = get_broadcast_address(&interface) {
                let task = task::spawn(broadcast_to(broadcast_ip, 1030));
                broadcast_tasks.push(task);
            }
        }
    }

    for task in broadcast_tasks {
        task.await.unwrap();
    }

    listener.await.unwrap();

    Ok(())
}

fn parse_args() -> Option<Ipv4Addr> {
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == "--bc-addr" {
            return args.next().and_then(|value| value.parse().ok());
        }
    }
    None
}

fn get_broadcast_address(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    interface.ips.iter().find_map(|ip| {
        if let pnet::ipnetwork::IpNetwork::V4(ipv4) = ip {
            Some(ipv4.broadcast())
        } else {
            None
        }
    })
}

async fn broadcast_to(broadcast_ip: Ipv4Addr, port: u16) {
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind socket");
    socket.set_broadcast(true).expect("Failed to set broadcast");

    let addr = SocketAddrV4::new(broadcast_ip, port);
    let message = b"CQ";

    if let Err(e) = socket.send_to(message, &addr).await {
        eprintln!("Failed to send broadcast to {}:{}. Error: {}", broadcast_ip, port, e);
    } else {
        println!("Sent broadcast to {}:{}", broadcast_ip, port);
    }
}
