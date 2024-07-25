
## Notes on Microservices

Microservices are a great architecture pattern that can allow teams to scale fast. It allows you to use different technologies and to differentiate system boundaries across your services. However, it doesn't need to be the default architecture. For most small teams, a modular monolithic architecture is better than using a microservices architecture.

They're good once your domain model has stabilized and you have clear boundaries.

Pros of Microservices
- Can use different technologies
- If one system fails, the other ones can still be active
- Easy to scale independent services
- Easy to deploy individual changes (no need to reboot the entire application)
- Easy to split teams based on your org

Cons of Microservices
- Too many new technologies for microservices
- More expensive
- Harder to manage reporting (need log aggregation tools)
- Harder to monitor issues
- Security risk as information flows between processes
- Can be slower because of networking costs
- Keeping data consistent across microservices

### What makes a good microservices boundary?

Any interaction from a service that depends on another one means the services are coupled. We want to modify and update the services independently.

We generally want strong cohesion and low coupling.

**Information Hiding**
- Programs should hide as many details from each other as possible
- We want to hide as many details as possible from the end user
- *The connection between modules are the assumptions they make about each other*

**Cohesion**
- Programs should be cohesive internally but loosely coupled externally
- *Code that changes together, stays together* 
- We want to make changes in as few places as possible. If we need to make changes in multiple places it will be a pain 
- E.g., config values change behavior across the code

**Coupling**
- Changes to one service shouldn't require changes to others
- Classic mistakes are services that depend on each other (see below)
- This can happen when a service changes state in another service, when it depends on many services downstream, or when it needs to pass data through other services to do tasks

### Types of coupling

![[Pasted image 20240612112812.png]]

Domain
- The microservice interacts with another microservice
- If it depends in too many it might be an issue
- It is generally considered to be a loose form of coupling

![[Pasted image 20240613141735.png|500]]

**Pass-through**
- The microservice needs to pass through a request to another microservice
- We might be coupling the microservices because the downstream microservice might need to know how the once-removed service works
- It might be easier to build the whole package in the same microservice then return the full result to the final microservice

*Bad*
![[Pasted image 20240613141749.png|500]]

*Better*
![[Pasted image 20240613142005.png|500]]

*Best*
![[Pasted image 20240613141937.png|500]]


**Common** 
- Happens when 2+ ms make use of common data 
- Happens when they used a shared database or filesystem
- IF the schema changes it will require changes in each consumer, it's hard to change as a result
- It can also overwhelm the database if multiple consumers are using it

*Bad*
![[Pasted image 20240613142137.png|500]]

*Best*
![[Pasted image 20240613142309.png|500]]

**Content**
- A microservice goes into another one and changes its database
- Really bad, can result in unforeseen changes in the raw data that can affect another service's response
- The database becomes part of the contract

![[Pasted image 20240613142408.png|500]]

## Implementation

### Technologies Available

- gRPC
- REST
- GraphQL
- Message Brokers