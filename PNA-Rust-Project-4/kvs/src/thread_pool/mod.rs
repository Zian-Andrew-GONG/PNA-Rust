use crate::Result;

mod native;
pub use self::native::NaiveThreadPool;

mod shared_queue;
pub use self::shared_queue::SharedQueueThreadPool;

mod rayon;
pub use self::rayon::RayonThreadPool;


pub trait ThreadPool {
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
