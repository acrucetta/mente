
## Notes on xv6 OS

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

Anatomy os a system call

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
