# Backlog
Implement "show_line_numbers" functinality. It's requiring changes in format output - basically it need to return not only found lines, but rather couple (String, u32). But it also requires implement to validate the file so that number of lines inside that file cannot be bigger than u32::MAX

# Prerequisites
Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.

# Running
1) Change directory to project
```
$ cd pgrep
```
2) Compile binary
```
$ cargo build --release
```

3) Running on file
```
./target/release/pgrep "string_to_find" <path_to file> [<path_to file_2> <path_to file_3> etc.]
```
Output if only single file provided:
```stdout
<Line 1>
<Line 2>
<Line 3>
```
Output if multiple files provided:
```stdout
file1:<Line 1>
file1:<Line 2>
file1:<Line 3>
file1:<Line 4>
file1:<Line 5>
file2:<Line 1>
file2:<Line 2>
file2:<Line 3>
file2:<Line 4>
file2:<Line 5>
```

Example for single file:
```
$ target/release/pgrep "INFO" tests/data/simple_logs.logs
2026-09-11T10:01:03Z INFO server started
2026-09-11T10:01:03Z INFO server has started
```

Example for multiple files:
```
$ target/release/pgrep "INFO" tests/data/simple_logs.logs tests/data/simple_logs_2.logs
tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server started
tests/data/simple_logs.logs:2026-09-11T10:01:03Z INFO server has started
tests/data/simple_logs_2.logs:INFO try to connect
```
