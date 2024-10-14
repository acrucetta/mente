
Overview
- Raft is a protocol for implementing distributed consensus.

### Components

Leader Election
- All nodes start as followers, they can become candidates or leaders over time
- The candidate becomes the majority if it receives most of the votes
- Each node has an *election timeout* (150-300ms), after a certain time it asks other nodes for a vote. If the node hasn't voted it will vote for the requesting node
- To commit an entry, the node replicates the value to the followers
- Messages are sent across nodes in "heartbeat timeouts"

Log Replication
- The leader receives a message
- Then it sends it to the other nodes via a "heartbeat"
- Once it receives the reply back, it sends back an acknowledge message to the client

Network issues
- Raft can sustain network partitions. If the client wants to change a node, and the partition has a majority, then it succeeds. It will send the message to the other nodes after.

## Algorithm

- We first elect a leader; he manages the logs
- The leader accepts log entries from clients and replicates it on servers
  - It also tells them when its safe to apply log entries to their state

Sub-problems:
- How to choose a leader if one fails?
- How to replicate the logs?
- How to keep the server safe? No commands overlap

### Communication

- RequestVote RPC
- AppendEntries RPC
  - Leaders send this with no log entries to maintain their authority
  - If a follower doesn't receive this for a long time it assumes there's no leader and beings a new election
  - Used to replicate log entries and form a heartbeat
  - Servers can retry if they don't receive a response in a timely manner

### Elections
- Follower increments its current term and becomes a candidate (votes for itself)
- It can either win, lose, or there can be no winner for some time
- A candidate wins with majority vote
- Once a candidate wins, it sends heartbeats to maintain its state
- We can have split votes (ties), so we have election timeouts (150-300 ms) to prevent this

### Log Replication
- Raft guarantees that:
  - If two entries in diff logs have the same idx and term, they have the same command
  - If two entries in diff logs have the same idx and term, the logs are identical in previos entries

### State

Log Entries:
- Term in which it was created (#)
- Command for the state machine

Leader:
- 

Peers:
- 

Sources: 
- https://thesecretlivesofdata.com/raft/
- https://pdos.csail.mit.edu/6.824/notes/raft_diagram.pdf
