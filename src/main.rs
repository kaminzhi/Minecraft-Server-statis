mod minecraft;
mod protocol;

use dotenv::dotenv;
use minecraft::fetch_server_status;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let host = env::var("HOST").expect("HOST environment variable not set");
    let port: u16 = env::var("PORT")
        .expect("PORT environment variable not set")
        .parse()
        .expect("PORT must be a valid u16");

    match fetch_server_status(&host, port).await {
        Ok(json_output) => {
            println!("{}", json_output);
        }
        Err(e) => {
            eprintln!("Failed to fetch server status: {}", e);
        }
    }
    Ok(())
}
