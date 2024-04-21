## Notes on C Programming

### Debugging

Run all commands with -Wall and Werror for good alerts.

Flags:
- o to name the output
- c to compile the code
- g to be able to debug it

To debug in macOS use lldb with the program output.

To debug sanitizing the output you can use the `clang` compiler.

`clang++ -O1 -g -fsanitize=address -fno-omit-frame-pointer -c example_UseAfterFree.cc` 


### String Functions

- strcpy() - copy string to buffer
- strstr() - match string with char
- strtok() - split into delimiter '\n'
- strsep() - split by delimiter '\n'

## Bits

```c
unsigned char x = 42;
42 in base 2 is 101010 -> 2^5 + 2^3 + 2^1
```

unsigned char uses eight binary digits or bits. Known as a single byte. It has a range of [0,255]. It is stored as: 00101010

`unsigned int y = 4` uses 4 bytes or 32 bits. So it is stored as: 00000000000000000000000000101010.
  
Binary, Octal, and Decimal
- decimal: base 10 (0-10)
- binary: base 2 (0,1)
- octal: base 8 (0-8)
- hexadecimal: base 16 (0-9) and (A-F) where A is 10 and F is 15

Values:
- Leading 0 denotes an octal int constant.
- Leading 0x denotes an hexadecimal int constant

```c
unsigned int a = 0736; // octal value
unsigned int y = 0x5E2; // hexadecimal value
unsigned int p = 0x12; // hexadecimal value
unsigned int q = 0xa8b; // A,B,C,D,E,F can be uppercase or lowercas
```

Char in C is limited:
- It can be non-negative int represented by 8 bits
- Signed chars range from -128 to 127. 
- Unsigned chars range from 0 to 255
- Or in Hex from 0x00 to 0xFF

Other types:
- short - 2 bytes
- int - 4 byes
- long - 8 bytes
- float - 4 bytes
- double - 8 bytes

`sizeof(expression)` tells us how many bytes the type has. It returns a type size_t and can be specified with %zu

### Operations

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

## Pointers

In C every single argument gets copied into parameters and the function uses a copy of the argument.

E.g., we write a home address on a piece of paper; and then copy it onto another piece of paper. We now have two points to that house.

We use pointers because it allows us to use *indirection*. Allow changes to be made without changing the reference.

Every byte in memory has a unique address.

### Pass by value
- Parameters are local variables that are initialized to a copy of the arguments. 
- Changing a parameter does not change the independent copy in the calling function.
- There are two copies of 7 and two copies of 13 in memory. 

```c
#include <stdio.h>

void swapA(int x, int y){
  int tmp=x;
  x = y;
  y = tmp;
  printf("in swapA: x = %d, y = %d\n", 
	x, y);
}

int main() { 
  int a = 7, b = 13;
  swapA(a, b);
  printf("in main: a = %d, b = %d\n", 
	a, b);
  return 0;
}
```

### Pass by Reference
- Parameters are local variables that are initialized to a copy of the arguments.
- When main gives swapB the value &a, it allows swapB to reach into main’s region of memory, and change the value of a.
- There is only one copy of 7 and only one copy of 13. 

```c
#include <stdio.h>
void swapB(int* x, int* y) {
  int tmp=*x;
  *x=*y;
  *y = tmp;
  printf("in swapB: *x = %d, *y = %d\n", 
	*x, *y);
}
int main() {
  int a = 7, b = 13;
  swapB(&a, &b);
  printf("in main: a = %d, b = %d\n", 
	a, b);
  return 0;
}
```

Functions in C can only return a single value but we can use pointers as workaround. We pass the address of the value. Then the return is either success or failed.

### Malloc, Heap and Free

Key Question: Local variables are stored on stack which will be recycled after function returns, how to store values visible to the caller function?

We use:  `malloc(sizeof(int))`

The problem below is that both z and s point to each other. When main dereferences s the content might be changed.

```c
int* sum(int *x, int *y) {
  int z = *x + *y;
  return &z;
}

int main(){
  int a = 1, b = 2;
  int *s = sum(&a, &b);
  printf("%d\n", *s);
  return 0;
}
```

Better; now we have main pointing to the heap.

```c
int* sum(int *x, int *y) {
  int *z = (int*)malloc(sizeof(int));
  *z = *x + *y;
  return z;
}

int main(){
  int a = 1, b = 2;
  int *s = sum(&a, &b);
  printf("%d\n", *s);
  free(s);
  return 0;
}
```

### Arrays in C

If we want an array of four doubles we'd need to allocate:
`4 * sizeof(double) == 32`

To allocate an array we can do:

```c
  int a[5]; // makes an array of length 5
  int b[] = {10, 20, 30, 40, 50}; // makes and initializes an array of length 5

b is a pointer to element 0
b[0] is the value of element 0 -> 10
*b is also the value of element 0
b+1 is a pointer to element 1
*(b+1) is the value of element 1
```
