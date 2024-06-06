
## Computer Architecture

## Key Terms
- Instruction Fetch
- Pipelining
- Cache Memory
- Virtual Memory

All computers stem from the "stored program concept". The computer is based on a fixed hardware platform capable of executing a fixed set of instructions. They can be combined like building blocks to yield sophisticated programs.

The von Neumann architecture:
- Computers can hold both data and instructions in memory.
- We can compute the instructions with a CPU that contains registers, an ALU and a control unit

A typical architecture:
![von neumann](https://upload.wikimedia.org/wikipedia/commons/thumb/e/e5/Von_Neumann_Architecture.svg/1200px-Von_Neumann_Architecture.svg.png)

### The Hack Computer (Nand To Tetris)

![hack cpu interface](https://blog.logancyang.com/images/cs4ds/hackcpuinput.png)

We can have general purpose or single purpose computers. General purpose are PCs and cellphones. Single purpose are embedded in cars, cameras, media stremers.

When computers shared the same address for data and instructions it's called the Harvard architecture. They're cheaper to build and can be optimized. It's the architecture of choice in many dedicated embedde computers.

Much of the work of computer architects goes towards achieving better performance. This is done by caching algorithms, optimizing access to I/O devices, parallelism, prefetching amongst others. 

The two main camps in design are: Complex Instruction Set Computing (CISC) and Reduced Instruction Set Computing (RISC). RISC camp builds simpler processors and tighter instruction set:s, arguing for better performance.

