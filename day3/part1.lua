alfa = "abcdefghijklmnopqrstuvwxyz"

priority = 0
for line in io.lines("input.txt") do
  length = string.len(line)
  c1,c2 = string.sub(line, 1, length/2), string.sub(line, length/2 + 1)
  dupe = ""
  for i=1, string.len(c1) do
    local char = string.sub(c1, i, i)
    for j=1, string.len(c2) do
      if string.sub(c2, j, j) == char then dupe = char goto founddupe end
    end
  end
  ::founddupe::

  isUpperCase = string.upper(dupe) == dupe
  alfaIndex = string.find(alfa, string.lower(dupe))

  priority = priority + alfaIndex + (isUpperCase and 26 or 0)
end

print(priority)
