use smart_home_client::tcp_client::TcpClient;
use std::io::stdin;

fn main() {
    let address = "127.0.0.1:55123";
    let client = TcpClient::connect(address);
    if client.is_err() {
        panic!("Can't connect to host!");
    }

    let stdin_handler = stdin();
    let mut client = client.unwrap();
    loop {
        let mut input = String::new();
        let result = stdin_handler.read_line(&mut input);
        if result.is_err() {
            let err = result.err().unwrap();
            println!("Failed while read stdin: {}", err);
            continue;
        }

        client.exec(input).unwrap();
        println!("{}", client.recv_result().unwrap());
    }
}
