use std::thread::{self, ScopedJoinHandle};

const THRESHOLD: usize = 1234;

pub fn split_work<R, T, F>(mut input: Vec<T>, work: F) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Send + Copy,
{
    if input.len() <= THRESHOLD {
        return handle_vector(input, work);
    }

    let num_chunks = input.chunks(THRESHOLD).count();
    let mut input_chunks = Vec::with_capacity(num_chunks);

    for chunk in (0..num_chunks).rev() {
        let input_tail_subvector = input.split_off(THRESHOLD * chunk);
        input_chunks.push(input_tail_subvector);
    }

    let mut output_chunks: Vec<Vec<R>> = Vec::with_capacity(num_chunks);

    thread::scope(|scope| {
        let mut output: Vec<ScopedJoinHandle<Vec<R>>> = Vec::with_capacity(num_chunks);

        for _ in 0..num_chunks {
            let input_chunk = input_chunks
                .pop()
                .expect("Number of chunks in output vector must be the same as in input vector");

            output.push(scope.spawn(move || handle_vector(input_chunk, work)));
        }

        output_chunks = output
            .into_iter()
            .map(|thread_h| {
                thread_h.join()
                    .expect("No ideas yet how to handle thread panic here")
            })
            .collect();
    });

    output_chunks.into_iter().flatten().collect()
}

fn handle_vector<R, T, F>(vector: Vec<T>, handle: F) -> Vec<R>
where
    F: Fn(T) -> R,
{
    vector.into_iter().map(|val| handle(val)).collect()
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
