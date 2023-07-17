

{books}

## Chapter 1

### Data Engineering Teams
The data engineer is a hub between data producers, such as software engineers, data architects, and DevOps or site-reliability engineers (SREs), and data consumers, such as data analysts, data scientists, and ML engineers. In addition, data engineers will interact with those in operational roles, such as DevOps engineers.

### ML Engineers vs. Data Engineer

The ML engineer overlaps with DE, but it develops more advanced ML techniques, train models, and designs and maintains infrastructure running ML processes. It emphasizes more MLOps and other mature practices such as DevOps.

## Chapter 2

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

Key Questions:
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
- 
