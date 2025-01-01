use crate::protocol::{read_varint, send_handshake, send_status_request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub online: u32,
    pub max: u32,
    pub sample: Option<Vec<Player>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Description {
    Text(String),
    Object { text: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftResponse {
    pub description: Description, // Adjusted to match possible nested JSON
    pub players: Players,
    pub version: Version,
    pub favicon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

pub async fn read_server_response<R: AsyncReadExt + Unpin>(
    stream: &mut R,
) -> tokio::io::Result<String> {
    let _packet_length = read_varint(stream).await?;
    let _packet_id = read_varint(stream).await?;
    let string_length = read_varint(stream).await?;
    let mut response_data = vec![0; string_length as usize];
    stream.read_exact(&mut response_data).await?;
    String::from_utf8(response_data).map_err(|e| {
        tokio::io::Error::new(
            tokio::io::ErrorKind::InvalidData,
            format!("UTF-8 error: {}", e),
        )
    })
}

pub fn parse_response(response: &str) -> Result<MinecraftResponse, serde_json::Error> {
    serde_json::from_str(response)
}

pub async fn fetch_server_status(
    host: &str,
    port: u16,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect((host, port)).await?;
    send_handshake(&mut stream, host, port).await?;
    send_status_request(&mut stream).await?;

    let response = read_server_response(&mut stream).await?;
    let data: MinecraftResponse = parse_response(&response)?;

    let description_text = match data.description {
        Description::Text(text) => text,
        Description::Object { text } => text,
    };

    let favicon_cleaned = if let Some(favicon_data) = data.favicon {
        if let Some(index) = favicon_data.find(",") {
            let actual_base64 = &favicon_data[index + 1..];
            Some(actual_base64.to_string())
        } else {
            Some(favicon_data)
        }
    } else {
        None
    };

    let players_sample =
        if data.players.online > 0 && data.players.sample.as_ref().map_or(true, |s| s.is_empty()) {
            Some(vec!["Not Support".to_string()])
        } else {
            data.players
                .sample
                .map(|s| s.into_iter().map(|p| p.name).collect::<Vec<_>>())
        };

    let result = json!({
        "host": host,
        "port": port,
        "description": description_text,
        "players": {
            "online": data.players.online,
            "max": data.players.max,
            "members": players_sample,
        },
        "version": data.version.name,
        "protocol": data.version.protocol,
        "favicon": favicon_cleaned,
    });

    // println!("Raw server response: {}", response);
    Ok(serde_json::to_string_pretty(&result)?)
}
