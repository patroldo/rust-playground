use std::{thread, time::Duration};

use threadpool::{
    ThreadPool, async_shared_queue_thread_pool::SharedChannelThreadPool,
    round_robin_thread_pool::RoundRobinThreadPool, shared_queue_thread_pool::SharedQueueThreadPool,
};

fn main() {
    let thread_pool = SharedChannelThreadPool::new(4);
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("123");
    });
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("456");
    });
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("789");
    });
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("qwe");
    });
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("rty");
    });
    thread_pool.execute_task(|| {
        thread::sleep(Duration::from_secs(3));
        println!("uio");
    });
    println!("WAiting when all gonna be finished");
    thread_pool.stop_threads();
}
