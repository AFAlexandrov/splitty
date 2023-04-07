use std::thread;

const THRESHOLD: usize = 1234;

pub fn split_work<R, T, F>(input: Vec<T>, handle: F) -> Vec<R>
where
    R: Sync + Send + Default + Clone,
    T: Sync + Send + Copy,
    F: Fn(T) -> R + Sync + Send + Copy,
{
    if input.len() <= THRESHOLD {
        return input.into_iter().map(|val| handle(val)).collect();
    }

    let mut result: Vec<R> = vec![R::default(); input.len()];

    thread::scope(|scope| {
        let mut result_chunks = result.chunks_mut(THRESHOLD);

        for input_chunk in input.chunks(THRESHOLD) {
            let result_chunk = result_chunks
                .next()
                .expect("Number of chunks in result vector must be the same as in input vector");

            scope.spawn(move || {
                handle_chunk(result_chunk, input_chunk, handle);
            });
        }
    });

    result
}

fn handle_chunk<R, T, F>(slice: &mut [R], input: &[T], handle: F)
where
    T: Copy,
    F: Fn(T) -> R,
{
    for (elt, result) in slice.iter_mut().enumerate() {
        *result = handle(input[elt]);
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
