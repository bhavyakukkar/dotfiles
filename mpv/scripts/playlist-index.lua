local count = nil
local index = nil

local function on_index_show()
  if index and index >= 0 and count and count >= 0 and index < count then
    print('----------------------------------------------> \27[1m[\27[0m ' ..
      index .. '. ' ..
      '\27[35;1m↓' .. (index + 1) .. '\27[0m ' ..
      '\27[32;1m↑' .. (count - index) .. '\27[0m ' ..
      '\27[33;1m/' .. count .. '\27[0m \27[1m]\27[0m')
  end
end

mp.observe_property("playlist-count", "number", function(name, value)
  count = value
  on_index_show()
end)
mp.observe_property("playlist-pos", "number", function(name, value)
  index = value
  on_index_show()
end)
