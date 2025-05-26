## Rust synchronous Programming benchmarks 

This comprehensive benchmark suite tests Tokio, async-std, and smol across five key scenarios:

### Benchmark Categories

1. Task Spawning - Raw task creation/completion speed
2. Channel Communication - Message passing between tasks
3. Timer Operations - Sleep/timeout efficiency
4. Concurrent Tasks - Performance under high load
5. TCP Echo Server - Real network I/O performance

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
