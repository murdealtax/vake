pub mod config;
pub mod serialize;

use log::{ debug, error };

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::fs::metadata;

use notify::{Event, RecursiveMode, Watcher};

use crate::parser::parse::Recipe;

#[derive(Clone)]
pub enum ActionType {
    Remove,
    Create,
    Path
}

#[derive(Clone)]
pub struct ProjectQueue {
    pub queue: HashMap<PathBuf, ActionType>
}

impl ProjectQueue {
    pub fn new() -> ProjectQueue {
        return Self { queue: HashMap::new() }
    }

    pub fn push(&mut self, path: PathBuf, action: ActionType) {
        self.queue.insert(path, action);
    }

    pub fn serialize(&self, recipe: Recipe) -> String {
        serialize::build(self, recipe)
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

pub fn daemon(directory: &PathBuf, queue: &mut ProjectQueue) -> Result<(), notify::Error> {
    debug!("Starting daemon on directory '{}'", directory.as_os_str().to_str().expect("Expected a specified directory"));
    let (sender, reciever) = mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher = notify::recommended_watcher(sender)?;

    let daemon = watcher.watch(&directory, RecursiveMode::Recursive);

    for res in reciever {
        match res {
            Ok(event) => process_event(event, queue),
            Err(e) => error!("watch error: {:?}", e),
        }
    }

    println!("Daemon");

    return daemon;
}

fn process_event(event: Event, queue: &mut ProjectQueue) {
    for path in event.paths {
        let metadata = metadata(path.clone());
        if metadata.is_err() {
            debug!("Removed {}", path.as_os_str().to_str().expect("Expected a file name"));
            queue.push(path, ActionType::Remove);
            continue;
        }

        let handle = metadata.unwrap();
        if handle.is_dir() {
            debug!("Recieved directory event on directory {}", path.as_os_str().to_str().expect("Expected a file name"));
            queue.push(path, ActionType::Path);
        } else if handle.is_file() {
            debug!("Recieved file event on file {}", path.as_os_str().to_str().expect("Expected a file name"));
            queue.push(path, ActionType::Create);
        }
    }
}