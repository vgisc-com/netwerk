// server.rs
use tokio::net::UdpSocket;
use raptorq::{Decoder, EncodingPacket, ObjectTransmissionInformation};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:55555").await?;
    println!("Server listening on {}", socket.local_addr()?);
    
    // https://docs.rs/raptorq/latest/raptorq/struct.ObjectTransmissionInformation.html
    let ot = ObjectTransmissionInformation::new(
    // Note: correct these comments 
        1396, // 1400 - 4 (UDP header)  
        150,  // 150 bytes of overhead  
        1,    // 1 symbol size
        1,    // 1 symbol count
        1     // 0 encoding symbol ID
    );

    let mut decoder = Decoder::new(ot);
    let mut buffer = vec![0u8; 2048];
    let mut data = Vec::new();

    while let Ok((size, src)) = socket.recv_from(&mut buffer).await {
        println!("Received {} bytes from {}: {:?}", size, src, &buffer[..size]);
        data.extend_from_slice(&buffer[..size]);

        while data.len() >= size {
            let packet = EncodingPacket::deserialize(&buffer[..size]);
            data.drain(..size);

            if let Some(data) = decoder.decode(packet) {
                println!("Decoded {} bytes, sending Pong to {}", data.len(), src);
                let pong_msg = "Pong".as_bytes();
                if let Err(e) = socket.send_to(pong_msg, &src).await {
                    println!("Failed to send Pong to {}: {}", src, e);
                } else {
                    println!("Pong sent successfully to {}", src);
                }
            }
        }
    }
    Ok(())
}
