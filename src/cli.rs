use lemmy_help::{LemmyHelp, Rename};
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
    modeline: bool,
    rename: Rename,
    files: Vec<PathBuf>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            modeline: true,
            rename: Rename::default(),
            files: vec![],
        }
    }
}

impl Cli {
    pub fn new() -> Result<Self, lexopt::Error> {
        let mut c = Cli::default();
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
                Short('M') | Long("no-modeline") => c.modeline = false,
                Short('f') | Long("prefix-func") => c.rename.func = true,
                Short('a') | Long("prefix-alias") => c.rename.alias = true,
                Short('c') | Long("prefix-class") => c.rename.class = true,
                Short('t') | Long("prefix-type") => c.rename.r#type = true,
                Value(val) => {
                    let file = PathBuf::from(&val);

                    if !file.is_file() {
                        return Err(lexopt::Error::UnexpectedArgument(OsString::from(format!(
                            "{} is not a file!",
                            file.display()
                        ))));
                    }

                    c.files.push(file)
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(c)
    }

    pub fn run(self) {
        let mut lemmy = LemmyHelp::new(self.rename);

        for f in self.files {
            let source = read_to_string(f).unwrap();
            lemmy.for_help(&source).unwrap();
        }

        print!("{lemmy}");

        if self.modeline {
            println!("vim:tw=78:ts=8:noet:ft=help:norl:");
        }
    }

    #[inline]
    pub fn help() {
        print!(
            "\
{NAME} {VERSION}
{AUTHOR}
{DESC}

USAGE:
    {NAME} [FLAGS] <FILES>...

ARGS:
    <FILES>...              Path to the files

FLAGS:
    -M, --no-modeline       Don't print modeline at the end
    -f, --prefix-func       Prefix function name with ---@mod name
    -a, --prefix-alias      Prefix ---@alias tag with return/---@mod name
    -c, --prefix-class      Prefix ---@class tag with return/---@mod name
    -t, --prefix-type       Prefix ---@type tag with ---@mod name
    -h, --help              Print help information
    -v, --version           Print version information

USAGE:
    {NAME} /path/to/first.lua /path/to/second.lua > doc.txt
    {NAME} -c -a /path/to/{{first,second,third}}.lua > doc.txt

NOTES:
    - The order of parsing + rendering is relative to the given files
"
        );
    }
}
