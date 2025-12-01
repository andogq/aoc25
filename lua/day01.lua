DAY = "01"

DIRECTIONS = {
	L = -1,
	R = 1,
}

---@param input string
---@return fun(): string | nil
local function lines(input)
	local iter = input:gmatch("[^\n]+")

	local next
	next = function()
		local line = iter()

		if line == nil then
			return nil
		end

		if line:len() == 0 then
			return next()
		end

		return line
	end

	return next
end

---@param line string
---@return integer
local function parse(line)
	local dir_c = line:sub(1, 1)
	local dir = DIRECTIONS[dir_c]
	if dir == nil then
		print("invalid dir: " .. dir_c)
		os.exit(1)
	end

	local abs = tonumber(line:sub(2))

	return dir * abs
end

---@param input string
---@return integer | nil
local function part1(input)
	local pos = 50
	local count = 0

	for line in lines(input) do
		local num = parse(line)

		pos = (pos + num) % 100

		if pos == 0 then
			count = count + 1
		end
	end

	return count
end

---@param input string
---@return integer | nil
local function part2(input)
	local pos = 50
	local count = 0

	for line in lines(input) do
		local num = parse(line)

		count = count + math.floor(math.abs(num) / 100)

		local end_pos = pos + (math.fmod(num, 100))

		if pos ~= 0 and (end_pos <= 0 or end_pos > 99) then
			count = count + 1
		end

		pos = math.fmod(end_pos + 100, 100)
	end

	return count
end

---Read the provided path into a string.
---@param path string
---@return string
local function read_or_panic(path)
	local file = io.open(path, "r")
	if file == nil then
		print("could not open file at " .. path)
		os.exit(1)
	end

	---@type string
	local contents = file:read("*a")

	file:close()

	return contents
end

---@param part fun(input: string): integer | nil
---@param input_kind "examples" | "inputs"
---@return integer | nil
local function run(part, input_kind)
	if input_kind ~= "examples" and input_kind ~= "inputs" then
		print("expected `input` to be `examples` or `inputs`, but found " .. input_kind)
		os.exit(1)
	end

	local path = "../data/" .. input_kind .. "/" .. DAY .. ".txt"
	local contents = read_or_panic(path)

	local result = part(contents)

	return result
end

---Assert `lhs == rhs`, or print the message and quit.
---@generic T
---@param lhs T
---@param rhs T
---@param message? string
local function assert_eq(lhs, rhs, message)
	if lhs == rhs then
		return
	end

	if message == nil then
		message = "assertion failed"
	end

	print(message .. ": " .. lhs .. " != " .. rhs)
	os.exit(1)
end

assert_eq(run(part1, "examples"), 3, "part 1 example")
print("part 1: " .. run(part1, "inputs"))

assert_eq(run(part2, "examples"), 6, "part 2 example")
print("part 2: " .. run(part2, "inputs"))
