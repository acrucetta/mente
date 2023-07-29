
## Virtual Machines

A virtual machine is a program that acts like a computer. It simulates a CPU along with a few other hardware components.

VMs allow us to port programs into several CPUs. 

![https://www.jmeiners.com/lc3-vm/img/vm.gif](https://www.jmeiners.com/lc3-vm/img/vm.gif)

The only cost is the overhead of the VM itself and the further abstraction from the machine. This is a trade-off that's taken.

Ethereum smart contracts are examples of virtual machines. They are run inside of a VM that has no access to the file system, network, disc, etc...


**Instruction Set**

Instructions have both an opcode and a set of parameters.

Each opcode represents tasks the CPU "knows" how to do. 

Examples:

```c
enum
{
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP    /* execute trap */
};
```
Assembly language is a readable and writable form encoded in plain text. A tool called an {assembler} transforms each line of text into a 16-bit binary.

![https://www.jmeiners.com/lc3-vm/img/assembler.gif](https://www.jmeiners.com/lc3-vm/img/assembler.gif)

An assembler convers assembly language (already low level) into machine code. The compiler translates high-level source code into lower level languages like assembly code or machine code.

