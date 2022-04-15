local U = {}

---@brief [[
--- This will document a module and will be found at the top of each file. It uses an internal markdown renderer
--- so you don't need to worry about formatting. It will wrap the lines into one paragraph and
--- will make sure that the max line width is < 80.
---
--- To start a new paragraph with a newline.
---
--- To explicitly do a breakline do a `<br>` at the end.<br>
--- This is useful sometimes
---
--- We also support itemize and enumerate
--- - Item 1
---   - Item 1.1 This item will be wrapped as well and the result will be as expected. This is really handy.
---     - Item 1.1.1
---   - Item 1.2
--- - Item 2
---
--- 1. Item
---   1.1. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et
---   aliquyam erat, sed diam voluptua.
---     1.1.1. Item
---   1.2. Item
--- 2. Item
---
--- <pre>
--- You can disable formatting with a
--- pre block.
--- This is useful if you want to draw a table or write some code
--- </pre>
---
---@brief ]]

print("---")

---@tag utils

print("---")

---@class CMode Comment modes - Can be manual or computed in operator-pending phase
---@field toggle number Toggle action
---@field comment number Comment action
---@field uncomment number Uncomment action
---@see VMode
---@see Mee

print("---")

---This is an amazing type and you should use it
---@type CMode this is something
U.cmode = {
	toggle = 0,
	comment = 1,
	uncomment = 2,
}

---This is an amazing type and you should use it
---@type CMode this is something
local cmode = {
	toggle = 0,
	comment = 1,
	uncomment = 2,
}

print("---")

---@alias VMode '"line"'|'"char"'|'"v"'|'"V"' Vim Mode. Read `:h map-operator`

print("---")

---@see math.min

print("---")

---Takes out the leading indent from lines
---@param str string
---@return string string Indent chars
---@return number string Length of the indent chars
---@see VMode
---@see Mee
function U.grab_indent(str)
	local _, len, indent = str:find("^(%s*)")
	return indent, len
end

---Takes out the leading indent from lines
---@param str string
---@return string string Indent chars
---@return number string Length of the indent chars
---@see VMode
---@see Mee
local function grab_indent2(str)
	local _, len, indent = str:find("^(%s*)")
	return indent, len
end

return U
