use async_std::net::UdpSocket;
use std::str;

pub async fn test_msg(port: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8081").await?;
    println!("Working on {}", socket.local_addr()?);
    let mut buf: Vec::<u8> = b"Test message!".to_vec();
    buf.resize(1024, 0u8);

    socket.connect(format!("127.0.0.1:{}", port)).await?;

    loop {
        let sent = socket.send(
            &buf
        ).await?;
        println!("==8080==");
        println!("Sent: {}", str::from_utf8(&buf).unwrap());
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
