
# Virtual Memory

## Scheduling

...

# Concurrency

## Thread: API

Key questions:
- What's the API behind pthreads and how do we use it?
  - How do we return values?
  - How do we use shared structures?
  - What does join do?
- What support do we need from the hardware in order to build useful synchronization primitives? 
- What support do we need from the OS?
- How can we build these primitives correctly and efficiently?
- How can programs use them to get the desired results?

## Thread: Locks

Key questions:
- How do we measure each of the principles behind concurrency? e.g., fairness, mutual exclusion, performance, and correctness
- How do we build an efficient lock? It should provide mutual exclusion at a low cost
- What are the hardware primitives for locks?
	- Test-and-Set (TAS)
	- Compare-and-Swap
	- Fetch-and-Add
	- Load-Link/Store-Conditional (LL/SC)
- How does each lock work? Can you explain the principles of each lock and why do we use it?
	- How does the Peterson lock work?
	- How does the TAS lock and the TTS lock work?
	- What does yield help us accomplish in locks?
		-  Yield is simply a system call that moves the caller from the running state to the ready state, and thus promotes another thread to running. Thus, the yielding thread essentially de-schedules itself.
		-  It doesn't address starvation given that some threads might yield indefinitely 
- What are the most used locks in practice?
- How do we measure the CPU impact of different types of locks?

## Thread: Conditional Variables

- **What is the ideal way to use a conditional variable in C?**
	- Use `pthread_mutex_lock` and `pthread_mutex_unlock` to protect the shared resource.
	- Use `pthread_cond_wait` to wait for a condition. This releases the mutex and blocks the thread until the condition is signaled.
	- Use `pthread_cond_signal` or `pthread_cond_broadcast` to signal waiting threads when the condition changes.
- **What is the difference between using a while vs. if statement in producer consumer problems with conditional variables?**
	- A `while` loop is used to re-check the condition after waking up because spurious wake-ups can occur. This ensures that the condition is still valid before proceeding.
	- An `if` statement checks the condition only once. This can lead to issues if a spurious wake-up occurs, as the thread might proceed even if the condition is not met. Aka race conditions.
- **What is the difference between using 1 or 2 conditional variables in producer consumer problems?**
	- A single condition variable is used to signal both producers and consumers. This can work but is less efficient because both types of threads wake up and re-check their conditions even if only one type of thread can proceed.
	- Separate condition variables for producers and consumers (`empty` and `fill`) provide more fine-grained control. Producers wait on `empty` (signaled when an item is consumed), and consumers wait on `fill` (signaled when an item is produced). This improves efficiency because only the relevant threads are woken up.
- **What is the role of the buffer in producer / consumer problems?**
	- It decouples producers and consumers. They can both access and consumer from the same data. Allows the producers and consumers to operate independently.
	- It acts as a temporary storage. It's helpful when the producer and consumer work at different speeds.
	- It allows for concurrent execution. Both threads can add and consume from the buffer.
- **Does buffer size matter in producer / consumer problems?**
	- Bigger buffers can take more memory and increase the complexity of managing the buffer.

### Thread: Semaphores

- How do semaphores work?
- When are semaphores useful?
- What are ways in which we can use semaphores? (e.g., counting, binary)

### Common Concurrency Bugs

- What are the most common non-deadlock bugs?
- What are atomicity violations in non-deadlock bugs?
- What are ordering violations in non-deadlock bugs?
- What are the 4 conditions for deadlocks to occur?

# File Systems

## Hard Disk Drives

- What is SCSI vs. STATA?
- Why does it take so long for a disk to read a given section?
- What is the main use of hard disk drives?
- What are the different scheduling mechanisms to seek information from the disk?
- What is the role of the OS in scheduling disk seeks?
- Why is it better to read data sequentially from the disk over randomly?
- What's the history behind hard disk drives?

### Files and Directories

- What are file descriptors in C?
- What are file descriptions in C?
- What is an `i-node` and why is it called like that?
- How do we delete files in the systems? (unlink)

### File Systems

- What are i-nodes? What kind of information is stored in an i-node?
- What are bitmaps?
- What is the difference and the trade-offs between static and dynamic partitioning in file systems?
- How do file systems use caching to improve response times? What are the trade-offs?
- How do file systems use buffering to improve response times? What are the trade-offs?
