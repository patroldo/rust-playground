## Summary
Learning and practising programming in rust. Here i'm going to place projects. List:
  - log-analyzer - utility which provides amount of "WARN", "ERROR" and "INFO" logs from file. Into stderr it prints amount of unparsed logs
  - pgrep - utility which search for string across provided files. The main point here is to work with multi threading. Implementation contains different approaches for processing multiple files concurrently(multi-thread) even though some are not optimized(like cloning the data)
  ## Rust Learning Projects
  
  The goal of these projects is to become comfortable with Rust through progressively more complex practical applications. Each project should introduce new Rust-specific concepts rather than only increasing application size.
  
  | Status | # | Project | Main learning focus | Key concepts | Done when |
  |---|---:|---|---|---|---|
  | ✅ | 1 | [**Log Analyzer**](https://github.com/patroldo/rust-playground/tree/dev/log-analyzer) | Rust fundamentals and error handling | `Result`, `Option`, `TryFrom`, enums, structs, `BufRead`, `map_err`, custom errors, unit/integration testing | Parses log files, reports statistics and malformed entries, handles I/O errors, and has black-box CLI tests |
  | ✅ | 2 | [**pgrep**](https://github.com/patroldo/rust-playground/tree/dev/pgrep) | Ownership and basic concurrency | borrowing, slices, threads, `Arc`, `thread::scope`, `'static` vs scoped lifetimes, parallel work, parameterized tests | Searches multiple files concurrently and several implementations pass the same behavioral test suite |
  | ✅ | 3 | [**Thread Pool / Job Executor**](https://github.com/patroldo/rust-playground/tree/dev/thread-pool) | Thread coordination and shared state | `Arc`, `Mutex`, channels, `Send`, `Sync`, worker threads, shutdown, `Drop` | Jobs can be submitted to a fixed set of workers and the pool shuts down cleanly |
  | ⬜ | 4 | [**Persistent Key-Value Store**](https://github.com/patroldo/rust-playground/tree/dev/kvdb) | Domain modeling and persistence | enums, serialization, file I/O, ownership boundaries, iterators, error types, modules | Supports `set`, `get`, `delete`, persists data to disk, and restores state after restart |
  | ⬜ | 5 | **TCP Chat Server** | Networking and communicating threads | `TcpListener`, `TcpStream`, channels, shared clients, `Arc<Mutex<_>>`, connection lifecycle | Multiple clients can connect, send messages, disconnect, and receive broadcasts |
  | ⬜ | 6 | **HTTP Server from Scratch** | Protocol parsing and application architecture | TCP, parsing, traits, routing, thread pool reuse, request/response modeling | Handles basic HTTP requests, routing, headers, errors, and concurrent clients without a web framework |
  | ⬜ | 7 | **Async HTTP/TCP Server** | Async Rust fundamentals | `async`/`await`, `Future`, Tokio tasks, async I/O, `mpsc`, `select!`, cancellation | Reimplements one previous networking project using Tokio and explains why the async design differs from the threaded one |
  | ⬜ | 8 | **Mini Redis** | Stateful concurrent server design | shared state, async concurrency, protocol framing, commands, synchronization, connection handling | Implements a small subset such as `GET`, `SET`, `DEL`, and optionally TTL over a network protocol |
  | ⬜ | 9 | **Custom Data Structure / Cache** | Deeper ownership and generic API design | generics, traits, iterators, lifetimes, `Entry`-style APIs, `Rc`/`Weak` where appropriate | Implements something such as an LRU cache with a clean generic API and good tests |
  | ⬜ | 10 | **Larger Rust Application** | Architecture and idiomatic Rust design | workspace structure, library boundaries, traits, dependency inversion, concurrency, testing, observability | A non-trivial application is split into coherent crates/modules and can evolve without large architectural rewrites |
  
  ### Optional advanced projects
  
  | Status | Project | Main learning focus |
  |---|---|---|
  | ⬜ | **Mini Git** | binary formats, filesystem modeling, hashing, recursive structures |
  | ⬜ | **Mini SQLite / Storage Engine** | pages, binary encoding, indexes, persistence, low-level data structures |
  | ⬜ | **BitTorrent Client** | networking, binary protocols, concurrency, state machines |
  | ⬜ | **Actor System** | message passing, task lifecycle, supervision, concurrency architecture |
  | ⬜ | **Embedded Rust Project** | `no_std`, hardware abstraction, ownership around peripherals and interrupts |
  
  ### Progress rules
  
  A project is considered complete when:
  
  - the main functionality works;
  - important failure paths are tested;
  - public APIs use appropriate borrowed types such as `&str`, `&[T]`, and `&Path` where applicable;
  - unnecessary `.clone()` calls are understood or removed;
  - concurrency decisions can be explained in terms of ownership and lifetimes;
  - there is no requirement to make the project production-ready before moving to the next one.
  
  The objective is not to perfect every project. The objective is to encounter a new Rust problem, understand it, implement a reasonable solution, and move forward.
