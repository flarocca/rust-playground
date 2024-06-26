use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

enum Status {
    Pending,
    Ready,
}

struct State {
    status: Status,
    counter: i32,
    waker: Option<Waker>,
}

struct MyFuture {
    state: Arc<Mutex<State>>,
}

impl MyFuture {
    fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(State {
            status: Status::Pending,
            counter: 0,
            waker: None,
        }));

        let thread_state = state.clone();

        thread::spawn(move || {
            thread::sleep(duration);

            let mut state = thread_state.lock().unwrap();
            state.status = Status::Ready;

            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        MyFuture { state }
    }
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        println!("Poll {} times", state.counter);

        match state.status {
            Status::Pending => {
                state.waker = Some(cx.waker().clone());
                state.counter += 1;
                Poll::Pending
            }
            Status::Ready => Poll::Ready("We are done!!!".to_owned()),
        }
    }
}

#[tokio::main]
async fn main() {
    let fut = MyFuture::new(Duration::from_secs(5));
    let result = fut.await;

    println!("After {}", result);
}

// https://www.turing.com/interview-questions/rust, Q34
