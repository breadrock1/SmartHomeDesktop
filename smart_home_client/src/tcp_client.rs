use crate::errors::{ConnectResult, RecvError, RecvResult};
use crate::errors::{SendError, SendResult};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use tokio::io;

pub struct TcpClient {
    tcp: TcpStream,
}

impl TcpClient {
    pub fn connect<T: ToSocketAddrs>(address: T) -> ConnectResult<Self> {
        let stream = TcpStream::connect(address).expect("Can't establish connect");
        Ok(Self { tcp: stream })
    }

    pub fn disconnect(&mut self) -> io::Result<()> {
        self.tcp.shutdown(Shutdown::Both)
    }

    pub fn exec(&mut self, command: String) -> SendResult {
        send_request(&mut self.tcp, command)
    }

    pub fn recv_result(&mut self) -> RecvResult {
        match recv_request(&mut self.tcp) {
            Ok(r) => Ok(format!("Result: {}", r)),
            Err(e) => Err(e),
        }
    }
}

pub fn send_request(stream: &mut TcpStream, str_data: String) -> SendResult {
    let output_buffer = str_data.as_bytes();
    let buffer_length = output_buffer.len() as u32;
    let result = stream.write_all(&buffer_length.to_be_bytes());
    if result.is_err() {
        let error = result.err().unwrap();
        return Err(SendError::Io(error));
    }
    match stream.write_all(output_buffer) {
        Ok(_) => Ok(str_data),
        Err(e) => Err(SendError::Io(e)),
    }
}

pub fn recv_request(stream: &mut TcpStream) -> RecvResult {
    let mut input_buffer = [0; 4];
    let _ = stream
        .read(&mut input_buffer)
        .map_err(|e| RecvError::ReadData(e.to_string()));
    let length = u32::from_be_bytes(input_buffer);

    let mut input_buffer = vec![0; length as _];
    let _ = stream
        .read(&mut input_buffer)
        .map_err(|e| RecvError::ReadData(e.to_string()));

    match String::from_utf8(input_buffer) {
        Ok(s) => Ok(s),
        Err(e) => Err(RecvError::ReadData(e.to_string())),
    }
}
