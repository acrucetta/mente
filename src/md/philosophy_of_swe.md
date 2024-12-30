
## Notes on Philosophy of Software Engineering

## Make Deep Modules

A deep module is one that has a simple interface, like an inverse hourglass. It hides all the complexity behind an action. Doesn't make the user have to know all the steps to finish a task.

Other notes:
- Avoid class leakage; classes depending on other classes
- Make classes contain all the knowledge required for one task (e.g., all the info required to read or write to a file)
- You can make functions have 70 lines or less a la Tiger Style
