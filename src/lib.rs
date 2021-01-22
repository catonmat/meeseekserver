use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {

    }

    ThreadPool { threads }
  }

  // f == Closure; trait bound required
  pub fn execute<F>(&self, f: F)
  where 
    // trait bounds
    // FnOnce() to hold a closure value; <F> is unit type ()
    // Send to send closure from 1 thread to another
    // 'static for a universal lifetime (don't know how long a thread will take to execute)
    F: FnOnce() + Send + 'static,
  {
  }
}
