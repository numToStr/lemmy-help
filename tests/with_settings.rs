use lemmy_help::{vimdoc::VimDoc, FromEmmy, LemmyHelp, Settings};

const CODE: &str = r#"
local U = {}

---@alias ID string

---@class User
---@field name string
---@field email string
---@field id ID

---A Pi
---@type number
U.Pi = 3.14

---Creates a PI
---@return number
---@usage `require('Pi'):create()`
function U:create()
    return self.Pi
end

return U
"#;

#[test]
fn rename_with_return() {
    let mut lemmy = LemmyHelp::new();
    let s = Settings {
        prefix_func: true,
        prefix_alias: true,
        prefix_class: true,
        prefix_type: true,
        ..Default::default()
    };

    lemmy.for_help(CODE, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&lemmy, &s).to_string(),
        "\
ID                                                                        *U.ID*

    Type: ~
        string


User                                                                    *U.User*

    Fields: ~
        {name}   (string)
        {email}  (string)
        {id}     (ID)


U.Pi                                                                      *U.Pi*
    A Pi

    Type: ~
        (number)


U:create()                                                            *U:create*
    Creates a PI

    Returns: ~
        {number}

    Usage: ~
        >
            require('Pi'):create()
        <


"
    );
}

#[test]
fn rename_with_mod() {
    let src = format!("---@mod awesome This is working {CODE}");

    let mut lemmy = LemmyHelp::new();
    let s = Settings {
        prefix_func: true,
        prefix_alias: true,
        prefix_class: true,
        prefix_type: true,
        ..Default::default()
    };

    lemmy.for_help(&src, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&lemmy, &s).to_string(),
        "\
================================================================================
This is working                                                        *awesome*

ID                                                                  *awesome.ID*

    Type: ~
        string


User                                                              *awesome.User*

    Fields: ~
        {name}   (string)
        {email}  (string)
        {id}     (ID)


U.Pi                                                                *awesome.Pi*
    A Pi

    Type: ~
        (number)


U:create()                                                      *awesome:create*
    Creates a PI

    Returns: ~
        {number}

    Usage: ~
        >
            require('Pi'):create()
        <


"
    );
}

#[test]
fn expand_opt() {
    let src = "
local M = {}

---@class HelloWorld
---@field message? string First message to the world
---@field private opts? table

---Prints given value
---@param message? string
function M.echo(message)
    return print(message)
end

return M
";

    let mut lemmy = LemmyHelp::new();
    let s = Settings {
        expand_opt: true,
        ..Default::default()
    };

    lemmy.for_help(src, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&lemmy, &s).to_string(),
        "\
HelloWorld                                                          *HelloWorld*

    Fields: ~
        {message}  (nil|string)  First message to the world


M.echo({message?})                                                      *M.echo*
    Prints given value

    Parameters: ~
        {message}  (nil|string)


"
    );
}
