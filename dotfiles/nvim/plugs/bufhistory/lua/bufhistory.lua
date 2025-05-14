local utils = require("bufhistory.utils")

--- @class SetupOptions
--- @field max_closed integer
--- @field skip_dirs boolean
local defaults = {
  max_closed = 100,
  skip_dirs = true,
}

--- @class mainMod
--- @field history [number, string][]
--- @field closed string[]
--- @field options SetupOptions
local M = {
  history = {},
  closed = {},
  options = defaults,
}

--- Setup the plugin
--- @param opts SetupOptions
function M.setup(opts)
  M.options = vim.tbl_deep_extend('force', {}, defaults, opts or {})
end

--- When entering a buffer
--- @param path string
--- @param bufrn number
function M.buf_entered(path, bufrn)
  if path == "" then
    return
  end
  if vim.fn.filereadable(path) == 0 then
    return
  end
  M.history = utils.table_remove_by(M.history, function(p)
    return p[2] == path
  end);
  table.insert(M.history, { bufrn, path })
end

--- When closing buffer
--- @param path string
function M.buf_closed(path)
  if vim.fn.filereadable(path) == 0 then
    return
  end
  table.insert(M.closed, path)
  M.history = utils.table_remove_by(M.history, function(p)
    return p[2] == path
  end)
end

return M
