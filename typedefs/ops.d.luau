declare ops: {
    --- sys
    graphite_module: (path: string) -> buffer?,
    set_global: (key: string, value: any) -> (),
    sandbox: () -> (),
    registry: () -> {[any]: any},
    set_memory_category: (name: string) -> (),
    set_memory_limit: (limit: number) -> (),
    heap_dump: () -> any,
    chunk_name: (level: number) -> string,

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