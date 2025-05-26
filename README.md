## Usage Instructions


1. **Run benchmarks:**
   ```bash
   ./run_benchmarks.sh
   ```

2. **Individual runtime tests:**
   ```bash
   cargo run --release --bin tokio_bench
   cargo run --release --bin async_std_bench
   cargo run --release --bin smol_bench
   ```

3. **Criterion detailed benchmarks:**
   ```bash
   cargo bench
   # Results in target/criterion/report/index.html
   ```

## What This Benchmarks

- **Task Spawning**: How quickly each runtime can spawn and complete tasks
- **Channel Communication**: Message passing performance between tasks  
- **Timer Operations**: Efficiency of timeout/delay operations
- **Concurrent Tasks**: Performance under high concurrency
- **TCP Echo Server**: Real-world network I/O performance