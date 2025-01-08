mod connection;
mod logger;
mod poll;
mod close;
mod sync;

use log::{ debug, error };
use std::net::Ipv4Addr;
use crate::parser::parse::Recipe;
use crate::watch::{self, ProjectQueue};

const DEFAULT_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_PORT: u16 = 9595;

pub fn serve(recipe: Recipe) {
    let mut queue: ProjectQueue = ProjectQueue::new();
    debug!("Starting internal server...");
    let active_directory = &recipe.clone().options.active_directory;
    let server = connection::listen(DEFAULT_ADDRESS, DEFAULT_PORT, &mut queue, recipe);

    if server.is_err() {
        error!("Failed to start server!");
        error!("{}", server.err().unwrap());
    }

    watch::daemon(active_directory, &mut queue).expect("Failed to watch!");
}