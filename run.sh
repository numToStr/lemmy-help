#!/bin/bash

cargo rr -- -fact \
    ~/Code/Comment.nvim/lua/Comment/{init.lua,config.lua} ~/Code/Comment.nvim/plugin/Comment.lua \
    ~/Code/Comment.nvim/lua/Comment/{api.lua,ft.lua,utils.lua,opfunc.lua,extra.lua} > ~/Code/Comment.nvim/doc/Comment.txt
