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
