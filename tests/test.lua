---@mod awesome.name Awesome module title

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

---@divider -
---@divider =
---@divider ~

---@tag cool-tag
---@tag another-cool-tag

---NOTE: Local functions are not part of the documentation
---Multiply two integar and print it
---@param this number First number
---@param that number Second number
local function mul(this, that)
    print(this * that)
end

---Add two integar and print it
---@param this number First number
---@param that number Second number
function U.sum(this, that)
    print(this + that)
end

---Subtract second from the first integar
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

---@class Human The Homosapien
---@field legs number Total number of legs
---@field public hands number Total number of hands
---@field brain boolean Does humans have brain?
---@field protected heart boolean Heart is protected
---@field private IQ number We need to hide this

---Creates a Human
---@return Human
---@usage `require('Human'):create()`
function U:create()
    return setmetatable({
        legs = 2,
        hands = 2,
        brain = false,
        heart = true,
        IQ = -1,
    }, { __index = self })
end

---@class Chai Ingredients for making chai
---@field milk string 1.5 cup
---@field water string 0.5 cup
---@field sugar string 3 tablespoon
---@field tea_leaves string 2 tablespoon
---@field cardamom string 2 pieces

---A object containing the recipe for making chai
---@type Chai
U.chai = {
    milk = '1.5 Cup',
    water = '0.5 Cup',
    sugar = '3 tablespoon',
    tea_leaves = '2 tablespoon',
    cardamom = '2 pieces',
}

---@alias Lines string[] All the lines in the buffer

---Returns all the content of the buffer
---@return Lines
function U.get_all()
    return vim.api.nvim_buf_get_lines(0, 0, -1, false)
end

---@private
---This is a private function which is exported
---But not considered as part of the API
function U.private()
    print('I am private!')
end

---@private
function U.no_emmy()
    print('Private func with no emmylua!')
end

return U
