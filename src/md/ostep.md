# Notes on Operating Systems in 3 Easy Pieces

### Anki Questions

#### System Calls

**What does wait() do?**
The parent process calls wait() to delay its execution until the child finishes executing. When the child is done, wait() returns to the parent.

```c
int main() {
-     int rc = fork(); // Fork a new process

    if (rc < 0) {
        // Fork failed; exit
        fprintf(stderr, "fork failed\n");
        exit(1);
    } else if (rc == 0) {
        // Child process
        printf("hello, I am child (pid:%d)\n", (int)getpid());
    } else {
        // Parent process
        int wc = wait(NULL);
        printf("hello, I am parent of %d (wc:%d) (pid:%d)\n", rc, wc, (int)getpid());
    }
    return 0;
}
```

**What does fork() do?**
The fork() system call is used to create a new process

**What does exec() do?**
This system call is useful when you want to run a program that is different from the calling program. For example, calling fork() in p2.c is only useful if you want to keep running copies of the same program

```

17 char *myargs[3];
18 myargs[0] = strdup("wc"); // program: "wc" (word count)
19 myargs[1] = strdup("p3.c"); // argument: file to count
20 myargs[2] = NULL; // marks end of array
21 execvp(myargs[0], myargs); // runs word count
```

How does pipe() work? What are the main components?
Why do we need different modes of execution for a CPU?
What is a trap, what is a trap table?
How can the OS effectively switch between processes?
What do we do when an OS process goes rogue? How does the CPU prevent it?

**What is a context switch?**

A context switch is conceptually simple: all the OS has to do is **save a few register values for the currently-executing process** (onto its kernel stack, for example) and **restore a few for the soon-to-be-executing process** (from its kernel stack). 

By doing so, the OS thus **ensures that when the return-from-trap instruction is finally executed, instead of returning to the process that was running, the system resumes execution of another process.**

**How does the OS "baby proof" the CPU? What are the equivalents?**

By during boot time setting up the trap handlers and starting an interrupt timer, and then by only running processes in a restricted mode. By doing so, the OS can feel quite assured that processes can run efficiently, only requiring OS intervention to perform privileged operations or when they have monopolized the CPU for too long and thus need to be switched out.

**What is a timer interrupt? Why do we need it?**

The addition of a timer interrupt gives the OS the ability to run again on a CPU even if processes act in a non-cooperative fashion. Thus, this hardware feature is essential in helping the OS maintain control of the machine.

### Scheduling

What are the key assumptions made in the simple scheduling examples?
1. Each job runs for the same amount of time. 
2. All jobs arrive at the same time. 
3. Once started, each job runs to completion. 
4. All jobs only use the CPU (no I/O). 
5. The run-time of each job is known.

What are the key metrics for evaluating scheduling algorithms?
- Response time: time of first run - time of arrival
- Turnaround time: time of completion - time of arrival

What are the main types of simple scheduling policies mentioned and what do they optimize for?
- First-in First-out (FIFO)
- Shortest Job First (SJF): Optimizes turnaround time
- Shortest Time-to-Completion First (STCF): Optimizes turnaround time
- Round Robin (RR): Optimizes response time

What is the Multi-Level Feedback Queue (MLFQ) and what does it optimize for?

MLFQ is a scheduling algorithm that varies job priority based on observed behavior. It optimizes for turnaround time by running shorter jobs first, while also trying to provide good responsiveness for interactive users. It learns about processes as they run and uses their history to predict future behavior.

What are the first two rules of the MLFQ?
- Rule 1: If Priority(A) > Priority(B), A runs (B doesn’t).
- Rule 2: If Priority(A) = Priority(B), A & B run in RR.

How does MLFQ handle newly arriving jobs and jobs that use their entire time slice?
- Rule 3: When a job enters the system, it is placed at the highest priority (the topmost queue).
- Rule 4a: If a job uses up an entire time slice while running, its priority is reduced (i.e., it moves down one queue).


## Lectures

### Lecture 1

CPUs give us the illusion of running a program.

CPUs give us the abstraction of a process. A running program.

Processes have a private memory and "address space"; they also have registers, and a stack pointer.

The computer can run programs in Kernel Mode (OS) and User Mod (user program). Limited number of things.

Special services to operate on the OS can be called system calls.

Boot Time:
- OS starts in Kernel mode; it tells the hardware where to jump to when the user specifies a trap #
- We set up trap handlers in the OS; tell H/W where the trap handlers are in OS memory

Key Questions
- What if OS wants to run a operation that's higher
- What if the OS wants to stop A, then run B

### Lecture 2

A CPU is basically a while loop that fetches the program counter. Figures out which instruction it is. And executes it.

```
while (1) {
  fetch(pc)
  decode
  increment pc
  (before executing)
    check permission (kernel or os)
    if not ok raise exception
      OS can get involved
  execute (can change pg)
    process interrupts
}
```

Before it executes; it checks permission. Is this instrunction OK to execute.
- It basically checks permissions; if the instruction is OK to execute do it. If not raise exception.

A timer interrupt runs every once in a while. It makes the CPU decide if it wants to run the current program or switch.

CPU virtualization mechanisms.

At boot time: OS runs first at priviledged mode (kernel mode). 
- Install handlers (tell hardware what to run)
- Tell H/W what code to run on exception/interrupt traps
- Initialize timer interrupt
- Ready ro run user programs

Timeline:
- A trap instruction:
	- Transitions us from user mode to kernel mode
	- Jumps into OS: target trap handlers
	- Save register state (to execute later)
	- Return from trap (opposite of above)

The OS must track different user processes. It uses a **process list**
- Per-process info:
	- state: ready, running

A big problem: some operations are slow; the OS needs to do what to do.
- 

## Book Notes

### Scheduling: Introduction

**Key assumptions:**
1. Each job runs for the same amount of time.
2. All jobs arrive at the same time.
3. Once started, each job runs to completion.
4. All jobs only use the CPU (i.e., they perform no I/O) 5. The run-time of each job is known.

Types of scheduling:
- **First-in first out**
- **Shortest job fist (SJF) - optimizes turnaround time**
	- This new scheduling discipline is known as Shortest Job First (SJF), and the name should be easy to remember because it describes the policy quite completely: it runs the shortest job first, then the next shortest, and so on.
- **Shortest time to completion first (STCF) - optimizes turnaround time**
	- Any time a new job enters the system, the STCF scheduler determines which of the remaining jobs (including the new job) has the least time left, and schedules that one. Thus, in our example, STCF would preempt A and run B and C to completion
- **Round robin - optimizes response time** 
	- The basic idea is simple: instead of running jobs to completion, RR runs a job for a time slice (sometimes called a scheduling quantum) and then switches to the next job in the run queue.
	- The shorter the time slice the better; deciding on this length is a trade-off
	- Any policy (such as RR) that is fair, i.e., that evenly divides the CPU among active processes on a small time scale, will perform poorly on metrics such as turnaround time

**Metrics**:
- Response time: time of first run - time of arrival
- Turnaround time: time of completion - time of arrival

### Scheduling: Multi-Level Feedback Queue

The Multi-level Feedback Queue (MLFQ) sched- uler was first described by Corbato et al. in 1962 [C+62] in a system known as the Compatible Time-Sharing System (CTSS), and this work, along with later work on Multics, led the ACM to award Corbato its highest honor, the Turing Award.

It optimizes for turnaround time. Done by running shorter jobs first. It tries to make the system feel responsive to interactive users. (minimizing staring at the screen and waiting)

*Key Question: How do we schedule without perfect knowledge?*

**MLFQ** varies the priority of a job based on its observed behavior. If, for example, a job repeatedly relinquishes the CPU while waiting for input from the keyboard, MLFQ will keep its priority high, as this is how an interactive process might behave. If, instead, a job uses the CPU intensively for long periods of time, MLFQ will reduce its priority. In this way, MLFQ will try to learn about processes as they run, and thus use the history of the job to predict its future behavior.

Thus, we arrive at the first two basic rules for MLFQ:
• Rule 1: If Priority(A) > Priority(B), A runs (B doesn’t).
• Rule 2: If Priority(A) = Priority(B), A & B run in RR.

**Changing Priorities**

- Rule 3: When a job enters the system, it is placed at the highest priority (the topmost queue).
- Rule 4a: If a job uses up an entire time slice while running, its priority is reduced (i.e., it moves down one queue).
- Rule4b: If it gives up the CPU before the time slice is up; stay at same priority level

Key issues:
- Starvation; what if we have a LOT of interactive jobs; long-running CPU jobs will never finish
- Gaming the scheduler: sneaky trick to get more time than its fair; e.g., issue a quick IO before the time slice is up

**Boosting priorities**

- Rule 5: After some time period S, move all the jobs in the system to the topmost queue.

It guarantees the processes won't starve by sitting in the top queue. It also shares the CPU with other high priority jobs. If a job becomes interactive it will receive a proper priority boost.

**Add Accounting**

- Rule 4: Once a job uses up its time allotment at a given level (re- gardless of how many times it has given up the CPU), its priority is reduced (i.e., it moves down one queue).

*MLFQ is interesting because instead of demanding a priori knowledge of the nature of a job, it instead observes the execution of a job and pri- oritizes it accordingly.*

### Scheduling: Proportional Share

In this case we want to optimize for each CPU process to obtain a certain percentage of CPU time. 

The key question is how can we degin a scheduler to share the CPU in a proportional manner.

To solve for this, we can use tickets and randomness. We pull a number from 0 to 99; assuming process A holds 0 through 74 and B 75 through 99. The winning ticket determies whether A or B runs.

It's not perfect but it will get us to a rough percentage over time.

Ticket Mechanisms
- Ticket currency: allow users to allocate tickets among their own jobs..
- Ticket transfer: in client/server settings, allow the client to send some tickets to the server
- Ticket inflation: a process can temporarily raise or lower the number of ticket it owns. If a process needs more CPU time it can raise the ticket value to reflect that need to the system

Implementation:
- We can use a linked list

head -> job A, tix 100 -> job b, tix 50 -> job c, tix 250 -> null

Keep going over the list with a given tix number and once you go over pick that job.

Deterministic Scheduling:
- We can also use "stride scheduling" which is basically representing each job's stride as the inverse of the number of tickets they hold
- At each point we pick the job with the lowest stride number. Then increase its number. The jobs with the lowest stride (highest tickets) will run more often 
- The downside is we have to maintain global state

Proportional-share scheduleres are more useful in domains that are easier to solve. E.g., a virtualized data center where you can assign CPU cycles to given VMs.

## Virtualization

Every address generated by a user program is a virtual address. The OS gives an illusion that it has a large and private memory. It wants to give programmers the illusion that you have a long contiguous address space to put the code into data.

### Abstraction: Address Spaces

Main Question:
- How can the OS build this abstraction of a private, potentially large address space for multiple running processes (all sharing memory) on top of a single, physical memory?

Isolation is key in building reliable systems. The OS wants to run programs independent of each other. If one fails, it wants to prevent the other ones from failing. Some OS even wall off the OS from others with microkernels. (compared with monolithic kernel designs)

**Goals of virtualizing memory:**
- Transparency - memory visible to the running program
- Efficiency - fast processes using little memory; use hardware support
- Protection - protect processes from one another

### Memory API

Correct memory management has been such a problem, in fact, that many newer languages have support for automatic memory manage- ment. In such languages, while you call something akin to malloc() to allocate memory (usually new or something similar to allocate a new object), you never have to call something to free space; rather, a garbage collector runs and figures out what memory you no longer have references to and frees it for you.

Common issues:
- Forgetting to allocate memory

```c
char *src = "hello";
char *dst; // oops! unallocated 
strcpy(dst, src); // segfault and dieA

char *src = "hello";
char *dst = (char *) malloc(strlen(src) + 1); 
strcpy(dst, src); // work properly
```

- Not allocating enough memory

```c
char *src = "hello";
char *dst = (char *) malloc(strlen(src)); // too small! 
strcpy(dst, src); // work properly
```

- Forgetting to initialize allocated memory
- Forgetting to free memory
- Freeing memory before you're done aka dangling pointers
- Double freeing memory

Good tools to prevent issues: check out both purify [HJ92] and valgrind [SN05]; both are excellent at helping you locate the source of your memory-related problems.

## Project 1B

What are system calls?
- A call between you and the machine, you call the OS and ask him to do stuff for you

### Virtual Memory: Address Translation

Address translation implies mapping the virtual addresses to the physical space. This happens in the kernel space since its a priviledged operation. If we don't find an address we might get a segmentation fault.

The most popular method is dynamic relocation which is:

`physical address = virtual address + base`

The hardware provides base and bound registers. The hardware needs to check whether the address is valid. This is done by the base and bounds register.

The OS also needs to know which areas of memory are free through the "free list".

```
ARG seed 0
ARG address space size 1k
ARG phys mem size 64k

Base-and-Bounds register information:

  Base   : 0x00008000 (decimal 32768)
  Limit  : 16384

Virtual Address Trace
  VA  0: 0x00000360 (decimal:  864) --> VALID: 0x00008360 (decimal: 33632)
  VA  1: 0x00000308 (decimal:  776) --> VALID: 0x00008308 (decimal: 33544)
  VA  2: 0x000001ae (decimal:  430) --> VALID: 0x000081ae (decimal: 33198)
  VA  3: 0x00000109 (decimal:  265) --> VALID: 0x00008109 (decimal: 33033)
  VA  4: 0x0000020b (decimal:  523) --> VALID: 0x0000820b (decimal: 33291)
```


## Segmentation

Key question: how do we support a large address space with many gaps between the stack and the heap.

If we segment, we can place each one of the segments in diff parts of the physical memory avoiding unused virtual address space.

The infamous term segmentation fault arises from a memory access on a segmented machine to an illegal address.

We can have coarse grained and fine-grained segmentation. 

There are many algorithms to minimize external framgentation. Which means there's not one perfect solution.

There are issues when we try allocating variable-sized segments in memory. The main one is external fragmentation. Because they get chopped up it can be hard to allocate the right amount of memory. We also still may have large segments of unused memory. The heap still needs the full allotted space to work.

```
ARG seed 2
ARG address space size 128
ARG phys mem size 1024k

Segment register information:

  Segment 0 base  (grows positive) : 0x00000000 (decimal 0)
  Segment 0 limit                  : 20

  Segment 1 base  (grows negative) : 0x00000200 (decimal 512)
  Segment 1 limit                  : 20

Virtual Address Trace
  VA  0: 0x0000007a (decimal:  122) --> VALID in SEG1: 0x000001fa (decimal:  506)
  VA  1: 0x00000079 (decimal:  121) --> VALID in SEG1: 0x000001f9 (decimal:  505)
  VA  2: 0x00000007 (decimal:    7) --> VALID in SEG0: 0x00000007 (decimal:    7)
  VA  3: 0x0000000a (decimal:   10) --> VALID in SEG0: 0x0000000a (decimal:   10)
  VA  4: 0x0000006a (decimal:  106) --> SEGMENTATION VIOLATION (SEG1)
```

In the sample address space above. We have 128 bye address spaces mapped two two segments. Segment 0 is the heap and segment 1 the stack (grows up). We can only map values it seems that grow from 0 to 20 or from 128 to 108. Therefore the valid addresses are 0-20 and 108-128. 106 causes a segment violation and it can't be mapped to our memory.

### Free-Space Management

Key question: how do we manage free space if we're satisfying variable-sized requests. How do we minimze framentation? What are the time and space overheads of alternate approaches?

Issues with free space:
- Fragmenting the memory

Strategies to manage free space
- Best fit: search through the free list, find chunks of free memory that are as big as the requested size. Then return one that is the smalles of the group of candidates. "Best fit chunk". We can end up paying for the exhaustive search.
- Worst fit: find the largest chunk, return it, keep the remaining large chunk on the free list
- First fit: return the first big enough chunk
- Next fit: keep a pointer to the location within the list where one was looking last. Avoids exhaustive looking.

**Segregated lists**

If an app has few popular-sized requests that it makes. Keep a separate list just to manage objects of that size. Anything else send to a more general allocator. An issue is how much memory do you keep for this speciall allocator. 

To fix this Jeff Bonwick invented the slab allocator. When the kernel boots up, it allocates object caches for kernel objects requested frequently. Thus these caches are segregated free lists of a given size and serve quickly. When they're running low on space they ask for slabs of memory from the general memory allocator.

**Buddy allocation**

Memory is thought of as a big space of size 2^N. When the request is made. THe search divides free space by two until a block that is big enough can be returned. Then its returned to the user.

When the block is freed. The allocator checks whether the buddy 8KB is free. If so, it coalesces the blocks. And so on and so forth. It stops when a buddy is found in use.

There are many trade-offs in these allocators. The more you know about the workloads, the more you can tune it to work better for that workload. Making it fast and spece-efficient is still a big challenge.

In the exercise, we can see how changing the order of the sizes can speed up searching through the lists. When we use worst fit, and sort in descending order, the searching is quick. If it was sorted in ascending order it would take close to O(N). 

When we coalesce the list, even if we have a lot of ops, we end up with a smaller free-list. I assume this would lower the latency of the process. 

When many of the operations were to free vs. allocate we returned more and less failures in allocating memory. It makes sense, when we run out of memory we start returning -1 to the malloc() functions.

### Paging: Introduction

Key question: how do we virtualize memory with pages to avoid the problems of segmentation.

### The Page Table

One of the most important data structures in the memory management subsystem of a modern OS is the page table.

It stores virtual-to-physical address translations. Letting the system know where each page of an address resides in physical memory.

Structure of a page table:

PFN | G | PAT | D | A | PCD | PWT | U/S | R/W | P

- We have a present bit
- A reference bit
- A read/write bit
- A user/supervisor bit
- Hardware caching bits
- An accessed bit
- A dirty bit
- And the page-frame-number itself

**Formula to calculate map from virtual to phyisical memory**

The main page table operation can be exemplified by:

`movl 21, %eax`

This means that we're referencing address 21. We want to translate virtual address 21 into the proper address 117. To do so we need to run the instructions below to find the physical address.

To find the location of the desired page table entry. 

```
// Extract the VPN from the virtual address
VPN = (virtualAddress & VPN_MASK) >> SHIFT

// Form the address of the PTE
PTEAddr = PageTableBase Register + (VPN * sizeof(PTE))
```

In the example above shift is 4 (number of bits in the1 offset). Assuming we have a virtual address 21 (010101); the masking turns this value into 010000; the shift turns it into 01 which would be the virtual page 1.

We then use this value to index the array of PTEs.

```
// Access is OK, form the physical address and fetch it
offset = virtual address & offset mask
physical address = (PFN << shift) | offset
```

With this physical address, the hardware can fetch the desired data from memory and put it into register eax.

Summary code:

```
// Extract the VPN from the virtual address
VPN = (VirtualAddress & VPN_MASK) >> SHIFT

// Form the address of the page-table entry (PTE)
PTEAddr = PTBR + (VPN * sizeof(PTE))

// Fetch the PTE
PTE = AccessMemory(PTEAddr)

// Check if process can access the page
if (PTE.Valid == False)
    RaiseException(SEGMENTATION_FAULT)
else if (CanAccess(PTE.ProtectBits) == False)
    RaiseException(PROTECTION_FAULT)
else
// Access is OK: form physical address and fetch it
offset = VirtualAddress & OFFSET_MASK
PhysAddr = (PTE.PFN << PFN_SHIFT) | offset
Register = AccessMemory(PhysAddr)
```

**Example translation**

Virtual Address 4: 0x00003a1e (Decimal: 14878)

**Extract VPN and Offset:**
- Hexadecimal 0x3a1e to binary is 11 1010 0001 1110.
- VPN: Top 2 bits → 11 (binary) → 3 (decimal).
- Offset: Lower 12 bits → 1010 0001 1110 → 0xa1e (hex).

**Check Page Table for VPN 3:**
- Entry: 0x80000006 (Valid, PFN = 6).

**Calculate Physical Address:**
- PFN 6 to base address: 6 * 4096 = 24576 or 0x6000.
- Physical Address: 0x6000 + 0xa1e = 0x6a1e.

### Reflection questions

What happens as address space grows?
- We get more potential page tables

What happens as we increase page size?
- We get less page tables; bigger ones

Why not use big page sizes in general?
- As we increase the size of the pages, we get less pages in the page table entry.
- The bigger the tables the more fragmentation.
- Bigger pages mean smaller processes must still be allocated a full large page.
- Larger pages can reduce the effectiveness of cache systems. Ihey can decrease the locality of reference.
- When page fault occur, the system must load the required page from disk. Larger pages means more transferred data per page fault, which can slow down the response time

What happens as you increase the percentage of pages that are allocated in each address space?
- We suffer more page faults because the physical memory is busy
- The system might have to swap pages in and out more frequently

### Paging: Faster Translations

Key question: how can we speed up address translation, and generally avoid the extra memory reference that paging seems to require? What kind of hardware support do we need?

To speed up translations we use a cache. We call it the translation-lookaside buffer (TLB). It is simply a hardware cache of popular translations. A better name would be an address-translation cache.

Example algorithm:

```
VPN = (VirtualAddress & VPN_MASK) >> SHIFT
(Success, TlbEntry) = TLB_Lookup(VPN)
if (Success == True)   // TLB Hit
    if (CanAccess(TlbEntry.ProtectBits) == True)
        Offset   = VirtualAddress & OFFSET_MASK
        PhysAddr = (TlbEntry.PFN << SHIFT) | Offset
        Register = AccessMemory(PhysAddr)
    else
        RaiseException(PROTECTION_FAULT)
else                  // TLB Miss
    PTEAddr = PTBR + (VPN * sizeof(PTE))
    PTE = AccessMemory(PTEAddr)
    if (PTE.Valid == False)
        RaiseException(SEGMENTATION_FAULT)
    else if (CanAccess(PTE.ProtectBits) == False)
        RaiseException(PROTECTION_FAULT)
    else
        TLB_Insert(VPN, PTE.PFN, PTE.ProtectBits)
        RetryInstruction()
```

**Caching**

Caching is an important principle of this process. It relies on spatial locality and temporal locality. Spatial locality means if a program accesses memory in an area X, it might access areas nearby in the future. Temporal locality means if an instruction is commonly accessed, it might be re-accessed again in the future. 

All of this relies on the cache being small. Once we try to grow it too much we will have the same issues of accessing other types of memory.

**Who handles TLB misses?**

Hardware and software people didn't trust each other. Thus the hardware sometimes would handle the TLB miss entirely. 

Most architectures now (RISC) handle the TLB with a software-managed TLB. On a TLB miss, the hardware raises an exception, which pauses the instruction stream, raises the priviledge to kernel mode and jumps to a trap handler.

**RISC vs. CISC**

There used to be two camps. Complex Instruction Set Comptuing (CISC) and Reduced ISC. 

CISC sets had a lot of instructions in them; each one was really powerful. The idea was that instructions should be high-level primitives, to make the assembly language easier to use.

RISC are the opposite. They argued to rip out as much hardware as possible and make whats left really fast. RISC chips were faster and MIPS and Sun were started out of it. 

Intel then started making their CISC chips faster and now we have both combinations.

**TLB Contents**

TLBs might have 32, 64, or 128 entires.

VPN | PFN | other bits

Key question: how do we handle the TLB during context switches?

A potential solution is to flush it every time. But this will be costly because then we need to incur new TLB misses until it is refreshed.

Another solution can be adding an identifier to the process that's using each entry. This is known as the Address Space Identifier (ASID). Similar to a process identifier but with fewer bits.

**Issue: Replacement Policies**

What happens when we need to replace a TLB entry? we can use the Least Recently Used method or a random method. Each one has its pros and cons. The LRU can behave unreasonably in certain occassions; so the random one might avoid edge cases in other times.

**Reflection questions:**
- Why do we use TLBs when managing address translations?
- What happens if we didn't have TLBs?
- How do we deal with TLBs across different processes?
- What's the max amount of entries and mappings a TLB could handle?

## Paging: Smaller Tables

Key question: how to make page tables smaller? Linear page tables are too big; taking up far too much memory on typical systems. How can we make them smaller?

If we make them too big, we have internal fragmentation. Often we just end up using 4KB and 8KB pages.

E.g., If we have a page table for a 16KB address space with 1KB pages; we might have 3-4 used pages but the other ones might be empty.

### Multi-Level Page Table

We use a page directory that points to each page table. Each entry in this page diretory has a valid bit and a page frame number. The valid bit indicates if any of the entries in the page table it points to is valid.

```
PDE (Page Directory Entries)
  Valid Bit -> Tell us if the page table has any valid entries
  Page Frame Number -> Points to the page table

Page Table
  Valid Bit
  Protection
  Page Frame Number

Page frame (X bits of info)
  Contains the a set of bits with information on the process

Page-Directory Entry:
PDEAddr = PageDirBase + (PDIndex + sizeof(PDE))
PTEAddr = (PDE.PFN << SHIFT) + (PTIndex * sizeof(PTE))
```

### Tip

Be wary of design trade-offs. Always implement the least complex system that achieves the task at hand. Avoid needless complexity, in prematurely-optimized code or other forms. It makes it harder to understand, maintain and debug the code.

Reflection question:
- Why do we use page tables in the first place: they're used to translate virtual to physical addresses. If the page table is too small we might have collissions?
- How do address translations work in multi-level page entries
- What are the time-space trade-offs of linear page tables vs. multi-level page tables

## Beyond Physical Memory: Mechanisms

General page mechanism:
- The hardware extracts the VPN (virtual page number) from the virtual address
- It then checks the TLB (dictionary cache) for a match; if its a hit, it return the physical address
- If not, it looks up the page table in memory (using the page table base register) and looks up the entry (page table entry) using the VPN as the index
- If the table is valid and present, we extract the page frame number, add it to the cache, and retry generating a cache hit next time

What if we try to access the page but its not in memory? It is normally determined through a "present bit". If its set to 1, we're good. If not (its on the disk) then we we cause a **page fault**

The page fault is handled by a **page fault handler** which is in the software.

If the page is not present, and has been swapped to disk. The OS is going to *swap* the page into memory to service the page fault. It will use the PTE to find the addrtess and issue the request to the disk.

**What if the OS is full?**

We then need to replace the page. Here comes the page replacement policy.

```
 PFN = FindFreePhysicalPage()
// no free page found
2 if (PFN == -1)
    // run replacement algorithm
    3 PFN = EvictPage()
// sleep (waiting for I/O)
4 DiskRead(PTE.DiskAddr, PFN)
// update page table with present
5 PTE.present = True
// bit and translation (PFN)
6 PTE.PFN = PFN
7 RetryInstruction()
```

To keep the memory free the OS has a low and high watermark. It proactively evicts tables until we have a certain amount of free pages. This is done by the page daemon.

Key questions:
- What type of storage devices are easier or faster to swap spaces?
- What happens when a program needs to access memory?

## Beyond Physical Memory: Policies

**Key question**: what page do we evict when we run out of memory?

We want to maximize the amount of cache hits. Number of times a page can be accessed in memory.

We use the "Average Memory Access Time"

$AMAT = T_M + P_{Miss} * T_D$

TM represents the cost of accessing memory, TD the cost of ac- cessing disk, and PMiss the probability of not finding the data in the cache (a miss); PMiss varies from 0.0 to 1.0,

**What is the optimal policy?**

Replace the page that will be accessed the furthest in the future. Resulting in the fewest cache misses.

There are 3 types of cache misses (Three Cs)
- **Compulsory miss aka cold start miss**: it occurs because it was empty to being with and you need to add it
- **Capacity miss**: the cache ran out of space and had to evict an item to bring a new one to the cache
- **Conflict miss**: no space available. Doesn't have in OS page cache because there's no restrictions where can put a page.

**Simpler Policy: FIFO**

We evict the caches in order. It doesn't perform as well as the optimal more complicated policy. Pages are placed in a queue when they enter the system. When we need to replace we evict the page at the end of the queue.

**Another Simple Policy: Random**

Simply picks a random page to replace under memory pressure. Can do better or worst. But simple to implement.

**Usage History Policy: LRU**

A more commonly- used property of a page is its **recency** of access; the more recently a page has been accessed, perhaps the more likely it will be accessed again.

This is based on the "principle of locality". Programs tend to access certain code and data structures frequently. We should use history to figure out which pages are important and keep them.

We then have Least Frequently Used and Least Recently Used.

**Types of Locality**
- Spatial: pages around a given page will be accessed more frequently
- Temporal: pages accessed often will be accessed again

The performance of each policy will vary based on the workload.
- Random workload: they all perform about the same.
- 80/20 workload (some memories get accessed more often): LRU and the Optimal perform better than FIFO and Random
- Looping workload (for loop): OPT and LRU perform better but random performs not too bad.

## Virtualization Key Concepts

- What is a system call and how does it work?
- What is a context switch?
- What are some common process scheduling algorithms?
- How does a process reserve memory?
- What are the different ways in which memory can be stored? (Segmentation and paging)
- How does segmentation work?
- What is the free-space management problem in memory allocation?
- What is the principle of locality in caching?
- How does paging work?
- What are page replacement algorithms and why are they necessary?
- How does the system map virtual memory to physical memory?
- What happens when the OS runs out of memory?
- What happens if we have huge page tables? What are the drawbacks?
	- e.g., Internal fragmentation
- How do page tables work?
- What are the different types of page tables?
- How are multi-level page tables better than linear ones?
- What is a TLB? When is it used?
- What happens during TLB misses? 
- What are some policies we use to replace cache in TLBs?
- How do segmentation fault occurs?

# Phase 2: Concurrency

## Introduction to Concurrency

Concurrency is great because it allows us to do parallel programming and to run the program while we wait for I/O operations.

Concurrency is harder than single-threaded programs because we now have to manage shared data and state across threads. 

We can run into issues such as data races and deadlocking among others.

To solve for this, Dijkstra invented atomic operations and sync primitives that allow us to give some guarantees to the program.

An example is sempahores. They guarantee at most N threads will pass through a critical section.

Another example is incrementing a counter with two threads, if we don't use sync primitives we may run into an indeterministic program. i.e., run it twice and get different results.

The most important questions are:
- What support do we need from the hardware in order to build useful synchronization primitives? 
- What support do we need from the OS?
- How can we build these primitives correctly and efficiently?
- How can programs use them to get the desired results?

The key terms are:
- A critical section: two threads accessing a shared resource
- A race condition: two threads targeting a critical section
- An indeterminate program vs. indeterminate: random vs. not
- Mutual exclusion primitives: 1 single thread ever entering critical sections

An useful tool to validate threads in C is helgrind, run it with `valgrind --tool=helgrind ./main-signal-cv`

## Thread APIs

- pthread_create()
- pthread_join()
- pthread_mutex_lock(&lock);
- pthread_mutex_unlock(&lock);
- pthread_cond(&lock);

## Locks

Key Question:
- How do we build an efficient lock? It should provide mutual exclusion at a low cost
- How does each lock work? Can you explain the principles of each lock and why do we use it?
- What are the most used locks in practice?
- How do we measure the CPU impact of different types of locks?

A good lock should provide:
- Fairness
- Mutual exclusion
- No starvation
- Performance (as low overhead as possible)

Locks can have hardware support and OS support to run properly. To build a good lock we need to assume a little hardware support. That way we can meet all the properties above.

### Spin lock using Test-and-set

As long as the lock is held by another thread, TestAndSet() will repeatedly return 1, and thus this thread will spin and spin until the lock is finally released.

```
typedef struct __lock_t {
    int flag;

void init(lock_t *lock) {
    // 0: lock is available, 1: lock is held
    lock->flag = 0;
}

void lock(lock_t *lock) {
		// It tests the old value while simultaneously sets it to
		// a new value
    while (TestAndSet(&lock->flag, 1) == 1)
        ; // spin-wait (do nothing)
}
void unlock(lock_t *lock) {
    lock->flag = 0;
}
```

### Compare-And-Swap

Checks if the value at an address is expected, if so updates it with a new value.

```
   void lock(lock_t *lock) {
          while (CompareAndSwap(&lock->flag, 0, 1) == 1)
					; // spin
```

### Load-Linked and Store-Conditional

- **Load-Linked**: fetches a value from memory and places it in a register
- **Store-Conditional**: only succeeds if no intervening store to the address has taken place

Potential lock: 
- We fetch a value from memory
- Only succeed if no store has taken place

If there's not a value stored in memory keep rotating.

E.g.,

```
int LoadLinked(int *ptr) {
    return *ptr;
}

int StoreConditional(int *ptr, int value) {
    if (no update to *ptr since LoadLinked to this address) {
        *ptr = value;
        return 1; // success!
    } else {
        return 0; // failed to update
    }
}

void lock(lock_t *lock) {
       while (1) {
           while (LoadLinked(&lock->flag) == 1)
               ; // spin until it’s zero
           if (StoreConditional(&lock->flag, 1) == 1)
               // if set-it-to-1 was a success: all done
               // otherwise: try it all over again
               return; 
   
   void unlock(lock_t *lock) {
       lock->flag = 0;
	}
```

### Fetch-And-Add

```
int FetchAndAdd(int *ptr) {
    int old = *ptr;
    *ptr = old + 1;
    return old;
}

typedef struct __lock_t {
    int ticket;
    int turn;
} lock_t;


void lock_init(lock_t *lock) {
    lock->ticket = 0;
    lock->turn = 0;
}

void lock(lock_t *lock) {
    int myturn = FetchAndAdd(&lock->ticket);
    while (lock->turn != myturn)
        ; // spin
}

void unlock(lock_t *lock) {
    lock->turn = lock->turn + 1;
}
```


### OS Support

In the case of some locks, the OS can allows us to yield or park certain threads. This works well in some cases. 

Yield lets us yield the CPU to other threads while our current thread is blocked.

Park allows us to put a calling thread to sleep and unpack  to wake it up by its `threadID`

## Lock-Based Concurrent Data Structures


## Conditional Variables

Key questions:
- How can we make a thread wait based on some condition? In a way that is efficient and doesn't waste CPU cycles?

**Condition variables**
A condition variable has two operations associated with it: wait() and signal()

E.g.,

```c
int done  = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t c  = PTHREAD_COND_INITIALIZER;

void thr_exit() {
    Pthread_mutex_lock(&m);
    done = 1;
    Pthread_cond_signal(&c);
    Pthread_mutex_unlock(&m);

}

void *child(void *arg) {
    printf("child\n");
    thr_exit();
    return NULL;

}

void thr_join() {
    Pthread_mutex_lock(&m);
    while (done == 0)
		Pthread_cond_wait(&c, &m);
	Pthread_mutex_unlock(&m);
} int

void main(int argc, char *argv[]) {
	printf("parent: begin\n");
	pthread_t p;
	Pthread_create(&p, NULL, child, NULL);
	thr_join();

	printf("parent: end\n");
	return 0; 
}
```

**Note:**
- Although it is strictly not necessary in all cases, it is likely simplest and best to hold the lock while signaling when using condition variables.
- Always use while loops with conditional variables (Mesa semantics); you want to re-check the condition 

### Semaphores

**Key question**
- How can we use semaphores instead of locks and condition variables? What is a semaphore and how do we use it?

> A semaphore is an object with an integer value that we can manipulate with two routines; in the POSIX standard, these routines are sem wait() and sem post()

`sem_wait(s)`: decrements the value of semaphore s by one, wait if the value is negative

`sem_post(s)`: increments value of s by one, if there are threads waiting, wake one

**Binary Semaphores**

\We can have multiple types of semaphores, an example of one is a `Binary Semaphore` which acts as a lock.

**Semaphores for Ordering**

We can also use semaphores to have a parent wait for its child to run. To do this, we set `s` as 0. 

![[Pasted image 20240625101238.png|500]]

**Producer/Consumer Problem**

> One simple way to think about it, thanks to Perry Kivolowitz, is to consider the **number of resources you are willing to give away immediately after initialization.** With the lock, it was 1, because you are willing to have the lock locked (given away) immediately after initialization. With the ordering case, it was 0, because there is nothing to give away at the start; only when the child thread is done is the resource created, at which point, the value is incremented to 1. Try this line of thinking on future semaphore problems, and see if it helps.

We can use multiple semaphores to add mutual exclusion.

```c
void *producer(void *arg) {
    int i;
    for (i = 0; i < loops; i++) {
        sem_wait(&empty);   // Wait for empty slot
        sem_wait(&mutex);   // Enter critical section
        put(i);             // Put item in the buffer
        sem_post(&mutex);   // Leave critical section
        sem_post(&full);    // Increment count of full slots
    }
}

void *consumer(void *arg) {
    int i, tmp;
    for (i = 0; i < loops; i++) {
        sem_wait(&full);    // Wait for full slot
        sem_wait(&mutex);   // Enter critical section
        tmp = get();        // Get item from the buffer
        sem_post(&mutex);   // Leave critical section
        sem_post(&empty);   // Increment count of empty slots
        printf("%d\n", tmp); // Print the consumed item
    }
}
```

**Reader-Writer Locks**

Reader-writer locks allow us to have multiple readers (threads) accessing a critical section to read data. If there's a writer modifying the shared data we won't allow readers to access this section.

```c
typedef struct _rwlock_t {
  sem_t lock;      // binary semaphore (basic lock)
  sem_t writelock; // allow ONE writer/MANY readers
  int   readers;   // #readers in critical section
  rwlock_t;

void rwlock_init(rwlock_t *rw) {
  rw->readers = 0;
  sem_init(&rw->lock, 0, 1);
  sem_init(&rw->writelock, 0, 1);

}

void rwlock_acquire_readlock(rwlock_t *rw) {
  sem_wait(&rw->lock);
  rw->readers++;
  if (rw->readers == 1) // first reader gets writelock
    sem_wait(&rw->writelock);
  sem_post(&rw->lock);

}

void rwlock_release_readlock(rwlock_t *rw) {
  sem_wait(&rw->lock);
  rw->readers--;
  if (rw->readers == 0) // last reader lets it go
    sem_post(&rw->writelock);
  sem_post(&rw->lock);
}

void rwlock_acquire_writelock(rwlock_t *rw) {
  sem_wait(&rw->writelock);
}

void rwlock_release_writelock(rwlock_t *rw) {
  sem_post(&rw->writelock);
}
```

### Common Concurrency Bugs

Key question:
- How to recognize and prevent common bugs?

#### Non-Deadlocks

**Atomicity Violations**

One thread tries to access a shared variable before the other. To fix it, we need to add mutex around the critical sections.

```c
Thread 1::
if (thd->proc_info) {
	fputs(thd->proc_info,...)
}

Thread 2::
thd->proc_info = NULL;

pthread_mutex_t proc_info_lock = PTHREAD_MUTEX_INITIALIZER;

Thread 1::
pthread_mutex_lock(&proc_info_lock);
if (thd->proc_info) {
    fputs(thd->proc_info, ...);
}
pthread_mutex_unlock(&proc_info_lock);

Thread 2::
pthread_mutex_lock(&proc_info_lock);
thd->proc_info = NULL;
pthread_mutex_unlock(&proc_info_lock);

```
**Ordering Violations**

Threads depend on each other and one thread might run before the other. Thread 2 will fail if it runs first.

```c

Thread 1::
void init() {
	mthread = create_thread()
}

Thread 2::
void mMain() {
	m_state = mthread->state
}
```

We need to initialize conditional variables and mutex to allow the first thread to run first.

```c
pthread_mutex_t mtLock = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t  mtCond = PTHREAD_COND_INITIALIZER;
int mtInit = 0;

Thread 1::
void Init() {
    ...
    mThread = PR_CreateThread(mMain, ...);
    
    // signal that the thread has been created...
    pthread_mutex_lock(&mtLock);
    mtInit = 1;
    pthread_cond_signal(&mtCond);
    pthread_mutex_unlock(&mtLock);
    ...
}

Thread 2::
void mMain(...) {
    ...
    // wait for the thread to be initialized...
    pthread_mutex_lock(&mtLock);
    while (mtInit == 0)
        pthread_cond_wait(&mtCond, &mtLock);
    pthread_mutex_unlock(&mtLock);
    
    mState = mThread->State;
}
```

#### Deadlock Bugs

Conditions for deadlocks are:
- Mutual exclusion
- Hold and wait
- No preemption
- Circular wait

*If any of these four conditions are not met, deadlock cannot occur.*

**How to prevent the common deadlocks?**

**Circular wait**
Use total ordering or partial ordering: acquire one lock before the other

```c
if (m1 > m2) { // grab in high-to-low address order
       pthread_mutex_lock(m1);
       pthread_mutex_lock(m2);

     } else {
       pthread_mutex_lock(m2);
       pthread_mutex_lock(m1);

     }
```

**Hold-and-wait**
The hold-and-wait requirement for deadlock can be avoided by acquiring all locks at once, atomically.

```c
pthread_mutex_lock(prevention); // begin acquisition
pthread_mutex_lock(L1);
pthread_mutex_lock(L2);
pthread_mutex_unlock(prevention); // end
```
**No preemption**
Keep trying to grab the lock constantly.

Big issue, a **live-lock**.Two threads could both be repeatedly attempting this sequence and repeatedly failing to acquire both locks. In this case, both systems are running through this code sequence over and over again (and thus it is not a deadlock), but progress is not being made, hence the name livelock

```c
top:
	pthread_mutex_lock(L1)
	if (pthread_mutex_trylock(L2) != 0) {
	    pthread_mutex_unlock(L1);
	    goto top;
	}
```

**Mutual exclusion**
Use a lock-free or wait-free approach. Use compare-and-swap instructions.

**Steps:**
- Read the value at the target memory address
- Compare it with an expected value
- If equal, swap it with a new value
- If not equal, do nothing

```c
int CompareAndSwap(int *address, int expected, int new) {
	if (*address == expected) {
		*address=new;
		return 1;
	}
	return 0;
}
```

**Scheduling**: used in small hardware OS where we know the tasks ahead of time.
**Detect and recover:** used in databases; they run periodical deadlock detectors for cycles.


### Event-Based Concurrency

> How can we build a concurrent server without using threads, and thus retain control over concurrency as well as avoid some of the problems that seem to plague multi-threaded applications?

We can use functions such as `select()` or `poll()` to constantly check for updates on a given event. 

We do this on an event loop: `while (1)`

Handling events has challenges:
- Handling state (you don't have a stack): we have to manually manage the stack.
- Handling blocking system calls (the event loop would sit idle)
- If you have more than 1 CPU, you need to handle multiple event handlers
- Page faults are harder to handle. They will block the server until the page fault completes
- Hard to manage over time. The semantics of the routine might change.

# File Systems

## I/O Devices

**Key questions**
- How should I/O be integrated into our systems? What should be the ideal mechanism?
- How can the OS check device status without frequent polling? Lowering CPU overhead to manage the device?
- How to lower PIO overheads? i.e., spending too much time moving data to and from the devices by hand.
	- How can we offload this work?
- How should hardware communicate with devices? Explicitly? Or abstracting?
- How can we build a device-neutral OS? Hide t he device interactions?

**System architecture**
In a traditional diagrams we have a CPU attached to the main memory via some kind of memory bus. They connect via a general I/O bus. Lower down we have peripheral buses too, such as SCSI, SATA, and USB. They connect to slower devices such as disks, mice and keyboard.

![[Pasted image 20240708193837.png|400]]

The faster the bus is, the shorter it must be. We put high performance components closer to the CPU.

Modern systems put the components close to the CPU. 

![[Pasted image 20240708193953.png|500]].


### Devices 

A canonical device needs the following.

![[Pasted image 20240708194121.png|500]]

A typical OS interaction will be like this.

```c
 While (STATUS == BUSY)
       ; // wait until device is not busy

   Write data to DATA register
   Write command to COMMAND register

       (starts the device and executes the command)
   While (STATUS == BUSY)

       ; // wait until device is done with your request
```

The OS waits until the device receives a command by polling it. The CPU sometimes will send data to the device via programmed I/O (PIO). The OS will write a command to the register. Finally, the OS will wait for the device to finish by polling in a loop.

Polling takes a lot of time, so we enable **interrupts**.

The interrupt will stop the I/O to run other operations. This is best for slower processes, if we have a fast I/O process we might slow down the overall performance of the system.

Sometimes we can do a hybrid approach. Use polling and interrupts. This allows to us to prevent **live-locks**. 

**Moving data efficiently with Direct Memory Access (DMA)**

A DMA engine is a device tailored to orchestrate transfers between devices and main memory without the CPU. It's like another machine that does the transfer if you give it instructions, freeing the CPU.

**How to fit devices into an OS?**

We use device drivers. This is actual code in the OS to map to have instructions on how to access the specific device. They involve most of the code in modern OS's, 70% of the lines of code in Linux.

With proper mapping, we can make the device feel as if it was part of the file system. 

![[Pasted image 20240708195259.png|500]]

## Hard Disk Drives

Key questions:
- How do modern hard-disk drives store data? What is the interface? How is the data actually laid out and accessed? How does disk scheduling improve performance?
- How can we implement SSTF-like scheduling but avoid starvation?
- How can we implement an algorithm that more closely approximates SJF by taking both seek and rotation into account?

Tips:
- Use disks sequentially: transfer data in a sequential manner or in large chunks; avoid small I/O ops at all costs
- Almost any question can be answered with “it depends”, as our colleague Miron Livny always says. However, use with caution, as if you answer too many questions this way, people will stop asking you questions alto- gether.

![[Pasted image 20240710101857.png]]
## Redundant Array of Inexpensive Disks (RAIDs)

Key questions:
- What are RAIDs?
- What are they used for? 
- What are the different levels?
- What kind of applications benefit from each level?
- What are software vs. hardware RAIDs?
- What are the design considerations when it comes to RAIDs?

## Files and Directories

Key questions:
- How should the OS manage a persistent device? What are the APIs?

A file is just a linear array of bytes, each of which you can read or write. Each file has a low-level name. It is often referred as inode number.

### Files & Commands

We can create a new file with `open()`; we can add flags to it such as: `O_CREAT`, `O_WRONLY` or `O_TRUNC`

```c
int fd = open("foo", O_CREAT|O_WRONLY|O_TRUNC, S_IRUSR|S_IWUSR);
```

When we create a file with `open()` it returns a [[file_descriptor]]. A file descriptor is an integer, private per process, used in UNIX systems to access files. An opaque handle that lets you do certain ops. A pointer to an object of type file.

Each process maintains an array of file descriptors. Each one refers to an entry in the system-wide open file table.

When we call `write()` (update a file in persistent storage) it acts as if it writes the updates right away but they actually go into a buffer in memory then get written after a few seconds.

To write to files right away, sometimes we use `fsync()` this guarantees that the writes will be performed asap in critical systems such as database mgmt systems. 

When we call `mv foo bar` these are the system calls that happen:

```c
int fd = open("foo.txt.tmp", O_WRONLY|O_CREAT|O_TRUNC,
              S_IRUSR|S_IWUSR);

write(fd, buffer, size); // write out new version of file
fsync(fd);
close(fd);
rename("foo.txt.tmp", "foo.txt");
```

When we call `rm foo` it calls the following actions:

```shell
prompt> strace rm foo
...
unlink("foo")                           = 0
```
### Directories

An empty directory contains `.` and `..`

We can modify them with `mkdir`, and `rmdir`

A key part of directories is `link`. It basically creates a new reference to the inode with your preferred file name.

```bash
prompt> echo hello > file
prompt> cat file
hello
prompt> ln file file2
prompt> cat file2
hello
```
The file is not copied. You just now have two names.

```bash
prompt> ls -i file file2
67158084 file
67158084 file2
prompt>
```

When we `unlink` we remove the references to that file. Over time, the system will check the reference counts to that file and if it reaches zero the system will free the inode and the related data blocks.

We can also use symbolic links with: 

```bash
prompt> echo hello > file
prompt> ln -s file file2
prompt> cat file2
hello
```

The issue is that they might create a dangling reference if we remove the file while keeping file2. They're its own type called: `symbolic link`

### Key terms

- A **file** is an array of bytes which can be created, read, written, and deleted. It has a low-level name (i.e., a number) that refers to it uniquely. The low-level name is often called an i-number.
- A **directory** is a collection of tuples, each of which contains a human-readable name and low-level name to which it maps. Each entry refers either to another directory or to a file. Each directory also has a low-level name (i-number) itself. A directory always has two special entries: the . entry, which refers to itself, and the .. entry, which refers to its parent.
- A directory tree organizes all files into a large tree starting at the **root**
- To access a file, a process must use a system call (usually, open()) to request permission from the operating system. If permission is granted, the OS returns a **file descriptor**, which can then be used for read or write access, as permissions and intent allow.
- Each file descriptor is a private, per-process entity, which refers to an entry in the **open file table**. The entry therein tracks which file this access refers to, the **current offset** of the file (i.e., which part of the file the next read or write will access), and other relevant information.
- To force updates to persistent media, a process must use fsync() or related calls. However, doing so correctly while maintaining high performance is challenging [P+14], so think carefully when doing so
- To have multiple human-readable names in the file system refer to the same underlying file, use hard links or symbolic links. Each is useful in different circumstances, so consider their strengths and weaknesses before usage. And remember, deleting a file is just performing that one last unlink() of it from the directory hierarchy.

### Implementing a File System

Key questions: 
- How do we implement a simple file system?
- What are the data structure it needs
- Even the simplest of operations like opening, reading, or writing a file incurs a huge number of I/O operations, scattered over the disk. What can a file system do to reduce the high costs of doing so many I/Os?

**i-Nodes**
Structure that holds metadata for a given file (length, permissions, and location of its blocks) short for index node. It is used to index into an array of on-disk inodes to find the inode of that number. (can also be called dnode, fnodes, etc)

When engineers design file systems, they hold these high level rules:
- Most files are small; 2K most common
- Avg. file size is growing; 200K is avg
- Most bytes are stored in large files; few big files occupy most of the space
- File systems contains lots of files; up to 100K on avg
- Directories are typically small; most 20 or fewer

### Locality and the Fast File System

**Key questions:**
- How can we organize file system data structures so as to improve per- formance? What types of allocation policies do we need on top of those data structures? How do we make the file system “disk aware”?

Researchers at Berkeley implemented the Fast File System. The main idea is to design structures and allocation policies to be "disk aware" and thus improve performance. 

### Log-Structured File Systems

These type of file systems focus on writes. They keep writing unto the file system. Whenever there's an overwrite, it creates a new block of memory with a more fresh version. 

The downside is that it needs to clean up those old versions over time. That means it needs to implement cleaning policies for the old data.

The system came from the revelation that there's a large gap between random I/O and sequential I/O performance. That file systems are not [[RAID]] aware

All data in LFS is written in a continuous sequential stream, taking advantage of the speed of sequential I/O.

When writing to disk, LFS buffers the updates (incl. metadata) in an in-memory segment. When its full, the data is written to a disk in one sequential long transfer.

**Finding Inodes**

Normally, inodes are stored in a fixed location in the disk. In the old UNIX file system it was easy, you found them in the fixed location the multiplied by the offset to find the inode you wanted.

In FFS they're split up into chunks and places a group of inodes within each cylinder group. 

In LFS, they're spread out across the disk. The latest version of the inode also keep moving. 

To solve this we use an inode map. It takes an inode number as input and produces the disk address of the most recent version.

How to find this inode map? We have a fixed location in-disk called the "**checkpoint region (CR)**". It contains pointers to the latest pieces of the inode map.

![[Pasted image 20240729095120.png|500]]

**Collecting garbage**

LFS must periodically find dead versions and clean them. Periodically, the LFS cleaner finds the old segments and determines which blocks are live. Then writes out a new set of segments just with the live blocks.

How to know which ones are live?

We use a segment summary block. 

Longer method: check the segment summary, then the imap, and check if the inode routes back to the address A. If it does then its alive.

```
(N, T) = SegmentSummary[A];
inode  = Read(imap[N]);
if (inode[T] == A)
    // block D is alive
else
    // block D is garbage
```
Shorter method: record a version number in the on-disk segment and compare them with the version in the imap.

**Recovering crashes**

What if the system crashes while LFS is writing to disk? What if it fails during writes to the logs or checkpoint region.

Checkpoint region failure
- Keep two CRs, one at either end of the disk and write to them alternately
- Write out first a header, then the body of the CRR, then one last block. If the timestamps are inconsistent then we know they failed

Log failure
- Recover the log by reading the checkpoint region. Use a **roll-forward**. Roll through the segments and see if they have valid updates.
- Recover the data and metadata written since the last checkpoint

### Flash-based SSDs

**Key SSD Terms:**
- Flash chips have many banks, each organized into blocks. Each block is further divided into pages
- Blocks are large (128KB-2MB); pages are small (1-8KB)
- To read, we issue a read command with an address and length
- Writing is harder, we need to erase the block, then program the page once. Careful doing this too often because the block will wear out.
- Flash reliability is mostly derived by wear out. If a block gets programmed too often, it is unusable
- A flash-based SSD behaves as a normal block-based read/write disk by using a flash translation layer
- Most FTLs are log-structured; reducing the cost of writing but you then have to deal with garbage collection
- The mapping table can be large, we use hybrid mapping or caching hot pieces when possible
- One last problem is wear leveling, we need to occasionally migrate data from blocks that are mostly read 

**Key questions:**
- How can we build a flash-based SSD? How can we handle the expensive nature of erasing? How can we build a device that lasts a long time, given that repeated overwrite will wear the device out? Will the march of progress in technology ever cease? Or cease to amaze?

There's a new rise in solid-state storage. They're better than disk drives because they have no mechanical or moving parts. They're simply built out of transistors, like memory and processors. Unlike DRAM they retain info despite power loss. 

**Basic flash operations:**
- Read a page: specify a page number and read it, takes 10s of a microsecond. The device can access any location uniformly, meaning its a [[random access device]].
- Erase a block: before writing to a block, we have to erase the entire block the page lies within. Takes a few milliseconds to complete.
- Program a page: Less expensive than erasing a block, but more costly than reading a page. Takes 100s of microseconds.

![[Pasted image 20240730085551.png]]

Therein, the author presents the basic operation latency of reads, programs, and erases across SLC, MLC, and TLC flash, which store 1, 2, and 3 bits of information per cell, respectively.

![[Pasted image 20240730090020.png]]

**SSD Performance And Cost**

**Performance:** 
- No mechanical components, similar to DRAM

![[Pasted image 20240730091016.png]]

The reason for such unexpectedly good random-write performance is due to the log-structured design of many SSDs, which transforms random writes into sequential ones and improves performance.

**Cost:**
The answer is simple: cost, or more specifically, cost per unit of capacity. Currently an SSD costs something like $150 for a 250-GB drive; such an SSD costs 60 cents per GB. A typical hard drive costs roughly $50 for 1-TB of storage, which means it costs 5 cents per GB. There is still more than a 10× difference in cost between these two storage media.


### Data Integrity and Protection

**Key questions:**
- How should systems ensure that the data written to storage is protected? What techniques are required? How can such techniques be made efficient, with both low space and time overheads?
- How should a storage system handle latent sector errors? How much extra machinery is needed to handle this form of partial failure?
- Given the silent nature of such failures, what can a storage system do to detect when corruption arises? What techniques are needed? How can one implement them efficiently?
- How should a storage system or disk controller detect misdirected writes? What additional features are required from the checksum?
- How should a storage system or disk controller detect lost writes? What additional features are required from the checksum?

Multiple issues can arise in disks. Therefore, it is important to have a few techniques to cross-check the disk health.

Here are common issues:
- Single-block failures
	- Latent-sector errors: a disk head touches the surface (head crash) and damages it making the bits unreadable. We use in-disk error correcting codes to determine when on-disk bits in a block are good
	- Block corruption: buggy firmware may write a block to wrong locations. They are silent faults. 
- Misdirected writes: arises in disk and RAID controllers which write the data to disk correctly, except in the wrong location
- Lost writes: occurs when the device informs us that a write has been completed but it was never persisted. The old contents are still there.


**Handling latent segment errors:**
- 

**Handling block corruption:**
- Use **[[checksums]]**, it validates using a math function the bytes in a given memory space. It gives a small summary of the contents of the data. It tells us whether the data has somehow been corrupted
- There's different checksum formulas, each one has their pros and cons.
	- XOR-based
		- Do an XOR of each set of binary bits
	- Additives
		- Add the decimal values of all the bits
	- [[Fletcher checksum]]
		- Use two running sums with modulo operations
	- Cyclic redundancy check

**Handling misdirected writes:**
- Add more information to each checksum, e.g., a physical identifier. If the stored information now contains the checksum C(D) and both the disk and sector numbers of the block, it is easy for the client to determine whether the correct information resides within a particular locale.
- ***Redundancy is the key to error detection and recovery***

**Handling lost writes:**
- None of our previous strategies help here.
- A basic approach could be to read after write. We could also add a check-sum in each file system inode and indirect block. If the write to a data block is lost, the checksum within the inode wont match the old data.

When to clean the disks and run all these operations? Overnight, every once in a while. It's called ***disk scrubbing***.

****