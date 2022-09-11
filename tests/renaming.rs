use lemmy_help::{LemmyHelp, Rename};

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
    let mut lemmy = LemmyHelp::new(Rename {
        func: true,
        alias: true,
        class: true,
        r#type: true,
    });

    lemmy.for_help(CODE).unwrap();

    assert_eq!(
        lemmy.to_string(),
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

    let mut lemmy = LemmyHelp::new(Rename {
        func: true,
        alias: true,
        class: true,
        r#type: true,
    });

    lemmy.for_help(&src).unwrap();

    assert_eq!(
        lemmy.to_string(),
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
