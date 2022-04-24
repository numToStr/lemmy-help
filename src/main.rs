mod cli;

use std::process::exit;

use cli::Cli;

fn main() {
    match Cli::new() {
        Ok(c) => c.run(),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
