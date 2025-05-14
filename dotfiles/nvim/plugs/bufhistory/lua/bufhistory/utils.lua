local M = {}

--- Check if tabkle contains the element
--- @generic T
--- @param table T[]
--- @param element T
--- @return boolean
function M.table_contains(table, element)
  for _, value in pairs(table) do
    if value == element then
      return true
    end
  end
  return false
end

--- Remove a value from the table
--- @generic T
--- @param t T[] list of items
--- @param f fun(val: T): boolean predicate based on it elements are deleted
--- @return T[]
function M.table_remove_by(t, f)
  local res = {}
  for _, val in ipairs(t) do
    if not f(val) then
      table.insert(res, val)
    end
  end
  return res
end

--- Returns list of names
--- @generic T
--- @generic G
--- @param entries T[]
--- @param f fun(val: T): G
--- @return G[]
function M.map(entries, f)
  local res = {}
  for _, value in pairs(entries) do
    table.insert(res, f(value))
  end
  return res
end

--- Reverses a list inplace
--- @generic T: any
--- @param list T[]
--- @return T[]
function M.reverse(list)
  local res = {}
  for i = 0, #list - 1 do
    table.insert(res, list[#list - i])
  end
  return res
end

--- Remove deleted files
--- @param list string[]
--- @return string[]
function M.remove_deleted_files(list)
  return M.table_remove_by(list, function(val)
    return vim.fn.filereadable(val) == 0
  end)
end

return M
