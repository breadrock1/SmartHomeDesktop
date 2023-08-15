use crate::errors::ConnectResult;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct UdpClient {
    pub udp: UdpSocket,
}

impl UdpClient {
    pub async fn connect<T: ToSocketAddrs>(address: T) -> ConnectResult<Self> {
        let stream = UdpSocket::bind(address)
            .await
            .expect("Can't establish connect");
        Ok(Self { udp: stream })
    }

    pub async fn send(&mut self, value: i32) {
        let msg_str = value.to_string();
        let msg_bytes = msg_str.as_bytes();
        let _ = self.udp.send(msg_bytes).await;
    }
}
