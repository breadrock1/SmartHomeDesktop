use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox};
use smart_home_client::tcp_client::*;
use std::sync::{Arc, Mutex};

pub struct HouseWidget {
    connection: Arc<Mutex<TcpClient>>,
    status: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Switch,
    Status,
}

async fn init_connection() -> TcpClient {
    let address = "127.0.0.1:55123";
    let mut client = TcpClient::connect(address).unwrap();
    let _ = client.exec("create s1".to_string()).unwrap();
    let _ = client.recv_result().unwrap();
    client
}

impl Sandbox for HouseWidget {
    type Message = Message;

    fn new() -> Self {
        let client = init_connection();
        let client = Arc::new(Mutex::new(client));
        Self {
            connection: client,
            status: String::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Smart TCP Socket")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Switch => {
                self.status.clear();
                let switch_cmd = "switch s1".to_string();
                let mut stream = self.connection.lock().unwrap();
                self.status = match stream.exec(switch_cmd) {
                    Ok(_) => stream.recv_result().unwrap(),
                    Err(e) => e.to_string(),
                };
            }
            Message::Status => {
                self.status.clear();
                let switch_cmd = "status s1".to_string();
                let mut stream = self.connection.lock().unwrap();
                self.status = match stream.exec(switch_cmd) {
                    Ok(_) => stream.recv_result().unwrap(),
                    Err(e) => e.to_string(),
                };
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button("Switch socket").on_press(Message::Switch),
            button("Show status").on_press(Message::Status),
            text(&self.status).size(100),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
