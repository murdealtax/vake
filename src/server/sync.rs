use std::sync::{Arc, Mutex};
use crate::watch::ProjectQueue;

use httparse::Header;
use crate::patch;
use crate::parser::parse::Recipe;

pub fn process(_headers: Vec<Header>, queue: &Arc<Mutex<ProjectQueue>>, recipe: Recipe) -> &'static [u8]  {
    let mut queue = queue.lock().unwrap();
    queue.all(&recipe);

    patch!("Client resync requested, sending repoll packet");

    return b"HTTP/1.1 200 OK\r\n\r\n";
}