-- file = "test.txt"
file = "input.txt"

grid = {}
width = string.len(io.open(file):read("*line"))
row = 1

-- parse input into 2d array
for line in io.lines(file) do

  grid[row] = {}
  for i=1,width do
    table.insert(grid[row], string.sub(line,i,i) + 0)
  end

  row = row + 1
end

-- test every tree for visibility
visibleCount = 0
for row=1,width do
  for col=1,width do
    tree = grid[row][col]
    visible = true

    -- check right
    for i=col+1,width do
      if grid[row][i] >= tree then
        visible = false
        break
      end
    end
    if visible then goto out end

    -- check left
    visible = true
    for i=col-1,1,-1 do
      if grid[row][i] and grid[row][i] >= tree then
        visible = false
        break
      end
    end
    if visible then goto out end

    -- check down
    visible = true
    for i=row+1,width do
      if grid[i] and grid[i][col] >= tree then
        visible = false
        break
      end
    end
    if visible then goto out end

    -- check up
    visible = true
    for i=row-1,1,-1 do
      if grid[i] and grid[i][col] >= tree then
        visible = false
        break
      end
    end
    if visible then goto out end

    ::out::
    -- if visible then print("big tree: ", tree) end
    visibleCount = visibleCount + (visible and 1 or 0)
  end
end

print(visibleCount)
