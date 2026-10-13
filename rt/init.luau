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
	local traceback: { sys.StackTraceElement } = {}
	while debug.info(level + 2, "s") do -- skip last two frames
		local source, line, func = debug.info(level, "slf")

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

local function handle_uncaught_error(e: sys.Error): never
	if type(e.err) ~= "string" then
		print(tostring(e.err))
		return ops.exit(1)
	end

	local where_source, where_line = string.match(e.err, "(.-):(%-?%d+): ")
	if not where_source then
		print(e.err)
		return ops.exit(1)
	end

	local where_line = tonumber(where_line) or -1
	local start_at = 1
	for i, frame in e.traceback do
		if frame.source == where_source and frame.line == where_line then
			start_at = i
			break
		end
	end

	local out: { string } = { `unhandled error: {tostring(e.err)}\nstacktrace:` }

	for i = start_at, #e.traceback do
		local frame = e.traceback[i]
		local line = if frame.line == -1 then "" else `:{frame.line}`
		local name, line_defined, source_defined = debug.info(frame.func, "nls")
		local name = if name == "" then if line_defined == 1 then "<main>" else "<anonymous>" else `'{name}'`
		local defined = if line_defined == -1 or name == "<main>" then "" else ` ({source_defined}:{line_defined})`
		table.insert(out, `    {frame.source}{line} in function {name}{defined}`)
	end

	print(table.concat(out, "\n"))
	return ops.exit(1)
end

if not ok then
	local err_handler: any = sys.error_handler() or handle_uncaught_error
	err_handler(err)
end
