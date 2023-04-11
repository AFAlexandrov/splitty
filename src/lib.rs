use std::thread;

const THRESHOLD: usize = 1234;

pub fn split_work<R, T, F>(mut input: Vec<T>, handle: F) -> Vec<R>
where
    T: Send,
    R: Send + Default + Clone,
    F: Fn(T) -> R + Send + Copy,
{
    if input.len() <= THRESHOLD {
        return input.into_iter().map(|val| handle(val)).collect();
    }

    let mut result: Vec<R> = vec![R::default(); input.len()];

    let num_chunks = input.chunks(THRESHOLD).count();
    let mut input_chunks = Vec::with_capacity(num_chunks);

    for chunk in (1..=num_chunks).rev() {
        let tail_subvector = input.split_off(THRESHOLD * (chunk - 1));
        input_chunks.push(tail_subvector);
    }

    thread::scope(|scope| {
        let mut result_chunks = result.chunks_mut(THRESHOLD);

        for _ in 0..num_chunks {
            let result_chunk = result_chunks
                .next()
                .expect("Number of chunks in result vector must be the same as in input vector");
            let input_chunk = input_chunks.pop().unwrap(); // same expectation as above

            scope.spawn(move || {
                handle_vec(result_chunk, input_chunk, handle);
            });
        }
    });

    result
}

fn handle_vec<R, T, F>(slice: &mut [R], mut input: Vec<T>, handle: F)
where
    F: Fn(T) -> R,
{
    for result in slice.iter_mut() {
        *result = handle(input.remove(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_result(input_len: i32) {
        let input: Vec<i32> = (0..input_len).collect();
        let direct_transform = |elt: i32| elt * 7;
        let reverse_transform = |elt: i32| elt / 7;

        let result = split_work(input.clone(), direct_transform);

        assert_eq!(
            input.len(),
            result.len(),
            "invalid len of the result vector"
        );

        for (idx, elt) in input.into_iter().enumerate() {
            assert_eq!(
                elt,
                reverse_transform(result[idx]),
                "invalid result of the work"
            );
        }
    }

    #[test]
    fn differrent_vector_len_test() {
        let input_len = THRESHOLD as i32;
        validate_result(0);
        validate_result(input_len / 2);
        validate_result(input_len);
        validate_result(input_len + 1);
        validate_result(input_len * 2);
        validate_result(input_len * 800);
        validate_result(input_len * 12345 + 1);
    }
}
