local lsp_zero = require('lsp-zero')
local lspconfig = require('lspconfig')
local cmp = require("cmp")
local telescope = require('telescope.builtin')
local trouble = require('trouble.api')


lsp_zero.on_attach(function(_, bufnr)
  -- see :help lsp-zero-keybindings
  -- to learn the available actions
  lsp_zero.default_keymaps({ buffer = bufnr })
  vim.keymap.set('n', '<C-S-i>', function()
    vim.lsp.buf.format({ async = false, timeout_ms = 1000 })
  end, { desc = "Format code", buffer = bufnr })
  vim.keymap.set('n', '<C-Space>', function()
    vim.lsp.buf.code_action()
  end, { desc = "Apply code action", buffer = bufnr })
  vim.keymap.set('n', '<leader>cp', function()
    trouble.toggle({
      mode = 'diagnostics',
      desc = "Code Problems",
      filter = { buf = bufnr },
      focus = true
    })
  end, { desc = "Show code problems for this buffer", buffer = bufnr })
  vim.keymap.set('n', '<leader>cgp', function()
    trouble.toggle({
      mode = 'diagnostics',
      desc = "Code Problems",
      focus = true,
    })
  end, { desc = "Show all code problems" })
  vim.keymap.set('n', '<leader>cs', function()
    trouble.toggle({
      mode = 'symbols',
      desc = "Code Symbols",
      focus = true,
    })
  end, { desc = "Show code symbols" })
  vim.keymap.set('n', 'gd', function()
    telescope.lsp_definitions {
      initial_mode = "normal",
    }
  end, { desc = "Go to definition", buffer = bufnr })
  vim.keymap.set('n', 'gr', function()
    telescope.lsp_references {
      initial_mode = "normal",
    }
  end, { desc = "Find all references" })
end)

-- capabilities.workspace = {
--   didChangeWatchedFiles = {
--     dynamicRegistration = true,
--   },
-- }

cmp.setup {
  mapping = cmp.mapping.preset.insert({
    ["<CR>"] = cmp.mapping.confirm({
      select = true,
      behavior = cmp.ConfirmBehavior.Replace
    }),
    ['<Tab>'] = lsp_zero.cmp_action().tab_complete(),
    ['<S-Tab>'] = lsp_zero.cmp_action().select_prev_or_fallback(),
    ['<C-Space>'] = cmp.mapping.complete(),
  }),
  sources = {
    { name = "copilot",  group_index = 2 },
    -- Other Sources
    { name = "nvim_lsp", group_index = 2 },
    { name = "path",     group_index = 2 },
    { name = "luasnip",  group_index = 2 },
  },
}
require('copilot_cmp').setup {}


lspconfig.rust_analyzer.setup {
  settings = {
    ["rust-analyzer"] = {
      -- Other Settings ...
      procMacro = {
        ignored = {
          -- leptos_macro = {
          --   -- optional: --
          --   -- "component",
          --   "server",
          -- },
        },
      },
      cargo = {
        loadOutDirsFromCheck = true,
        allFeatures = true,
      },
    },
  }
}
lspconfig.clangd.setup {}
lspconfig.lua_ls.setup {}
lspconfig.pyright.setup {}
lspconfig.ruff.setup {}
lspconfig.volar.setup {}
lspconfig.kotlin_language_server.setup {}
lspconfig.texlab.setup {}
lspconfig.marksman.setup {}
lspconfig.terraformls.setup {}
lspconfig.gopls.setup {}
-- lspconfig.elixirls.setup {}
lspconfig.lexical.setup {
  cmd = { "lexical" }
}
lspconfig.ts_ls.setup {
  init_options = {
    plugins = {
      {
        name = "@vue/typescript-plugin",
        location = "/usr/lib/node_modules/@vue/typescript-plugin/",
        languages = { "javascript", "typescript", "vue" },
      },
    },
  },
  filetypes = {
    "javascript",
    "typescript",
    "vue",
  },
}
lspconfig.helm_ls.setup {
  settings = {
    ['helm-ls'] = {
      yamlls = {
        path = "yaml-language-server",
      }
    }
  }
}
lspconfig.yamlls.setup {}


require('lsp_signature').setup {
  bind = true,
  hint_enable = false,
  handler_opts = {
    border = "rounded",
  },
  always_trigger = true,
}

vim.diagnostic.config({
  virtual_text = true,
  signs = true,
  update_in_insert = false,
  underline = true,
  severity_sort = true,
  float = true,
})
