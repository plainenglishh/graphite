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

local sys = {}

--- Sets the error handler.
function sys.set_error_handler(e: ErrorHandler?) end
--- Returns the current error handler.
function sys.error_handler(): ErrorHandler?
	return nil :: any
end

return sys
