declare ops: {
    --- sys
    std_module: (path: string) -> buffer?,
    set_global: (key: string, value: any) -> (),
    sandbox: () -> (),
    registry: () -> {[any]: any},
    
    -- env
    cwd: () -> string,
    args: () -> {string},
    exit: (exit_code: number) -> never,
    
    -- luau
    compile: (source: buffer) -> buffer,
    load: (bc: buffer, debug_name: string) -> any,
    
    -- fs
    read_file: (path: string) -> buffer,
}