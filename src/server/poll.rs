use std::sync::{Arc, Mutex};

use httparse::Header;
use crate::parser::parse::Recipe;
use crate::put;
use crate::watch::ProjectQueue;

pub fn process(_headers: Vec<Header>, queue: &Arc<Mutex<ProjectQueue>>, recipe: Recipe) -> String {
    let mut queue = queue.lock().unwrap();
    let length = queue.queue.len();

    if length == 0 {
        return "HTTP/1.1 200 OK\r\n\r\n".to_string();
    }

    let data = queue.serialize(recipe);

    put!("Updating client, sending \x1b[93m{}\x1b[0m bytes updating \x1b[93m{}\x1b[0m files", data.len(), queue.queue.len());
    queue.clear();

    return build_response(data);
}

fn build_response(data: String) -> String {
    let mut response = "HTTP/1.1 200 OK\r\n".to_owned();
    response.push_str("Content-Type: text/plain\r\n");
    response.push_str(&format!("Content-Length: {}", data.len()));
    response.push_str("\r\n\r\n");
    response.push_str(&data);

    return response;
}