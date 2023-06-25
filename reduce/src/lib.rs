use std::num::NonZeroUsize;
use std::sync::mpsc;
use std::thread;


pub fn parallel_foldl<T, U>(
    s: &[T],
    init: U,
    f: fn(U, &T) -> U,
    r: fn(U, U) -> U
) -> U
where
    T: Sync,
    U: Copy + Send
{
    // Default to 16 units of parallelism if can't be queried.
    let parallelism =
        thread::available_parallelism().unwrap_or(
            unsafe{ NonZeroUsize::new_unchecked(16) }
        );
    // Calculate minimum number of elements to process per thread.
    let min_elem_per_thread = s.len() / parallelism;
    // Calculate number of elements that aren't evenly distributed.
    let remainder = s.len() % parallelism;
    // Create transmitter and receiver for communication.
    let (tx, rx) = mpsc::channel();

    thread::scope(|scope| {

        for thread_id in 0..parallelism.get() {

            // Calculate base index for thread-local fold.
            let base_index = thread_id * min_elem_per_thread;
            // Calculate the bounding index for thread-local fold.
            let bounding_index = base_index + min_elem_per_thread;
            // Clone sender so that it can be moved into closure.
            let tx = tx.clone();

            scope.spawn(move || {
                tx.send(
                    s[base_index..bounding_index]
                        .iter().fold(init, f)
                )
                .expect("Receiver has been dropped.");
            });
        }
    });

    // Drop the last sender to stop `rx` waiting for message.
    drop(tx);
    // Combine thread-local folds with reducing function `r`.
    rx.iter().reduce(r).unwrap()
}