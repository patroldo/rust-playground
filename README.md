## Summary
Learning and practising programming in rust. Here i'm going to place projects. List:
  - log-analyzer - utility which provides amount of "WARN", "ERROR" and "INFO" logs from file. Into stderr it prints amount of unparsed logs
  - pgrep - utility which search for string across provided files. The main point here is to work with multi threading. Implementation contains different approaches for processing multiple files concurrently(multi-thread) even though some are not optimized(like cloning the data)
