use std::{error::Error, fmt};


pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            return Err(PoolCreationError::SizeError);
        }
        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {
    
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    SizeError
}

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoolCreationError::SizeError => {
                return write!(f, "Size is too big")
            }
        }
    }
}
