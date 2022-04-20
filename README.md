<h1 align="center">🤝 lemmy-help</h1>
<p align="center"><sup>Everyone needs help, so lemmy-help you</sup></p>

<!-- image -->

### What?

`lemmy-help` is a emmylua parser as well as a CLI which takes that parsed tree and converts it into vim help docs.

### Installation

- Using `cargo`

```sh
cargo install lemmy-help
```

- Using releases

Check out the [Release page](https://github.com/numToStr/lemmy-help/releases) for prebuild binaries available for different operating systems.

### Usage

Using the CLI is simple just give it the path to the lua files; it will parse them and prints the help doc to **stdout**

> NOTE: The **order** of parsing + rendering is same as in which they are defined

```sh
lemmy-help \
    -f "/path/to/first/file" \
    -f "/path/to/second/file" \
```

### Credits

- TJ's [docgen](https://github.com/tjdevries/tree-sitter-lua#docgen) module
- [mini.doc](https://github.com/echasnovski/mini.nvim#minidoc) from `mini.nvim` plugin
