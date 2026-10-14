local ops: typeof(ops) = ...

do
	local typedefs_stub = setmetatable({}, {
		__index = function()
			error("attempt to index typedefs", 2)
		end,
	})

	local graphite_require_cache = {}
	local function require(path: string): any
		local context: string = ops.chunk_name(2)

		if string.sub(path, 1, 4) == "@std" then
			local module_name = string.sub(path, 6)
			local module_chunk_name = `@@graphite/std/{module_name}`
			if context == module_chunk_name then
				return typedefs_stub
			end

			if graphite_require_cache[module_name] then
				return graphite_require_cache[module_name]
			end

			local bytecode = ops.graphite_module(`std/{module_name}`)
			if not bytecode then
				error(`unknown std module '{module_name}'`)
			end

			local ret = ops.load(bytecode, module_chunk_name)(ops)
			graphite_require_cache[module_name] = ret
			return ret
		end

		error(`invalid require {path}`)
	end

	ops.set_global("require", require)
	ops.sandbox()
end

local sys = require("@std/sys")

local function collect_err_info<T>(err: T): sys.Error
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
end

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

local args = ops.args()
local target_path = args[1] or error("provide target path")
local src = ops.read_file(target_path)
local loaded = ops.load(src, `@{target_path}`)

local ok, err = xpcall(loaded, collect_err_info)
if not ok then
	local err_handler: any = sys.error_handler() or handle_uncaught_error
	local err_handler_ok, err_handler_err = xpcall(err_handler, collect_err_info, err)
	if not err_handler_ok then
		handle_uncaught_error(err_handler_err)
	end

	ops.exit(1)
end
