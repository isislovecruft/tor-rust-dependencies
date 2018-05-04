#![feature(test)]

extern crate test;
extern crate rand;

const RAND_BENCH_N: u64 = 1000;

use test::{black_box, Bencher};

use rand::{SeedableRng, SmallRng, Rng, thread_rng};
use rand::seq::*;

#[bench]
fn misc_gen_bool(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    b.iter(|| {
        let mut accum = true;
        for _ in 0..::RAND_BENCH_N {
            accum ^= rng.gen_bool(0.18);
        }
        black_box(accum);
    })
}

#[bench]
fn misc_gen_bool_var(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    b.iter(|| {
        let mut p = 0.18;
        let mut accum = true;
        for _ in 0..::RAND_BENCH_N {
            accum ^= rng.gen_bool(p);
            p += 0.0001;
        }
        black_box(accum);
    })
}

#[bench]
fn misc_shuffle_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let x : &mut [usize] = &mut [1; 100];
    b.iter(|| {
        rng.shuffle(x);
        black_box(&x);
    })
}

#[bench]
fn misc_sample_iter_10_of_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_iter(&mut rng, x, 10).unwrap_or_else(|e| e));
    })
}

#[bench]
fn misc_sample_slice_10_of_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_slice(&mut rng, x, 10));
    })
}

#[bench]
fn misc_sample_slice_ref_10_of_100(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
    let x : &[usize] = &[1; 100];
    b.iter(|| {
        black_box(sample_slice_ref(&mut rng, x, 10));
    })
}

macro_rules! sample_indices {
    ($name:ident, $amount:expr, $length:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
            b.iter(|| {
                black_box(sample_indices(&mut rng, $length, $amount));
            })
        }
    }
}

sample_indices!(misc_sample_indices_10_of_1k, 10, 1000);
sample_indices!(misc_sample_indices_50_of_1k, 50, 1000);
sample_indices!(misc_sample_indices_100_of_1k, 100, 1000);

#[bench]
fn gen_1k_iter_repeat(b: &mut Bencher) {
    use std::iter;
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    b.iter(|| {
        let v: Vec<u64> = iter::repeat(()).map(|()| rng.gen()).take(128).collect();
        black_box(v);
    });
    b.bytes = 1024;
}

#[bench]
#[allow(deprecated)]
fn gen_1k_gen_iter(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    b.iter(|| {
        let v: Vec<u64> = rng.gen_iter().take(128).collect();
        black_box(v);
    });
    b.bytes = 1024;
}

#[bench]
fn gen_1k_sample_iter(b: &mut Bencher) {
    use rand::distributions::{Distribution, Standard};
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    b.iter(|| {
        let v: Vec<u64> = Standard.sample_iter(&mut rng).take(128).collect();
        black_box(v);
    });
    b.bytes = 1024;
}

#[bench]
fn gen_1k_fill(b: &mut Bencher) {
    let mut rng = SmallRng::from_rng(&mut thread_rng()).unwrap();
    let mut buf = [0u64; 128];
    b.iter(|| {
        rng.fill(&mut buf[..]);
        black_box(buf);
    });
    b.bytes = 1024;
}
