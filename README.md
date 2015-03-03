# pcg

A Rust implementation of the [PCG](http://www.pcg-random.org) PRNG.

The PCG algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability. It is
slightly faster than `rand::XorShiftRng` and provides much
higher-quality output.

This particular implementation uses a 128-bit state value, has a period
of 2^64, and uses the `XSH-RR` output function.
