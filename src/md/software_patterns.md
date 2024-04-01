
## Notes on Software Patterns

> Note: Be careful of falling into "architecture hypnosis". Patterns are something you should refer to when you have a sufficiently complex problem that is constantly causing friction to your team or self as you add more features to a game. If you can get away with simply having a loop or a few if statements for some small feature, just do those and don't go fancy finding some composite or factory pattern or whatever. Definitely one of the more common mistakes I made approaching a game in my college days.

We often see two types of code:
- Coupled balls of mud with no regard for design
- EnterpriseSingletonVisitorFacadeFactoryBuilder

Most code uses no design patterns, and suffers for it. Dependencies sprawl and tangle, and we end up having to hold entire systems in our head in order to get anything done. This makes it hard to recognize the components in a system.

On the other hand, when we learn design patterns we often overcorrect and stuff design patterns anywhere we can possibly fit them, leading to tons of indirection and incidental complexity. Some people like to say that in this kind of systems "everything happens somewhere else". This makes it hard to recognize what a component does.

I think "good code" lives somewhere in the middle, where we do just enough design to keep things decoupled, but not so much that we end in indirection hell.

### Command 

Resources:
- https://refactoring.guru/design-patterns/command


Flywheel


Observer
