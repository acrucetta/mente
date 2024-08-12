### Analysis of Algorithms

What's the order of growth of common algorithms?

![Alt text](image-3.png)

### Stacks and Queues

- Why does the stack have constant push and pop time?
  - Because we just need to return the first item or last item in the structure without iterating over its values
- What are advantages of a LinkedList over a stack?
  - Arrays provide immediate access to any item; but we need to know the size on initialization
  - Linked List use space proportional to size but we need a reference to access an item
- When do we use the LinkedList, when do we use the stack
  - Depending on the access patterns and memory consumption; stacks are easy for push/pop but to find items it takes a bit more time. We do have resizing issues.
- What's the time complexity of Stacks?
- What's the time complexity of LinkedLists?
- What's the time complexity of Queues?
- What is the origin of LinkedLists?
  - They were initially used by LISP in 1950s as the primary structure for all programs and data. It presented challenges because they're hard to debug. 

### Sorting

- How does Selection Sort work? Give an example 
- How does Shell Sort work? Give an example 
- How does Insertion Sort work? Give an example 
- How does Merge Sort work? Give an example
- When to use each of these sorts?
- Why are some sorts better than others in specific occasions?
- What is stability in sorts?
	- Stability means that some sorts presents the same order of items with a given key; others might shuffle the keys each sort

### Priority Queues

- What do priority queues allow us to do?
	- They allows us to find the largest M items in a steam of N items in NlogM (as compared with other data structures that have NM)
- How are priority queues implemented?
	- They're implemented with Min Heap or Max Heap data structures. Their insertion time is O(nLogn)
- How do Min Heap and Max Heap data structures work?
- How does Heapsort work? Give an example
- What is the time complexity of Heapsort?
	- O(N Log N); because during insertion it takes N Log N then we just pull each item at O(1) from the top depending if its a Min or Max heap

### Symbol Tables (Hash Tables)

- What are potential implementations of symbol tables (dictionaries)?
- How can symbol tables get implemented with Linked Lists?
- How can symbol tables get implemented with Hash functions?

### Graphs

- What are the different representations of graphs?
	- Adjacency list 
		- Array [0,1,2,3] connected to a Linked List [1->2->null]
- What is a path in a graph?
	- Sequence of vertices connected by edges
- What is a cycle in a graph?
	- A path whose first and last vertices are the same
- How do we check if there's a path between A and B?
- How do we find the shortest paths in graphs?
- How do we detect if there's a cycle in the graph?

DFS

What is the pseudocode for DFS?
```python
def dfs(graph, start):
    visited = set()

    def dfs_recursive(current):
        if current in visited:
            return

        visited.add(current)

        for neighbor in graph[current]:
            dfs_recursive(neighbor)

    dfs_recursive(start)
```

What is the pseudocode for BFS?

```python
def bfs(graph, start):
    visited = set()
    queue = [start]

    while queue:
        current = queue.pop(0)
        if current in visited:
            continue

        visited.add(current)

        for neighbor in graph[current]:
            if neighbor not in visited:
                queue.append(neighbor)
```

How do we do a topological sort of appointments? Give an example?