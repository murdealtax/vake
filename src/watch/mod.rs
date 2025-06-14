pub mod config;
pub mod serialize;

use log::{ debug, error };

use std::{collections::HashMap, sync::Arc};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Mutex};
use std::fs::{self, metadata};

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

    pub fn serialize(&mut self, recipe: Recipe) -> String {
        serialize::build(self, recipe)
    }

    pub fn all(&mut self, recipe: &Recipe) {
        let dir = &recipe.options.active_directory;
        Self::visit_files(dir, &mut |path| {
            debug!("Sending file {:?}", path);
            self.push(path, ActionType::Create);
        });
    }

    fn visit_files(path: &Path, f: &mut impl FnMut(PathBuf)) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    f(path);
                } else if path.is_dir() {
                    Self::visit_files(&path, f);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

pub fn daemon(directory: &PathBuf, queue: &Arc<Mutex<ProjectQueue>>) -> Result<(), notify::Error> {
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

    return daemon;
}

fn process_event(event: Event, queue: &Arc<Mutex<ProjectQueue>>) {
    let mut queue = queue.lock().unwrap();
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