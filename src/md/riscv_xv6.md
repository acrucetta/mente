## Notes on XV6 Risc-V

## Key Terms
- inode: contains metadata about a file. E.g., size, type, number of links. It is basically a data structure that describes a file-system object such as a file or a directory.
- ELF Headers
- stack
- guard page

## RISCV System Overview

Features of xv6
- Processes
- Virtual address spaces, page tables
- Files, directories
- Pipes
- Multi-tasking, time slicing
- 21 sys calls

It contains the main Unix sys calls: grep, ls, grep, echo, kill, ln...

What is missing?
- All of the complexity of a "real" OS
- User IDs, login, file protection, mountable filesystems
- Paging to disk
- Sockets, support for networks
- IPC (interprocess comms)
- Device drivers (only 2)
- User code / apps


## General Features

"SMP": shared memory multiprocessor
- main memory (ram) is shared
- 128 mbytes (fixed with define)

Memory Management
- Page Size = 4096 bytes
- Single free list
- No variable-sized allocation
- No malloc
- Page tables have
    - three levels
    - one table per process plus one for the kernel
    - pages are marked as r/w/x/u/v

Scheduler
- Round robin
- Size of timeslice is fixed (1M cycles)
- All cores share one "ready queue"
- Next time slice may be on a diff core

Boot sequence
- QEMU
    - Loads kernel code at a fixed address
- No bootloader/boot block or bios

Locking
- Spin locks 
- sleep() ; wakeup()

"param.h"
- fixed limits (# of processes and open files)
- several arrays (e.g. pids)

Address Space
- Max VA
- Trampoline Page (not available to user)
- Trap Frame (not available to user)
- HEAP
- Stack (1 page)
- Guard Page (1 page) - not available to user
- Data and code (Loaded from ELF exec file)

RISC Architecture
- Multiple schemes (SV 32, 39 or 48)
- xv6 uses SV39 architecture
- Virtual address size:
    - 39 bits
    - 2^39 = 512 GB

### Anatomy of a system call

User space
- user.h
- usys.S
  - Short sub-routine for all the sys calls
  - Calls syscall.h -> looks for the number of the syscall there
- syscall.C

Process:
- User mode calls TRAP
- Trap operates in Kernel Mode
- Then we call sret to go back into user mode
TO REVIEW

## System Calls

### Exec()

[Video](https://www.youtube.com/watch?v=bp0qo4-ozEg&list=PLbtzT1TYeoMhTPzyTZboW_j7TPAnjv9XB&index=38)

-  ELF File Format: Linux / Unix
-  Mach-O: MacOs, iOS
-  Windows: PE

 ELF File Format (contained in elf.h file)
 - File Header (fixed format)
     - Has info on the system type, executable code, entry points, where in file the program header begins
     - Little endian vs. big endian
 - Program Headers (each is fixed size)
     - Type
     - Flags
     - Offset
     - Virtual Address (vaddr): Where in memory
     - File Size: How much data in the segment
     - Memory Size: How many bytes in memory segment
     - Align
 - Segments (chunk of data bytes)
 - Other stuff (section headers, relocation info, debugging info)

`sys_exec`

- Called before the `exec()` function

`exec()`
- Begins with a transaction
- It ensures the file exists and loads it into `ip` (inode)
- Then it locks the file with `ilock`
- Checks magic number; if it's OK we keep going
- We then call `proc_pagetable`; create a new virtual address space and have the function point to the new address space (`pagetable`)
- We the ego through a for loop of the `program headers`
	- We check the type field, ensure it's not ELF_PROG_LOAD ("loadable segment")
	- We check that memory size is not less than file size
	- We then check that the virtual address for overflows
		- e.g., `ph.vaddr + ph.memsz < v.addr` this happens when its negative (they are unsigned integers)
		- If overflow occurs this will be negative
	- We check that the address is a multiple of page size
	- We call `uvmalloc`
		- We provide a size large enough to contain the segment we're holding
		- We call flags2perm
			- Take the offset with the executable bit
			- It ensures the executable and writable bits are set
		- If there's a problem we return 0
- Load the segment to the virtual address space
	- We tell it the file size `ph.filesz` and where in the file those files begin `ph.off`
- Once we're done with the loop we unlock and end the operation
- We then call `uvmalloc` with the page table
    - We round up to the next page boundary
    - We allocate the current size plus two pages (for the stack and the guard page)
    - The pages will be marked as readable and accessible in user mode (U/R/W)
- We then call `uvmclear` to mark the guard page not accesible in usermode

We now deal with argument strings
- We want to push each of the argument strings to the stack and push an array with pointers to those arguments
- We then copy the string to the position of the stack pointer
- `ustack[argc]` we save a pointer

We then push the array of argv[] pointers
- Copy argc+1 (null pointer)



#### loadseg(pagetable_t pagetable, uint64 va, struct inode *ip, uint offset, uint sz)

*Load a program segment into pagetable at virtual address va.*
*va must be page-aligned and the pages from va to va+sz must already be mapped. Returns 0 on success, -1 on failure.*

- Takes address at which we load the data, the inode, the offset and the size of bits we're moving into the address space
- We will loop in units of page size until we move all the `sz` bytes.
- We call `walkaddr` which will give us the physical address of the page in kernel memory
- We finally call readi with the inode and the physical address

#### uvmalloc(pagetable_t pagetable, uint64 oldsz, uint64 newsz, int xperm)
*Allocate PTEs and physical memory to grow process from oldsz to newsz, which need not be page aligned.  Returns new size or 0 on error.*

**TO REVIEW**

### Fork()
 - Calls allocproc: obtain new proc struct with a new PID
 - Copy virtual address space
     - all data pages are copied
     - if there are any issues we free the proc structure
 - Initialize proc structure
     - sz
     - trapframe:
     - name
     - ofile: copy open files
     - cwd: copy current working dir
     - parent: set the parent piointer to the parents proc
     - state: make it runnable
 - Return the PID of the process

 Child Process:
 - Alloc Proc
     - Initializes context
     - Zeroes registers

