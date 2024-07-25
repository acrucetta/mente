# C Tools

## Make files



## LLDB Cheat Sheet

#### Starting and Controlling Execution
- **Start a program:** `lldb ./program`
- **Run the program:** `run` or `r`
- **Set a breakpoint:** `breakpoint set --name main` or `b main`
- **List breakpoints:** `breakpoint list` or `b l`
- **Delete a breakpoint:** `breakpoint delete <breakpoint-id>` or `b del <breakpoint-id>`
- **Continue execution:** `continue` or `c`
- **Step over:** `thread step-over` or `n`
- **Step into:** `thread step-in` or `s`
- **Step out:** `thread step-out` or `finish`

#### Inspecting State
- **Print variable:** `frame variable <variable-name>` or `p <variable-name>`
- **Print all local variables:** `frame variable` or `fr v`
- **Backtrace:** `thread backtrace` or `bt`
- **View source code:** `source list` or `list`
- **Evaluate expression:** `expression <expression>` or `p <expression>`

#### Advanced Usage
- **Attach to a process:** `lldb -p <pid>`
- **Detach from a process:** `process detach`
- **Set environment variable:** `settings set target.env-vars <VAR>=<value>`

## Valgrind Cheat Sheet

#### Basic Commands
- **Run a program with Valgrind:** `valgrind ./program`
- **Check for memory leaks:** `valgrind --leak-check=yes ./program`
- **Show reachable memory at exit:** `valgrind --leak-check=full --show-reachable=yes ./program`
- **Use Valgrind with gdb:** `valgrind --vgdb=yes --vgdb-error=0 ./program`

#### Common Options
- **Track origins of uninitialized memory:** `--track-origins=yes`
- **Suppress specific errors:** `--suppressions=<file>`
- **Log output to a file:** `--log-file=<filename>`
- **Limit stack traces:** `--num-callers=<number>`

## Obj Dump Cheat Sheet

#### Basic Commands
- **Display all headers:** `objdump -x <file>`
- **Display disassembly:** `objdump -d <file>`
- **Display section headers:** `objdump -h <file>`
- **Display symbol table:** `objdump -t <file>`

#### Detailed Options
- **Disassemble specific function:** `objdump -d -j <section> <file>`
- **Display file offsets:** `objdump -D <file>`
- **Display relocation entries:** `objdump -r <file>`
- **Display dynamic symbols:** `objdump -T <file>`

These commands and options provide a foundational toolkit for using LLDB, Valgrind, and Obj Dump effectively in C development.

## Track Data Races

### GCC

1. To check for data races using dynamic analysis tools, you can compile this code with ThreadSanitizer. Here's how you can do it using GCC:

`gcc -fsanitize=thread -g thread_safety_example.c -o thread_safety_example -lpthread`

2. Run the program

`./thread_safety_example`

### Helgrind

1. First, compile your C program with debugging symbols:

`gcc -g your_program.c -o your_program -lpthread`

2. Then run your program using Valgrind with the Helgrind tool:

`valgrind --tool=helgrind ./your_program`

