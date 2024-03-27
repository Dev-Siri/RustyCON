use std::{env, error::Error};

use packet::{max_packet_size_bytes, str_to_packet_type, Packet};
use tokio::{io::{stdin, AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt}, net::TcpStream};

mod packet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let peer_connection_url = args.get(1).ok_or("Connection URL is required to connect to peer")?;

    let stream = TcpStream::connect(peer_connection_url).await?;
    let (mut read, mut write) = stream.into_split();

    tokio::spawn(async move {
        let mut line = String::new();
        let mut reader = BufReader::new(stdin());

        loop {
            reader.read_line(&mut line).await.unwrap();

            let mut parts = line.split_whitespace();
        
            let id: i32 = parts.next().unwrap().parse().unwrap();
            
            let packet_type_str = parts.next().unwrap();
            let packet_type = str_to_packet_type(packet_type_str).unwrap();

            let body = parts.collect::<Vec<&str>>().join(" ");

            let packet = Packet::new(packet_type, id, body).unwrap();

            write.write_all(&Packet::to_bytes(packet).unwrap()).await.unwrap();
            line.clear();
        }
    });

    loop {
        let mut packet_buf: Vec<u8> = vec![];
        let mut temp_buf = vec![0; max_packet_size_bytes()];
        let bytes = read.read(&mut temp_buf).await?;

        if bytes == 0 {
            println!("Connection closed by remote");
            break;
        }

        packet_buf.resize(packet_buf.len() + bytes, 0);
        packet_buf.copy_from_slice(&temp_buf[..bytes]);

        let bytes_as_packet = Packet::from_bytes(packet_buf).expect("Failed to convert byte response to Packet");

        println!("{:?}", bytes_as_packet);
    }

    Ok(())
}
