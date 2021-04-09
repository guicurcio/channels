use std::sync::{Arc, Mutex};

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}
pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

struct Inner<T> {
    // Holds the data that is shared. The things on the channel.
    queue: Vec<T>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner { queue: Vec::new() };
    let inner = Arc::new(Mutex::new(inner));
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}
