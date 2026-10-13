local sys = require("@std/sys")

sys.set_error_handler(function(err: sys.Error)
	print("it's gone pear shaped!")
	print(err.err)
end)
sys.set_error_handler(nil)

local function inner()
	error("shit", 1)
end

local function outer()
	inner()
end

outer()
