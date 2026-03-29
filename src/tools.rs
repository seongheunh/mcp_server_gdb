use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock};

use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;

use crate::gdb::GDBManager;
use crate::mi::GDB;

pub static GDB_MANAGER: LazyLock<Arc<GDBManager>> =
    LazyLock::new(|| Arc::new(GDBManager::default()));

pub fn init_gdb_manager() {
    LazyLock::force(&GDB_MANAGER);
}

#[tool(
    name = "create_session",
    description = "Create a new GDB debugging session with optional parameters,\
                   returns a session ID (UUID) if successful",
    params(
        program = "if provided, path to the executable to debug",
        nh = "if provided, do not read ~/.gdbinit file",
        nx = "if provided, do not read any .gdbinit files in any directory",
        quiet = "if provided, do not print version number on startup",
        cd = "if provided, change current directory to DIR",
        bps = "if provided, set serial port baud rate used for remote debugging",
        symbol_file = "if provided, read symbols from SYMFILE",
        core_file = "if provided, analyze the core dump COREFILE",
        proc_id = "if provided, attach to running process PID",
        command = "if provided, execute GDB commands from FILE",
        source_dir = "if provided, search for source files in DIR",
        args = "if provided, arguments to be passed to the inferior program",
        tty = "if provided, use TTY for input/output by the program being debugged",
        gdb_path = "if provided, path to the GDB executable",
    )
)]
pub async fn create_session_tool(
    program: Option<PathBuf>,
    nh: Option<bool>,
    nx: Option<bool>,
    quiet: Option<bool>,
    cd: Option<PathBuf>,
    bps: Option<u32>,
    symbol_file: Option<PathBuf>,
    core_file: Option<PathBuf>,
    proc_id: Option<u32>,
    command: Option<PathBuf>,
    source_dir: Option<PathBuf>,
    args: Option<Vec<OsString>>,
    tty: Option<PathBuf>,
    gdb_path: Option<PathBuf>,
) -> Result<ToolResponseContent> {
    let session = GDB_MANAGER
        .create_session(
            program,
            nh,
            nx,
            quiet,
            cd,
            bps,
            symbol_file,
            core_file,
            proc_id,
            command,
            source_dir,
            args,
            tty,
            gdb_path,
        )
        .await?;
    Ok(tool_text_content!(format!("Created GDB session: {}", session)))
}

#[tool(
    name = "get_session",
    description = "Get a GDB debugging session by ID",
    params(session_id = "The ID of the GDB session")
)]
pub async fn get_session_tool(session_id: String) -> Result<ToolResponseContent> {
    let session = GDB_MANAGER.get_session(&session_id).await?;
    Ok(tool_text_content!(format!("Session: {}", serde_json::to_string(&session)?)))
}

#[tool(name = "get_all_sessions", description = "Get all GDB debugging sessions", params())]
pub async fn get_all_sessions_tool() -> Result<ToolResponseContent> {
    let sessions = GDB_MANAGER.get_all_sessions().await?;
    Ok(tool_text_content!(format!("Sessions: {}", serde_json::to_string(&sessions)?)))
}

#[tool(
    name = "close_session",
    description = "Close a GDB debugging session",
    params(session_id = "The ID of the GDB session")
)]
pub async fn close_session_tool(session_id: String) -> Result<ToolResponseContent> {
    GDB_MANAGER.close_session(&session_id).await?;
    Ok(tool_text_content!("Closed GDB session".to_string()))
}

#[tool(
    name = "start_debugging",
    description = "Start debugging in a session",
    params(session_id = "The ID of the GDB session")
)]
pub async fn start_debugging_tool(session_id: String) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.start_debugging(&session_id).await?;
    Ok(tool_text_content!(format!("Started debugging: {}", ret)))
}

#[tool(
    name = "stop_debugging",
    description = "Stop debugging in a session",
    params(session_id = "The ID of the GDB session")
)]
pub async fn stop_debugging_tool(session_id: String) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.stop_debugging(&session_id).await?;
    Ok(tool_text_content!(format!("Stopped debugging: {}", ret)))
}

#[tool(
    name = "get_breakpoints",
    description = "Get all breakpoints in the current GDB session",
    params(session_id = "The ID of the GDB session")
)]
pub async fn get_breakpoints_tool(session_id: String) -> Result<ToolResponseContent> {
    let breakpoints = GDB_MANAGER.get_breakpoints(&session_id).await?;
    Ok(tool_text_content!(format!("Breakpoints: {}", serde_json::to_string(&breakpoints)?)))
}

#[tool(
    name = "set_breakpoint",
    description = "Set a breakpoint in the code",
    params(
        session_id = "The ID of the GDB session",
        file = "Source file path",
        line = "Line number"
    )
)]
pub async fn set_breakpoint_tool(
    session_id: String,
    file: String,
    line: usize,
) -> Result<ToolResponseContent> {
    let breakpoint = GDB_MANAGER.set_breakpoint(&session_id, &PathBuf::from(file), line).await?;
    Ok(tool_text_content!(format!("Set breakpoint: {}", serde_json::to_string(&breakpoint)?)))
}

#[tool(
    name = "delete_breakpoint",
    description = "Delete one or more breakpoints in the code",
    params(
        session_id = "The ID of the GDB session",
        breakpoints = "The array of the breakpoint numbers to delete"
    )
)]
pub async fn delete_breakpoint_tool(
    session_id: String,
    breakpoints: Vec<String>,
) -> Result<ToolResponseContent> {
    GDB_MANAGER.delete_breakpoint(&session_id, breakpoints).await?;
    Ok(tool_text_content!("Breakpoints deleted".to_string()))
}

#[tool(
    name = "get_stack_frames",
    description = "Get stack frames in the current GDB session",
    params(session_id = "The ID of the GDB session")
)]
pub async fn get_stack_frames_tool(session_id: String) -> Result<ToolResponseContent> {
    let frames = GDB_MANAGER.get_stack_frames(&session_id).await?;
    Ok(tool_text_content!(format!("Stack frames: {}", serde_json::to_string(&frames)?)))
}

#[tool(
    name = "get_local_variables",
    description = "Get local variables in the current stack frame",
    params(
        session_id = "The ID of the GDB session",
        frame_id = "The ID of the stack frame, defaults to 0, the topest frame"
    )
)]
pub async fn get_local_variables_tool(
    session_id: String,
    frame_id: Option<usize>,
) -> Result<ToolResponseContent> {
    let variables = GDB_MANAGER.get_local_variables(&session_id, frame_id).await?;
    Ok(tool_text_content!(format!("Local variables: {}", serde_json::to_string(&variables)?)))
}

#[tool(
    name = "get_registers",
    description = "Get registers in the current GDB session",
    params(
        session_id = "The ID of the GDB session",
        reg_list = "The array of the registers to get",
    )
)]
pub async fn get_registers_tool(
    session_id: String,
    reg_list: Option<Vec<String>>,
) -> Result<ToolResponseContent> {
    let registers = GDB_MANAGER.get_registers(&session_id, reg_list).await?;
    Ok(tool_text_content!(format!("Registers: {}", serde_json::to_string(&registers)?)))
}

#[tool(
    name = "get_register_names",
    description = "Get register names in the current GDB session",
    params(
        session_id = "The ID of the GDB session",
        reg_list = "The array of the registers to get",
    )
)]
pub async fn get_register_names_tool(
    session_id: String,
    reg_list: Option<Vec<String>>,
) -> Result<ToolResponseContent> {
    let registers = GDB_MANAGER.get_register_names(&session_id, reg_list).await?;
    Ok(tool_text_content!(format!("Registers: {}", serde_json::to_string(&registers)?)))
}

#[tool(
    name = "read_memory",
    description = "Read the memory in the current GDB session. \
        This command attempts to read all accessible memory regions in the specified range. \
        First, all regions marked as unreadable in the memory map (if one is defined) will be skipped. \
        See Memory Region Attributes. Second, GDB will attempt to read the remaining regions. \
        For each one, if reading full region results in an errors, GDB will try to read a subset of the region. \
        In general, every single memory unit in the region may be readable or not, \
        and the only way to read every readable unit is to try a read at every address, \
        which is not practical. Therefore, GDB will attempt to read all accessible memory units at either beginning \
        or the end of the region, using a binary division scheme. This heuristic works well for reading across \
        a memory map boundary. Note that if a region has a readable range that is neither \
        at the beginning or the end, GDB will not read it.\
        The command will return a JSON object with the following fields: \
            begin: The start address of the memory block, as hexadecimal literal. \
            end: The end address of the memory block, as hexadecimal literal. \
            offset: The offset of the memory block, as hexadecimal literal, relative to the start address passed to -data-read-memory-bytes.\
            contents: The contents of the memory block, in hex bytes.",
    params(
        session_id = "The ID of the GDB session",
        address = "An expression specifying the address of the first addressable memory unit to be read. \
            Complex expressions containing embedded white space should be quoted using the C convention.",
        count = "The number of addressable memory units to read. This should be an integer literal.",
        offset = "The offset relative to address at which to start reading. This should be an integer literal. \
            This option is provided so that a frontend is not required to first evaluate address and \
            then perform address arithmetic itself.",
    )
)]
pub async fn read_memory_tool(
    session_id: String,
    address: String,
    count: usize,
    offset: Option<isize>,
) -> Result<ToolResponseContent> {
    let memory = GDB_MANAGER.read_memory(&session_id, offset, address, count).await?;
    Ok(tool_text_content!(format!("Memory: {}", serde_json::to_string(&memory)?)))
}

#[tool(
    name = "continue_execution",
    description = "Continue program execution",
    params(session_id = "The ID of the GDB session")
)]
pub async fn continue_execution_tool(session_id: String) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.continue_execution(&session_id).await?;
    Ok(tool_text_content!(format!("Continued execution: {}", ret)))
}

#[tool(
    name = "step_execution",
    description = "Step into next line",
    params(session_id = "The ID of the GDB session")
)]
pub async fn step_execution_tool(session_id: String) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.step_execution(&session_id).await?;
    Ok(tool_text_content!(format!("Stepped into next line: {}", ret)))
}

#[tool(
    name = "next_execution",
    description = "Step over next line",
    params(session_id = "The ID of the GDB session")
)]
pub async fn next_execution_tool(session_id: String) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.next_execution(&session_id).await?;
    Ok(tool_text_content!(format!("Stepped over next line: {}", ret)))
}

#[tool(
    name = "set_breakpoint_at_address",
    description = "Set a breakpoint at a memory address. Useful for stripped binaries without debug symbols.",
    params(
        session_id = "The ID of the GDB session",
        address = "Memory address to set breakpoint at, as a decimal integer (e.g. the decimal value of 0x401234)"
    )
)]
pub async fn set_breakpoint_at_address_tool(
    session_id: String,
    address: usize,
) -> Result<ToolResponseContent> {
    let breakpoint = GDB_MANAGER.set_breakpoint_at_address(&session_id, address).await?;
    Ok(tool_text_content!(format!(
        "Set breakpoint at address: {}",
        serde_json::to_string(&breakpoint)?
    )))
}

#[tool(
    name = "set_breakpoint_at_function",
    description = "Set a breakpoint at a function name in a specific file.",
    params(
        session_id = "The ID of the GDB session",
        file = "Source file path",
        function = "Function name"
    )
)]
pub async fn set_breakpoint_at_function_tool(
    session_id: String,
    file: String,
    function: String,
) -> Result<ToolResponseContent> {
    let breakpoint = GDB_MANAGER
        .set_breakpoint_at_function(&session_id, &PathBuf::from(file), &function)
        .await?;
    Ok(tool_text_content!(format!(
        "Set breakpoint at function: {}",
        serde_json::to_string(&breakpoint)?
    )))
}

#[tool(
    name = "execute_raw_command",
    description = "Execute an arbitrary GDB CLI command. This passes the command through \
                   GDB's interpreter-exec console interface. Useful for commands not covered \
                   by other tools, such as: 'x/20gx $rsp', 'info proc mappings', \
                   'set *0x401234 = 0x90', 'target remote host:port', 'vmmap', \
                   'disassemble main', 'info functions', 'find /b 0x400000, 0x500000, 0x41', etc.",
    params(
        session_id = "The ID of the GDB session",
        command = "The GDB CLI command to execute (e.g. 'x/20gx $rsp', 'info proc mappings')"
    )
)]
pub async fn execute_raw_command_tool(
    session_id: String,
    command: String,
) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.execute_raw_command(&session_id, &command).await?;
    Ok(tool_text_content!(format!("Command output: {}", ret)))
}

#[tool(
    name = "disassemble",
    description = "Disassemble a memory range by address. Returns assembly instructions \
                   in the specified address range.",
    params(
        session_id = "The ID of the GDB session",
        start_addr = "Start address of the range to disassemble (decimal integer)",
        end_addr = "End address of the range to disassemble (decimal integer)",
        with_opcodes = "If true, include raw opcodes alongside assembly mnemonics"
    )
)]
pub async fn disassemble_tool(
    session_id: String,
    start_addr: usize,
    end_addr: usize,
    with_opcodes: Option<bool>,
) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER
        .disassemble_address(&session_id, start_addr, end_addr, with_opcodes.unwrap_or(false))
        .await?;
    Ok(tool_text_content!(format!("Disassembly: {}", ret)))
}

#[tool(
    name = "evaluate_expression",
    description = "Evaluate an expression in the current debugging context. \
                   Useful for inspecting values, casting pointers, dereferencing, \
                   and arithmetic. E.g. '*(int*)0x7fffffffe000', '$rsp+0x10', '(char*)$rdi'.",
    params(
        session_id = "The ID of the GDB session",
        expression = "The expression to evaluate"
    )
)]
pub async fn evaluate_expression_tool(
    session_id: String,
    expression: String,
) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.evaluate_expression(&session_id, &expression).await?;
    Ok(tool_text_content!(format!("Result: {}", ret)))
}

#[tool(
    name = "write_memory",
    description = "Write hex bytes to memory at the specified address. \
                   The contents should be a hex string (e.g. '41424344' to write 'ABCD'). \
                   Useful for patching binaries in memory during exploit development.",
    params(
        session_id = "The ID of the GDB session",
        address = "The memory address to write to (e.g. '0x7fffffffe000')",
        contents = "Hex string of bytes to write (e.g. '41424344' for ABCD)"
    )
)]
pub async fn write_memory_tool(
    session_id: String,
    address: String,
    contents: String,
) -> Result<ToolResponseContent> {
    let ret = GDB_MANAGER.write_memory(&session_id, &address, &contents).await?;
    Ok(tool_text_content!(format!("Memory written: {}", ret)))
}
