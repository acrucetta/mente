## Elastic Search

Elastic search started off as a scalable Lucene; horizontally scalable search engine. It has been a competitor for Hadoop, Spark, and Flink

It's a server that handles JSON requests.

### Elastic Stack

**Kibana**
- Web UI for searching and visualizing
- Complex aggregations, graphs, charts
- Often used for log analysis

ES is not only for searching text anymore. 

**Logstash / Beats**
- Ways to feed data into Elastic Search
- FileBeat can monitor log files, parse them, and import into Elastic Search in near-real-time
- Not just log files

**X-Pack**
- Security
- Machine Learning
- Graph Exploration

### Basic Concepts

**Documents**: 
- Row in the database
- Something you're searching for
- Any structured JSON data works
- Every document has a unique ID and a type

E.g.,

```json
{
    name: "baby carrots"
    category: "vegetables"
    brand: "365"
}
```

**Indices**
- Highest level entity
- Can contain collection of types -> collection of documents
- Documents that share similar traits are groupped into an index
- Indices are just a virtual representation; doesn't store in disk

**Restful API**
 
ES works via HTTP requests and JSON data. Any language or tool that can handle HTTP can use Elastic Search. You don't need anything beyond HTTP requests.

**Client APIs**

Most languages have specialized ES libraries

**Analytic Tools**

Web based UI to view indices and explore them without code


### Architecture

Documents are hashed to a particular shard. Each shard may be on a different node in a cluster. Every shard is a self-contained Lucene index of its own.

The index has two primary shards and two replicas. The application round robins requests among nodes.

Write requests are routed to the primary shard, then replicated.

Read requests are routed to the primary or any replicas

If we go over the amopunt of shards (200K documents) we can horizontally scale to hold more data. Sharding speeds up the search. We can run a search on all the shards in parallel.

To prevent losing data, elastic search replicates data across clusters (cross-cluster replication). CCR provides a way to automatically synchronize indices from your primary cluster to a secondary remote cluster.  



