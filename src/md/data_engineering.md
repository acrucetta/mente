

{books}

## Chapter 1: Overview of Data Engineering

![lifecycle](https://www.oreilly.com/api/v2/epubs/9781098108298/files/assets/fode_0201.png)

### Data Engineering Teams
The data engineer is a hub between data producers, such as software engineers, data architects, and DevOps or site-reliability engineers (SREs), and data consumers, such as data analysts, data scientists, and ML engineers. In addition, data engineers will interact with those in operational roles, such as DevOps engineers.

### ML Engineers vs. Data Engineer

The ML engineer overlaps with DE, but it develops more advanced ML techniques, train models, and designs and maintains infrastructure running ML processes. It emphasizes more MLOps and other mature practices such as DevOps.

## Chapter 2: Main components of Data Engineering

### Data Generation
- Data is coming from a variety of sources now
- It's hard to keep in the same schema. Schema evolution is now considered part of the Agile approach.

### Storage
- Cloud architectures often have several storage solutions
- Hot data is data commonly retrieved many times per day. It should be geared for fast retrieval
- Cold data is seldom queried and mostly for archival purposes.

### Ingestion
- A common bottleneck of the lifecycle; source systems are outside of your direct control and might have data of poor quality
- The separation of storage and compute makes streaming more popular
- Can my downstream services handle the volume of data I'm sending them?
- Batch is still widely used because of legacy systems, streaming can be useful but it creates increasing amounts of complexity

**Key Questions:**
- What are the use cases for the data?
- What is the destination?
- In what volume will it arrive?
- What format is the data in?

### Transformation
- Change data into correct types, put records into standard formats, remove bad ones
- Large-scale aggregation for reporting or featurizing data for ML processes

### Serving
- Data is served for analytics, ML, and reverse ETL
- Before investing into ML, companies need to build a solid data foundation
- This means setting up the best systems and architecttures across the data engineering and ML lifecycle.
- Analytics then ML
- Reverse ETLs feed bvack the data into source systems. It's useful for businesses relying on SaaS.

---

### Data Ops

- DataOps automation has a similar framework and workflow to DevOps, consisting of change management (environment, code, and data version control), continuous integration/continuous deployment (CI/CD), and configuration as code.
- DatOps consist of automation, observability and monitoring, and incident response

#### Observability and Monitoring
- Data is a silent killer; bad data can linger in reports for a long time
- You need to implement observing, monitoring, logging, alerting, and tracing 
- DODD focused on making data observability a first-class consideration in the data engineering lifecycle

---

### Data Architecture

#### Types of data architectures
- Data warehouse: structured, includes the compute
  - A subject-oriented, integrated, nonvolatile, and time-variant collection of data in support of mgmt decisions
  - Separates OLAP from OLTP
  - Centralized and organizes data
  - e.g., Snowflake, Amazon Redshift
- Data lake: unstructured, data can be in any format
  - Common during big data era
  - Data can be queried with MapReduce, Spark, Presto, etc...
- Data lakehouse: combines benefits from both, introduced by Databricks
  - Includes control, data management and data structures
  - Still houses data in an object storage and supports a variety of query and transformation engines
  
#### Modern Data Stack
- Instead of using monolithic toolsets, use cloud-based, plug and play, off-the-shelf components
- Includes data pipelines, storage, monitoring, etc...

---

## Chapter 5: Data Generation

### Data Logs
- Insert only: always retain the records, add a timestamp

### Messages and Streams
- Message queues and streaming platforms are often used interchangeably
- A message is raw data communicated across two or more systems
  - A message is normally sent through a message queue from a publisher to a consumer
  - Once the message is delivered, it is removed from the queue
- A stream is an append only log of event records
  - You use streams when you care about what happened over many events

### Types of time
- Event time; event is generated
- Ingestion time; event is ingested
- Process time; event is processed

### Ways of ingesting data
- APIs
  - REST
  - GraphQL
  - WebHooks
    - Event based data transmission pattern
    - Data source can be an application backend, a web page, or a mobile app
  - RPC and gRPC: remote procedure call library; bidirectional exchange of data over HTTP/2
    - Imposes more technical standards than REST
- Message queues
  - Critical for decoupled microservices and event-driven architectures
  - Need to keep in mind the frequency of delivery, message ordering and scalability
  - Some issues with queues can be the order of the messages
  - Ideally, should be idempotent; outcome is the same after processing it multiple times
- Event streaming platform
  - Produces data in an ordered log of records
  - Includes
    - Topics - collection of related events
    - Stream partitions
  - Tend to be more fault tolerant because they're distributed

### Ingestion undercurrents
- Security: how is the data secured and encrypted; do we have a VPN, or an SSO?
- Data management: who manages the data, what's the quality, what if the schema changes, is the data HIPAA?
- DataOps: how will we know when there's an issue, how are we monitoring, is the schema conformant, what do we do if something bad happens
- Architecture: what if the system fails, what do we do if we lose data, how available are the sources, who's in charge of them
- Orchestration: how often do we recieve the data, can we integrate with the upstream application team
- SWE: Can the code access with the right credentials, how do we authenticate, how do we access (API, REST), can we parallelize the work, how do we deploy code changes?

