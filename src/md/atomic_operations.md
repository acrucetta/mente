
## History of Atomic Operations and Locks

### Key questions
- Who invented atomic operations?
- How does hardware support atomic operations?

### Research

Key Figures and Milestones in the Development of Atomic Operations
Early Mutual Exclusion Concepts:

Edsger Dijkstra: In the 1960s, Dijkstra introduced the concept of semaphores, which are pivotal in the development of synchronization mechanisms. Semaphores are a type of synchronization primitive that can be considered a precursor to atomic operations in ensuring mutual exclusion.
Semaphores: They are a signaling mechanism to control access to shared resources, crucial in preventing race conditions.
Test-and-Set:

The test-and-set instruction is one of the earliest hardware-supported atomic operations. It was developed in the 1960s and 1970s. This operation allows a process to test and modify a memory location in a single, indivisible step.
Gary L. Peterson: Known for his work on Peterson's Algorithm, a software-based mutual exclusion algorithm that relies on the test-and-set instruction.
Compare-and-Swap (CAS):

Michael J. Fischer and Michael O. Rabin: Their work in the early 1980s led to the development of the compare-and-swap (CAS) operation, which has become a fundamental building block for many modern lock-free data structures and algorithms.
CAS Operation: It compares the contents of a memory location to a given value and, only if they match, modifies the contents to a new value. This atomicity is critical for non-blocking algorithms.

Transactional Memory:
Maurice Herlihy: In the early 1990s, Herlihy's work on wait-free synchronization and transactional memory has been influential. His research introduced advanced concepts that furthered the development of atomic operations.
Transactional Memory: Allows sequences of read and write operations to be executed atomically, simplifying concurrent programming by abstracting the complexities of lock management.

