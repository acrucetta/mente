
## Notes on Modern C

### Chapter 3

The value 0 represents logical false. Any value diff from 0 represents logical true.

e.g., if (i) {}

We never should compare to 0, false, or true.

### Chapter 5

- Use size_t for sizes, cardinalities, or ordinal numbers
- Use unsigned for small quantities that can't be negative
- Use signed for small quantities that bear a sign

Objects of type const are read-only.

Enums.

```c
enum corvid { magpie , raven , jay , corvid_num , };

char const*const bird[corvid_num] = {
  [raven] = "raven", [magpie] = "magpie", [jay] = "jay",
};

for (unsigned i = 0; i < corvid_num; ++i)
  printf("Corvid␣%u␣is␣the␣%s\n", i, bird[i]);
```

Macros:

```c
# define M_PI 3.14159...
```

### Chapter 6 - Derived data types

Array length. There are two categories of arrays: fixed-length arraysC (FLAs) and variable-length arrays

The first are a concept that has been present in C since the beginning; this feature is shared with many other programming languages. The second was introduced in C99 and is relatively unique to C


**Array Length**
The length of an array A is (sizeof A)/(sizeof A[0]).

Array parameters behave as if the array is passed by reference
- Passing by reference means that you get a pointer as part of the array

```c
#include <stdio.h>
void swap_double(double a[static 2]) { double tmp = a[0];
a[0] = a[1];
a[1] = tmp;
}
int main(void) {
double A[2] = { 1.0, 2.0, };
swap_double(A); printf("A[0]␣=␣%g,␣A[1]␣=␣%g\n", A[0], A[1]);
}
```

### Characters

Strings are basically character arrays.

We can use functions that operate on char arrays:
- memcpy(target, source, len)
- memcmp(s0,s1,len) - compares two arrays
- memchr(s,c,len) - searches for the first appearance of c

Then we have:
- strlen(s)
- strcpy(target,source)
