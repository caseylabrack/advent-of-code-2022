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

-- test each tree and save highest scenic score
highestScenicScore = -math.huge
for row=1,width do
  for col=1,width do
    score = 0
    tree = grid[row][col]

    -- check right
    rightScore = 0
    for i=col+1,width do
      rightScore = rightScore + 1
      if grid[row][i] >= tree then
        break
      end
    end

    -- check left
    leftScore = 0
    for i=col-1,1,-1 do
      leftScore = leftScore + 1
      if grid[row][i] and grid[row][i] >= tree then
        break
      end
    end

    -- check down
    downScore = 0
    for i=row+1,width do
      downScore = downScore + 1
      if grid[i] and grid[i][col] >= tree then
        break
      end
    end

    -- check up
    upScore = 0
    for i=row-1,1,-1 do
      upScore = upScore + 1
      if grid[i] and grid[i][col] >= tree then
        break
      end
    end

    thisSceneScore = rightScore * leftScore * downScore * upScore
    if highestScenicScore < thisSceneScore then
      highestScenicScore = thisSceneScore
    end

  end
end

print(highestScenicScore)
