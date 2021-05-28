/// When `concurrent` feature is enabled, creates a parallel iterator; otherwise, creates a
/// regular iterator. Optionally, `min_length` can be used to specify the minimum length of
/// iterator to be processed in each thread.
/// Adapted from: https://github.com/arkworks-rs/utils/blob/master/src/lib.rs
#[macro_export]
macro_rules! iter {
    ($e: expr) => {{
        #[cfg(feature = "concurrent")]
        let result = $e.par_iter();

        #[cfg(not(feature = "concurrent"))]
        let result = $e.iter();

        result
    }};
    ($e: expr, $min_len: expr) => {{
        #[cfg(feature = "concurrent")]
        let result = $e.par_iter().with_min_len($min_len);

        #[cfg(not(feature = "concurrent"))]
        let result = $e.iter();

        result
    }};
}

/// When `concurrent` feature is enabled, creates a mutable parallel iterator; otherwise,
/// creates a regular iterator. Optionally, `min_length` can be used to specify the minimum
/// length of iterator to be processed in each thread.
/// Adapted from: https://github.com/arkworks-rs/utils/blob/master/src/lib.rs
#[macro_export]
macro_rules! iter_mut {
    ($e: expr) => {{
        #[cfg(feature = "concurrent")]
        let result = $e.par_iter_mut();

        #[cfg(not(feature = "concurrent"))]
        let result = $e.iter_mut();

        result
    }};
    ($e: expr, $min_len: expr) => {{
        #[cfg(feature = "concurrent")]
        let result = $e.par_iter_mut().with_min_len($min_len);

        #[cfg(not(feature = "concurrent"))]
        let result = $e.iter_mut();

        result
    }};
}

/// When `concurrent` feature is enabled, breaks the slice into batches and processes each
/// batch in a separate thread; otherwise, the entire slice is processed as a single batch
/// in one thread. Optionally, `min_batch_size` can be used to specify the minimum size of
/// the resulting batches.
#[macro_export]
macro_rules! batch_iter_mut {
    ($e: expr, $c: expr) => {
        #[cfg(feature = "concurrent")]
        {
            let batch_size = $e.len() / rayon::current_num_threads().next_power_of_two();
            if batch_size < 1 {
                $c($e, 0);
            }
            else {
                $e.par_chunks_mut(batch_size).enumerate().for_each(|(i, batch)| {
                    $c(batch, i * batch_size);
                });
            }
        }

        #[cfg(not(feature = "concurrent"))]
        $c($e, 0);
    };
    ($e: expr, $min_batch_size: expr, $c: expr) => {
        #[cfg(feature = "concurrent")]
        {
            let batch_size = $e.len() / rayon::current_num_threads().next_power_of_two();
            if batch_size < $min_batch_size {
                $c($e, 0);
            }
            else {
                $e.par_chunks_mut(batch_size).enumerate().for_each(|(i, batch)| {
                    $c(batch, i * batch_size);
                });
            }
        }

        #[cfg(not(feature = "concurrent"))]
        $c($e, 0);
    };
}
