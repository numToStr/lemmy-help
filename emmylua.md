## Writing Emmylua

- Spec - https://emmylua.github.io/annotation.html
- LLS - https://github.com/sumneko/lua-language-server/wiki/EmmyLua-Annotations

> NOTE: `lemmy-help` follows LLS implementation more closely than the spec

Following are the tags that you can use to create docs

#### Brief

A `brief` can be used to describe a module or even to add some footnote etc.

```lua
---@brief [[
---@comment
---@brief ]]
```

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

#### Module

This tag can be used to add a heading for the module and change the prefix of every exported _function and type_.

> NOTE: This should be defined **at the top** of the file.

```lua
---@mod <name> [desc]
```

```lua
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
Human module                                                         *mod.Human*

Human                                                                    *Human*
    The Homosapien

    Fields: ~
        {legs}   (number)   Total number of legs
        {hands}  (number)   Total number of hands
        {brain}  (boolean)  Does humans have brain?


U.DEFAULT                                                    *mod.Human.DEFAULT*
    Default traits of a human

    Type: ~
        (Human)


U:create()                                                    *mod.Human:create*
    Creates a Human

    Returns: ~
        {Human}

    Usage: ~
        >
            require('Human'):create()
        <
```

#### Tag

This can used to create an alternate tag for your module, functions etc.

```lua
---@tag cool-tag
---@tag another-cool-tag
```

```
                                                                      *cool-tag*
                                                              *another-cool-tag*
```

#### Divider

This tag can be used to add a divider/separator between section or anything you desire

```lua
---@divider <char>
```

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

#### Functions

A function contains multiple tags which form its structure. Like `---@param` for parameter, `---@return` for the return value, `---@see` for other related things and `---@usage` for example

```lua
---@comment
---@param <name> <type> <desc>
---@return <type> <name> <desc>
---@see <ref>
---@usage `<code>`
```

> NOTE: All tag can be used multiple times except `---@usage`

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

#### Class

Classes can be used to better structure your code and can be referenced as an argument to a function or it's return value. You can define it once and use it multiple times.

```lua
---@class <name> <desc>
---@field <name> <type> <desc>
---@see <ref>
```

> NOTE: `---@field` and `---@see` can be used multiple times

```lua
local H = {}

---@class Human The Homosapien
---@field legs number Total number of legs
---@field hands number Total number of hands
---@field brain boolean Does humans have brain?

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


H:create()                                                            *H:create*
    Creates a Human

    Returns: ~
        {Human}

    Usage: ~
        >
            require('Human'):create()
        <
```

#### Type

You can use `---@type` to document static objects, constants etc.

```lua
---@comment
---@type <type> <desc>
---@usage `<code>`
```

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

#### Alias

This can be used to make a type alias. It is helpful if you are using the same the type multiple times.

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

### Private

You can use `---@private` tag to discard any part of the code that is exported but it is not considered to be a part of the public API

> NOTE:

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
