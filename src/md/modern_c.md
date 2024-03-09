
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

## Chapter 9: Coding Style

> Coding style is not a question of taste but of culture. When you enter a new project, you enter a new cultural space.

- Coding style is a matter of culture. Be tolerant and patient.
- Code formatting is a matter of visual habits. It should be automatically provided
by your environment such that you and your co-workers can read and write code
effortlessly.
- Naming of variables, functions, and types is an art and plays a central role in the
comprehensiveness of your code.

## Chapter 10: Program Organization

> As with coding style, beginners tend to underestimate the effort that should be put into code and project organization and docu- mentation: unfortunately, many of us have to go through the experience of reading our own code some time after we wrote it, and not having any clue what it was all about.

Most programmers will only read the interface of your code. Fewer the implementation. Code structure and documentation go hand in hand.

This rule is reflected in the use of two different kinds of C source files: header filesC , usually ending with ".h"; and translation unitsC (TU), ending with ".c".

C has no “built-in” documentation standard. But in recent years, a cross-platform public domain tool has been widely adopted in many projects: doxygen.

Example Doxygen:

```c
** @brief use the Heron process to approximate @a a to the
** power of ˋ1/kˋ **
** Or in other words this computes the @f$k^{th}@f$ root of @a a
As a special feature, if @a k is ˋ-1ˋ it computes the multiplicative inverse of @a a.
@param a must be greater than ˋ0.0ˋ
@param k should not be ˋ0ˋ and otherwise be between ˋDBL_MIN_EXP*FLT_RDXRDXˋ and ˋDBL_MAX_EXP*FLT_RDXRDXˋ.
@see FLT_RDXRDX

double heron(double a, signed k);
```
> Good programming only needs to explain the ideas and prerequisites that are not obvious (the difficult part). The structure of the code shows what it does and how.

> Another requirement is to have an obvious flow of control through visually clearly distinctive structuring in {} blocks that are linked together with comprehensive control statements. This is the reason why the use of goto is discouraged: it can break the flow of control

> Global variables are frowned upon. They make code inflexible (the object to operate on is fixed), are difficult to predict (the places of modification are scattered all over), and are difficult to maintain.

Use pure functions as much as possible.
- Pure functions are functions that have no side effects and return a value that depends only on their arguments.

Examples of not pure functions:
- The function reads part of the program’s changeable state by means other than through its arguments.
- The function modifies a global object.
- The function keeps a persistent internal state between calls.5
- The function does IO

Takeaways:
- For each part of a program, we have to distinguish the object (what are we do- ing?), the purpose (what are we doing it for?), the method (how are we doing it?) and the implementation (in which manner are we doing it?).
- The function and type interfaces are the essence of software design. Changing them later is expensive.
- Complicated reasoning should be avoided and made explicit where necessary.
