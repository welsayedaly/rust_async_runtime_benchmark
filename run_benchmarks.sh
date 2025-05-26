#!/bin/bash

echo "🔥 Running Async Runtime Benchmarks"
echo "===================================="

# Build all binaries
echo "Building binaries..."
cargo build --release --bins

echo -e "\n🚀 Running Tokio benchmark..."
cargo run --release --bin tokio_bench

echo -e "\n🌊 Running async-std benchmark..."
cargo run --release --bin async_std_bench

echo -e "\n⚡ Running smol benchmark..."
cargo run --release --bin smol_bench

echo -e "\n📈 Running Criterion benchmarks..."
cargo bench

echo -e "\n✅ Benchmarks complete!"
echo "Check target/criterion/report/index.html for detailed results"