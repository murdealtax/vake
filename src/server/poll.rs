use httparse::Header;
use crate::put;

pub fn process(headers: Vec<Header>) -> &'static [u8] {
    put!("Updating client, sending \x1b[93mX\x1b[0m bytes updating \x1b[93mX\x1b[0m files");

    return b"HTTP/1.1 200 OK\r\n\r\n";
}