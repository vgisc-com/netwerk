// client.rs
use tokio::net::UdpSocket;
use raptorq::Encoder;
use rand::{Rng, thread_rng};
use std::{error::Error, time::Instant};
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let dst = "127.0.0.1:55555";
    println!("Client bound to {} and sending to {}", socket.local_addr()?, dst);

    loop {
        let mut data = vec![0u8; 1400];
        thread_rng().fill(&mut data[..]);

        let encoder = Encoder::with_defaults(&data, 1400);
        let packets = encoder.get_encoded_packets(15)
            .into_iter()
            .map(|packet| packet.serialize())
            .collect::<Vec<_>>();

        for packet in packets {
            let start_time = Instant::now();
            socket.send_to(&packet, &dst).await?;
            println!("Packet sent to {} with {} bytes", dst, packet.len());
        

            let mut buffer = [0u8; 1024];
            let result = timeout(Duration::from_secs(5), socket.recv_from(&mut buffer)).await;

            match result {
                Ok(Ok((size, src))) => {
                    if size >= 4 && buffer[..size] == *b"Pong" {
                        let elapsed = start_time.elapsed();
                        println!("Received pong from {} in {}ms", src, elapsed.as_millis());
                    } else {
                        println!("Unexpected message from {}", src);
                    }
                },
                Ok(Err(e)) => println!("Failed to receive pong: {}", e),
                Err(_) => println!("Timeout waiting for pong, retrying..."),
            }
        }
    }
}

