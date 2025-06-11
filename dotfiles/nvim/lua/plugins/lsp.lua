return {
  {
    "folke/lazydev.nvim",
    ft = "lua", -- only load on lua files
    opts = {
      library = {
        { path = "LazyVim",            words = { "LazyVim" } },
        -- See the configuration section for more details
        -- Load luvit types when the `vim.uv` word is found
        { path = "luvit-meta/library", words = { "vim%.uv" } },
      },
      enabled = function(root_dir)
        return not vim.uv.fs_stat(root_dir .. "/.luarc.json")
      end,
    },
  },
  { 'VonHeikemen/lsp-zero.nvim', branch = 'v3.x' },
  { 'neovim/nvim-lspconfig' },
  { 'hrsh7th/cmp-nvim-lsp' },
  { 'hrsh7th/nvim-cmp',          commit = "7e348da" },
  { 'L3MON4D3/LuaSnip' },
  {
    'zbirenbaum/copilot.lua',
    opts = {
      suggestion = { enabled = false },
      panel = { enabled = false },
      filetypes = { markdown = true }
    },
  },
  {
    "zbirenbaum/copilot-cmp",
  },
  {
    'towolf/vim-helm',
    -- To not start yamlls on helm files
    ft = 'helm'
  },
  {
    "echasnovski/mini.comment",
    event = "VeryLazy",
    opts = {
      options = {
        custom_commentstring = function()
          return require("ts_context_commentstring.internal").calculate_commentstring() or
              vim.bo.commentstring
        end,
        mappings = {
          comment = 'gc',
          comment_line = 'gcc',
          comment_visual = 'gc'
        }
      },
    }
  },
  {
    "JoosepAlviste/nvim-ts-context-commentstring",
    lazy = true,
    opts = {
      enable_autocmd = false,
    },
  },
  {
    "folke/trouble.nvim",
    cmd = "Trouble",
    opts = {
      auto_close = true,
      focus = true,
    },
  },
  {
    "ray-x/lsp_signature.nvim",
    event = "VeryLazy",
    opts = {},
    config = function(_, opts) require('lsp_signature').setup(opts) end
  },
  {
    "linux-cultist/venv-selector.nvim",
    branch = "regexp", -- This is the regexp branch, use this for the new version
    dependencies = {
      "neovim/nvim-lspconfig",
      "mfussenegger/nvim-dap", "mfussenegger/nvim-dap-python", --optional
      { "nvim-telescope/telescope.nvim", branch = "0.1.x", dependencies = { "nvim-lua/plenary.nvim" } },
    },
    lazy = false,
    keys = {
      -- Keymap to open VenvSelector to pick a venv.
      { '<leader>vs', '<cmd>VenvSelect<cr>' },
      -- Keymap to retrieve the venv from a cache (the one previously used for the same project directory).
      { '<leader>vc', '<cmd>VenvSelectCached<cr>' },
    },
    ---@type venv-selector.Config
    opts = {
      -- Your settings go here
    },
  },
}
