# Introduction to Data Mining Project

## How to Use

1. Install [Rust](https://www.rust-lang.org/learn/get-started)

2. Open the command line in the directory.

3. Run ```cargo run --release -- -h``` to get the help menu.

4. To get the help menu of a specific algorithm, run ```cargo run --release -- [algorithm] -h```

5. To time the algorithm run ```cargo run -t --release -t [algorithm]```

All outputs are printed to standard out.

## Algorithms

### Apriori

This subcommand runs the apriori algorithm with the counting being done by a Hash Tree.  

This Hash Tree contains internal and leaf nodes.  The internal nodes contain a static array of pointers to either internal or leaf nodes.  To get the correct index of the internal node for each candidate itemset, we hash the corresponding element.  The leaf node is just a vector.

### Apriori-TiD

This algorithn runs the Apriori-TiD algorithm.  This algorithm counts through the use of transaction IDs that are just a Hashset of previous frequent itemsets.

### Apriori Hybrid

This algorithm starts with using Apriori to count then switches to using Apriori-TiD after some certain pass.  The rationale behind this is that Apriori is fast for smaller itemsets, but Apriori-TiD is faster for larger itemsets due to there being less transaction IDs.

### Count Distribution

This algorithm is a parallelized version of Apriori where each thread gets a partition of the transactions.  They receive a candidate itemset and count their frequency in the transactions.  Then, they return the counts and one process adds up all the counts and generates the frequent and next candidate itemset.

### Count Distribution Hybrid

This algorithm is the parallelized version of Apriori Hybrid.

### Apriori Trie

This algorithm uses a Hash Trie to count candidate itemsets.  The rationale behind this is that tries can be more space efficient and faster than the Hash Tree.