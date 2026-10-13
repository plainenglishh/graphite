local ops: typeof(ops) = ...

local std_cyclic = setmetatable({}, {
	__index = function()
		error("attempt to index from std typedefs", 2)
	end,
})

local std_module_cache = {}
local function require_(path: string): any
	local context: string = debug.info(2, "s")
	if string.sub(path, 1, 4) == "@std" then
		if context == path then
			return std_cyclic -- short circuit cyclic in std to allow typedefs
		end

		local module_name = string.sub(path, 6)
		if std_module_cache[module_name] then
			return std_module_cache[module_name]
		end

		local bytes = ops.std_module(module_name)
		if not bytes then
			error(`unknown std module '{module_name}'`)
		end

		local ret = ops.load(bytes, `@@std/{module_name}`)(ops)
		std_module_cache[module_name] = ret
		return ret
	end

	error(`invalid require {path}`)
end

ops.set_global("require", require_)
ops.sandbox()

local sys = require("@std/sys")

local args = ops.args()
local target_path = args[1] or error("provide target path")
local src = ops.read_file(target_path)
local loaded = ops.load(src, `@{target_path}`)

local ok, err = xpcall(loaded, function(err)
	local level = 2
	local traceback = {}
	while true do
		local source, line, func = debug.info(level, "slf")
		if not source then
			break
		end

		table.insert(traceback, {
			source = source,
			line = line,
			func = func,
		})

		level += 1
	end

	return {
		err = err,
		traceback = traceback,
	}
end)

local function handle_uncaught_error(e: sys.Error)
	print(e.err)
	print("stacktrace:")
	for _, frame in e.traceback do
		local name, line_defined = debug.info(frame.func, "nl")
		print(`    {frame.source}:{frame.line} function {name}`)
	end
end

if not ok then
	local err_handler: any = sys.error_handler() or handle_uncaught_error
	err_handler(err)
end
