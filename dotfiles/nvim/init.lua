require("config.lazy")
require("config.lsp")

-- Optional module, can be
-- not available in your setup
pcall(require, "config.intree")

vim.o.tabstop = 2;
vim.o.number = true
vim.o.shiftwidth = 2
vim.o.expandtab = true
vim.o.softtabstop = 2
vim.opt.clipboard = "unnamedplus"
