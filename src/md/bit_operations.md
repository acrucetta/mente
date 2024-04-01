
## Notes on Bit Operations

AND, XOR, OR, work like typical boolean operations

Left Shift <<

Shifts bit to the right and eliminates whatever extra bits we have. Adds N bits to the right.

```c
uint8_t a = 0x02u;   // 0b 0000 0010 = 0x02
uint8_t b = a << 3; // 0b 0001 0000 = 0x10 
printf("After shift: 0x%x\n", b);

// Output
// After shift: 0x10
```

Right Shift >>

Adds N 0 bits to the left and pushes the ones in the right out.

```c
uint8_t a = 0xAAu;   // 1010 1010 = 0xAA
uint8_t b = a >> 4; // 0000 1010 - 0x0A
printf("After shift: 0x%X\n", b);

// Output
// After shift: 0xA
```