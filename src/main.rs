//! Program entry point.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod config;
mod globals;

use anyhow::Result;
use config::Config;

fn setup_logging() -> Result<()> {
    todo!()
}

fn main() {
    if !Config::exists().unwrap() {
        println!("Creating new configuration");
        Config::create_new().unwrap();
        println!(
            "Created new configuration file at \"{}\", please edit it before running again.",
            Config::get_path().unwrap().display()
        );
        return;
    };
    let config = Config::load_from_disk().unwrap();

    // ...
}
