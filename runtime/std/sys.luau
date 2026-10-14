local ops: typeof(ops) = ...
local sys = require("@std/sys")
local sys = {}

local error_handler: sys.ErrorHandler?

function sys.set_error_handler(handler: sys.ErrorHandler)
	assert(type(handler) == "function" or type(handler) == "nil")
	error_handler = handler
end

function sys.error_handler(): sys.ErrorHandler?
	return error_handler
end

function sys.set_memory_category(name: string)
	ops.set_memory_category(name)
end

function sys.set_memory_limit(limit: number)
	ops.set_memory_limit(limit)
end

function sys.heap_dump(): sys.HeapDump
	return ops.heap_dump()
end

return sys
