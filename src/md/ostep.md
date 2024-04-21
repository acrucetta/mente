
## Notes on Operating Systems in 3 Easy Pieces

### Anki Questions

#### System Calls

**What does wait() do?**
The parent process calls wait() to delay its execution until the child finishes executing. When the child is done, wait() returns to the parent.

```c
int main() {
    int rc = fork(); // Fork a new process

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

Segregated lists: if an app has few popular-sized requests that it makes. Keep a separate list just to manage objects of that size. Anything else send to a more general allocator. An issue is how much memory do you keep for this speciall allocator. 

To fix this Jeff Bonwick invented the slab allocator. When the kernel boots up, it allocates object caches for kernel objects requested frequently. Thus these caches are segregated free lists of a given size and serve quickly. When they're running low on space they ask for slabs of memory from the general memory allocator.


