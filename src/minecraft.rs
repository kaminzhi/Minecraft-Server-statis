use crate::protocol::read_varint;
use serde::Deserialize;
use serde_json::Value;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct MinecraftResponse {
    pub description: Value,
    pub players: Players,
    pub version: Version,
    pub favicon: Option<String>,
}

#[derive(Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
}

#[derive(Deserialize)]
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
    Ok(String::from_utf8(response_data).expect("Invalid UTF-8 response"))
}

pub fn parse_response(response: &str) -> Result<MinecraftResponse, serde_json::Error> {
    serde_json::from_str(response)
}
