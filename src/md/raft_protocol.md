
Overview
- Raft is a protocol for implementing distributed consensus.

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

Sources: 
- https://thesecretlivesofdata.com/raft/
- https://pdos.csail.mit.edu/6.824/notes/raft_diagram.pdf