use std::future::Future;

use tokio::runtime::Handle;

pub fn run_async<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    let handle = Handle::current();

    std::thread::spawn(move || handle.block_on(future))
        .join()
        .unwrap()
}
