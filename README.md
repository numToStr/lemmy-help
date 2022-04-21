<h1 align="center">🤝 lemmy-help</h1>
<p align="center"><sup>Everyone needs help, so lemmy-help you</sup></p>

![lemmy-help](https://user-images.githubusercontent.com/24727447/164242728-170ac7fa-a093-46b6-b423-960325d94313.gif "Generating help docs")

### What?

`lemmy-help` is a emmylua parser as well as a CLI which takes that parsed tree and converts it into vim help docs.

### Installation

- Using `cargo`

```bash
cargo install lemmy-help --features=cli
```

- Using releases

Check out the [release page](https://github.com/numToStr/lemmy-help/releases) for prebuild binaries available for different operating systems.

### Emmylua

To properly generate docs you should follow emmylua spec. The parser is capable of parsing most (not all) of the emmylua syntax. You can read the following doc which can give you the idea on how to properly write emmylua comments.

- [Writing emmylua docs](./emmylua.md)

### Usage

Using the CLI is simple just give it the path to the lua files; it will parse them and prints the help doc to **stdout**

> NOTE: The **order** of parsing + rendering is same as in which they are defined

```bash
lemmy-help \
    -f "/path/to/first/file" \
    -f "/path/to/second/file" \
    -f "/path/to/third/file" > doc.txt
```

### Credits

- TJ's [docgen](https://github.com/tjdevries/tree-sitter-lua#docgen) module
- [mini.doc](https://github.com/echasnovski/mini.nvim#minidoc) from `mini.nvim` plugin
