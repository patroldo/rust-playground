pub mod async_shared_queue_thread_pool;
pub mod round_robin_thread_pool;
pub mod shared_queue_thread_pool;

pub trait ThreadPool {
    fn new(thread_pool_size: usize) -> Self;
    fn execute_task(&self, fun: impl FnOnce() + Send + 'static);
    fn stop_threads(self);
}

trait Worker {
    fn stop(self);
}

pub enum WorkerMessage {
    SomeFun(Box<dyn FnOnce() + Send + 'static>),
    Exit,
}
