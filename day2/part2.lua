-- A rock, B paper, C scissors
-- X rock, Y paper, Z scissors

lose = {
  ["A"] = "Z",
  ["B"] = "X",
  ["C"] = "Y",
}

win = {
  ["A"] = "Y",
  ["B"] = "Z",
  ["C"] = "X",
}

draw = {
  ["A"] = "X",
  ["B"] = "Y",
  ["C"] = "Z",
}

shapeScore = {
  ["X"] = 1,
  ["Y"] = 2,
  ["Z"] = 3,
}

outcomes = {
  ["X"] = 0, -- lose
  ["Y"] = 3, -- draw
  ["Z"] = 6, -- win
}

total = 0
for line in io.lines("input.txt") do
  p1, outcome = string.match(line, "(%a) (%a)")
  total = total + outcomes[outcome]

  if outcome == "X" then
    total = total + shapeScore[lose[p1]]
  elseif outcome == "Y" then
    total = total + shapeScore[draw[p1]]
  else
    total = total + shapeScore[win[p1]]
  end
end

print(total)
