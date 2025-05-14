local bufhistory = require('bufhistory')
local utils = require('bufhistory.utils')

local M = {}


function M.restore_file()
  if #bufhistory.closed == 0 then
    return
  end
  bufhistory.closed = utils.remove_deleted_files(bufhistory.closed)
  local last_path = bufhistory.closed[#bufhistory.closed]
  table.remove(bufhistory.closed, #bufhistory.closed)
  vim.cmd('e ' .. last_path)
end

function M.close_by_index(index)
  vim.api.nvim_buf_delete(bufhistory.history[index][1], { force = true })
end

return M
