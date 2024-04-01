
## Notes on C Programming

### Debugging

Run all commands with -Wall and Werror for good alerts.

Flags:
- o to name the output
- c to compile the code
- g to be able to debug it

To debug in macOS use lldb with the program output.

### Pointers

In C every single argument gets copied into parameters and the function uses a copy of the argument.

E.g., we write a home address on a piece of paper; and then copy it onto another piece of paper. We now have two points to that house.

### String Functions

- strcpy() - copy string to buffer
- strstr() - match string with char
- strtok() - split into delimiter '\n'
- strsep() - split by delimiter '\n'
