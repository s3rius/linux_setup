local bufhistory = require('bufhistory')

local autogrp = vim.api.nvim_create_augroup('bufhistory', { clear = true })

vim.api.nvim_create_autocmd('BufEnter', {
  desc = "Entering buffers",
  group = autogrp,
  callback = function()
    local bufrn = tonumber(vim.fn.expand('<abuf>'))
    local current_file = vim.fn.expand('<afile>')
    bufhistory.buf_entered(current_file, bufrn)
  end
})

vim.api.nvim_create_autocmd('BufDelete', {
  desc = "Closing buffer",
  group = autogrp,
  callback = function()
    local current_file = vim.fn.expand('<afile>')
    bufhistory.buf_closed(current_file)
  end
})


