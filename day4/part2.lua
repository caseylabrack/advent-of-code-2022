overlaps = 0
for line in io.lines("input.txt") do
  x1,x2,y1,y2 = string.match(line, "(%d+)-(%d+),(%d+)-(%d+)")
  x1,x2,y1,y2 = x1 + 0, x2 + 0, y1 + 0, y2 + 0 -- convert to number
  if x2-x1 > y2-y1 then -- first segment is longest
    if (y1 >= x1 and y1 <= x2) or (y2 <= x2 and y2 >= x1) then --if either end of second segment in this range, then it's an overlap
      overlaps = overlaps + 1
    end
  else -- second segment longest
    if (x1 >= y1 and x1 <= y2) or (x2 <= y2 and x2 >= y1) then
      overlaps = overlaps + 1
    end
  end
end

print(overlaps)
