# User Manual

## Getting Started
```
cargo run
```

### Prerequisites
- cargo


### Installing

change directory into the project's root, and then run

```
cargo install --path .
```

The command-line application will then be installed and can be run:

```
rust_black_tree <new | add | delete | print | clear | quit>
```

### Example usage of CLI

#### new/create/n/c

Allows for creation of a Red Black tree, AVL tree, or Binary Search tree

    new rb
    new avl
    new bst
    create rb
    n avl
    c bst

#### add/insert/i/a

Insert a value into the tree

    add 5
    insert 2
    i 6
    a 9

#### delete/del/remove/d/r

Delete a value from the tree

    delete 5
    del 2
    remove 6
    d 9
    r 10

Note: non-existent values in the tree passed to the delete command will be ignored

#### print/p

Print the current tree

    print
    p

#### clear/clr

Clear the terminal screen

    clear
    clr


#### quit/exit

Exit the CLI

    quit
    exit

## Running the tests

```
cargo test
```

## Benchmarking

Benchmark reports can be found in `target/criterion/report/index.html


```
cargo bench
```

###### Note: it is recommended to skip the binary search tree benchmark since it takes a long time to run

    cargo bench bench_rbtree
    
    cargo bench bench_avltree

## Authors

* Arun Woosaree
* Jacob Reckhard
* Alexander Rostron
