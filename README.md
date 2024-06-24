
# Welcome to Search Benchmark, the Game!

Credits: This repository has been forked from Tantivy's [search-benchmark-game](https://github.com/quickwit-oss/search-benchmark-game/), originally developed by [Jason Wolfe](https://github.com/jason-wolfe/search-index-benchmark-game).

This repository is standardized benchmark for comparing the speed of various
aspects of search engine technologies. 

The results are available [here](https://seekstorm.github.io/search-benchmark-game/).

This benchmark is both
- **for users** to make it easy for users to compare different libraries
- **for library** developers to identify optimization opportunities by comparing
their implementation to other implementations.

## The benchmark

Different search engine implementation are benched over different real-life tests.
The corpus used is the English wikipedia. Stemming is disabled. Queries have been derived
 from the [AOL query dataset](https://en.wikipedia.org/wiki/AOL_search_data_leak)
 (but do not contain any personal information).

Out of a random sample of query, we filtered queries that had at least two terms and yield at least 1 hit when searches as
a phrase query.

For each of these query, we then run them as :
- `intersection`
- `unions`
- `phrase queries`

with the following collection options :
- `COUNT` only count documents, no need to score them
- `TOP 10` : Identify the 10 documents with the best BM25 score.
- `TOP 10 + COUNT`: Identify the 10  documents with the best BM25 score, and count the matching documents.

We also reintroduced artificially a couple of term queries with different term frequencies.

All tests are run once in order to make sure that
- all of the data is loaded and in page cache
- Java's JIT already kicked in.

Test are run in a single thread.
Out of 10 runs, we only retain the best score, so Garbage Collection likely does not matter.

### Benchmark environment

The results file that is included in this repository was generated using the following benchmark
environment:
 - ThinkPad P1 Gen 6
 - Processor Intel(R) Core(TM) i7-13700H
 - SK hynix 2TB SSD PCIe 4.0 NVMe M.2
 - Ubuntu 24.04 LTS
 - Rust 1.79
 - OpenJDK 17

## Engine specific detail

### Lucene

- [Lucene](https://github.com/apache/lucene) license: Apache-2.0 language: Java
- Query cache is disabled.
- GC should not influence the results as we pick the best out of 5 runs.
- The `-bp` variant implements document reordering via the bipartite graph partitioning algorithm, also called recursive graph bisection.

### Tantivy

- [Tantivy](https://github.com/quickwit-oss/tantivy) license: MIT language: Rust
- Tantivy returns slightly more results because its tokenizer handles apostrophes differently.
- Tantivy and Lucene both use BM25 and should return almost identical scores.

### SeekStorm

- [SeekStorm](https://github.com/SeekStorm/SeekStorm) license: Apache-2.0 language: Rust
- Query cache is disabled.

# Reproducing

These instructions will get you a copy of the project up and running on your local machine.

### Prerequisites

The lucene benchmarks requires Java, the most recent version is recommended.
The tantivy benchmarks and benchmark driver code requires Cargo. This can be installed using [rustup](https://www.rustup.rs/).

### Installing

Clone this repo.

```
git clone git@github.com:seekstorm/search-benchmark-game.git
```

## Running

Checkout the [Makefile](Makefile) for all available commands. You can adjust the `ENGINES` parameter for a different set of engines.

Run `make corpus` to download and unzip the corpus used in the benchmark.
```
make corpus
```

Run `make index` to create the indices for the engines.

```
make index
```

Run `make bench` to build the different project and run the benches.
This command may take more than 30mn.

```
make bench
```

The results are outputted in a `results.json` file.

You can then check your results out by running:

```
make serve
```

And open the following in your browser: [http://localhost:8000/](http://localhost:8000/)


# Adding another search engine

See `CONTRIBUTE.md`.
