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

-- find max the imperative way
mostCals = -math.huge
for _, elf in ipairs(elves) do
  local total = 0

  for _,snack in ipairs(elf) do
    total = total + snack
  end

  if total > mostCals then mostCals = total end
end

print(mostCals)
