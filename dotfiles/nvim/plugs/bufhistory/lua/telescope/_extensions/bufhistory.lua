local pickers    = require("telescope.pickers")
local finders    = require("telescope.finders")
local config     = require('telescope.config')
local make_entry = require('telescope.make_entry')
local bufhistory = require('bufhistory')
local utils      = require("bufhistory.utils")

local function show_bufhistory(orig_opts)
  local opts = orig_opts or {}
  local filemap = utils.map(bufhistory.history, function(val)
    return val[2]
  end)
  pickers.new(opts, {
    prompt_title = "Recent files",
    finder = finders.new_table {
      results = utils.reverse(utils.remove_deleted_files(filemap)),
      entry_maker = make_entry.gen_from_file(opts)
    },
    previewer = config.values.file_previewer(opts),
    default_selection_index = math.min(#bufhistory.history, 2),
  }):find()
end

return require('telescope').register_extension {
  exports = { show = show_bufhistory }
}
