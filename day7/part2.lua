-- io.input("test.txt")
io.input("input.txt")

disk = {name = "/"}
currentDir = disk
currentDir.subfolders = {}

-- parsing terminal line by line
line = io.read() line = io.read() -- skip first line

while line ~= nil do

  -- all commands
  if string.sub(line, 1, 1) == "$" then
    if line == "$ ls" then
      print("skipping", line)
      goto continue
    end

    -- change directory
    destinationDirectory = string.match(line, "cd (.+)")

    if destinationDirectory == ".." then
      print("navigated up")
      currentDir = currentDir.parent
    else
      for _,sub in pairs(currentDir.subfolders) do
        if sub.name == destinationDirectory then
          currentDir = sub
          print("change to subfolder", line)
          break
        end
      end
    end

  -- create subfolder
  elseif string.sub(line, 1, 3) == "dir" then
    print("creating subfolder", line)
    table.insert(currentDir.subfolders, {name = string.match(line, "dir (%a+)"), subfolders = {}, parent = currentDir})

  -- file
  else
    print("adding file", line)
    table.insert(currentDir, string.match(line, "(%d+)") + 0)
  end

  :: continue::
  line = io.read()
end

-- find total file size of folder, including subfolders, recursively
-- cache file size in a size property on the folder
function folderSize (f)
  if f == nil then
    return 0
  end

  if f.size == nil then

    local filesTotal = 0
    for k,size in ipairs(f) do
      filesTotal = filesTotal + size
    end

    local subfoldersTotal = 0
    for _,sub in pairs(f.subfolders) do
      subfoldersTotal = subfoldersTotal + folderSize(sub)
    end
    f.size = filesTotal + subfoldersTotal
  end

  return f.size
end

-- return a flattened list of references to all folders in the out-var 'all'
function allFolders (f, all)

  table.insert(all, f)

  for _,subfolder in pairs(f.subfolders) do
    allFolders(subfolder, all)
  end
end

unusedSpace = 70000000 - folderSize(disk)
needAtLeast = 30000000 - unusedSpace

out = {}
allFolders(disk, out)

smallestAboveThreshold = math.huge
for _,folder in ipairs(out) do
  if folder.size < smallestAboveThreshold and folder.size > needAtLeast then
    smallestAboveThreshold = folder.size
  end
end

print(smallestAboveThreshold)
