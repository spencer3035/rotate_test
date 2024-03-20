// Get vectors for benchmarking/testing
pub fn get_vectors() -> Vec<Vec<usize>> {
    vec![
        (1..100).into_iter().collect(),
        (1..1000).into_iter().collect(),
    ]
}

// Remainder method benchmarks to be faster than std methods? Even though it uses one
#[inline]
pub fn rotate_rem_euclid<T>(slice: &mut [T], rot: isize) {
    if slice.is_empty() {
        return;
    }
    slice.rotate_right(rot.rem_euclid(slice.len() as isize) as usize)
}

#[inline]
pub fn rotate_right_std<T>(slice: &mut [T], rot: usize) {
    slice.rotate_right(rot);
}

#[inline]
pub fn rotate_left_std<T>(slice: &mut [T], rot: usize) {
    slice.rotate_left(rot);
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fmt::Debug;

    fn rotate_all_right<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], usize))
    where
        T: Clone + Debug + PartialEq,
    {
        for vec in vectors.iter_mut() {
            for ii in 0..vec.len() {
                let mut vec_before = vec.to_vec();
                vec_before.rotate_right(ii);
                rotate_fn(vec, ii);
                assert_eq!(vec_before, *vec);
            }
        }
    }

    fn rotate_all_left<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], usize))
    where
        T: Clone + Debug + PartialEq,
    {
        for vec in vectors.iter_mut() {
            for ii in 0..vec.len() {
                let mut vec_before = vec.to_vec();
                vec_before.rotate_left(ii);
                rotate_fn(vec, ii);
                assert_eq!(vec_before, *vec);
            }
        }
    }

    fn rotate_all<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], isize))
    where
        T: Clone + Debug + PartialEq,
    {
        // Right
        for vec in vectors.iter_mut() {
            for ii in 0..vec.len() {
                let mut vec_before = vec.to_vec();
                vec_before.rotate_right(ii);
                rotate_fn(vec, ii as isize);
                assert_eq!(vec_before, *vec);
            }
        }
        // Left
        for vec in vectors.iter_mut() {
            for ii in 0..vec.len() {
                let mut vec_before = vec.to_vec();
                vec_before.rotate_left(ii);
                rotate_fn(vec, ii as isize * -1);
                assert_eq!(vec_before, *vec);
            }
        }
    }

    #[test]
    fn test_rotate() {
        let mut vectors = get_vectors();
        rotate_all_right(&mut vectors, rotate_right_std);
        rotate_all_left(&mut vectors, rotate_left_std);

        rotate_all(&mut vectors, rotate_rem_euclid);
    }
}
