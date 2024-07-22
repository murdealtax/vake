use httparse::Header;
use crate::patch;

pub fn process(headers: Vec<Header>) -> &'static [u8]  {
    patch!("Client resync requested, sending repoll packet");

    return b"HTTP/1.1 200 OK\r\n\r\n";
}