use std::{thread, time::Duration};

use threadpool::{ThreadPool, shared_queue_thread_pool::SharedQueueThreadPool};

fn main() {
    let thread_pool = SharedQueueThreadPool::new(4);
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
