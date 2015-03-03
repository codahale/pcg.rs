# pcg
[![Docs](http://img.shields.io/badge/godoc-reference-blue.svg)](http://codahale.github.io/pcg/pcg/)
[![Build Status](https://travis-ci.org/codahale/pcg.svg)](https://travis-ci.org/codahale/pcg)
[![Apache V2 License](http://img.shields.io/badge/license-Apache%20V2-blue.svg)](https://github.com/codahale/pcg/blob/master/LICENSE)

A Rust implementation of the [PCG](http://www.pcg-random.org) PRNG.

The PCG algorithm is not suitable for cryptographic purposes but
provides an excellent combination of speed and unpredictability. It is
slightly faster than `rand::XorShiftRng` and provides much
higher-quality output.

This particular implementation uses a 128-bit state value, has a period
of 2^64, and uses the `XSH-RR` output function.
