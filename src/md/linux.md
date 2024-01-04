## Linux

*Everything is a file in Unix*

## Survey of Linux Fundamentals

### Linux Kernel

**The kernel takes care of:**
- Process scheduling - run multiple processes simultaneously in memory. 
- Memory management - share memory across processes using virtual memory management (processes are isolated in the kernel)
- Provision of a file system
- Creation and termination of processes - kernels can load a new program into memory and provide it with resources
- Access to devices - monitors, keyboards etc...
- Networking - receives and transmits network messages
- Provision of a system call API - processes can request the kernel to do tasks with the system calls

![https://learn.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/images/userandkernelmode01.png](https://learn.microsoft.com/en-us/windows-hardware/drivers/gettingstarted/images/userandkernelmode01.png)

We have a kernel mode and user mode. Certain operations can only be performed in Kernel mode. E.g., halting instruction for the system, accessing the memory-management hardware, initiating I/O operations.

It can be useful to see programming from the kernel's POV.

Processes can't talk to each other, they're completely isolated and doesn't know where its files are held. It is not "self-aware".

By contrast the kernel knows and controls everything. It facilitates the running of all the processes. A process can't create another process but it can request that the kernel create it.

### Files and Directories

![https://substackcdn.com/image/fetch/w_1456,c_limit,f_webp,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F9365b775-7879-478d-b8f4-3ea75a91147d_1344x1536.jpeg](https://substackcdn.com/image/fetch/w_1456,c_limit,f_webp,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F9365b775-7879-478d-b8f4-3ea75a91147d_1344x1536.jpeg)

Each file is marked with a type. Some file types can be devices, pipes, sockets, directories and symbolic links. It doesn't need to be a regular file.

Directories contain links to themselves (.) and to their parent directory (..)

Symbolic links provide alternative names for a file. A normal link is a filename plus pointer entry in a directory list. Symbolic links point to target files.

Absolute path names being with a slash (/). E.g., /home/mtk/.bashrc

Relative paths, begin with "../mtk/.bashrc"

The system divides ownership into: owner, group, and others.

### File I/O

The Kernel provides one file type: a sequential stream of bytes. It uses the universal operations: open, read, write, close.

A process inherits 3 file descriptors: 0 std input, 1 std output, 2 std error. C uses these methods with its stdio library (e.g., open, fclose, scan, printf, fgets, fputs)

### Programs 

Programs come in two forms: source code (readable text) and binary machine language instructions.

You can apply filters to them e.g., grep, tr, sort, wc, sed and awk.

### Processes

A process is an instance of an executing program. The kernel loads the code of the program into virtual memory, allocates space for the variables and sets up the data structures.

A process contains segments:
- Text - instructions 
- Data - static variables used
- Heap - area from which programs can allocate extra memory
- Stack - area of memory that grows and shrinks as functions are called 

We can create a new process with the fork() sys call. The child node created inherits copies of the parent's data, stack, and heap segments. The child uses the execve() sys call to load and execute a new program. (C uses this exec to build additional libraries)

Each process has a unique identifier (PID) and a parent identifier (PPID). 

A process can be terminated with: _exit() or by being killed with a signal. 0 normally means the process succeeded and nonzero indicates some error.

When we boot a system Linux starts a process called init (parent of all processes). Everything derives from it.

A daemon is a special-purpose process. It is long-lived and runs in the background. E.g., syslogd, for system logs, and htpd, to serve web pages.

All processes have soft and hard memory limits that can be adjusted with the given privileges.

### Interprocess Communication and Synchronization 

Most Linux processes run on their own, but some need to communicated with each other through IPC (interprocess communication)

It includes:
- signals
- pipes (|)
- sockets (transfer data to other hosts)
- file locking - protecting file
- message queues - exchange packets of data
- semaphores - sync processes
- shared memory 

#### Signals

They are often called software interrupts. They are defined as SIGxxxx.

Some examples are:
- interrupt character (CTRL+c)
- a process terminated
- a timer is over
- invalid memory address

The kill command can send a signal. The process can ignore it, be killed or suspended.

Each program executed by the shell starts in a new process. E.g., `ls -l | sort -k5n | less` includes 3 processes.


## Commands

### File Systems
File System Commands:
- `ls`
	- -a all files
	- -d directory
	- -c classify
	- -h human readable
	- -r reverse
	- -S sort by file size
- `file`
- `less`

Directories
- bin - binaries must be present for the system to boot and run
- boot - contains the kernel
- dev - device nodes
- etc - system wide config files
- home - normal config
- lib - shared libraries
- media - mount points for removable media
- mnt - removable storage devices
- opt - optional software
- root - home
- tmp - temp files
- user - directory tree by user

Symbolic Links: 
- soft link: a file can be reference by multiple names
	- e.g., a program needs access to a shared resource
- hard links: allow files to have multiple names

### Manipulating Files and Directories

Commands:
- cp - copy files
	- `cp file 1 file 2` - simple
	- `cp -I file1 file2` - interactive mode
	- `cp file1 file2 dir1` - copy file 1 and 2 to dir1
	- `cp dir1/* dir2` - all files in dir1 copied to dir2
- mv - move and rename files
	- `mv file1 file2` - moves file 1 to file 2; if file 2 exists it is overwritten. If not, it's created
	- `mv file1 file2 dir1` - moves file 1 and 2 into dir 1
- mkdir - create dirs
- rm - remove and dirs files
	- `rm -r dir` - deletes all contents in directory
	- `rm *.html` - removes all files with the extension *.html
- ln - create hard and symbolic links
	- `ln file link` - link file to link
	- `ln -s file link` - hard link file to link
		- *A hard link is indistinguishable from the file itself. Unlike a directory list containing a symbolic link, a directory list containing a hard link shows no special indication of the link.*

### Working with Commands

Commands:
- type - how a command name is interpreted
- which - show the executable program
- man - display man page
- apropos - list of appropriate commands
- info - command info entry
- whatis - brief description of command
- alias - alias for command
	- `alias foo='cd /usr ; ls ; cd -'`


### Redirection commands

Commands:
- cat - concatenate files
- sort - sort text
- uniq - report or omit repeated lines
- wc - print newline word and bytecounts
- grep - print lines matching a pattern
- head - first part of file
- tail - lass part of file
- tee - read from stdin and write to stdout

Piping command example
- `ls /bin /usr/bin | sort | uniq | grep zip`

### Echo Command

- Arithmetic Expansion: `echo $((2+2))`
- Brace Expansion: `echo Front-{A,B,C}-Back`
- Range of Numbers/Letters
	- `echo Number {1...5}`
	- `echo {Z..A}`

### Keyboard Tricks

- clear - clear the screen
- history - history list of commands

