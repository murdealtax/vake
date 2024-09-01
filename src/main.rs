use log::info;
use std::io::Write;
use clap::Parser;

use wake::cli::Options;
use wake::watch::config;
use wake::parser::{lex, parse, check};
use wake::server;

use std::fs;

fn main() {
    let options = Options::parse();
    let log_filter = match options.global.verbosity {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    let log_env = env_logger::Env::default().default_filter_or(log_filter);

    env_logger::Builder::from_env(log_env)
        .format(|buf, record| {
            
            let level = record.level();
            let style = buf.default_level_style(level);
            
            writeln!(buf, "\x1b[90m[{style}{}{style:#}\x1b[90m]\x1b[0m {}", level, record.args())
        })
        .init();

    let config_path = config::check_config();
    let contents = fs::read_to_string(config_path)
        .expect("Should have been able to read the file");

    let tokens = lex::init(contents.as_str());
    let recipe = parse::init(tokens);

    check::init(&recipe);

    info!("Loaded recipe with {} entry(s) and {} association(s)", recipe.entries.len(), recipe.associations.len());
    server::serve();
}
