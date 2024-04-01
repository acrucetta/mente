
## Notes on Parallel Programming

### Introduction

Race Condition: A race condition occurs in Go when two or more goroutines try to access and modify the same data at the same time. This can lead to data corruption
, inconsistent states, or crashes.

Atomicity: within the context it operates it is indivisible or uninterruptible. Atomicity is important because if something is atomic, implicitly it is safe within concurrent contexts. This allow us to run safe parallel programs.

Deadlock: concurrent parts of the program are waiting for one another.

Types of deadlock:
- Mutual exclusion - process p1 holds exclusive rights for p2
- Wait for condition - hold a resource and wait for an additional resource
- No preemption - resource is held by a concurrent process and it can only be released  by that process
- Circular wait - p1 is waiting for p2 but p2 is waiting for p1

Livelock: the program runs concurrent ops but these ops don't move the code forward.

A common reason livelocks are written: two or more concurrent processes attempting to prevent a deadlock without coordination.

Finding a balance between polite and greedy locking:
- If you utilize memory access synchronization, you’ll have to find a balance between preferring coarse-grained synchronization for performance, and fine-grained syn‐ chronization for fairness. When it comes time to performance tune your application, to start with, I highly recommend you constrain memory access synchronization only to critical sections; 

When exposing functions, methods, and variables in problem spaces that involve concurrency, do your colleagues and future self a favor: err on the side of verbose comments, and try and cover these three aspects.

• Who is responsible for the concurrency?
• How is the problem space mapped onto concurrency primitives?
• Who is responsible for the synchronization?

### Communicating Processes

> Concurrency is a property of the code; parallelism is a property of the running program.

> The first is that we do not write parallel code, only concurrent code that we hope will be run in parallel. Once again, parallelism is a property of the runtime of our program, not the code.

Go created the primitives of channels based on Hoare's research on input/output of a program being an underexplored part of computer science.

> One of Go’s mottos is “Share memory by communicating, don’t communicate by shar‐ ing memory.
 That said, Go does provide traditional locking mechanisms in the sync package. Most locking issues can be solved using either channels or traditional locks.
  So which should you use?
  Use whichever is most expressive and/or most simple.

### Concurrency Primitives

```go

func main() {
  go sayHello()
}

func sayHello() {
  fmt.PrintLn("hello")
}
```

Coroutines are simply concurrent subroutines (functions, closures, or methods in Go) that are nonpreemptive—that is, they cannot be interrupted. Instead, coroutines have multiple points throughout which allow for suspension or reentry.

To create a join point, we need to sync the main goroutine and the sayHello goroutine.

```go
var wg sync.WaitGroup 
sayHello := func() {
defer wg.Done()
        fmt.Println("hello")
    }
wg.Add(1)
go sayHello() 
wg.Wait()
```

### Sync Package

WaitGroup

- Good way to wait for concurrent ops to complete when you don't care about the result of the op or you have other means of collecting the result.
- If none are true, use channels and a select statement.
- You can think of a WaitGroup like a concurrent-safe counter: calls to Add increment the counter by the integer passed in, and calls to Done decrement the counter by one. Calls to Wait block until the counter is zero.

```go
wg.Add(1) 
go func() {
defer wg.Done()
fmt.Println("1st goroutine sleeping...") time.Sleep(1)
}()

wg.Wait()
```

Mutex
- Used to protect a given critical area
- To borrow a Goism, whereas channels share memory by communicating, a Mutex shares mem‐ ory by creating a convention developers must follow to synchronize access to the memory. You are responsible for coordinating access to this memory by guarding access to it with a mutex.

Cond
- ...a rendezvous point for goroutines waiting for or announcing the occurrence
of an event
- A way for go routines to sleep while waiting for a task to be done

```go

// Not good
for conditionTrue() == false {
  time.Sleep(1*time.Millisecond)
}

// Better
c := sync.NewCond(&sync.Mutex{}) 

c.L.Lock()
for conditionTrue() == false {
  c.Wait() // This call suspends the go routine, it doesn't only block it; allows other routines
  // to run on the OS thread
}
c.L.Unlock()
```

Longer example:

```go
func main() {
	c := sync.NewCond(&sync.Mutex{})
	queue := make([]interface{}, 0, 10)

	removeFromQueue := func(delay time.Duration) {
		time.Sleep(delay)
		c.L.Lock()
		queue = queue[1:]
		fmt.Println("Removed from queue")
		c.L.Unlock()
		c.Signal() // Notify waiting goroutine that an item has been removed
	}

	for i := 0; i < 10; i++ {
		c.L.Lock()

		// Wait while the queue is full (has 2 elements)
		for len(queue) == 2 {
			c.Wait() // Wait for a signal indicating an item has been removed
		}

		fmt.Println("Adding to queue")
		queue = append(queue, struct{}{}) // Add an element to the queue
		go removeFromQueue(1 * time.Second) // Start a goroutine to remove an element after 1 second

		c.L.Unlock() // Unlock the mutex to allow other goroutines to access the queue
	}
}
```

Pool
- Pool is a concurrent-safe implementation of the object pool pattern
- At a high level, a the pool pattern is a way to create and make available a fixed num‐ ber, or pool, of things for use. It’s commonly used to constrain the creation of things that are expensive (e.g., database connections) so that only a fixed number of them are ever created, but an indeterminate number of operations can still request access to these things