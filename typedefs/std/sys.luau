export type StackTraceElement = {
	line: number,
	source: string,
	func: (...any) -> ...any,
}

export type Error = {
	err: Error,
	traceback: { StackTraceElement },
}

export type ErrorHandler = (err: Error) -> ()

export type HeapDump = {
	total_size: number,
	categories: { [string]: number },
	types: { [string]: { count: number, size: number } },
	userdata: { [string]: { count: number, size: number } },
}

local sys = {}

--- Sets the error handler.
function sys.set_error_handler(e: ErrorHandler?) end
--- Returns the current error handler.
function sys.error_handler(): ErrorHandler?
	return nil :: any
end
function sys.set_memory_category(name: string) end

function sys.set_memory_limit(limit: number) end

function sys.heap_dump(): HeapDump
	return nil :: any
end

return sys
