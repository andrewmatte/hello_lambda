# Avobe

## nearest neighbour vector lookup CLI

Steps 1 through 3

- Use serde_bin_vec.py to get your data into binary format for lookup (example data is 10k vectors in words_subset.data)
- Deploy to a FaaS wrapped, like in the test_search.py example, for immutable data or use locally
- $$$ profit $$$


This tool is great for simple, non-approximate lookup of the top thousand IDs. You can hack it to print distances too but it's assumed that you want some kind of result. Currently IDs are padded to 32 chars but you can do whatever you want.

This project is free to use, temporarily under the only condition that you have to email me once whether it worked nicely for you.

### Compilation instructions

cargo build --release

There are some compilation warnings. I ignore them. It's fast because it's using #unsafe for the manhattan distance.

### Performance

It's pretty good. On a single 4.7GHz core, it reads your input vector, reads the file, calculates all distances, sorts and prints the top 1000 in 0.03s for 10k vecs of 512 dims.