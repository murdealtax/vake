use std::net::Ipv4Addr;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt}, net::TcpListener};

use log::{ info, debug, warn };

use crate::server::poll;
use crate::server::close;
use crate::server::sync;

#[tokio::main]
pub async fn listen( address: Ipv4Addr, port: u16 ) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind((address, port)).await?;
    info!("Server is listening at {address}:{port}");

    loop {
        let (socket, address) = listener.accept().await?;

        tokio::spawn(async move {
            connection_handler(socket, address).await;
        });
    }
}

async fn connection_handler(mut socket: tokio::net::TcpStream, address: std::net::SocketAddr) {
    let mut reader = tokio::io::BufReader::new(&mut socket);
    let mut buffer = String::new();

    debug!("Incoming request from {}", address);

    let mut line_count = 0;
    while reader.read_line(&mut buffer).await.unwrap() > 0 {
        if buffer.ends_with("\n\n") || buffer.ends_with("\n\r\n") {
            break;
        }

        line_count += 1;
    }

    debug!("Received header of size {} bytes with {} lines", buffer.len(), line_count);

    let bytes = buffer.as_bytes();
    let mut headers = vec![httparse::EMPTY_HEADER; line_count];
    let mut req = httparse::Request::new(&mut headers);
    let result = req.parse(&bytes).unwrap();

    if result.is_complete() {
        debug!("Received a complete request from {}", address);
        match req.method.unwrap() {
            "PUT" => {
                poll::process(headers);
            },
            "DELETE" => {
                close::process(headers);
            },
            "PATCH" => {
                sync::process(headers);
            },
            _ => {
                warn!("Received an unsupported request from {}", address);
                socket.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").await.unwrap();
            }
        }

    } else {
        warn!("Client sent an incomplete request, closing connection");
    }
}