//! An implementation of the [PCG](http://www.pcg-random.org) random number
//! generator.

extern crate rand;

mod pcg;

pub use pcg::PcgRng;
