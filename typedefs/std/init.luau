local std = {}

local sys = require("@self/sys")
export type sys = { -- god willing `sys.Error` will be supported as sugar for `index<sys, "Error">` soon!
	StackTraceElement: sys.StackTraceElement,
	Error: sys.Error,
	ErrorHandler: sys.ErrorHandler,
	HeapDump: sys.HeapDump,
}
std.sys = sys

return std
