use std::time::Duration;

pub const ITERATIONS: usize = 10_000;
pub const CONCURRENT_TASKS: usize = 1_000;

pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub tasks_per_second: f64,
    pub memory_usage: Option<usize>,
}

impl BenchmarkResult {
    pub fn new(name: String, duration: Duration, task_count: usize) -> Self {
        let tasks_per_second = task_count as f64 / duration.as_secs_f64();
        Self {
            name,
            duration,
            tasks_per_second,
            memory_usage: None,
        }
    }

    pub fn print(&self) {
        println!("=== {} ===", self.name);
        println!("Duration: {:?}", self.duration);
        println!("Tasks/sec: {:.2}", self.tasks_per_second);
        if let Some(mem) = self.memory_usage {
            println!("Memory: {} KB", mem / 1024);
        }
        println!();
    }
}

// Simulated CPU-bound work
pub fn cpu_work(iterations: usize) -> u64 {
    let mut sum = 0u64;
    for i in 0..iterations {
        sum = sum.wrapping_add(i as u64 * 17 + 42);
    }
    sum
}

