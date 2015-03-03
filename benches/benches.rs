#![feature(test)]

extern crate pcg;
extern crate rand;
extern crate test;

use pcg::PcgRng;
use test::Bencher;
use rand::Rng;

#[bench]
fn next_u32(b: &mut Bencher) {
    let mut rng = PcgRng::new_unseeded();

    b.iter(|| {
        rng.next_u32()
    })
}

#[bench]
fn fill_bytes(b: &mut Bencher) {
    b.bytes = 1024*1024;
    let mut rng = PcgRng::new_unseeded();

    let mut x = vec![0; b.bytes as usize];

    b.iter(|| {
        rng.fill_bytes(x.as_mut_slice())
    })
}
