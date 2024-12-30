use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn write_varint<W: AsyncWriteExt + Unpin>(
    stream: &mut W,
    mut value: i32,
) -> tokio::io::Result<()> {
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        stream.write_all(&[temp]).await?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub async fn read_varint<R: AsyncReadExt + Unpin>(stream: &mut R) -> tokio::io::Result<i32> {
    let mut value = 0;
    let mut position = 0;
    let mut current_byte;

    loop {
        let mut buffer = [0; 1];
        stream.read_exact(&mut buffer).await?;
        current_byte = buffer[0];
        value |= ((current_byte & 0b01111111) as i32) << position;

        if (current_byte & 0b10000000) == 0 {
            break;
        }
        position += 7;
    }
    Ok(value)
}

pub async fn send_handshake(
    stream: &mut TcpStream,
    host: &str,
    port: u16,
) -> tokio::io::Result<()> {
    let mut handshake = Vec::new();
    handshake.push(0x00);
    write_varint(&mut handshake, 763).await?;
    write_varint(&mut handshake, host.len() as i32).await?;
    handshake.extend_from_slice(host.as_bytes());
    handshake.push(((port >> 8) & 0xFF) as u8);
    handshake.push((port & 0xFF) as u8);
    handshake.push(0x01);

    let mut handshake_packet = Vec::new();
    write_varint(&mut handshake_packet, handshake.len() as i32).await?;
    handshake_packet.extend_from_slice(&handshake);
    stream.write_all(&handshake_packet).await?;
    Ok(())
}

pub async fn send_status_request(stream: &mut TcpStream) -> tokio::io::Result<()> {
    let status_request = [0x01, 0x00];
    stream.write_all(&status_request).await?;
    Ok(())
}
