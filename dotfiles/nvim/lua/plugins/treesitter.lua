return {
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
    config = function()
      local configs = require("nvim-treesitter.configs")

      configs.setup({
        ensure_installed = {
          "c",
          "lua",
          "vim",
          "vimdoc",
          "query",
          "elixir",
          "heex",
          "javascript",
          "html",
          "python",
          "rust",
          "hcl",
          "terraform",
        },
        sync_install = false,
        highlight = { enable = true },
        indent = { enable = true },
        auto_install = true,
        ignore_install = {},
        modules = {}
      })
    end
  } }
