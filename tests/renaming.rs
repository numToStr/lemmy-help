use lemmy_help::{LemmyHelp, Rename};

#[test]
fn rename_with_return() {
    let src = r#"
    local U = {}

    ---@alias ID string

    ---@class User
    ---@field name string
    ---@field email string
    ---@field id ID

    return U
    "#;

    let mut lemmy = LemmyHelp::with_rename(Rename {
        class: true,
        alias: true,
    });

    lemmy.for_help(src).unwrap();

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


"
    );
}

#[test]
fn rename_with_mod() {
    let src = r#"
    ---@mod awesome This is working

    local U = {}

    ---@alias ID string

    ---@class User
    ---@field name string
    ---@field email string
    ---@field id ID

    return U
    "#;

    let mut lemmy = LemmyHelp::with_rename(Rename {
        class: true,
        alias: true,
    });

    lemmy.for_help(src).unwrap();

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


"
    );
}
