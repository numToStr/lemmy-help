use lemmy_help::{parser, vimdoc::VimDoc, FromEmmy, Layout, Settings};

use lexopt::{
    Arg::{Long, Short, Value},
    Parser, ValueExt,
};
use std::{fs::read_to_string, path::PathBuf, str::FromStr};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub struct Cli {
    modeline: bool,
    settings: Settings,
    files: Vec<PathBuf>,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            modeline: true,
            settings: Settings::default(),
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
                Short('l') | Long("layout") => {
                    let layout = parser.value()?.string()?;
                    c.settings.layout =
                        Layout::from_str(&layout).map_err(|_| lexopt::Error::UnexpectedValue {
                            option: "layout".into(),
                            value: layout.into(),
                        })?;
                }
                Short('i') | Long("indent") => {
                    c.settings.indent_width = parser.value()?.parse()?;
                }
                Short('M') | Long("no-modeline") => c.modeline = false,
                Short('f') | Long("prefix-func") => c.settings.prefix_func = true,
                Short('a') | Long("prefix-alias") => c.settings.prefix_alias = true,
                Short('c') | Long("prefix-class") => c.settings.prefix_class = true,
                Short('t') | Long("prefix-type") => c.settings.prefix_type = true,
                Long("expand-opt") => c.settings.expand_opt = true,
                Value(val) => {
                    let file = PathBuf::from(&val);
                    if !file.is_file() {
                        return Err(lexopt::Error::UnexpectedArgument(
                            format!("{} is not a file!", file.display()).into(),
                        ));
                    }
                    c.files.push(file)
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(c)
    }

    pub fn run(self) {
        let mut help_doc = String::new();

        // FIXME: toc entries
        for f in self.files {
            let source = read_to_string(f).unwrap();
            let ast = parser(&source, &self.settings);
            let doc = VimDoc::from_emmy(&ast, &self.settings);
            help_doc.push_str(&doc.to_string());
        }

        print!("{help_doc}");

        if self.modeline {
            println!("vim:tw=78:ts=8:noet:ft=help:norl:");
        }
    }

    #[inline]
    pub fn help() {
        print!(
            r#"{NAME} {VERSION}
{AUTHOR}
{DESC}

USAGE:
    {NAME} [FLAGS] [OPTIONS] <FILES>...

ARGS:
    <FILES>...                  Path to lua files

FLAGS:
    -h, --help                  Print help information
    -v, --version               Print version information
    -M, --no-modeline           Don't print modeline at the end
    -f, --prefix-func           Prefix function name with ---@mod name
    -a, --prefix-alias          Prefix ---@alias tag with return/---@mod name
    -c, --prefix-class          Prefix ---@class tag with return/---@mod name
    -t, --prefix-type           Prefix ---@type tag with ---@mod name
        --expand-opt            Expand '?' (optional) to 'nil' type

OPTIONS:
    -i, --indent <u8>           Controls the indent width [default: 4]
    -l, --layout <layout>       Vimdoc text layout [default: 'default']
                                - "default" : Default layout
                                - "compact[:n=0]" : Aligns [desc] with <type>
                                  and uses {{n}}, if provided, to indent the
                                  following new lines. This option only
                                  affects ---@field and ---@param tags
                                - "mini[:n=0]" : Aligns [desc] from the start
                                  and uses {{n}}, if provided, to indent the
                                  following new lines. This option affects
                                  ---@field, ---@param and ---@return tags

USAGE:
    {NAME} /path/to/first.lua /path/to/second.lua > doc/PLUGIN_NAME.txt
    {NAME} -c -a /path/to/{{first,second,third}}.lua > doc/PLUGIN_NAME.txt
    {NAME} --layout compact:2 /path/to/plugin.lua > doc/PLUGIN_NAME.txt

NOTES:
    - The order of parsing + rendering is relative to the given files
"#
        );
    }
}
