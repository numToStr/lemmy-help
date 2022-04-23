use clap::{crate_authors, crate_description, crate_version, AppSettings, Parser};
use lemmy_help::{frontend::Lua, LemmyHelp};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
    global_setting = AppSettings::DeriveDisplayOrder,
)]
pub struct Cli {
    /// Path to the files
    pub files: Vec<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn run(&self) {
        let mut lemmy = LemmyHelp::new();

        // NOTE: can we use threads? but if so, what about ordering?
        for f in &self.files {
            let source = read_to_string(f).unwrap();
            let source = Lua::parse(&source).unwrap();
            lemmy.for_help(&source).unwrap();
        }

        print!("{lemmy}");
        println!("vim:tw=78:ts=8:noet:ft=help:norl:");
    }
}
