use log::{ debug, warn };
use std::path::{Path, PathBuf};

const DEFAULT_vakeFILE: &str = r#"# Vakefile Options
:active_directory = "."
:entry_name = "main.lua"

# Use LocalScripts on the client
client :: LocalScript

# Example Recipe
server -> ServerScriptService
client -> StarterPlayerScripts"#;

pub fn check_config() -> PathBuf {
    debug!("Checking for existance of vakefile");

    let names = vec![".vakefile", ".vake", "vakefile"];
    let mut found = false;
    let mut path = Path::new("vakefile").to_path_buf();

    for name in names {
        debug!("Checking for possible vakefile at {}", name);
        if std::fs::metadata(name).is_ok() {
            debug!("Found vakefile at {}", name);
            found = true;
            path = Path::new(name).to_path_buf();
            break;
        }
    }

    if !found {
        warn!("No vakefile found, creating a new one...");
        std::fs::write("vakefile", DEFAULT_vakeFILE).expect("Failed to create vakefile");
    }

    return path;
}