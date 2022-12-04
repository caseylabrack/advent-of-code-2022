contained = 0
for line in io.lines("input.txt") do
  x1,x2,y1,y2 = string.match(line, "(%d+)-(%d+),(%d+)-(%d+)")
  x1,x2,y1,y2 = x1 + 0, x2 + 0, y1 + 0, y2 + 0 -- convert to number
  if (x1 <= y1 and x2 >= y2) or (y1 <= x1 and y2 >= x2) then
    contained = contained + 1
  end
end

print(contained)
