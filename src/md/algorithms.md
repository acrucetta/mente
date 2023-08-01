
## Algorithms - Coursera Notes

Algorithms: method for solving a problem.
Data structure: method to store information.

## Week 1

Steps to develop a usable algorithm:
1. Model the problem
2. Find an algorithm to solve it
3. Fast enought / fits in memory?
4. If not, figure out why
5. Find a way to address the problem
6. Iterate until satisfied

### Dynamic Connectivity

Is there a path between two objects? Used in many applications.

We need to implement: find query and union command.

Find query: check if two objects are in the same component.
Union: replace components with their union.

We need to check our API design before implementing it.

Quick Find (eager approach):
- Data structure: integer array id[] of size N
- Interpretation: two objects are connected if they have the same ID.

Cost model: numer of array accesses.

Quick Union (lazy approach):
