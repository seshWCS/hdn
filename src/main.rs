use crate::server::run;
use clap::Parser;
use std::net::IpAddr;

mod info;
mod log;
mod server;

#[derive(Parser)]
struct Address {
    #[clap(short, long)]
    ip: IpAddr,
    #[clap(short, long, default_value = "1337")]
    port: u16,
}

fn main() {
    let addr = Address::parse();
    run(addr.ip, addr.port);
}
