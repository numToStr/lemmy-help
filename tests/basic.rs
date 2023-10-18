use lemmy_help::{vimdoc::VimDoc, FromEmmy, LemmyHelp, Settings};

macro_rules! lemmy {
    ($($src: expr),*) => {{
        let mut lemmy = LemmyHelp::default();
        let s = Settings::default();
        $(
            lemmy.for_help($src, &s).unwrap();
        )*
        VimDoc::from_emmy(&lemmy, &s).to_string()
    }};
}

#[test]
fn brief() {
    let src = r#"
    local U = {}

    ---@brief [[
    ---Any summary you wanted to write you can write here.
    ---There is no formatting here,
    ---the way you write in here, will be shown
    ---exactly in the help-doc
    ---
    ---An empty line can be used to denote a paragraph
    ---
    ---You can also write anything, like ordered list
    ---    1. first
    ---    2. second
    ---    3. third
    ---
    ---Some code blocks, but IDK whether it will be highlighted or not
    ---
    --->
    ---    for i = 1, 10, 1 do
    ---        print(("%s Lua is awesome"):format(i))
    ---    end
    ---<
    ---
    ---NOTE: remember there is no formatting or text wrapping
    ---@brief ]]

    return U
    "#;

    assert_eq!(
        lemmy!(src),
        r#"Any summary you wanted to write you can write here.
There is no formatting here,
the way you write in here, will be shown
exactly in the help-doc

An empty line can be used to denote a paragraph

You can also write anything, like ordered list
    1. first
    2. second
    3. third

Some code blocks, but IDK whether it will be highlighted or not

>
    for i = 1, 10, 1 do
        print(("%s Lua is awesome"):format(i))
    end
<

NOTE: remember there is no formatting or text wrapping

"#
    )
}

#[test]
fn divider_and_tag() {
    let src = "
    local U = {}

    ---@divider =
    ---@tag kinda.module

    ---@divider -
    ---@tag kinda.section

    return U
    ";

    assert_eq!(
        lemmy!(src),
        "\
==============================================================================

                                                                  *kinda.module*
------------------------------------------------------------------------------

                                                                 *kinda.section*
"
    )
}

#[test]
fn classes() {
    let src = "
    local U = {}

    ---The Homosapien
    ---@class Human
    ---@field legs number Total number of legs
    ---@field hands number Total number of hands
    ---@field brain boolean Does humans have brain?

    ---@class SuperSecret
    ---@field first number First ingredient
    ---@field public second? number Second ingredient
    ---@field third number Third ingredient
    ---@field todo? number
    ---@field protected __secret_1 number Secret ingredient first
    ---@field private __secret_2? number

    ---Plugin's configuration
    ---@class CommentConfig
    ---@field padding boolean Add a space b/w comment and the line
    ---Whether the cursor should stay at its position
    ---NOTE: This only affects NORMAL mode mappings and doesn't work with dot-repeat
    ---
    ---@field sticky boolean
    ---Lines to be ignored while comment/uncomment.
    ---Could be a regex string or a function that returns a regex string.
    ---Example: Use '^$' to ignore empty lines
    ---
    ---@field ignore string|fun():string
    ---@field pre_hook fun(ctx:CommentCtx):string Function to be called before comment/uncomment
    ---@field post_hook fun(ctx:CommentCtx) Function to be called after comment/uncomment

    ---@class XMen : Homosapien
    ---@field power number Power quantifier

    ---@class (exact) XactMen
    ---@field power number Power quantifier

    return U
    ";

    assert_eq!(
        lemmy!(src),
        "\
Human                                                                    *Human*
    The Homosapien

    Fields: ~
        {legs}   (number)   Total number of legs
        {hands}  (number)   Total number of hands
        {brain}  (boolean)  Does humans have brain?


SuperSecret                                                        *SuperSecret*

    Fields: ~
        {first}    (number)  First ingredient
        {second?}  (number)  Second ingredient
        {third}    (number)  Third ingredient
        {todo?}    (number)


CommentConfig                                                    *CommentConfig*
    Plugin's configuration

    Fields: ~
        {padding}    (boolean)                     Add a space b/w comment and the line
        {sticky}     (boolean)                     Whether the cursor should stay at its position
                                                   NOTE: This only affects NORMAL mode mappings and doesn't work with dot-repeat

        {ignore}     (string|fun():string)         Lines to be ignored while comment/uncomment.
                                                   Could be a regex string or a function that returns a regex string.
                                                   Example: Use '^$' to ignore empty lines

        {pre_hook}   (fun(ctx:CommentCtx):string)  Function to be called before comment/uncomment
        {post_hook}  (fun(ctx:CommentCtx))         Function to be called after comment/uncomment


XMen : Homosapien                                                         *XMen*

    Fields: ~
        {power}  (number)  Power quantifier


XactMen                                                                *XactMen*

    Fields: ~
        {power}  (number)  Power quantifier


"
    )
}

#[test]
fn functions() {
    let src = r#"
    local U = { foo = {} }

    ---NOTE: Local functions are not part of the documentation
    ---Multiply two integer and print it
    ---@param this number First number
    ---@param that number Second number
    local function mul(this, that)
        print(this * that)
    end

    ---Add two integer and print it
    ---
    ---NOTE: This will be part of the public API
    ---@param this number First number
    ---@param that? number
    function U.sum(this, that)
        print(this + that or 0)
    end

    ---Subtract second from the first integer
    ---@param this number
    ---@param that number Second number
    ---@return number
    ---Some secret number that
    ---we don't know about
    ---@usage `require('module.U').sub(10, 5)`
    function U.sub(this, that)
        return this - that
    end

    ---This is a magical function
    ---@param this number Non-magical number #1
    ---@param that number Non-magical number #2
    ---@return number _ The magical number #1
    ---@return number #The magical number #2
    ---and the fun part is the description can span
    ---
    ---over mulitple lines and preserves empty lines
    ---@see U.mul
    ---@see U.sum
    ---@see U.sub
    U.magical = function(this, that)
        return (U.mul(this, that) / U.sum(that, this)), (U.sum(this, that) * U.sub(that, this))
    end

    ---Function with variable arguments
    ---@param ... string[]
    function U.with_var_arg(...) end

    ---@param x integer
    ---@param ... unknown
    function U.with_var_arg_end(x, ...) end

    ---Class method deep access
    ---@return table
    function U.foo:bar()
        self.__index = self
        return setmetatable({}, self)
    end

    ---Method deep access
    ---@return table
    function U.foo.baz()
        return U.foo:bar()
    end

    return U
    "#;

    assert_eq!(
        lemmy!(src),
        "\
U.sum({this}, {that?})                                                   *U.sum*
    Add two integer and print it

    NOTE: This will be part of the public API

    Parameters: ~
        {this}   (number)  First number
        {that?}  (number)


U.sub({this}, {that})                                                    *U.sub*
    Subtract second from the first integer

    Parameters: ~
        {this}  (number)
        {that}  (number)  Second number

    Returns: ~
        (number)  Some secret number that
                  we don't know about

    Usage: ~
>lua
        require('module.U').sub(10, 5)
<


U.magical({this}, {that})                                            *U.magical*
    This is a magical function

    Parameters: ~
        {this}  (number)  Non-magical number #1
        {that}  (number)  Non-magical number #2

    Returns: ~
        (number)  The magical number #1
        (number)  The magical number #2
                  and the fun part is the description can span

                  over mulitple lines and preserves empty lines

    See: ~
        |U.mul|
        |U.sum|
        |U.sub|


U.with_var_arg({...})                                           *U.with_var_arg*
    Function with variable arguments

    Parameters: ~
        {...}  (string[])


U.with_var_arg_end({x}, {...})                              *U.with_var_arg_end*

    Parameters: ~
        {x}    (integer)
        {...}  (unknown)


U.foo:bar()                                                          *U.foo:bar*
    Class method deep access

    Returns: ~
        (table)


U.foo.baz()                                                          *U.foo.baz*
    Method deep access

    Returns: ~
        (table)


"
    )
}

#[test]
fn multiline_param() {
    let src = r#"
    local U = {}

    ---Trigger a rebuild of one or more projects.
    ---@param opts table|nil optional configuration options:
    ---  * {select_mode} (JdtProjectSelectMode) Show prompt
    ---     to select projects or select all. Defaults
    ---     to 'prompt'
    ---
    ---  * {full_build} (boolean) full rebuild or
    ---     incremental build. Defaults to true (full build)
    ---@param reserverd table|nil reserved for the future use
    ---@return boolean
    function U.multi_line(opts, reserverd)
        print(vim.inspect(opts), vim.inspect(reserverd))

        return true
    end

    ---Multiline but missing description
    ---@param n number
    ---This is a special
    ---
    ---number
    ---@param m number
    ---And this is also
    ---
    ---@return number
    function U.missing_desc(n, m)
        return n + m
    end

    return U
    "#;

    assert_eq!(
        lemmy!(src),
        "\
U.multi_line({opts}, {reserverd})                                 *U.multi_line*
    Trigger a rebuild of one or more projects.

    Parameters: ~
        {opts}       (table|nil)  optional configuration options:
                                    * {select_mode} (JdtProjectSelectMode) Show prompt
                                       to select projects or select all. Defaults
                                       to 'prompt'

                                    * {full_build} (boolean) full rebuild or
                                       incremental build. Defaults to true (full build)
        {reserverd}  (table|nil)  reserved for the future use

    Returns: ~
        (boolean)


U.missing_desc({n}, {m})                                        *U.missing_desc*
    Multiline but missing description

    Parameters: ~
        {n}  (number)  This is a special

                       number
        {m}  (number)  And this is also


    Returns: ~
        (number)


"
    )
}

#[test]
fn module() {
    let src = "
    ---@mod mod.intro Introduction
    ---@brief [[
    ---
    ---We can have multiple `---@mod` tags so that we can have a block only for text.
    ---This is for the cases where you want bunch of block only just for text
    ---and does not contains any code.
    ---
    ---@brief ]]

    ---@mod mod.Human Human module

    local U = {}

    ---The Homosapien
    ---@class Human
    ---@field legs number Total number of legs
    ---@field hands number Total number of hands
    ---@field brain boolean Does humans have brain?

    ---Default traits of a human
    ---@type Human
    U.DEFAULT = {
        legs = 2,
        hands = 2,
        brain = false,
    }

    ---Creates a Human
    ---@return Human
    ---@usage [[
    ---local H = require('Human')
    ---local human = H:create()
    ---
    ---print(getmetatable(human))
    ---@usage ]]
    function U:create()
        return setmetatable(self.DEFAULT, { __index = self })
    end

    return U
    ";

    let src2 = r#"
    ---@mod wo.desc

    local U = {}

    return U
    "#;

    assert_eq!(
        lemmy!(src, src2),
        "\
==============================================================================
Introduction                                                         *mod.intro*


We can have multiple `---@mod` tags so that we can have a block only for text.
This is for the cases where you want bunch of block only just for text
and does not contains any code.


==============================================================================
Human module                                                         *mod.Human*

Human                                                                    *Human*
    The Homosapien

    Fields: ~
        {legs}   (number)   Total number of legs
        {hands}  (number)   Total number of hands
        {brain}  (boolean)  Does humans have brain?


U.DEFAULT                                                            *U.DEFAULT*
    Default traits of a human

    Type: ~
        (Human)


U:create()                                                            *U:create*
    Creates a Human

    Returns: ~
        (Human)

    Usage: ~
>lua
        local H = require('Human')
        local human = H:create()

        print(getmetatable(human))
<


==============================================================================
                                                                       *wo.desc*

"
    )
}

#[test]
fn table_of_contents() {
    let src = "
    ---@toc my-plugin.contents

    ---@mod first.module First Module

    ---@mod second.module Second Module

    ---@mod third.module Third Module

    local U = {}

    return U
    ";

    assert_eq!(
        lemmy!(src),
        "\
==============================================================================
Table of Contents                                           *my-plugin.contents*

First Module ···················································· |first.module|
Second Module ·················································· |second.module|
Third Module ···················································· |third.module|

==============================================================================
First Module                                                      *first.module*

==============================================================================
Second Module                                                    *second.module*

==============================================================================
Third Module                                                      *third.module*

"
    );
}

#[test]
fn alias_and_type() {
    let src = r#"
    local U = {}

    ---@alias NoDesc string

    ---All the lines in the buffer
    ---@alias Lines string[]

    ---Vim operator-mode motions.
    ---
    ---Read `:h map-operator`
    ---@alias VMode
    ---| '"line"' # Vertical motion
    ---| '"char"' # Horizontal motion
    ---| 'v'
    ---| `some.ident` # Some identifier

    ---Returns all the content of the buffer
    ---@return Lines
    function U.get_all()
        return vim.api.nvim_buf_get_lines(0, 0, -1, false)
    end

    ---List of all the lines in the buffer
    ---It can be more than one
    ---@type Lines lines in a buffer
    ---@see Lines
    U.LINES = {}

    ---Global vim mode
    ---@type VMode
    ---@usage `print(require('plugin').VMODE)`
    U.VMODE = 'line'

    return U
    "#;

    assert_eq!(
        lemmy!(src),
        r#"NoDesc                                                                  *NoDesc*

    Type: ~
        string


Lines                                                                    *Lines*
    All the lines in the buffer

    Type: ~
        string[]


VMode                                                                    *VMode*
    Vim operator-mode motions.

    Read `:h map-operator`

    Variants: ~
        ("line")      Vertical motion
        ("char")      Horizontal motion
        ("v")
        (some.ident)  Some identifier


U.get_all()                                                          *U.get_all*
    Returns all the content of the buffer

    Returns: ~
        (Lines)


U.LINES                                                                *U.LINES*
    List of all the lines in the buffer
    It can be more than one

    Type: ~
        (Lines)  lines in a buffer

    See: ~
        |Lines|


U.VMODE                                                                *U.VMODE*
    Global vim mode

    Type: ~
        (VMode)

    Usage: ~
>lua
        print(require('plugin').VMODE)
<


"#
    )
}

#[test]
fn usage() {
    let src = "
    local U = {}

    ---Prints a message
    ---@param msg string Message
    ---@usage lua [[
    ---require('module.U').sum(10, 5)
    ---@usage ]]
    function U.echo(msg)
        print(msg)
    end

    ---Add a number to 10
    ---@param this number A number
    ---@usage `require('module.U').sum(5)`
    function U.sum(this)
        print(10 + this)
    end

    return U
    ";

    assert_eq!(
        lemmy!(src),
        "\
U.echo({msg})                                                           *U.echo*
    Prints a message

    Parameters: ~
        {msg}  (string)  Message

    Usage: ~
>lua
        require('module.U').sum(10, 5)
<


U.sum({this})                                                            *U.sum*
    Add a number to 10

    Parameters: ~
        {this}  (number)  A number

    Usage: ~
>lua
        require('module.U').sum(5)
<


"
    )
}

#[test]
fn private() {
    let src = "
    local U = {}

    ---@private
    ---This is a private function which is exported
    ---But not considered as part of the API
    function U.private()
        print('I am private!')
    end

    ---Only this will be documented
    function U.ok()
        print('Ok! I am exported')
    end

    ---@protected
    function U.no_emmy()
        print('Protected func with no emmylua!')
    end

    return U
    ";

    assert_eq!(
        lemmy!(src),
        "\
U.ok()                                                                    *U.ok*
    Only this will be documented


"
    )
}

#[test]
fn export() {
    let src = "
    ---@mod module.config Configuration

    local Config = {}

    ---Get the config
    ---@return number
    function Config:get()
        return 3.14
    end

    ---@export Config
    return setmetatable(Config, {
        __index = function(this, k)
            return this.state[k]
        end,
        __newindex = function(this, k, v)
            this.state[k] = v
        end,
    })
    ";

    assert_eq!(
        lemmy!(src),
        "\
==============================================================================
Configuration                                                    *module.config*

Config:get()                                                        *Config:get*
    Get the config

    Returns: ~
        (number)


"
    )
}
