# Prerequisites
Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.

# Running
1) Change directory to project
```
$ cd kvdb
```
2) Compile binary
```
$ cargo build --release
```

3) Running
```
./target/release/kvdb <in-memory|file-based> [DB_FILENAME] [--create]
```
Parameters details:
- if selected db type is "in-memory", such db is going to store all data in ram. Data will be lost between restarts. Doesn't need DB_FILENAME to be passed
- if selected db type is "file-based", such db will store all data in file provided in "DB_FILENAME". If no file exists, it will fail with "No such file or directory error". Data is persisted in file, so it's possible to run new instance of db from the same file. It wasn't attempted to run 2 instances from the single file
- "--create" and "DB_FILENAME" are parameters related only to "file-based" db type. DB_FILENAME - path to actual file where to store the data and read data from. "--create" - in case if provided file doesn't exist, app tries to create it

4) REPL will run run. Example of interaction:
```
$ ./target/release/kvdb file-based test --create
kvdb> set key1 val1
Ok
kvdb> get key1
val1
kvdb> delete key1
val1
kvdb> get key1
NoSuchKey
kvdb> exit
$
```

Possible commands in REPL:
- "set <key> <value>" - setting "key" to "value". Return Ok, if succeeded
- "get <key>" - tries to get value of "key". If exists, returns value. If doesn't, returns "NoSuchKey"
- "delete <key>" - tries to delete value of "key". If exists, returns value and delete key-value pair from db. If such key doesn't exists, it returns "NoSuchKey"
