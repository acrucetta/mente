
## Module 0

Machine Language: Overview

It needs to tell the hardware what to do (operations), it needs to know which instruction to perform at a given space of time, and it needs to tell the hardware what to operate on (addressing).

Rarely people write in machine language; too complicated; people would rather program in higher level languages.

```
Instruction: 010001000110010

Translates to: ADD R3 R2
```

A symbolic assembler can translate "index" to Mem[129]

Machine Language Elements
- What are the supported ops
- What do they operate on
- How is the program controlled

Machine Operations:
- Usually correspond to what's in the hardware
  - Arithmetic operations
  - Logical operations
  - Flow control ("go to instruction X")

### Memory
- Accessing a memory location is expensive: need to supply a long address.
- Solution: Memory Hierarchy
  - Registers; inside of the CPU
    - Data registers (R0, R1, R2)
    - Address registers (ARO, AR1, AR2) 
  - We have fast memory close to the CPU (cache)
  - Main memory, slower, but bigger
  - Disk, slower, but bigger


### Addressing Modes:

- Register: Add R1, R2
- Direct: Add R1, M[200]
- Indirect: Add R1, @A
- Immediate: Add R1, 5



