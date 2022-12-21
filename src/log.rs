use crate::info::Request;
use chrono::Utc;
use std::net::IpAddr;

#[derive(Debug)]
pub enum Log {
    RequestType(Request),
    ConnectionEstablished,
    ConnectionLost,
}

impl Log {
    pub fn print(&self, ip: IpAddr, size: usize) {
        print!(
            "{}, [{}] ",
            ip.to_string(),
            Utc::now().format("%d/%b/%Y:%T %z")
        );
        match self {
            Log::RequestType(request) => {
                print!("Received request to ");
                match request {
                    Request::Store { key, hash } => {
                        print!("write new value \"{}\" by key \"{}\". ", &hash, &key);
                    }
                    Request::Load { key } => {
                        print!("get value by key \"{}\". ", &key);
                    }
                }
            }
            Log::ConnectionEstablished => {
                print!("Connection established. ");
            }
            Log::ConnectionLost => {
                print!("Connection lost. ");
            }
        }
        print!("Storage size: {}\n", size);
    }
}
