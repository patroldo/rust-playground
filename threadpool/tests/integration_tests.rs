use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rstest::rstest;
use threadpool::{
    ThreadPool, async_shared_channel_thread_pool::SharedChannelThreadPool,
    round_robin_thread_pool::RoundRobinThreadPool, shared_queue_thread_pool::SharedQueueThreadPool,
};

#[rstest]
#[case(RoundRobinThreadPool::new(1))]
#[case(SharedQueueThreadPool::new(1))]
#[case(SharedChannelThreadPool::new(1))]
fn test_single_thread_one_execution(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    let x_arc_clone = x.clone();
    thread_pool.execute_task(move || {
        let mut g = x_arc_clone.lock().unwrap();
        *g += 1;
    });
    thread_pool.stop_threads();
    let x = *x.lock().unwrap();
    assert_eq!(1, x);
}

#[rstest]
#[case(RoundRobinThreadPool::new(1))]
#[case(SharedQueueThreadPool::new(1))]
#[case(SharedChannelThreadPool::new(1))]
fn test_single_thread_hunder_executions(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    for _ in 0..100 {
        let x_arc_clone = x.clone();
        thread_pool.execute_task(move || {
            let mut g = x_arc_clone.lock().unwrap();
            *g += 1;
        });
    }
    thread_pool.stop_threads();
    let x = *x.lock().unwrap();
    assert_eq!(100, x);
}

#[rstest]
#[case(RoundRobinThreadPool::new(1))]
#[case(SharedQueueThreadPool::new(1))]
#[case(SharedChannelThreadPool::new(1))]
fn test_single_thread_two_executions(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let x_arc_clone = x.clone();
    let y_arc_clone = y.clone();
    let current_time_stamp_before_executing = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    thread_pool.execute_task(move || {
        let mut g = x_arc_clone.lock().unwrap();
        *g += 1;
        thread::sleep(Duration::from_secs(1));
    });
    thread_pool.execute_task(move || {
        let mut g = y_arc_clone.lock().unwrap();
        *g += 1;
        thread::sleep(Duration::from_secs(2));
    });
    thread_pool.stop_threads();
    let current_time_stamp_after_executing = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let minimum_interval = Duration::from_secs(3).as_millis();
    let x = *x.lock().unwrap();
    let y = *y.lock().unwrap();
    assert_eq!(1, x);
    assert_eq!(1, y);
    assert!(
        (current_time_stamp_after_executing - current_time_stamp_before_executing)
            >= minimum_interval
    )
}

#[rstest]
#[case(RoundRobinThreadPool::new(2))]
#[case(SharedQueueThreadPool::new(2))]
#[case(SharedChannelThreadPool::new(2))]
fn test_two_threads_one_execution(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    let x_arc_clone = x.clone();
    thread_pool.execute_task(move || {
        let mut g = x_arc_clone.lock().unwrap();
        *g += 1;
        thread::sleep(Duration::from_secs(1));
    });
    thread_pool.stop_threads();
    let x = *x.lock().unwrap();
    assert_eq!(1, x);
}

#[rstest]
#[case(RoundRobinThreadPool::new(2))]
#[case(SharedQueueThreadPool::new(2))]
#[case(SharedChannelThreadPool::new(2))]
fn test_two_threads_two_executions(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    let y = Arc::new(Mutex::new(0));
    let x_arc_clone = x.clone();
    let y_arc_clone = y.clone();
    let current_time_stamp_before_executing = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    thread_pool.execute_task(move || {
        let mut g = x_arc_clone.lock().unwrap();
        *g += 1;
        thread::sleep(Duration::from_secs(1));
    });
    thread_pool.execute_task(move || {
        let mut g = y_arc_clone.lock().unwrap();
        *g += 1;
        thread::sleep(Duration::from_secs(2));
    });
    thread_pool.stop_threads();
    let current_time_stamp_after_executing = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let maximum_interval = Duration::from_secs(3).as_millis();
    let x = *x.lock().unwrap();
    let y = *y.lock().unwrap();
    assert_eq!(1, x);
    assert_eq!(1, y);
    assert!(
        (current_time_stamp_after_executing - current_time_stamp_before_executing)
            < maximum_interval
    )
}

#[rstest]
#[case(RoundRobinThreadPool::new(4))]
#[case(SharedQueueThreadPool::new(4))]
#[case(SharedChannelThreadPool::new(4))]
fn test_four_threads_hunder_executions(#[case] thread_pool: impl ThreadPool) {
    let x = Arc::new(Mutex::new(0));
    for _ in 0..100 {
        let x_arc_clone = x.clone();
        thread_pool.execute_task(move || {
            let mut g = x_arc_clone.lock().unwrap();
            *g += 1;
        });
    }
    thread_pool.stop_threads();
    let x = *x.lock().unwrap();
    assert_eq!(100, x);
}
