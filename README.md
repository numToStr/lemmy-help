<h1 align="center">🤝 lemmy-help</h1>
<p align="center"><sup>Everyone needs help, so lemmy-help you</sup></p>

![lemmy-help](https://user-images.githubusercontent.com/24727447/164423469-b26fea39-2ef7-497c-8156-5a4c01bc30f8.gif "Generating help docs")

### What?

`lemmy-help` is a emmylua parser as well as a CLI which takes that parsed tree and converts it into vim help docs.

### Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/lemmy-help.svg)](https://repology.org/project/lemmy-help/versions)

- Using `cargo`

```bash
cargo install lemmy-help --features=cli
```

- Arch Linux

```bash
# Using yay
yay -S lemmy-help

# Using paru
paru -S lemmy-help
```

- Using releases

Check out the [release page](https://github.com/numToStr/lemmy-help/releases) for prebuild binaries available for different operating systems.

### Emmylua

To properly generate docs you should follow emmylua spec. The parser is capable of parsing most (not all) of the emmylua syntax. You can read the following doc which can give you the idea on how to properly write emmylua comments.

- [Writing emmylua docs](./emmylua.md)

### Usage

Using the CLI is simple just give it the path to the lua files; it will parse them and prints help doc onto **stdout**

```bash
lemmy-help /path/to/{first,second,third}.lua > doc/PLUGIN_NAME.txt
```

### Cli

```text
lemmy-help

USAGE:
    lemmy-help [FLAGS] [OPTIONS] <FILES>...

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
                                  and uses {n}, if provided, to indent the
                                  following new lines. This option only
                                  affects ---@field and ---@param tags
                                - "mini[:n=0]" : Aligns [desc] from the start
                                  and uses {n}, if provided, to indent the
                                  following new lines. This option affects
                                  ---@field, ---@param and ---@return tags

USAGE:
    lemmy-help /path/to/first.lua /path/to/second.lua > doc/PLUGIN_NAME.txt
    lemmy-help -c -a /path/to/{first,second,third}.lua > doc/PLUGIN_NAME.txt
    lemmy-help --layout compact:2 /path/to/plugin.lua > doc/PLUGIN_NAME.txt

NOTES:
    - The order of parsing + rendering is relative to the given files
```

### CI

```yaml
name: lemmy-help

on: [push]

env:
  PLUGIN_NAME: plugin-name

jobs:
  docs:
    runs-on: ubuntu-latest
    name: emmylua to vimdoc
    steps:
      - uses: actions/checkout@v2

      - name: Generating help
        run: |
          curl -Lq https://github.com/numToStr/lemmy-help/releases/latest/download/lemmy-help-x86_64-unknown-linux-gnu.tar.gz | tar xz
          ./lemmy-help [args] <path> > doc/${{env.PLUGIN_NAME}}.txt

      - name: Commit
        uses: stefanzweifel/git-auto-commit-action@v4
        with:
          branch: ${{ github.head_ref }}
          commit_message: "chore(docs): auto-generate vimdoc"
          file_pattern: doc/*.txt
```

### Credits

- TJ's [docgen](https://github.com/tjdevries/tree-sitter-lua#docgen) module
- [mini.doc](https://github.com/echasnovski/mini.nvim#minidoc) from `mini.nvim` plugin
