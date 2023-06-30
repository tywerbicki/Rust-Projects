use std::num::NonZeroUsize;
use std::sync::mpsc;
use std::thread;

// Default to 16 units of parallelism if can't be queried.
const DEFAULT_PARALLELISM: NonZeroUsize =
    unsafe{ NonZeroUsize::new_unchecked(16) };

/// Performs a parallel foldl over `s`.
/// 
/// Each thread performs a thread-local foldl over a portion of `s`
/// using folding function `f`. The results of the thread-local foldls
/// are then aggregated using reducing function `r`.
/// 
/// # Examples
/// 
/// ```
/// use reduce;
/// use std::vec::Vec;
/// 
/// let x = Vec::from_iter(0..100);
/// let a = 2;
/// let b = 8;
/// 
/// let ser_sum_ax_b: u32 =
///     x.iter().map(|x| (a*x) + b).sum();
/// 
/// let par_sum_ax_b = reduce::parallel_foldl(
///     &x,
///     0,
///     |f, g| f + (a*g) + b,
///     |f, g| f + g
/// );
/// 
/// assert_eq!(ser_sum_ax_b, par_sum_ax_b);
/// 
/// ```
/// 
pub fn parallel_foldl<T, U, F, R>(
    s: &[T],
    init: U,
    f: F,
    r: R
) -> U
where
    T: Sync,
    U: Copy + Send,
    F: Fn(U, &T) -> U + Copy + Send,
    R: FnMut(U, U) -> U
{
    let parallelism =
        thread::available_parallelism().unwrap_or(DEFAULT_PARALLELISM);
    // Calculate minimum number of elements to process per thread.
    let min_elem_per_thread = s.len() / parallelism;
    // Calculate number of elements that aren't evenly distributed.
    let remainder = s.len() % parallelism;
    // Create transmitter and receiver for communication.
    let (tx, rx) = mpsc::channel();

    thread::scope(|scope| {

        for thread_id in 0..parallelism.get() {

            // Clone sender so that it can be moved into closure.
            let tx = tx.clone();

            scope.spawn(move || {

                // Calculate base index for thread-local fold.
                let base_index =
                    thread_id * (min_elem_per_thread + 1) -
                    thread_id.saturating_sub(remainder);
                // Calculate the bounding index for thread-local fold.
                let bounding_index =
                    base_index + min_elem_per_thread +
                    ((thread_id < remainder) as usize);

                tx.send(
                    s[base_index..bounding_index]
                        .iter()
                        .fold(init, f)
                )
                .expect("Receiver has been dropped.");
            });
        }
    });

    // Drop the last sender to stop `rx` waiting for messages.
    drop(tx);
    // Combine thread-local folds with reducing function `r`.
    unsafe{ rx.iter().reduce(r).unwrap_unchecked() }
}