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

## Strings

Strings in C are "just" char arrays. Type is char*

Chars are stored as numbers according to a code. ASCII is common for english.

To store the char code, write just the single quotes 'H'. To print it as char we use '%c'

We need to either track ther lengths and pass around an unsigned int or use the char sentinel '\0'. But we need to remember adding it anytime we create strings.

To write strings we can do:

```c
// this is read only
// cannot be changed using the pointer
char *s1 = "hello"; // sentinel added automatically

// allocated in stack
// can be changed
char s4[] = {'H','e','l'...,'\0'} // here we need to add the sentinel

char *s5 = (char*)malloc(sizeof(char)*20); // 19 + sentinel
```

To print strings we can use

```c
  char a[] = "abcde";
  printf("%s\n", a); // abcde
  printf("%p\n", a); // 0x16ba47578
  printf("%c\n", *a); // a
```

To calculate the length of string we could use the sentinel as below:

```c
unsigned int strlen2(char* s) {
	unsigned int i = 0;
	while (s[i]!='\0') {
		i++;
	}
	return i;
}

unsigned int strlen3(char *s) {
  char *t = s;
  while (*t) {
    t++;
  } 
  return t-s;
}

int main() {
  char* s = "Hello World!";
  printf("%u\n", strlen3(s));
}
```

### Iteration v. Recursion

Both can do the same thing. Iteration can do it without the overhead of function calls and without using stack memory. 

Recursion can keep more previous state that has already been executed. Most stack size is 1MB. If we keep maintaining state we will run out of stack memory.

The simplicity of recursion comes at the cost of time and space efficiency.

### Memory Issues

**Memory Overread**

This happens when we read more bytes than we're supposed to.

E.g.,

```c
char a[] = "hello!";
  int i = 1;
  while (i++) {
    printf("%c at %p\n", a[i], a+i);
  }
```

**Memory Overwrite**

We end up overwriting the memory spaces.

```c
  char src[] = "hello!";
  char dst[5];
  int len = strlen(src);
  for (int i = 0; i < len; i++)
    dst[i] = src[i];
  dst[len] = '\0'; 
  printf("src: %s\n", src);
  // !
  printf("dst: %s\n", dst);
  // hello!
```

**Memory Leak**

The memory location to which area 1 was pointing to earlier becomes an orphan.

```c
  char *area1 = malloc(10); // 0x600003150030
  char *area2 = malloc(10); // 0x600003150040
area1 = area2;
// cannot free 0x600003150030 anymore
```

### Structs

Key questions:
- How do structs bundles multiple values in one entity
- How to use structs as parameter type, return type, field, array element, etc
- What is a struct vs. struct pointer: Difference between . and ->

Use struct . for actual structs and -> for pointers

### Union

Key questions:
- When is a union preferred over a struct?
- How does a union represent different datatypes?
- How to access each field in a union?
- How many bytes does a union occupy in memory?

## Threading 

Main thread functions in C:
- pthread_t tid; - declare the thread
- pthread_create(thread_id, NULL, func, args) - create the thread
    - the function will be of type `void *(*start_routine)void*)` aka a function pointer
    - args is of type `void *arg`
- pthread_join(thread_id, NULL) - wait for the thread to finish
    - this function blocks the caller thread until this thread id is finished

To use a thread we pass it the function pointer

E.g.,

```c
void *print_message(void *arg) {
  char *str = (char*)arg;
  printf("%s\n", str);
  return NULL;
}
int main() {
  pthread_t tid;
  pthread_create(&tid, NULL, print_message, "hello!");
  pthread_join(tid, NULL);
  printf("done");
}
```

A function pointer points to the code; it can be used to simplify calling other functions or creating more powerful functions such as a map command

E.g.,

```c
void func(){
  printf("hello!\n");
}
void bar(){
  void (*f)(void) = func; 
  // f is a function pointer
  f(); // prints out "hello!\n"
} 
```

E.g., map implementation

```c
void map(int (*fun)(int), int *arr, int len) {
  for (int i = 0; i < len; i++){
    arr[i] = fun(arr[i]);
  }
}

int half(int x) {
  return x/2;
}

int main(){
  int arr[] = {1, 2, 3, 4, 5};
  map(half, arr, 5); // pass function "half" as an argument
  // arr becomes 0, 1, 1, 2, 2
} 
```

To run multiple threads we can build an array of threads:

```c
void *func(void *arg){
  // ignored. see last slide
}

int main() {
  pthread_t tids[NUM_THREADS];
  int total = 0;
  for (int i = 0; i < NUM_THREADS; i++)
    pthread_create(&tids[i], NULL, func, i);

  for (int i = 0; i < NUM_THREADS; i++){
    pthread_join(tids[i], NULL);
    total += sums[i];
  }
}
```

### Data Sharing in Threads

We can't guarantee concurrent process will run at a given order

E.g.,

```c
void *func(void *arg) {
  char *str = (char*)arg;
  printf("%s\n", str);
  return NULL;
}
int main(){
  pthread_t tid1, tid2;
  printf("start!\n");
  pthread_create(&tid1, NULL, func, "peer1");
  pthread_create(&tid2, NULL, func, "peer2");
  printf("main!\n");
  pthread_join(tid1, NULL);
  pthread_join(tid2, NULL);
  printf("end!\n");
} 
```

We can't tell whether peer1 will run before peer2. Or wehther the main thread will print "main" before or after the threads.

**Memory Layout**

- **Memory space** is shared by all threads
- Each thread has its own stack segment (created by func call within the thread)
    - Local variables cannot be directly accessed by other threads; could be done via pointers
- **Heap** is shared across all threads
- **Data** segment contains global variables and is shared by all threads
- **Code** is shared by all threads ("text segment")

![threads](../media/img/threads.png)

**We can use structures to pass arguments and return values**

```c
typedef struct params params;
struct params{int a1, a2, ret;};

void *func(void *arg){
  params *p = (params*)arg;
  p->ret = p->a1 + p->a2;
  return NULL;
}

int main(){
  params *p = (params*)malloc(sizeof(params));
  p->a1 = 1;
  p->a2 = 2;
  pthread_t tid;
  pthread_create(&tid, NULL, func, p);
  pthread_join(tid, NULL);
  printf("ret %d\n", p->ret);
}
```

We could use *return* to return the value too...

```
void *func(void *arg){
  params *p = (params*)arg;
  return p->a1 + p->a2; // same as pthread_exit(code)
}

// We later just use pthread join and pass the return address
pthread_join(tid, &ret);
```

We can also return multiple values with *pthread_exit(res)*

```
struct params{int a1, a2;};
struct results{int sum, mul;};

void *func(void *arg){
  params *p = (params*)arg;   
  results *res = (results*)malloc(sizeof(results));
  res->sum = p->a1 + p->a2;
  res->mul = p->a1 * p->a2;
  pthread_exit(res);
}

int main(){ 
  params *p = (params*)malloc(sizeof(params));
  p->a1 = 1;
  p->a2 = 2;
  pthread_t tid;
  pthread_create(&tid, NULL, func, p);
  results *ret;
	
	// load ret to the results pointer
  pthread_join(tid, &ret);  
  
  // then we can access it
  printf("sum=%d, mul=%d\n", ret->sum, ret->mul); 
}
```

### Key Questions
- How do we build a function and pass it as a pointer in threads?
- How do we build a structure with multiple values and update it concurrently across threads?
- How do we ensure functions run in a given order?

### Key Topics for Quiz
- basic C syntax
- conditionals
- C types
- C functions
- binary numbers, signed and unsigned
- related number systems (octal and hexadecimal)
- bit operations
- packed representations (as in storing three color components in one int)
- loops
- pointers
- arrays
- clarity on the stack/heap distinction
- strings
- malloc/free
