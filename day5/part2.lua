-- io.input("test.txt")
io.input("input.txt")

stacks = {}

-- parsing the stacks
line = io.read()
while line ~= "" do

  stack = 1
  for i=2,string.len(line),4 do
    crate = string.sub(line, i, i)
    if crate ~= "" and crate ~= " " then
      if stacks[stack] == nil then stacks[stack] = {} end
      table.insert(stacks[stack], 1, crate)
    end
    stack = stack + 1
  end

  line = io.read()
end

-- parsing instructions
instruction = io.read()
while instruction ~= nil do
  num,stack1,stack2 = string.match(instruction, "move (%d+) from (%d+) to (%d+)")
  num,stack1,stack2 = tonumber(num), tonumber(stack1), tonumber(stack2)

  -- load up crane arm
  local buff = {}
  for i=1,num do
    table.insert(buff, 1, table.remove(stacks[stack1]))
  end

  -- unload crane
  for _,crate in ipairs(buff) do
    table.insert(stacks[stack2], crate)
  end

  instruction = io.read()
end

-- solution string
solution = ""
for _,stack in ipairs(stacks) do
  solution = solution .. stack[#stack]
end

print(solution)
