
## Application Logging

Logs should be greppable. It should be focused entirely on a single line of log. NEVER over multiple lines.

We want to answer questions such as:
- When did user 55 last log in?
- Is our incoming connection receiving messages?
- How many times did our web server go dead?
- Did any transaction fail to complete?

Problems to avoid:
1. Log is missing necessary information
2. Log unsuitable for grep because of redundant information (if it mentions a website on every linge we can't find those that were activated by the website)
3. Information is split across more than one line
4. Error reported to user but not logged

e.g., 

try {
  do something
} catch {
  do nothing
}

Logs can be saved into multiple files depending on how often they happen:
- Message log
- Connection log
- Stacktrace log

Logs must contain timestamps with timezone, to the millisecond or nanosecond.

Always log to local disk, don't try to log to the network or to shared folders.



