priority = 0
group = {}
for line in io.lines("input.txt") do
  table.insert(group, line)

  if #group == 3 then

    -- sort group by length descending
    -- ruck sacks need to be ordered by length to guarantee every letter is checked in loops
    table.sort(group, function (a,b) return string.len(a) > string.len(b) end)

    -- find the duplicate character in the three strings
    local dupe = ""
    for i=1,string.len(group[1]) do
      for j=1,string.len(group[2]) do
        for l=1,string.len(group[3]) do
          local char = string.sub(group[1], i, i)
          if char == string.sub(group[2], j, j) and char == string.sub(group[3], l, l) then
            dupe = char
            goto gottem
          end
        end
      end
    end
    ::gottem::

    -- convert duplicate character to a priority score and update total
    isUpperCase = string.upper(dupe) == dupe
    alfaIndex = string.find("abcdefghijklmnopqrstuvwxyz", string.lower(dupe))
    priority = priority + alfaIndex + (isUpperCase and 26 or 0)

    -- start new group
    group = {}
  end
end

print(priority)
