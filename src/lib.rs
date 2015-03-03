//! An implementation of the [PCG](http://www.pcg-random.org) random number
//! generator, which is only slightly slower than `rand::XorShiftRng` while
//! providing higher-quality output.
//!
//! ```ignore
//! use rand::{Rng, SeedableRng};
//! use pcg::PcgRng;
//!
//! let mut rng = PcgRng::from_seed([100, 200]);
//!
//! let x = rng.gen_range(10, 100);
//!
//! assert!(x >= 10);
//! assert!(x <= 100);
//!
//! ```

extern crate rand;

mod pcg;

pub use pcg::PcgRng;
