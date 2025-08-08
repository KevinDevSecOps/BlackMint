use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::net::TcpSocket;
use tokio::task;
use std::sync::mpsc;
use clap::Parser;

// Configuración desde CLI
#[derive(Parser)]
#[command(name = "BlackMint Port Scanner")]
#[command(version = "1.0")]
#[command(about = "Escaneo de puertos async en Rust", long_about = None)]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value_t = 1)]
    start_port: u16,

    #[arg(short, long, default_value_t = 1024)]
    end_port: u16,

    #[arg(short, long, default_value_t = 500)]
    concurrency: usize,
}

async fn scan_port(addr: SocketAddr, timeout: u64) -> Option<u16> {
    let socket = TcpSocket::new_v4().unwrap();
    match tokio::time::timeout(
        Duration::from_millis(timeout),
        socket.connect(addr)
    ).await {
        Ok(Ok(_)) => Some(addr.port()),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (tx, rx) = mpsc::channel();

    println!("🔍 Escaneando {} (puertos {}-{})", args.target, args.start_port, args.end_port);

    for port in args.start_port..=args.end_port {
        let tx = tx.clone();
        let target = args.target.clone();
        
        task::spawn(async move {
            let addr = format!("{}:{}", target, port)
                .parse::<SocketAddr>().unwrap();
            
            if let Some(port) = scan_port(addr, 500).await {
                tx.send(port).unwrap();
            }
        });
    }

    drop(tx); // Cerramos el transmitter para que el receiver termine
    let mut open_ports: Vec<u16> = rx.iter().collect();
    open_ports.sort();

    println!("✅ Puertos abiertos:");
    for port in open_ports {
        println!("🟢 {}", port);
    }
}
