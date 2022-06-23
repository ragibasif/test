# wc

Inspired by coreutil 'wc'.  
This small project will read in the input file, count the number of chars, words, and
lines, and display the information to the command line.

Lines: Counts each line with text. Does not count empty lines.  
Words: Counts words by checking for white space. Does not count white space.  
Characters: Counts characters including space. Does not count '\n'.

Output is colorized.

## How to use

Rust and cargo need to be installed. Clone the repository.  

```sh
cargo run -- -i <FILE>
```

To run sample:  

```sh
cargo run -- -i sample.txt
```
