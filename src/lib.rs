pub struct ThreadPool;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    ThreadPool
  }

  // f == Closure; trait bound required
  pub fn execute<F>(&self, f: F)
  where 
    // trait bounds
    // FnOnce() to hold a closure value
    // Send to send closure from 1 thread to another
    // 'static for a universal lifetime (don't know how long a thread will take to execute)
    F: FnOnce() + Send + 'static,
  {
  }
}
