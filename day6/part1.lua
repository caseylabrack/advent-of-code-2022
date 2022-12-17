-- input = io.open("test.txt"):read("*all")
input = io.open("input.txt"):read("*all")

l = string.len(input) - 4
for i=1,l do

  chars = string.sub(input, i, i+3)
  t = {}
  for j=1,4 do
    char = string.sub(chars,j,j)
    t[char] = 1 + (t[char] or 0)
  end

  allAreUnique = true
  for k,v in pairs(t) do
    if v > 1 then
      allAreUnique = false
      break
    end
  end

  if allAreUnique then
    print(i+3)
    break
  end
end
