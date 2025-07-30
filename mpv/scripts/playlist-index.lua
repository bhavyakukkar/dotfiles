function on_index_show(name, value)
  if value >= 0 then print('--------> ' .. value) end
end

mp.observe_property("playlist-pos", "number", on_index_show)
