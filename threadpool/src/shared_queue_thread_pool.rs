use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        mpsc::{Sender, channel},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{ThreadPool, Worker};

type TasksQueueType = VecDeque<Box<dyn FnOnce() + Send + 'static>>;

#[derive(PartialEq)]
enum WorkerMessage {
    Exit,
}

struct PullWorker {
    join_handler: JoinHandle<()>,
    sender: Sender<WorkerMessage>,
}

pub struct SharedQueueThreadPool {
    workers: Vec<PullWorker>,
    tasks_queue: Arc<Mutex<TasksQueueType>>,
}

impl PullWorker {
    fn new(tasks_queue: Arc<Mutex<TasksQueueType>>) -> Self {
        let (tx, rx) = channel();
        let join_handler = thread::spawn(move || {
            loop {
                if rx.try_recv().is_ok() {
                    break;
                }
                let optional_task;
                {
                    optional_task = (*tasks_queue.lock().unwrap()).pop_front();
                }
                if let Some(task) = optional_task {
                    task();
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        Self {
            join_handler,
            sender: tx,
        }
    }
}

impl Worker for PullWorker {
    fn stop(self) {
        self.sender.send(WorkerMessage::Exit).unwrap();
        self.join_handler.join().unwrap();
    }
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(thread_pool_size: usize) -> Self {
        let tasks = Arc::new(Mutex::new(VecDeque::new()));
        let workers = (0..thread_pool_size)
            .into_iter()
            .map(|_| PullWorker::new(tasks.clone()))
            .collect();
        Self {
            workers,
            tasks_queue: tasks,
        }
    }

    fn execute_task(&self, fun: impl FnOnce() + Send + 'static) {
        let mut mutex_guard = self.tasks_queue.lock().unwrap();
        (*mutex_guard).push_back(Box::new(fun));
    }

    fn stop_threads(self) {
        self.workers.into_iter().for_each(|w| w.stop());
    }
}
