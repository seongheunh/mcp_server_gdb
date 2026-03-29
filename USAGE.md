# GDB MCP Server - Usage Guide

## Tools

### Session Management
| Tool | Description |
|---|---|
| `create_session` | Create a GDB session. Specify binary with `program` parameter. Can be created without program for remote debugging |
| `close_session` | Close a session |
| `get_session` / `get_all_sessions` | Query sessions |

### Execution Control
| Tool | Description |
|---|---|
| `start_debugging` | Start program execution (`run`) |
| `stop_debugging` | Interrupt execution (`interrupt`) |
| `continue_execution` | Resume execution after breakpoint |
| `step_execution` | Step into (enter function) |
| `next_execution` | Step over (skip function) |

### Breakpoints
| Tool | Description |
|---|---|
| `set_breakpoint` | Source file:line based BP |
| `set_breakpoint_at_address` | **Address-based BP** - for stripped binaries. Address is passed as decimal integer |
| `set_breakpoint_at_function` | File:function name based BP |
| `delete_breakpoint` | Delete BP |
| `get_breakpoints` | List all BPs |

### Inspection
| Tool | Description |
|---|---|
| `read_memory` | Read N bytes from address (hex dump) |
| `write_memory` | Write hex bytes to address (e.g. `"41424344"`) |
| `get_registers` | Query register values |
| `get_register_names` | List available register names |
| `get_local_variables` | Local variables in current stack frame |
| `get_stack_frames` | Query call stack |
| `evaluate_expression` | Evaluate expression (`*(int*)$rsp`, `$rdi+0x10`, etc.) |
| `disassemble` | Disassemble address range. Use `with_opcodes=true` for raw opcodes |

### Raw Command (catch-all)
| Tool | Description |
|---|---|
| `execute_raw_command` | **Execute arbitrary GDB CLI command**. Anything not covered by other tools can be done with this |

`execute_raw_command` examples:
```
x/20gx $rsp                    # stack dump
x/10i $rip                     # disassemble at current location
info proc mappings             # memory map (check ASLR)
vmmap                          # pwndbg memory map
target remote localhost:1234   # connect to remote gdbserver
symbol-file /path/to/vuln      # load symbol file
set *(int*)0x404020 = 0x42     # patch memory value
find /b 0x400000, 0x500000, 0x41  # search byte pattern
info functions                 # list functions
got                            # pwndbg GOT table
heap                           # pwndbg heap analysis
```

---

## Workflows

### 1. Local Binary

```
create_session(program="/path/to/vuln")
set_breakpoint_at_address(address=0x401234)   # address as decimal
start_debugging()
get_registers() / read_memory() / disassemble()
```

### 2. Docker-based Challenge

Follow these steps for challenges that provide a Docker environment.

#### Step 1: Modify docker-compose.yml
Add the following if not present, using Bash:
```yaml
services:
  vuln:
    cap_add:
      - SYS_PTRACE
    security_opt:
      - seccomp:unconfined
    ports:
      - "1234:1234"
```

#### Step 2: Start container & run gdbserver
```bash
docker-compose up -d
docker exec <container> bash -c "apt-get update && apt-get install -y gdbserver"
docker exec <container> gdbserver :1234 /path/to/vuln
```

Skip the install step if `gdbserver` is already installed.
To attach to an already running process:
```bash
docker exec <container> gdbserver --attach :1234 <pid>
```

#### Step 3: Connect via MCP
```
create_session()                                          # empty session without program
execute_raw_command("target remote localhost:1234")        # connect to gdbserver
execute_raw_command("symbol-file /local/path/to/vuln")    # load symbols from local binary (optional)
set_breakpoint_at_address(address=0x401234)
execute_raw_command("continue")
```

### 3. Exploit Testing

Use pwntools for exploits, MCP for analysis.

```
# Terminal/Bash: run pwntools exploit
python3 exploit.py

# MCP: inspect runtime state via GDB simultaneously
read_memory(address="$rsp", count=64)
get_registers()
execute_raw_command("x/10gx $rsp")
```

Do not hardcode absolute addresses (ASLR-dependent) obtained from GDB into exploit code.
Use GDB only for verifying fixed offsets and struct layouts.

### 4. Multi-binary Analysis

When analyzing two or more binaries, create independent sessions for each.

```
session_a = create_session(program="/path/to/binary_a")
session_b = create_session(program="/path/to/binary_b")

# Control independently
set_breakpoint_at_address(session_id=session_a, address=0x401000)
set_breakpoint_at_address(session_id=session_b, address=0x402000)
start_debugging(session_id=session_a)
start_debugging(session_id=session_b)

# Analyze each
get_registers(session_id=session_a)
read_memory(session_id=session_b, address="$rsp", count=64)
```

For sequential analysis, close_session one before opening the next.
For concurrent analysis, use session IDs to control them in parallel.
