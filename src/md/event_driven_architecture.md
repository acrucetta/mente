
An event-driven architecture uses events to trigger and communicate between decoupled services and is common in modern applications built with microservices. 

An event is a change in state, or an update, like an item being placed in a shopping cart on an e-commerce website. **Events can either carry the state (the item purchased, its price, and a delivery address) or events can be identifiers** (a notification that an order was shipped).

Event-driven architectures have three key components: event producers, event routers, and event consumers.

In Request/Response the API are the building blocks. In event-driven. data is the building block.

It's powered by Apache Kafka.

**Why use it?**

- Event persistence
- Single point of failure
- Performance

**When to use?**

![[Pasted image 20240618115255.png]]

### Components

Pub/Sub Model
- Consumers (APIs)
- Brokers
	- Manages who gets what message
- Producers
	- Produces the messages

### Architecture Patterns

The value of a pattern is telling you things to watch out for. Most systems have some form of these patterns. 

1. Event notification
	- A key element of event notification is that the source system doesn't really care much about the response
	- An application creates an event, that goes into a queue
2. Event-driven state transfer
	1. Since we have more availability we have less consistency (eventual)
	2. We try to keep the data consistent across all services 
3. Event sourcing
	- Upsides
		- Auditing, debugging, historic state, alternative state, memory image
		- With more RAM we can do a lot more processes without using a database
	- Downsides
		- Unfamiliar, external systems, event schema, identifiers
	- You have logs and application state
	- You could rebuild the application state based on the logs
	- Version control is a form of event sourcing system (builds app state off log changes)
	- Another example is the accounting ledger
4. CQRS (Command Query Responsibility Segregation)
	- You have a write component and a read component
	- Similar pattern is used in other databases (oltp and olap)

### Design Patterns

- Unit of Work
- Dependency Inversion
	- All components (prod / consumer) depend on an abstraction (event broker)

### Disadvantages
- Keeping data consistent
	- EDA uses "eventual consistency"; it might cause problems if the data is not aligned across all systems
	- We can use a cache layer to mitigate this (e.g., redis)
- Duplicate messages
	- If the server goes offline and back online, it might get duplicates
	- We can use unique IDs to not process the same events twice
- More complex
	- More components to build
	- Harder to debug when we have issues


## Azure

**Service Bus vs. Event Grid**
- Azure Service Bus is an **enterprise messaging product**. It covers queuing, pub/sub, and has multiple compute based features. Receiving is done via polling (long polling) and usually, a namespace is accessed within/by a single organization.
- Azure Event Grid is a **notification service**. Its sole purpose is to enable pub/sub between event generators and subscribers. It has no queuing semantics. Message delivery is push-based and only a few compute based features are available unlike with Service Bus
### Azure Service Bus

- Fully managed message broker with message queues and pub-sub topics.
- Benefits:
	- Load-balancing
	- Routes data 
	- Coordinates transactional work

**Concepts**

**Queues**
- Messages are sent to and received from **queues**. Queues store messages until the receiving application is available to receive and process them.
- Service Bus keeps messages in memory or volatile storage until client reports them as accepted.
- Messages are delivered in **pull** mode, only delivering messages when requested.

**Topics**
- You can also use **topics** to send and receive messages. While a queue is often used for point-to-point communication, topics are useful in publish-subscribe scenarios.
- Topics can have multiple subscribers
- You can define rules on a subscription. A subscription can have a filter to define a condition for the message. It also has an optional action to modify the metadata

**Namespaces**
- A namespace is a container for all messaging components (queues and topics). A namespace can have one or more queues and topics and it often serves as an application container.
### Resources

**Videos**
- [Event Driven Architecture Playlist](https://www.youtube.com/watch?v=8UlLgOf20Ho&list=PL4JxLacgYgqTgS8qQPC17fM-NWMTr5GW6)
- [What is Event Driven Architecture](https://www.youtube.com/watch?v=DQ5Cbt8DQbM)
- [The Many Meanings of Event-Driven Architecture • Martin Fowler • GOTO 2017](https://www.youtube.com/watch?v=STKCRSUsyP0&t=244s)
- [What do you mean by “Event-Driven”?](https://martinfowler.com/articles/201701-event-driven.html)

**Books**
- Architecture Patterns with Python