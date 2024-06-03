
## Notes on LLMs

## Key Terms:
- Embeddings
- Retriever
- RAG
- Chain-of-thought
- Vector databases 
- HNSW - Hierarchical Navigable Small Worlds: a key method for approximate nearest neighbor search in high-dimensional vector databases, for example in the context of embeddings from neural networks in large language models. Databases that use HNSW as search index include: Apache Lucene Vector Search (e.g., Elasticsearch uses the HNSW algorithm to support efficient kNN search)

## Embeddings

Embeddings are ways of storing similar words together as low-level numbers. It uses a pre-trained neural network to process some text and then output an array of numbers e.g., [-0.5,1.0] etc...

Similar words are closer in the matrix space.

We normally store these embeddings into a vector DB. An example would be:

```sql
CREATE TABLE embeddings AS (
    text string,
    array json_agg[int]
)
```

To query this table, we can get embeddings from our search word and take the dot product of our embeddings and the embeddings column in that table. Then we just get the top K values.

Embeddings were popularized by Google in 2013 with statements such as “king - man + woman = queen.” The gist of it, as you may know, is that we can express words as vectors that encode their semantics in a meaningful way.

Some embedding models are: skip-gram and bag of words.

## Retrievers

It's job is to find relevant documents or pieces of information that can help answer a query. It takes the input query and searches a DB to retrieve info that might be useful to generate the response

Types:
- Dense retrievers: they use NN to create dense vector embeddings of the text; good for semantic similarities
- Sparse retrievers: rely on term-matching techniques like TF-IDF or BM25. They're good at finding docs with exact keyword matches

## Storing embeddings

To store embeddings you can use Postgres with pgvector vs. more advanced Vector-DBs. These DBs are queried with dot-products between your search term and the embedding space

E.g.,

```sql
CREATE TABLE documents (  
  id SERIAL PRIMARY KEY,
  document bytea
  ...
)
CREATE TABLE embeddings (
  id SERIAL PRIMARY KEY, 
  document_id INT NOT NULL, 
  chunk VARCHAR NOT NULL, 
  embeddings vector(384), 
  ...
);
```

## Lang Chain

- Components
    - LLM wrappers
    - Prompt templates
    - Indices for relevant info retrieval
- Chains
    - Assemble components
- Agents
    - Allow us to execute Python code

## RAG

What it is:
- Used to enrich prompts with your documents
- Most widely used technique now, doesn't require training an LLM

Tools:
- Lang chain 

How to use RAG?
- Get a corpus
- Load it to the prompt interface
- Pass it to the LLM when querying data

### RAG Stack
- Current RAG stack to build a QA system
    - DOC -> Chunks -> Vector DB -> Chunk -> LLM
- Main components are:
    - Data: can we store additional info beyond raw text
    - Embeddings: can we optimize our embedding representations
    - Retrieval: can we do better than top-k embedding lookup?
    - Synthesis can we use llm for more than generation? (LLMs for reasoning)

### Optimizing RAGs

- Table stacks:
    - Better parsers
    - Chunk sizes
    - Hybrid search
    - Metadata filters
- Advanced retrieval
    - Reranking
    - Recursive retrieval
    - Embedded tables
- Fine-tuning
    - Embedding fine-tuning / LLM fine-tuning
Agentic Behavior
    - Routing
    - Query planning
    - Multi-document agent

Chunk sizes:
- Tuning your chunk sizes can have outsized impacts. Not obvious more tokens = more performance

Metadata filtering:
- Context you can inject into each text chunk
- Adding page number, document titles, summary of adjacent chunks
- E.g., give me risk factors in 2021; then we provide metadata tags {year:2021}

Multi-document agents
- There's certain questions that top-k RAG can't answer
- We use fact-based QA and summarization over subsets of documents
- We add chain of thought and query planning

Embeddings
- Embedding reps are not optimized over your dataset
- Generate a synthetic query dataset from raw text chunks using LLMs

Fine-tuning LLMs
- Weaker LLMs are good at synthesis reasoning
- We can generate a synthetic dataset from raw chunks?

### Challenges with Naive RAG

Bad retrieval
- Low precision: not all chunks are relevant
- Low recall: not all relevant chunks are retrieved
- Outdated information
