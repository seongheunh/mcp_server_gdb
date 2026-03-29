# MCP Server GDB (Modified)

> Based on [pansila/mcp_server_gdb](https://github.com/pansila/mcp_server_gdb). Modified for pwnable/CTF exploit development workflows.

### Changes from upstream

- **New tools for binary exploitation**
  - `execute_raw_command` — Run arbitrary GDB CLI commands (`x/20gx $rsp`, `target remote`, `vmmap`, etc.)
  - `set_breakpoint_at_address` — Address-based breakpoints for stripped binaries
  - `set_breakpoint_at_function` — Function name-based breakpoints
  - `disassemble` — Disassemble memory ranges with optional raw opcodes
  - `evaluate_expression` — Evaluate expressions in debugging context (`*(int*)$rsp`, `(char*)$rdi`)
  - `write_memory` — Write hex bytes to memory for runtime patching
- **Default SSE port changed from 8080 to 9000** (avoids conflict with Ghidra MCP)
- **Added `USAGE.md`** — Tool reference and workflow guide (local, Docker, multi-binary, exploit testing)

### Quick Start

**1. Start the server (SSE transport):**
```bash
/home/user/mcp_server_gdb/target/release/mcp-server-gdb sse
```

**2. Register MCP in Claude Code (one-time, global):**
```bash
claude mcp add --transport sse --scope user gdb http://127.0.0.1:9000/sse
```

For detailed tool reference and workflow examples (local, Docker, exploit testing, multi-binary), see [`USAGE.md`](USAGE.md).

---

A GDB/MI protocol server based on the MCP protocol, providing remote application debugging capabilities with AI assistants.

## Features

- Create and manage GDB debug sessions
- Set and manage breakpoints
- View stack information and variables
- Control program execution (run, pause, step, etc.)
- Support concurrent multi-session debugging
- A built-in TUI to inspect agent behaviors so that you can improve your prompt (WIP)

## Installation

### Pre-built Binaries
Find the binaries in the release page, choose one per your working platform, then you can run it directly.

### Build From Source
Clone the repository and build it by cargo
```bash
cargo build --release
cargo run
```

### Using Nix
If you have Nix installed, you can run the project without cloning:

#### Run locally (after cloning)
```bash
nix run .
```

#### Run remotely from GitHub
```bash
nix run "git+https://github.com/pansila/mcp_server_gdb.git" -- --help

```

#### Development environment
To enter a development shell with all dependencies:
```bash
nix develop
```

## Usage

1. Just run it directly: `./mcp-server-gdb`
2. The server supports two transport modes:
   - Stdio (default): Standard input/output transport
   - SSE: Server-Sent Events transport, default at `http://127.0.0.1:8080`

## Configuration

You can adjust server configuration by modifying the `src/config.rs` file or by environment variables:

- Server IP Address
- Server port
- GDB command timeout time (in seconds)

## Supported MCP Tools

### Session Management

- `create_session` - Create a new GDB debugging session
- `get_session` - Get specific session information
- `get_all_sessions` - Get all sessions
- `close_session` - Close session

### Debug Control

- `start_debugging` - Start debugging
- `stop_debugging` - Stop debugging
- `continue_execution` - Continue execution
- `step_execution` - Step into next line
- `next_execution` - Step over next line

### Breakpoint Management

- `get_breakpoints` - Get breakpoint list
- `set_breakpoint` - Set breakpoint
- `delete_breakpoint` - Delete breakpoint

### Debug Information

- `get_stack_frames` - Get stack frame information
- `get_local_variables` - Get local variables
- `get_registers` - Get registers
- `read_memory` - Read memory contents

## License

MIT
