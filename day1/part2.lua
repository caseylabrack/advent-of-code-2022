elves = {}
table.insert(elves, {})

-- build array (elves) of arrays (snacks)
for line in io.lines("input.txt") do
  if line == "" then
    table.insert(elves, {})
  else
    table.insert(elves[#elves], line + 0)
  end
end

-- flatten elves into array of totals
flatElves = {}
for _,elf in ipairs(elves) do
  local total = 0
  for _,snack in ipairs(elf) do
    total = total + snack
  end
  table.insert(flatElves, total)
end

table.sort(flatElves, function (a,b) return a > b end) -- descending (default is ascending)

print(flatElves[1] + flatElves[2] + flatElves[3])
