mod connection;
mod logger;
mod poll;
mod close;
mod sync;

use log::debug;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::parser::parse::Recipe;
use crate::watch::{self, ProjectQueue};

const DEFAULT_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_PORT: u16 = 9595;

pub fn serve(recipe: Recipe) {
    let queue: ProjectQueue = ProjectQueue::new();
    debug!("Starting internal server...");
    let active_directory = &recipe.clone().options.active_directory;
    let queue = Arc::new(Mutex::new(queue));
    let queue_thread = queue.clone();
    
    thread::spawn(move || {
        connection::listen(DEFAULT_ADDRESS, DEFAULT_PORT, queue_thread, recipe)
    });

    watch::daemon(active_directory, &queue).expect("Failed to watch!");
}