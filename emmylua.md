## Writing Emmylua

- Spec - https://emmylua.github.io/annotation.html
- LLS - https://github.com/sumneko/lua-language-server/wiki/EmmyLua-Annotations

> NOTE: `lemmy-help` follows LLS implementation more closely than the spec

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
        >
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

A function contains multiple tags which form its structure. Like `---@param` for parameter, `---@return` for the return value, `---@see` for other related things and `---@usage` for example

- Syntax

```lua
---@comment
---@param <name> <type> [desc]
---@comment
---@return <type> [name] [desc]
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
function U.sum(this, that)
    print(this + that)
end

---Subtract second from the first integer
---@param this number First number
---@param that number Second number
---@return number
---@usage `require("module.U").sub(10, 5)`
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

return U
```

- Output

```help
U.sum({this}, {that})                                                    *U.sum*
    Add two integer and print it

    Parameters: ~
        {this}  (number)  First number
        {that}  (number)  Second number


U.sub({this}, {that})                                                    *U.sub*
    Subtract second from the first integer

    Parameters: ~
        {this}  (number)  First number
        {that}  (number)  Second number

    Returns: ~
        {number}

    Usage: ~
        >
            require("module.U").sub(10, 5)
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
```

### Class

Classes can be used to better structure your code and can be referenced as an argument to a function or it's return value. You can define it once and use it multiple times.

- Syntax

```lua
---@class <name> [desc]
---@comment
---@field [public|protected|private] <name> <type> [desc]
---@see <ref>
```

> NOTE: `---@field` and `---@see` can be used multiple times

- Input

```lua
local H = {}

---@class Human The Homosapien
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?
---Traits that one human can have
---It could be one, two or hundered
---@field trait table
---@field protected heart boolean Heart is protected
---@field private IQ number We need to hide this

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


H:create()                                                            *H:create*
    Creates a Human

    Returns: ~
        {Human}

    Usage: ~
        >
            require('Human'):create()
        <
```

### Type

You can use `---@type` to document static objects, constants etc.

- Syntax

```lua
---@comment
---@type <type> [desc]
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

### Alias

This tag can be used to make a type alias. It is helpful if you are using the same the type multiple times.

- Syntax

```lua
---@alias <name> <type> [desc]
```

- Input

```lua
local U = {}

---@alias Lines string[] All the lines in the buffer

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

- Input

```lua
local U = {}

---Vim operator-mode motions.
---
---Read `:h map-operator`
---@alias VMode
---| 'line' Vertical motion
---| 'char' Horizontal motion
---| 'v'
---| 'V' # Visual Line Mode

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
        ('line')  Vertical motion
        ('char')  Horizontal motion
        ('v')
        ('V')     Visual Line Mode


U.VMODE                                                                *U.VMODE*
    Global vim mode

    Type: ~
        (VMode)
```

### Private

This tag can be used to discard any part of the code that is exported but it is not considered to be a part of the public API

- Syntax

```lua
---@private
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

---@private
function U.no_emmy()
    print('Private func with no emmylua!')
end

return U
```

- Output

```help
U.ok()                                                                    *U.ok*
    Only this will be documented
```
