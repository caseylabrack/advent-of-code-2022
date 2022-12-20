-- io.input("test.txt")
io.input("input.txt")

allFolders = {}
disk = {name = "/"}
currentDir = disk
currentDir.subfolders = {}

-- parsing terminal line by line
line = io.read() line = io.read() -- skip first line

while line ~= nil do
  -- print(line)

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

-- find total file size of folder, including subfolders recursively
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

function tconcat (a,b)
  local r = {}
  for _,v in pairs(a) do
    table.insert(r,v)
  end
  for _,v in pairs(b) do
    table.insert(r,v)
  end
  return r
end

-- return a flattened list of all folders in the out-var 'all'
function allFolders (f, all)

  table.insert(all, f)

  for _,subfolder in pairs(f.subfolders) do
    allFolders(subfolder, all)
  end
end

out = {}
allFolders(disk, out)

smallOnes = {}
smallOnesTotal = 0
for _,f in ipairs(out) do
  local size = folderSize(f)
  if size <= 100000 then
    smallOnesTotal = smallOnesTotal + size
    -- table.insert(smallOnes, size)
  end
end

print(smallOnesTotal)
-- print(folderSize(disk.subfolders[2]))
