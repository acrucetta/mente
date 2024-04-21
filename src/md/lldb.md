
## Notes on LLDB Commands

### Summary

Movement
- n - step over
- s - step into
- c - continue until next breakpoint
- n - next

Backtraces
- bt - backtrace (or bt 30)
- r - run
- q - quit

Breakpoints
- b <file:line> breakpoint
- b <function name> function breakpoint
- br l - list breakpoints
- br del # - delete breakpoint #

Printing
- p - print variable
- down - stack frame down
- f - display the code surrounding a place


### Details

* Print object
```
(lldb) po responseObject
(lldb) po [responseObject objectForKey@"state"]
```

* p - Print primitive type

### Breakpoints
* List breakpoints
```
br l
```
* br delete - Delete breakpoint
```
(lldb) br delete 1
```
* br e - Enable breakpoint
* br di - Disable breakpoint
* b - Add breakpoint
```
(lldb) b MyViewController.m:30
```
* br set - Add symbolic breakpoint
```
(lldb) br set -n viewDidLoad
```
* Conditional break
``` objc
for(PlayerItem *item in player.inventory) {
    totalValue += item.value;
}
```
Set a conditional breakpoint that triggers only when `totalValue` is greater than 1000:
```
(lldb) b MerchantViewController.m:32
Breakpoint 3: where = lootcollector`-[MerchantViewController] ...
(lldb) br mod -c "totalValue > 1000" 3
```
Reset the condition:
```
(lldb) br mod -c "" 3
```
* Run a debugger command from a breakpoint
```
(lldb) br com add 2
Enter your debugger command(s). Type 'DONE' to end.
> bt
> continue
> DONE
```
* Resume excution
```
(lldb) continue
```
* Step over
```
(lldb) n
```
* Step in
```
(lldb) s
```
* Print backtrace
```
(lldb) bt
```
