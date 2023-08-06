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
