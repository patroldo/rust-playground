use std::{
    sync::{Arc, Condvar, Mutex, mpsc::channel},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use assert_cmd::assert;
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

#[rstest]
#[case(RoundRobinThreadPool::new(2))]
#[case(SharedQueueThreadPool::new(2))]
#[case(SharedChannelThreadPool::new(2))]
fn check_threads_concurrency(#[case] thread_pool: impl ThreadPool) {
    let (tx, rx) = channel();
    let condvar = Arc::new((Mutex::new(false), Condvar::new()));
    let condvar_clone = condvar.clone();
    let tx_clone = tx.clone();

    thread_pool.execute_task(move || {
        tx_clone.send(()).unwrap();
        let mut guard = condvar_clone.0.lock().unwrap();
        while !*guard {
            guard = condvar_clone.1.wait(guard).unwrap();
        }
    });
    let condvar_clone = condvar.clone();
    let tx_clone = tx.clone();
    thread_pool.execute_task(move || {
        tx_clone.send(()).unwrap();
        let mut guard = condvar_clone.0.lock().unwrap();
        while !*guard {
            guard = condvar_clone.1.wait(guard).unwrap();
        }
    });
    let one_res = rx.recv_timeout(Duration::from_secs(1));
    let second_res = rx.recv_timeout(Duration::from_secs(1));
    {
        let mut guard = condvar.0.lock().unwrap();
        *guard = true;
    }
    condvar.1.notify_all();
    thread_pool.stop_threads();
    assert!(one_res.is_ok());
    assert!(second_res.is_ok());
}

#[rstest]
#[case(RoundRobinThreadPool::new(1))]
#[case(SharedQueueThreadPool::new(1))]
#[case(SharedChannelThreadPool::new(1))]
fn single_worker_does_not_execute_jobs_concurrently(#[case] thread_pool: impl ThreadPool) {
    use std::sync::mpsc::RecvTimeoutError;

    let (tx, rx) = channel();
    let condvar = Arc::new((Mutex::new(false), Condvar::new()));
    let condvar_clone = condvar.clone();
    let tx_clone = tx.clone();
    thread_pool.execute_task(move || {
        tx_clone.send(()).unwrap();
        let mut guard = condvar_clone.0.lock().unwrap();
        while !*guard {
            guard = condvar_clone.1.wait(guard).unwrap();
        }
    });
    let condvar_clone = condvar.clone();
    let tx_clone = tx.clone();
    thread_pool.execute_task(move || {
        tx_clone.send(()).unwrap();
        let mut guard = condvar_clone.0.lock().unwrap();
        while !*guard {
            guard = condvar_clone.1.wait(guard).unwrap();
        }
    });
    let one_res = rx.recv_timeout(Duration::from_secs(1));
    let second_res = rx.recv_timeout(Duration::from_secs(1));
    {
        let mut guard = condvar.0.lock().unwrap();
        *guard = true;
    }
    condvar.1.notify_all();
    thread_pool.stop_threads();
    assert!(one_res.is_ok());
    assert!(matches!(second_res, Err(RecvTimeoutError::Timeout)));
}
