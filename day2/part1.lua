-- A rock, B paper, C scissors
-- X rock, Y paper, Z scissors

beatenBy = {
  ["X"] = "B",
  ["Y"] = "C",
  ["Z"] = "A",
}

beats = {
  ["X"] = "C",
  ["Y"] = "A",
  ["Z"] = "B",
}

shapeScore = {
  ["X"] = 1,
  ["Y"] = 2,
  ["Z"] = 3,
}

total = 0
for line in io.lines("input.txt") do
  p1, p2 = string.match(line, "(%a) (%a)")
  total = total + shapeScore[p2]
  if beatenBy[p2] == p1 then
    total = total + 0
  elseif beats[p2] == p1 then
    total = total + 6
  else
    total = total + 3
  end
end

print(total)
