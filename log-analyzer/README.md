# Prerequisites
Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.

# Running
1) Change directory to project
```
$ cd log-analyzer
```
2) Compile binary
```
$ cargo build --release
```

3) Running on file
```
./target/release/log-analyzer <path_to file>
```
Output:
```stdout
INFO: X
WARN: Y
ERROR: Z
```
```stderr
Unparsable logs: G
```

Example:
```
$ ./target/release/log-analyzer ./tests/data/example.logs 2>&1
Unparsable logs: 0
INFO: 1
WARN: 1
ERROR: 1
```
