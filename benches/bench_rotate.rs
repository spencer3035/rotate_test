use criterion::*;

use rotate_test::*;

use std::fmt::Debug;

// Constant ranges to increase benchmark reliability
const URANGE: std::ops::Range<usize> = 0..100;
const IRANGE: std::ops::Range<isize> = -50..50;

#[inline]
fn rotate_all_right<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], usize))
where
    T: Clone + Debug + PartialEq,
{
    for vec in vectors.iter_mut() {
        for ii in URANGE {
            rotate_fn(vec, ii);
        }
    }
}

#[inline]
fn rotate_all_left<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], usize))
where
    T: Clone + Debug + PartialEq,
{
    for vec in vectors.iter_mut() {
        for ii in URANGE {
            rotate_fn(vec, ii);
        }
    }
}

#[inline]
fn rotate_all<T>(vectors: &mut Vec<Vec<T>>, rotate_fn: fn(&mut [T], isize))
where
    T: Clone + Debug + PartialEq,
{
    // Right
    for vec in vectors.iter_mut() {
        for ii in IRANGE {
            rotate_fn(vec, ii as isize);
        }
    }
    // Left
    for vec in vectors.iter_mut() {
        for ii in IRANGE {
            rotate_fn(vec, ii as isize * -1);
        }
    }
}
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut vectors = get_vectors();
    // Sanity check to make sure the benchmarks do the same indexes
    assert_eq!(URANGE.len(), IRANGE.len());

    c.bench_function("rotate std", |b| {
        b.iter(|| {
            rotate_all_left(&mut vectors, rotate_left_std);
            rotate_all_right(&mut vectors, rotate_right_std);
        })
    });
    c.bench_function("rotate wrapping", |b| {
        b.iter(|| rotate_all(&mut vectors, rotate_rem_euclid))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
