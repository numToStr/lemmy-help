## Writing Emmylua

> **NOTE** - `lemmy-help` follows [LLS implementation](https://github.com/sumneko/lua-language-server/wiki/Annotations) of emmylua annotations with some little addons to better support the vimdoc generation.

Following are the tags that you can use to create docs

### Brief

This tag can be used to describe a module or even to add some footnote etc.

- Syntax

```lua
---@brief [[
---@comment
---@brief ]]
```

- Input

```lua
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
```

- Output

```help
Any summary you wanted to write you can write here.
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
```

### Module

This tag can be used to add a heading for a section. This tag also has the following properties:

1. This can appear multiple times in a file but only the last `---@mod` will be used to rename prefixes.

   > Use `--prefix-{func,alias,class,type}` cli options to rename function, alias, class, and type name prefixes relatively
   > See [`tests/renaming`](./tests/renaming.rs)

2. Also adds a entries in the [`Table of Contents`](#table-of-contents)

- Syntax

```lua
---@mod <name> [desc]
```

- Input

```lua
---@mod mod.intro Introduction
---@brief [[
---
---We can have multiple `---@mod` tags so that we can have a block only for text.
---This is for the cases where you want bunch of block only just for text
---and does not contains any code.
---
---You can write anything in here like some usage or something:
---
--->
---require('Comment').setup({
---    ignore = '^$',
---    pre_hook = function(ctx)
---        require('Comment.jsx').calculate(ctx)
---    end
---})
---<
---
---@brief ]]

---@mod mod.Human Human module

local H = {}

---@class Human The Homosapien
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?

---Default traits of a human
---@type Human
H.DEFAULT = {
    legs = 2,
    hands = 2,
    brain = false,
}

---Creates a Human
---@return Human
---@usage `require('Human'):create()`
function H:create()
    return setmetatable(self.DEFAULT, { __index = self })
end

return H
```

- Output

```help
================================================================================
Introduction                                                         *mod.intro*


We can have multiple `---@mod` tags so that we can have a block only for text.
This is for the cases where you want bunch of block only just for text
and does not contains any code.

You can write anything in here like some usage or something:

>
require('Comment').setup({
    ignore = '^$',
    pre_hook = function(ctx)
        require('Comment.jsx').calculate(ctx)
    end
})
<


================================================================================
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
        {Human}

    Usage: ~
>lua
        require('Human'):create()
<
```

### Table of Contents

This tag can be used to generate a _Table of Contents_ section. It uses [`---@mod`](#module) tags for the entries.

- Syntax

```lua
---@toc <tag>
```

- Input

```lua
---@toc my-plugin.contents

---@mod first.module First Module

---@mod second.module Second Module

---@mod third.module Third Module

local U = {}

return U
```

- Output

```help
================================================================================
Table of Contents                                           *my-plugin.contents*

First Module······················································|first.module|
Second Module····················································|second.module|
Third Module······················································|third.module|

================================================================================
First Module                                                      *first.module*

================================================================================
Second Module                                                    *second.module*

================================================================================
Third Module                                                      *third.module*
```

### Tag

This tag can used to create an alternate tag for your module, functions etc.

- Syntax

```lua
---@tag <name>
```

- Input

```lua
---@tag cool-tag
---@tag another-cool-tag
```

- Output

```
                                                                      *cool-tag*
                                                              *another-cool-tag*
```

### Divider

This tag can be used to add a divider/separator between section or anything you desire

- Syntax

```lua
---@divider <char>
```

- Input

```lua
---@divider -
---@divider =
---@divider ~
```

- Output

```help
--------------------------------------------------------------------------------

================================================================================

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

### Functions

A function contains multiple tags which form its structure. Like `---@param` for parameter, `---@return` for the return value, `---@see` for other related things and [`---@usage`](#usage) for example

- Syntax

```lua
---@comment
---@param <name[?]> <type[|type...]> [description]
---@comment
---@return <type> [<name> [comment] | [name] #<comment>]
---@comment
---@see <ref>
---@usage `<code>`
```

> NOTE: All tag can be used multiple times except `---@usage`

- Input

```lua
local U = {}

---NOTE: Local functions are not part of the documentation
---Multiply two integer and print it
---@param this number First number
---@param that number Second number
local function mul(this, that)
    print(this * that)
end

---Add two integer and print it
---@param this number First number
---@param that number Second number
---@usage `require("module.U").sum(10, 5)`
function U.sum(this, that)
    print(this + that)
end

---Subtract second from the first integer
---@param this number First number
---@param that number Second number
---@return number
---@usage [[
---local M = require("module.U")
---
---print(M.sub(10 - 5))
---@usage ]]
function U.sub(this, that)
    return this - that
end

---This is a magical function
---@param this number Non-magical number #1
---@param that number Non-magical number #2
---@return number _ The magical number #1
---@return number _ The magical number #2
---@see U.mul
---@see U.sum
---@see U.sub
U.magical = function(this, that)
    return (U.mul(this, that) / U.sum(that, this)), (U.sum(this, that) * U.sub(that, this))
end

---Trigger a rebuild of one or more projects.
---@param opts table|nil optional configuration options:
---  * {select_mode} (JdtProjectSelectMode) Show prompt
---     to select projects or select all. Defaults
---     to 'prompt'
---
---  * {full_build} (boolean) full rebuild or
---     incremental build. Defaults to true (full build)
---@param reserverd table|nil reserved for the future use
---@return boolean _ This is description of return
---statement that can be expanded to mutliple lines
function U.multi_line(opts, reserverd)
    print(vim.inspect(opts), vim.inspect(reserverd))
    return true
end

return U
```

- Output

```help
U.sum({this}, {that})                                                    *U.sum*
    Add two integer and print it

    Parameters: ~
        {this}  (number)  First number
        {that}  (number)  Second number

    Usage: ~
>lua
        require("module.U").sum(10, 5)
<

U.sub({this}, {that})                                                    *U.sub*
    Subtract second from the first integer

    Parameters: ~
        {this}  (number)  First number
        {that}  (number)  Second number

    Returns: ~
        {number}

    Usage: ~
>lua
        local M = require("module.U")

        print(M.sub(10 - 5))
<


U.magical({this}, {that})                                            *U.magical*
    This is a magical function

    Parameters: ~
        {this}  (number)  Non-magical number #1
        {that}  (number)  Non-magical number #2

    Returns: ~
        {number}  The magical number #1
        {number}  The magical number #2

    See: ~
        |U.mul|
        |U.sum|
        |U.sub|


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
        {boolean}  This is description of return
                   statement that can be expanded to mutliple lines
```

### Class

Classes can be used to better structure your code and can be referenced as an argument to a function or it's return value. You can define it once and use it multiple times.

- Syntax

```lua
---@comment
---@class [(exact)] <name>[: <parent>]
---@comment
---@field [public|protected|private] <name[?]> <type> [desc]
---@see <ref>
```

> NOTE: `---@field` and `---@see` can be used multiple times

- Input

```lua
local H = {}

---The Homosapien
---@class Human
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?
---Traits that one human can have
---It could be one, two or hundered
---@field trait table
---@field protected heart boolean Heart is protected
---@field private IQ number We need to hide this

---@class XMen : Human
---@field power number Power quantifier

---Creates a Human
---@return Human
---@usage `require('Human'):create()`
function H:create()
    return setmetatable({
        legs = 2,
        hands = 2,
        brain = false
    }, { __index = self })
end

return H
```

- Output

```help
Human                                                                    *Human*
    The Homosapien

    Fields: ~
        {legs}   (number)   Total number of legs
        {hands}  (number)   Total number of hands
        {brain}  (boolean)  Does humans have brain?
        {trait}  (table)    Traits that one human can have
                            It could be one, two or hundered


XMen : Homosapien                                                         *XMen*

    Fields: ~
        {power}  (number)  Power quantifier


H:create()                                                            *H:create*
    Creates a Human

    Returns: ~
        {Human}

    Usage: ~
>lua
        require('Human'):create()
<
```

### Type

You can use `---@type` to document static objects, constants etc.

- Syntax

```lua
---@comment
---@type <type> [desc]
---@see <tag>
---@usage `<code>`
```

- Input

```lua
local U = {}

---@class Chai Ingredients for making chai
---@field milk string 1.5 cup
---@field water string 0.5 cup
---@field sugar string 3 tablespoon
---@field tea_leaves string 2 tablespoon
---@field cardamom string 2 pieces

---A object containing the recipe for making chai
---@type Chai
U.chai = {
    milk = "1.5 Cup",
    water = "0.5 Cup",
    sugar = "3 table spoon",
    tea_leaves = "2 table spoon",
    cardamom = "2 pieces",
}

return U
```

- Output

```help
Chai                                                                      *Chai*
    Ingredients for making chai

    Fields: ~
        {milk}        (string)  1.5 cup
        {water}       (string)  0.5 cup
        {sugar}       (string)  3 tablespoon
        {tea_leaves}  (string)  2 tablespoon
        {cardamom}    (string)  2 pieces


U.chai                                                                  *U.chai*
    A object containing the recipe for making chai

    Type: ~
        (Chai)
```

### Usage

This tag is used to show code usage of functions and [`---@type`](#type). Code inside `---@usage` will be rendered as codeblock. Optionally, a `lang` can be provided to get syntax highlighting (defaults to `lua`).

- Syntax

1. Single-line

```lua
---@usage [lang] `<code>`
```

2. Multi-line

```lua
---@usage [lang] [[
---<code>...
---@usage ]]
```

- Input

```lua
local U = {}

---Prints a message
---@param msg string Message
---@usage lua [[
---require("module.U").sum(10, 5)
---@usage ]]
function U.echo(msg)
    print(msg)
end

---Add two integer and print it
---@param this number First number
---@param that number Second number
---@usage `require("module.U").sum(10, 5)`
function U.sum(this, that)
    print(this + that)
end

return U
```

- Output

```
U.echo({msg})                                                           *U.echo*
    Prints a message

    Parameters: ~
        {msg}  (string)  Message

    Usage: ~
>lua
        require("module.U").sum(10, 5)
<


U.sum({this}, {that})                                                    *U.sum*
    Add two integer and print it

    Parameters: ~
        {this}  (number)  First number
        {that}  (number)  Second number

    Usage: ~
>lua
        require("module.U").sum(10, 5)
<
```

### Alias

This tag can be used to make a type alias. It is helpful if you are using the same the type multiple times.

- Syntax

```lua
---@comment
---@alias <name> <type>
```

- Input

```lua
local U = {}

---All the lines in the buffer
---@alias Lines string[]

---Returns all the content of the buffer
---@return Lines
function U.get_all()
    return vim.api.nvim_buf_get_lines(0, 0, -1, false)
end

return U
```

- Output

```help
Lines                                                                    *Lines*
    All the lines in the buffer

    Type: ~
        string[]


U.get_all()                                                          *U.get_all*
    Returns all the content of the buffer

    Returns: ~
        {Lines}
```

### Enum

You can define a (pseudo) enum using [`---@alias`](#alias).

- Syntax

```lua
---@alias <name> <type>
---| '<literal>' [# description]
---| `<identifier>` [# description]
```

- Input

```lua
local U = {}

---Vim operator-mode motions.
---
---Read `:h map-operator`
---@alias VMode
---| '"line"' # Vertical motion
---| '"char"' # Horizontal motion
---| 'v'
---| `some.ident` # Some identifier

---Global vim mode
---@type VMode
U.VMODE = 'line'

return U
```

- Output

```help
VMode                                                                    *VMode*
    Vim operator-mode motions.

    Read `:h map-operator`

    Variants: ~
        ("line")      Vertical motion
        ("char")      Horizontal motion
        ("v")
        (some.ident)  Some identifier


U.VMODE                                                                *U.VMODE*
    Global vim mode

    Type: ~
        (VMode)
```

### Private

One of the following tags can be used to discard any part of the code that is not considered a part of the public API. All these tags behaves exactly same when it comes to vimdoc generation but have different use cases when used together with LLS.

- Spec: [`---@private`](https://github.com/sumneko/lua-language-server/wiki/Annotations#private), [`---@protected`](https://github.com/sumneko/lua-language-server/wiki/Annotations#protected), [`---@package`](https://github.com/sumneko/lua-language-server/wiki/Annotations#package)

- Syntax

```lua
---@private

---@protected

---@package
```

- Input

```lua
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
```

- Output

```help
U.ok()                                                                    *U.ok*
    Only this will be documented
```

### Export

This tag is used to manually tag the exported object. This is required for cases where `lemmy-help` is unable to parse the `return` statement at the end such as `return setmetatable(...)`. But keep in mind the following:

1. Anything after this tag is NA, so make sure this is the last tag
2. Tag should be followed by the exact identifier that needs to be exported
3. This has nothing to do with `---@mod`

- Syntax

```lua
---@export <name>
```

- Input

```lua
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
```

- Output

```help
================================================================================
Configuration                                                    *module.config*

Config:get()                                                        *Config:get*
    Get the config

    Returns: ~
        {number}
```
