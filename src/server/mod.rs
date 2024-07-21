mod connection;
mod logger;

use log::{ debug, error };
use std::net::Ipv4Addr;

const DEFAULT_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_PORT: u16 = 9595;

pub fn serve() {
    debug!("Starting internal server...");
    let server = connection::listen(DEFAULT_ADDRESS, DEFAULT_PORT);

    if server.is_err() {
        error!("Failed to start server!");
        error!("{}", server.err().unwrap());
    }
}