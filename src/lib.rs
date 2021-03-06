use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// The deque wraps around, an implementation with modulo pops up in mind, where we update how the deque wraps
// around cyclically while removing elements from it.
// Contrast this with using a vec where you need to resize and fill the holes of a removed object.

// ARC: You can clone arc and there's still only one instance of the thing that's inside.
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            // I only want to clone the Arc, and not what's inside Arc, which may happen if naively implementing clone for inner which may clone whatever inner is.
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    // Holds the data that is shared. The things on the channel.
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}
