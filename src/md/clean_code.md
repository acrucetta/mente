
## Notes on Clean Code

### From blogs

**Source**: [Advice to a novice programmer](https://blog.plover.com/prog/katara-advice.html)

It's important to remove as much friction as possible from your basic process. Otherwise it's like trying to cook with dull knives and rusty pots, except worse because it interrupts your train of thought. You can't do good work with bad tools.

When you start the next project, start it in VScode in the beginning. And maybe set aside an hour or two before you start in earnest, just to go through the VSCode tutorial and familiarize yourself with its basic features, without trying to do that at the same time you are actually thinking about your homework. This will pay off quickly.

Don't cut corners writing code:
- It's tempting to use the first variable of a function instead of building a suggestive one. But remembering imposes a tiny cost every time you do it. These tiny costs seem insignificant. But they compound.
- Optimize your code for quick and easy reading, at the cost of slower and more careful writing
- Use interfaces when abstracting code. When you add a method and then call it from different places you advance the program by extending the number of operations it can do without you thinking of them
- If something is messy, it might be tempting to imagine it doesn't matter. It does matter. The costs are small but compound. Invest in cleaning up messy code. Otherwise it will get worse over time.

Debugging is a science. Always have a clear mind of what question are you trying to answer. Before you debug think *What do I think will happen in this breakpoint?* 

Use the finger of blame. What part of the code is really responsible for the problem. Until you find the smoking gun.


### From Philosopy of Software Design

...
