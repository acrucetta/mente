## Algorithms - Coursera Notes

Algorithms: method for solving a problem.
Data structure: method to store information.

## Week 1: Quick Union

Steps to develop a usable algorithm:

1. Model the problem
2. Find an algorithm to solve it
3. Fast enought / fits in memory?
4. If not, figure out why
5. Find a way to address the problem
6. Iterate until satisfied

### Dynamic Connectivity

Is there a path between two objects? Used in many applications. The union-find is a problem of maintaining a collection of disjoint sets and performing two operations.

We need to implement: find query and union command.

Find query: check if two objects are in the same component.
Union: replace components with their union.

We need to check our API design before implementing it.

**Quick Find (eager approach)**:

- Data structure: integer array id[] of size N
- Interpretation: two objects are connected if they have the same ID.

Cost model: numer of array accesses.

Find: check if p and q have the same id.

Union: to merge components containing p and q, change all entries whose id equals id[p] to id[q].

**Quick Union (lazy approach):**

- Data structure: integer array id[] of size N
- Interpretation: id[i] is parent of i
- Root of i is id[id[id[...id[i]...]]]

Find: check if p and q have the same root.

Union: to merge components containing p and q, set the id of p's root to the id of q's root.

Quick-find: union too expensive. Trees are flat.

Quick-union: trees can get tall. Find too expensive (could be N array accesses).

### Improvements

Weighted quick-union

- Modify quick-union to avoid tall trees
- Keep track of size of each tree (number of objects)
- Balance by linking root of smaller tree to root of larger tree

In sumamry, we try to avoid tall trees as we iterate through the array.

![weighted tree comparison](image-1.png)

```python

class QuickUnion:
    def __init__(self, n):
        self.id = [i for i in range(n)]
        self.sz = [1 for i in range(n)]

    def root(self, i):
        while i != self.id[i]:
            i = self.id[i]
        return i

    def connected(self, p, q):
        return self.root(p) == self.root(q)

    def union(self, p, q):
        i = self.root(p)
        j = self.root(q)
        self.id[i] = j

    def weighted_union(self, p, q):
        i = self.root(p)
        j = self.root(q)
        if i == j:
            return
        if self.sz[i] < self.sz[j]:
            self.id[i] = j
            self.sz[j] += self.sz[i]
        else:
            self.id[j] = i
            self.sz[i] += self.sz[j]
```

**Running time:**

- Find: takes time proportional to depth of p and q
- Union: takes constant time, given roots

Depth of any node x is at most lg N.

### Union-Find Applications

- Games
- Dynamic connectivity
- Percolation

**Percolation**

N by N grid sites. A system percolates iff top and botom are connected by open sites.

Can be thought of as water flowing through surfaces. Or in Social networks if we want to know whether people are connected.

**The subtext of the problem is:**

- We model the problem
- Then we find an algorithm
- We check whether it's fast or not
- We address the problem
- And iterate...

## Week 2: Analysis of Algorithms

The key is running time; we used to have a cranking machine; how many cranks we need to do to compute.

Why analyze algorithms?

- Predict performance
- Compare algorithms
- Provide guarantees
- Understand theoretical basis

One of the most important ones is the FFT algorithm; takes only $N log N$ steps. Another one is the N-body simulation.

We use the scientific method to analyze algorithms: Observe, hypothesize, predict, verify, and validate.

Experiments must be **reproducible** and **falsifiable**.

**3-Sum Example**

How many distinct integers add up to zero.

Brute force: do a triple for loop. (it will take n^3)

**Mathematical models of runtime**

Donald Knuth first proposed the total run-time when programs were running for too long.

E.g., how many instructions as a function of input size N?

Turing said we should just count the most expensive operations instead of each addition.

By focusing on one operation you can simplify the frequency counts.

**Order of growth classifications**

Small set of functions: log N, N, NlogN, N^2, N^3, 2^N

![Alt text](image-3.png)

Based on binary search we can find a better algorithm for 3-Sum. We can use N^2 log N instead of N^3.

Instructions:

- Sort the N integers
  - Insertion sort: N^2
- For each pair of integers a and b, binary search for -(a+b)
  - Binary search: log N
- Only count if a[i] < a[j]] < a[k]

### Types of analysis

We have best case, worst case, and average case. Lower bound on cost, upper bound on cost, and expected cost.

We can have different approaches:
1. Design for worst case
2. Randomize the input

The main approach is to reduce variability by focusing on the worst case scenario. We want to find an optimal algorithm.

We have many notations
- Big Theta ($Big \theta$): asymptotic order of growth
- Big Oh: to develop upper bounds
- Big Omega: to develop lower bounds

Example:
- 3 Sum
	- Improved algorithm gives us O($N^2logN$)
	- Lower bound (proof that no algorithm can do better): $\omega(N)$

The approach:
- Develop algorithm
- Prove a lower bound

We can also have tilde notation. It's used to provide an approximate model. 

### Memory

Typical memory usage for primitive types:
- Boolean (1); Char (2); Double (8); Int (8)
- Int[][] ~4MN; int[] ~4N+24; 

Typical memory usage for objects in Java:
- Object overhead 16 bytes
- Ref 8 bytes
- Padding multiple of 8 bytes

## Week 2

### Stacks and queues
- They're fundamental data types
- Stack has push and pop (LIFO)
- Queue has enqueue and dequeue (FIFO)

### Stacks
- push(); pop(); isEmpty()
- We build the stack with a LinkedList
	- push - insert node to the beginning
	- pop - remove node from the beginning

```java
pop()
String item = first.item;
first = first.next;

push()
Node oldfirst = first;
first = new Node();
first.item = "not";
first.next= oldfirst
```

- Every op takes constant time in the worst case.
- A tack with N items uses ~40N bytes.
- Every object in Java has 16 bytes of overhead.

**Alternative Implementation**
- Use array s[] to store N items on stack
- push(): add new item at S[N]
- pop(): remove item from S[N-1]
- Cons: need to define the capacity ahead of time

We have to worry about loitering in Java. To avoid that we need to set the removed item to Null so it can be reclaimed by the garbage collector.

**Resizing arrays implementations**

Q: How can we grow the array?
A: If the array is full, create a new array of twice the size and copy the items. ~3N.

Q: How to shrink array?
A: Wait until the array is one-quarter full. Invariant; if the array is always between 25% and 100%

The worst case for push and pop will be N. 

**Memory:** It uses between 8N and 32N. 

### Tradeoffs
- Linked list: 
	- Every op takes constant time in worst case
	- We need to use extra time and space to deal with the links
- Stack
	- Every op takes constant amortized time
	- Less wasted space

### Queues

**LinkedList**

```
enqueue()

String item = first.item;
first = first.next;
return item;

dequeue()

Node oldlast = last;
Node last = new Node();
last.item ="not";
last.next = null;

oldlast.next = last;
```

