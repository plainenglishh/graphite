local std = require("@std")

--sys.set_error_handler(function(err: sys.Error)
--	print("it's gone pear shaped!")
--
--
--end)
for k, v in std.sys.heap_dump().categories do
	print(k, v)
end
local function inner()
	error("shit", 1)
end

local a: index<std.sys, "Error"> -- ugly but works i guess :p

local function outer()
	inner()
end

outer()
