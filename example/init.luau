local sys = require("@std/sys")

--sys.set_error_handler(function(err: sys.Error)
--	print("it's gone pear shaped!")
--
--
--end)
for k, v in sys.heap_dump().categories do
	print(k, v)
end
local function inner()
	error("shit", 1)
end

local function outer()
	inner()
end

outer()
