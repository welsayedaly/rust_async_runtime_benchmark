use std::time::Instant;
use smol::{channel, net::{TcpListener, TcpStream}};
use futures::prelude::*;

mod common;
use common::*;

fn main() {
    smol::block_on(async {
        println!("⚡ smol Benchmark Suite");
        println!("======================\n");

        let results = vec![
            task_spawn_benchmark().await,
            channel_benchmark().await,
            timer_benchmark().await,
            concurrent_tasks_benchmark().await,
            tcp_echo_benchmark().await,
        ];

        println!("\n📊 smol Summary:");
        for result in results {
            result.print();
        }
    });
}

async fn task_spawn_benchmark() -> BenchmarkResult {
    let start = Instant::now();
    let mut handles = Vec::new();

    for i in 0..ITERATIONS {
        let handle = smol::spawn(async move {
            cpu_work(100);
            i
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await;
    }

    BenchmarkResult::new("Task Spawn".to_string(), start.elapsed(), ITERATIONS)
}

async fn channel_benchmark() -> BenchmarkResult {
    let start = Instant::now();
    let (tx, rx) = channel::bounded(100);

    // Spawn producer
    let producer = smol::spawn(async move {
        for i in 0..ITERATIONS {
            tx.send(i).await.unwrap();
        }
    });

    // Spawn consumer
    let consumer = smol::spawn(async move {
        let mut count = 0;
        while let Ok(_) = rx.recv().await {
            count += 1;
            if count == ITERATIONS {
                break;
            }
        }
    });

    producer.await;
    consumer.await;

    BenchmarkResult::new("Channel Communication".to_string(), start.elapsed(), ITERATIONS)
}

async fn timer_benchmark() -> BenchmarkResult {
    let start = Instant::now();
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = smol::spawn(async {
            smol::Timer::after(std::time::Duration::from_millis(1)).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    BenchmarkResult::new("Timer Operations".to_string(), start.elapsed(), 1000)
}

async fn concurrent_tasks_benchmark() -> BenchmarkResult {
    let start = Instant::now();
    let mut handles = Vec::new();

    for i in 0..CONCURRENT_TASKS {
        let handle = smol::spawn(async move {
            // Mix of CPU and async work
            cpu_work(1000);
            smol::Timer::after(std::time::Duration::from_millis(1)).await;
            cpu_work(1000);
            i
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    BenchmarkResult::new("Concurrent Tasks".to_string(), start.elapsed(), CONCURRENT_TASKS)
}

async fn tcp_echo_benchmark() -> BenchmarkResult {
    let start = Instant::now();
    
    // Start echo server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    smol::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            smol::spawn(async move {
                let mut stream = stream;
                let mut buf = [0; 1024];
                loop {
                    match stream.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            stream.write_all(&buf[0..n]).await.unwrap();
                        }
                        Err(_) => break,
                    }
                }
            }).detach();
        }
    }).detach();

    // Client connections
    let mut handles = Vec::new();
    for i in 0..100 {
        let handle = smol::spawn(async move {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            let msg = format!("Hello {}", i);
            stream.write_all(msg.as_bytes()).await.unwrap();
            
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf).await.unwrap();
            assert_eq!(&buf[0..n], msg.as_bytes());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    BenchmarkResult::new("TCP Echo Server".to_string(), start.elapsed(), 100)
}