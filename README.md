# Introduction to Data Mining Project

## How to Use

1. Install [Rust](https://www.rust-lang.org/learn/get-started)

2. Open the command line in the directory.

3. Run `cargo run --release -- -h` to get the help menu.

4. To get the help menu of a specific algorithm, run `cargo run --release -- [algorithm] -h`

5. To time the algorithm run `cargo run -t --release -t [algorithm]`

All outputs are printed to standard out. Use `> [file]` operator to send the output to a file. The inputs should be a file with lines being a transaction. Each transaction should be made up of a space separated list of item IDs.

For example,

```
0 2 5 9 11
2 6 3 4 12 63 23
```

## Algorithms

### Apriori

This subcommand runs the apriori algorithm with the counting being done by a Hash Tree.

This Hash Tree contains internal and leaf nodes. The internal nodes contain a static array of pointers to either internal or leaf nodes. To get the correct index of the internal node for each candidate itemset, we hash the corresponding element. The leaf node is just a vector.

```
cargo run --release -- apriori [File] [Support Count]
```

### Apriori-TiD

This algorithn runs the Apriori-TiD algorithm. This algorithm counts through the use of transaction IDs that are just a Hashset of previous frequent itemsets.

```
cargo run --release -- apriori-tid [File] [Support Count]
```

### Apriori Hybrid

This algorithm starts with using Apriori to count then switches to using Apriori-TiD after some certain pass. The rationale behind this is that Apriori is fast for smaller itemsets, but Apriori-TiD is faster for larger itemsets due to there being less transaction IDs.

```
cargo run --release -- apriori-hybrid [File] [Support Count] [Pass to switch]
```

### Count Distribution

This algorithm is a parallelized version of Apriori where each thread gets a partition of the transactions. They receive a candidate itemset and count their frequency in the transactions. Then, they return the counts and one process adds up all the counts and generates the frequent and next candidate itemset.

```
cargo run --release -- count-distribution [File] [Support Count] [Threads]
```

### Count Distribution Hybrid

This algorithm is the parallelized version of Apriori Hybrid.

```
cargo run --release -- count-distribution [File] [Support Count] [Pass to Switch > 2] [Threads]
```

### Apriori Trie

This algorithm uses a Hash Trie to count candidate itemsets. The rationale behind this is that tries can be more space efficient and faster than the Hash Tree.

```
cargo run --release -- apriori-trie [File] [Support Count]
```
### FP Growth

This algorithm uses an FP Tree to find frequent itemsets. The rationale behind this is that FP Trees are more memory-efficient and can find patterns instead of counting.

```
cargo run --release -- fp-growth [File] [Support Count]
```