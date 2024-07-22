use httparse::Header;
use tokio::io::AsyncWriteExt;
use crate::delete;

pub fn process(headers: Vec<Header>) -> &'static [u8]  {
    delete!("Client connection closed, awaiting resync");

    return b"HTTP/1.1 200 OK\r\n\r\n";
}