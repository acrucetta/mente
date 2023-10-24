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


{
    name: "baby carrots"
    category: "vegetables"
    brand: "365"
}


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


## Architecture

Documents are hashed to a particular shard. Each shard may be on a different node in a cluster. Every shard is a self-contained Lucene index of its own.

The index has two primary shards and two replicas. The application round robins requests among nodes.

Write requests are routed to the primary shard, then replicated.

Read requests are routed to the primary or any replicas

If we go over the amopunt of shards (200K documents) we can horizontally scale to hold more data. Sharding speeds up the search. We can run a search on all the shards in parallel.

To prevent losing data, elastic search replicates data across clusters (cross-cluster replication). CCR provides a way to automatically synchronize indices from your primary cluster to a secondary remote cluster.  

### Sharding

Sharding allows us to divide indices into smaller pieces.

e.g., Node A (500 gb); Node B (500 gb)

Index (600 GB) gets placed into both indices.

- A shard is an independent index
- A shard has no predefined size; it grows as documents are added
- A shard may store up to about 2 billion documents

Why we shard:
- Allows us to store more documents
- Easier to fit large indices
- Improved parallel performance

Configuring shards:
- We use to have 5 shards by default
- Indices are now created with one shard by default
- We can modify the shards with the split/shrink API

How many shards:
- If you anticipate millions of docs, consider a couple of shards
- Otherwise, use the default shards

### Replication

Elastic search natively supports replication. Setting it up can be complicated.

How does replication work?
- Replication creates copies of shards (replica shards)
- A shard that was replicated is called "Primary shards"

Node B
[Primary Shard B][Replica A1][Replica A2]

Choosing replicas:
- How many replicas are ideal; depends on the use case
- IS the data stored elsewhere?
- We should replicate shards at least twice

Snapshots:
- Snapshots are also back-ups
- Snapshots can be taken at the index level or for the entire cluster
- We can take a snashot before running a query; replication can't help with this but it can with a snapshot (rollback to snapshot)

Increasing throughput with replication:
- Replica shards can serve diff search requests simultaneously
- Elastic can route to the best shared
- CPU paralellization can improve performance if multiple shards are on the same node

### Node Roles

Default roles: dim (data, ingest, master)

Master node:
- A node may be elected as the cluster's master nodes
- A master node is responsible for creating and deleting indices
 
Data role:
- Enables a node to store data
- Storing data includes performing queries related to that data, such as queries

Ingest role:
- Node that ingests data
- Ingest pipelines are a series of steps performed when indexing documents 
- A simplified version of Logstash, directly within Elastic search

Machine Learning
- Identifies node as ML node
- useful for running ML jobs

Coordination nodes
- Processes request; not possible for single setting
- Essentially a load balancer

When to change node roles:
- It depends; useful for large clusters
- Typically done when optimizing the cluster to scale
- Only change when you know what you're doing

## Managing Documents

- Elastic search documents are immutable
- The existing document is replaced with a modified document
- We can do the same at the app level

**Creating and deleting indices**

DELETE /pages

PUT /products
{
	"settings" : {
	...
	}
}


POST /products/_doc/100
{
"name":"Coffee Maker"
"price":64
"in_stock":10
}

GET /product/_doc/100


**Update field**

POST /products/_update/100
{
	"doc: {
		"in_stock":3
	}
}

**Add field**

POST /products/_update/100
{
	"doc: {
		"tags": ["electronics"]
	}
}



## Commands

- GET /cluster/health
- GET /_cat/indices


