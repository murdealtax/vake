use log::{ debug, info };
use std::io::Write;
use clap::Parser;

use wake::cli::Options;
use wake::server;

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

    debug!("Debugging enabled!");
    info!("Starting Wake...");
    server::serve();
}
