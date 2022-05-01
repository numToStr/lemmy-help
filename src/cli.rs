use lemmy_help::LemmyHelp;
use lexopt::{
    Arg::{Long, Short, Value},
    Parser,
};
use std::{ffi::OsString, fs::read_to_string, path::PathBuf};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub struct Cli {
    files: Vec<PathBuf>,
    modeline: bool,
}

impl Cli {
    pub fn new() -> Result<Self, lexopt::Error> {
        let mut files = vec![];
        let mut modeline = true;
        let mut parser = Parser::from_env();

        while let Some(arg) = parser.next()? {
            match arg {
                Short('v') | Long("version") => {
                    println!("{NAME} {VERSION}");
                    std::process::exit(0);
                }
                Short('h') | Long("help") => {
                    Self::help();
                    std::process::exit(0);
                }
                Short('M') | Long("no-modeline") => {
                    modeline = false;
                }
                Value(val) => {
                    let file = PathBuf::from(&val);

                    if !file.is_file() {
                        return Err(lexopt::Error::UnexpectedArgument(OsString::from(format!(
                            "{} is not a file!",
                            file.display()
                        ))));
                    }

                    files.push(file)
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Self { files, modeline })
    }

    pub fn run(&self) {
        let mut lemmy = LemmyHelp::new();

        for f in &self.files {
            let source = read_to_string(f).unwrap();
            lemmy.for_help(&source).unwrap();
        }

        print!("{lemmy}");

        if self.modeline {
            println!("vim:tw=78:ts=8:noet:ft=help:norl:");
        }
    }

    pub fn help() {
        print!(
            "\
{NAME} {VERSION}
{AUTHOR}
{DESC}

USAGE:
    {NAME} [FILES]...

ARGS:
    <FILES>...              Path to the files

OPTIONS:
    -M, --no-modeline       Don't print modeline at the end
    -h, --help              Print help information
    -v, --version           Print version information
"
        );
    }
}
