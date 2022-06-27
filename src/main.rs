mod cli;

use std::process::exit;

use cli::Cli;

fn main() {
    match Cli::new() {
        Ok(c) => {
            if let Err(e) = c.run() {
                eprintln!("{e}");
                exit(1)
            }
        }
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
