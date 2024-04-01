
## Notes on Operating Systems in 3 Easy Pieces

### Anki Questions

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

**How does pipe() work? What are the main components?**

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