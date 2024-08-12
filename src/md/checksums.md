
## How are they used

- The sender calculates a checksum of the data; it sends it along
- The receiver calculates the checksum, if they match, the data is likely intact

## Formulas
### XOR Checksum

Let's calculate the XOR checksum for the byte sequence: [0x45, 0x6D, 0x7A, 0x23]

Step-by-step calculation:

1. Initialize checksum = 0x00
2. 0x00 ^ 0x45 = 0x45
3. 0x45 ^ 0x6D = 0x28
4. 0x28 ^ 0x7A = 0x52
5. 0x52 ^ 0x23 = 0x71

Final XOR checksum: 0x71

Binary representation of the process:

Copy

  ```
  00000000 (initial)
^ 01000101 (0x45)
  --------
  01000101
^ 01101101 (0x6D)
  --------
  00101000
^ 01111010 (0x7A)
  --------
  01010010
^ 00100011 (0x23)
  --------
  01110001 (0x71 - final checksum)
```


Let's calculate the 8-bit addition checksum for the same sequence: [0x45, 0x6D, 0x7A, 0x23]

Step-by-step calculation:

1. Sum all bytes: 0x45 + 0x6D + 0x7A + 0x23 = 0x14F
2. Take the least significant byte: 0x4F

Final 8-bit addition checksum: 0x4F

Detailed addition:

Copy

```
0x45 (69 in decimal)
+ 0x6D (109 in decimal)
+ 0x7A (122 in decimal)
+ 0x23 (35 in decimal)
  ----
  0x14F (335 in decimal)

Take least significant byte: 0x4F (79 in decimal)
```

### Fletcher's Checksum (16-bit)

Now let's calculate the Fletcher-16 checksum for the sequence: [0x45, 0x6D, 0x7A, 0x23]

Step-by-step calculation:

1. Initialize sum1 = 0, sum2 = 0
2. For 0x45: sum1 = (0 + 0x45) % 255 = 69 sum2 = (0 + 69) % 255 = 69
3. For 0x6D: sum1 = (69 + 0x6D) % 255 = 178 sum2 = (69 + 178) % 255 = 247
4. For 0x7A: sum1 = (178 + 0x7A) % 255 = 45 sum2 = (247 + 45) % 255 = 37
5. For 0x23: sum1 = (45 + 0x23) % 255 = 80 sum2 = (37 + 80) % 255 = 117

Final Fletcher-16 checksum: (sum2 << 8) | sum1 = (117 << 8) | 80 = 0x7550
