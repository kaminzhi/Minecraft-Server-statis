use tokio::io;
use tokio::net::TcpStream;
mod protocol;
use protocol::{read_varint, send_handshake, send_status_request};

#[tokio::main]
async fn main() -> io::Result<()> {
    let host = "mc.hypixel.net";
    let port = 25565;

    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(addr).await?;

    println!("Connected to the server!");

    println!("Sending handshake...");
    send_handshake(&mut stream, host, port).await?;
    println!("Handshake sent.");

    println!("Sending status request...");
    send_status_request(&mut stream).await?;
    println!("Status request sent.");

    println!("Waiting for response...");
    let response = read_varint(&mut stream).await?;
    println!("Response received: {}", response);

    Ok(())
}
