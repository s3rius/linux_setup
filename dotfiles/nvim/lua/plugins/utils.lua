return {
  {
    "dstein64/vim-startuptime",
    -- lazy-load on a command
    cmd = "StartupTime",
    -- init is called during startup. Configuration for vim plugins typically should be set in an init function
    init = function()
      vim.g.startuptime_tries = 10
    end,
  },
  {
    "nvim-tree/nvim-tree.lua",
    version = "*",
    lazy = false,
    dependencies = {
      "nvim-tree/nvim-web-devicons",
    },
    config = function()
      require("nvim-tree").setup {
        filters = {
          enable = true,
          git_ignored = true,
          dotfiles = false,
          git_clean = false,
          no_buffer = false,
          custom = {
            "^CVS$",
            "^__pycache__$",
            "^\\.git$",
            "^\\.svn$",
            "^\\.hg$",
            "^\\.DS_Store$",
            "^\\.classpath$",
            "^\\.project$",
            "^\\.venv$",
            "^\\.pytest_cache$",
            "^\\.mypy_cache$",
            "^\\.ruff_cache$",
            "^\\.terragrunt-cache$"
          }
        }
      }
      vim.keymap.set('n', '<leader>tt', function()
        vim.cmd [[NvimTreeToggle]]
      end, { desc = "Toggle file tree" });
      vim.keymap.set('n', '<leader>tf', function()
        vim.cmd [[NvimTreeFocus]]
      end, { desc = "Focus file tree" });
    end,
  },
  {
    'romgrk/barbar.nvim',
    dependencies = {
      'lewis6991/gitsigns.nvim',     -- OPTIONAL: for git status
      'nvim-tree/nvim-web-devicons', -- OPTIONAL: for file icons
    },
    init = function()
      vim.g.barbar_auto_setup = false;
      vim.keymap.set('n', '<leader>bp', function()
        vim.cmd [[BufferPrevious]];
      end, { noremap = true, silent = true, desc = "Previous buffer" })
      -- vim.keymap.set('n', '<leader>bn', function()
      --   vim.cmd [[BufferNext]];
      -- end, { noremap = true, silent = true, desc = "Next buffer" })
      vim.keymap.set('n', '<leader>bc', function()
        vim.cmd [[BufferClose]];
      end, { noremap = true, silent = true, desc = "Close active buffer" })
      vim.keymap.set('n', '<leader>bk', function()
        vim.cmd [[BufferClose!]]
      end, { noremap = true, silent = true, desc = "Force kill the buffer" })
      vim.keymap.set('n', '<leader>bs', function()
        vim.cmd [[BufferPick]];
      end, { noremap = true, silent = true, desc = "Switch active buffer" })
    end,
    opts = {
      -- lazy.nvim will automatically call setup for you. put your options here, anything missing will use the default:
      -- animation = true,
      -- insert_at_start = true,
      -- …etc.
      animation = false,
    },
  },
  {
    "hadronized/hop.nvim",
    init = function()
      require('hop').setup {};
      vim.keymap.set('n', '<leader><leader>w', function()
        vim.cmd [[HopWordAC]];
      end, { desc = "EasyMotion forward" });
      vim.keymap.set('n', '<leader><leader>b', function()
        vim.cmd [[HopWordBC]];
      end, { desc = "Easy motion backward" });
    end,
  },
  -- I have a separate config.mappings file where I require which-key.
  -- With lazy the plugin will be automatically loaded when it is required somewhere
  {
    'akinsho/toggleterm.nvim',
    config = true,
    opts = {
      open_mapping = [[<C-`>]],
      shell = vim.o.shell,
      title_pos = 'left',
    }
  },
  {
    'windwp/nvim-autopairs',
    event = "InsertEnter",
    config = true
  },
  { "stevearc/dressing.nvim", event = "VeryLazy" },
  {
    "folke/which-key.nvim",
    event = "VeryLazy",
    init = function()
      vim.o.timeout = true
      vim.o.timeoutlen = 300
    end,
    opts = {
      -- your configuration comes here
      -- or leave it empty to use the default settings
      -- refer to the configuration section below
    }
  },
  {
    'nvim-telescope/telescope.nvim',
    branch = '0.1.x',
    dependencies = { 'nvim-lua/plenary.nvim' },
    opts = {
      defaults = {
        initial_mode = "insert"
      },
    },
    init = function()
      require('telescope').load_extension('bufhistory')
      local telescope = require("telescope.builtin")
      vim.keymap.set('n', '<leader>ff', telescope.find_files, { desc = "Find files" })
      vim.keymap.set('n', '<leader>fg', telescope.live_grep, { desc = "Live grep" })
      vim.keymap.set('n', '<leader>bf', telescope.buffers, { desc = "Search buffers" })
      vim.keymap.set('n', '<leader>fh', telescope.help_tags, { desc = "Search help tags" })
      vim.keymap.set('n', '<leader>fo', telescope.oldfiles, { desc = "Old files" })
      vim.keymap.set('n', '<leader>fr', telescope.resume, { desc = "Resume" })
      vim.keymap.set('n', '<leader>bl', function()
        require('telescope').extensions.bufhistory.show({
          initial_mode = "normal",
          attach_mappings = function(prompt_bufnr, map)
            local action_state = require("telescope.actions.state")
            local action = require("telescope.actions")
            map("n", "c", function(_prompt_bufnr)
              print("TODO!")
              action.remove_selection(prompt_bufnr)
              require("bufhistory.api").close_by_index(action_state.get_selected_entry()["index"])
            end)
            return true
          end

        })
      end, { silent = true, noremap = true, desc = "List buffers" })
    end
  },
  {
    name = "bufhistory",
    dir = vim.fs.joinpath(vim.fs.dirname(vim.fn.expand('$MYVIMRC')), 'plugs/bufhistory'),
    opts = {},
    init = function()
      local api = require('bufhistory.api')
      vim.keymap.set('n', '<C-S-t>', function()
        api.restore_file()
      end, { silent = true, noremap = true, desc = "Restore closed buffer" })
    end
  },
{
    "3rd/image.nvim",
    build = false, -- so that it doesn't build the rock https://github.com/3rd/image.nvim/issues/91#issuecomment-2453430239
    opts = {
        processor = "magick_cli",
    }
}
}
