use rand::{thread_rng, Rng};
use smart_home_client::udp_client::UdpClient;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let address = "[::]:0";
    let client = match UdpClient::connect(address).await {
        Ok(r) => r,
        Err(_) => panic!("Can't connect to host!"),
    };

    let mut rng = thread_rng();
    let timeout = Duration::from_secs(2);
    loop {
        let temperature = rng.gen_range(20..300);
        let tempr_str = temperature.to_string();

        match client
            .udp
            .send_to(tempr_str.as_bytes(), "localhost:2000")
            .await
        {
            Ok(r) => println!("Sent {} bytes", r),
            Err(e) => println!("Failed while sending datagram: {}", e),
        };

        thread::sleep(timeout);
    }
}
