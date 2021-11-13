use async_std::net::UdpSocket;
use futures::executor::block_on;
use std::{thread, str};
mod peer;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!("Listening on {}", socket.local_addr()?);

    let mut buf = vec![0u8; 1024];

    thread::spawn(move || {
        block_on(peer::test_msg("8080"));
    });

    loop {
        let (recv, peer) = socket.recv_from(&mut buf).await?;
        let sent = socket.send_to(&buf[..recv], &peer).await?;
        println!("==8081==");
        println!("Received: {}", str::from_utf8(&buf).unwrap());
        println!("Sent {} out of {} bytes to {}", sent, recv, peer);
    }
}

