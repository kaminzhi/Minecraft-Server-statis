mod minecraft;
mod protocol;

use dotenv::dotenv;
use minecraft::{parse_response, read_server_response};
use protocol::{send_handshake, send_status_request};
use std::env;
use std::io::{self, Write};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    dotenv().ok();

    let server_address =
        env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS environment variable not set");
    let host = env::var("HOST").expect("HOST environment variable not set");
    let port: u16 = env::var("PORT")
        .expect("PORT environment variable not set")
        .parse()
        .expect("PORT must be a valid u16");

    loop {
        print!("Enter command (exit to quit):");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        } else if input == "help" {
            println!("  get status  - Fetch server status");
            println!("  exit        - Exit the program");
        } else if input == "get status" {
            let mut stream = TcpStream::connect(&server_address)
                .await
                .expect("Unable to connect to the server");

            send_handshake(&mut stream, &host, port).await?;
            send_status_request(&mut stream).await?;

            let response = read_server_response(&mut stream).await?;
            match parse_response(&response) {
                Ok(data) => {
                    println!("Description: {}", data.description);
                    println!("Players: {}/{}", data.players.online, data.players.max);
                    println!(
                        "Version: {} (protocol {})",
                        data.version.name, data.version.protocol
                    );
                    if let Some(favicon_data) = data.favicon {
                        if let Some(index) = favicon_data.find(",") {
                            let actual_base64 = &favicon_data[index + 1..];
                            println!("Favicon base64 data: {}", actual_base64);
                        } else {
                            println!("Favicon base64 data: {}", favicon_data);
                        }
                    }
                }
                Err(e) => {
                    println!("Error parsing server response: {}", e);
                }
            }
        } else {
            println!("Unknown command: {}", input);
        }
    }

    println!("Exiting...");

    Ok(())
}
