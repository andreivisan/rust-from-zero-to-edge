use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex}, task::{Context, Poll, Waker},
};

struct Shared<T> {
    queue: VecDeque<T>,
    closed: bool,
    waker: Option<Waker>,
}

struct MiniSender<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

struct MiniStream<T> {
    shared: Arc<Mutex<Shared<T>>>,
}

struct Next<'a, T> {
    stream: &'a mut MiniStream<T>,
}

impl<T> MiniSender<T> {
    fn send(&self, value: T) {
        let mut shared = self.shared.lock().expect("stream mutex poisoned");
        shared.queue.push_back(value);
        if let waker = Some(shared.waker.take()) { waker.wake(); }
    }
}

impl<T> Drop for MiniSender<T> {
    fn drop(&mut self) {
        let mut shared = self.shared.lock().expect("stream mutex poisoned");
        shared.closed = true;
    }
}

impl<T> MiniStream<T> {
    fn next(&mut self) -> Next<'_, T> {
        Next { stream: self }
    }
}

impl<'a, T> Future for Next<'a, T> {
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut next = self.stream.shared.lock().expect("stream mutex poisoned");
        match next.queue.pop_front() {
            Some(value) => { return Poll::Ready(Some(value)); },
            None => {
                if next.closed {
                    return Poll::Ready(None);
                }
                next.waker = Some(cx.waker().clone());
                return Poll::Pending;
            }
        }
    }
}

fn mini_stream<T>() -> (MiniSender<T>, MiniStream<T>) {
    let shared_queue = Arc::new(Mutex::new(Shared {
        queue: VecDeque::new(),
        closed: false,
        waker: None,
    }));
    let shared_queue_cloned = Arc::clone(&shared_queue);
    let mini_sender = MiniSender {
        shared: shared_queue,
    };
    let mini_stream = MiniStream {
        shared: shared_queue_cloned,
    };
    (mini_sender, mini_stream)
}

#[cfg(test)]
mod tests {
    use super::mini_stream;
    use std::thread;

    #[test]
    fn producer_thread_sends_non_clone_values_in_order_then_closes() {
        // No Clone or Copy: sending must transfer ownership of each message.
        struct Message(u32);

        let (sender, stream) = mini_stream();
        let producer = thread::spawn(move || {
            for number in 0..100 {
                sender.send(Message(number));
            }
            // Leaving the thread drops the sender and closes the stream.
        });

        producer.join().expect("producer thread panicked");

        // Until receiving is implemented, inspect the queue directly.
        let mut shared = stream.shared.lock().expect("stream mutex poisoned");
        assert!(shared.closed);
        for expected in 0..100 {
            let message = shared.queue.pop_front().expect("missing queued message");
            assert_eq!(message.0, expected);
        }
        assert!(shared.queue.is_empty());
    }

    #[test]
    fn empty_stream_stays_open_until_sender_is_dropped() {
        let (sender, stream) = mini_stream::<String>();

        {
            let shared = stream.shared.lock().expect("stream mutex poisoned");
            assert!(shared.queue.is_empty());
            assert!(!shared.closed);
        } // Unlock before dropping the sender, which needs the same mutex.

        drop(sender);

        let shared = stream.shared.lock().expect("stream mutex poisoned");
        assert!(shared.queue.is_empty());
        assert!(shared.closed);
    }
}
