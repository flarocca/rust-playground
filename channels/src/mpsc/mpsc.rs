use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{Arc, Condvar, Mutex},
};

/// Sender
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(value);

        drop(inner);

        self.shared.receivers_available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;

        let senders = inner.senders;

        drop(inner);

        if senders == 0 {
            self.shared.receivers_available.notify_one();
        }
    }
}

/// SyncSender
pub struct SyncSender<T> {
    shared: Arc<Shared<T>>,
    capacity: usize,
}

impl<T: Debug> SyncSender<T> {
    pub fn send(&mut self, value: T) {
        loop {
            let mut inner = self.shared.inner.lock().unwrap();
            if inner.queue.len() == self.capacity {
                println!("<<< Queue is full {:?} >>>", value);
                let _unused = self.shared.capacity_available.wait(inner).unwrap();
            } else {
                println!("<<< Queue has space {:?} >>>", value);
                inner.queue.push_back(value);

                drop(inner);

                self.shared.receivers_available.notify_one();
                break;
            }
        }
    }
}

impl<T> Clone for SyncSender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Self {
            shared: Arc::clone(&self.shared),
            capacity: self.capacity,
        }
    }
}

impl<T> Drop for SyncSender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;

        let senders = inner.senders;

        drop(inner);

        if senders == 0 {
            self.shared.receivers_available.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> Option<T> {
        if let Some(data) = self.buffer.pop_front() {
            return Some(data);
        }

        loop {
            let mut inner = self.shared.inner.lock().unwrap();
            match inner.queue.pop_front() {
                Some(data) => {
                    if inner.queue.is_empty() == false {
                        std::mem::swap(&mut inner.queue, &mut self.buffer);
                        self.shared.capacity_available.notify_all();
                    }
                    return Some(data);
                }
                None if inner.senders == 0 => return None,
                None => {
                    let _unused = self.shared.receivers_available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receive()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    receivers_available: Condvar,
    capacity_available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner::<T> {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared::<T> {
        inner: Mutex::new(inner),
        receivers_available: Condvar::default(),
        capacity_available: Condvar::default(),
    };
    let shared = Arc::new(shared);

    (
        Sender::<T> {
            shared: shared.clone(),
        },
        Receiver::<T> {
            shared,
            buffer: VecDeque::default(),
        },
    )
}

pub fn sync_channel<T>(capacity: usize) -> (SyncSender<T>, Receiver<T>) {
    let inner = Inner::<T> {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared::<T> {
        inner: Mutex::new(inner),
        receivers_available: Condvar::default(),
        capacity_available: Condvar::default(),
    };
    let shared = Arc::new(shared);

    (
        SyncSender::<T> {
            shared: shared.clone(),
            capacity,
        },
        Receiver::<T> {
            shared,
            buffer: VecDeque::default(),
        },
    )
}
