
## Notes on Pointers

### Where do pointers come from? 

Pointers were introuced in the B programming language

Thus, if p is a cell containing the index of (or address of, or pointer to) another cell, *p refers to the contents of the pointed-to cell, either as a value in an expression or as the target of an assignment.

Because pointers in BCPL and B are merely integer indices in the memory array, arithmetic on them is meaningful: if p is the address of a cell, then p+1 is the address of the next cell. This convention is the basis for the semantics of arrays in both languages

Finally, the B and BCPL model implied overhead in dealing with pointers: the language rules, by defining a pointer as an index in an array of words, forced pointers to be represented as word indices. Each pointer reference generated a run-time scale conversion from the pointer to the byte address expected by the hardware.

The rule, which survives in today's C, is that values of array type are converted, when they appear in expressions, into pointers to the first of the objects making up the array.

int i, *pi, **ppi;

declare an integer, a pointer to an integer, a pointer to a pointer to an integer. The syntax of these declarations reflects the observation that i, *pi, and **ppi all yield an int type when used in an expression.

Similarly,

int f(), *f(), (*f)();

declare a function returning an integer, a function returning a pointer to an integer, a pointer to a function returning an integer;

int *api[10], (*pai)[10];

declare an array of pointers to integers, and a pointer to an array of integers. In all these cases the declaration of a variable resembles its usage in an expression whose type is the one named at the head of the declaration.

The scheme of type composition adopted by C owes considerable debt to Algol 68, although it did not, perhaps, emerge in a form that Algol's adherents would approve of.
The central notion I captured from Algol was a type structure
based on atomic types (including structures), composed into arrays, pointers (references),
and functions (procedures)

After creating the type system, the associated syntax, and the compiler for the new language, I felt that it deserved a new name; NB seemed insufficiently distinctive. I decided to follow the single-letter style and called it C, leaving open the question whether the name represented a progression through the alphabet or through the letters in BCPL.

### Why do we use pointers?

(http://cslibrary.stanford.edu/102/PointersAndMemory.pdf)

Pointers solve two common software problems. First, pointers allow different sections of
code to share information easily. You can get the same effect by copying information
back and forth, but pointers solve the problem better. Second, pointers enable complex
"linked" data structures like linked lists and binary trees.

One way to think about pointer code is that operates at two levels — pointer level and
pointee level. The trick is that both levels need to be initialized and connected for things
to work. (1) the pointer must be allocated, (1) the pointee must be allocated, and (3) the
pointer must be assigned to point to the pointee. It's rare to forget step (1). But forget (2)
or (3), and the whole thing will blow up at the first dereference. Remember to account for
both levels

No matter how complex a pointer structure gets, the list of rules remains short.
• A pointer stores a reference to its pointee. The pointee, in turn, stores
something useful.
• The dereference operation on a pointer accesses its pointee. A pointer may
only be dereferenced after it has been assigned to refer to a pointee. Most
pointer bugs involve violating this one rule.
• Allocating a pointer does not automatically assign it to refer to a pointee.
Assigning the pointer to refer to a specific pointee is a separate operation
which is easy to forget.
• Assignment between two pointers makes them refer to the same pointee
which introduces sharing.


### What do we use instead now?

- Refrences (Java, C#)
- Garbage Collection (Java, Python)  
  - Object References In languages with garbage collection, such as Java or C#, objects are typically accessed through references rather than raw pointers. These references are essentially pointers managed by the runtime environment. When an object is created, a reference to it is returned, and this reference can be used to access the object's data and methods.
  - Unreachable Objects Objects that are no longer reachable from the root set are considered garbage and are eligible for collection. The garbage collector frees the memory occupied by these unreachable objects, making it available for future allocations.
- Automatic pointers (C++)
- Ownership system (Rust)


