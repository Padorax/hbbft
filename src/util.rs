//! Utility functions
//!
//! Functions not large enough to warrant their own crate or module, but flexible enough to be used
//! in multiple disjunct places in the library. May also contain backports, workarounds.

use std::fmt;

use hex_fmt::HexFmt;

/// Prints a byte slice as shortened hexadecimal in debug output.
pub fn fmt_hex<T: AsRef<[u8]>>(bytes: T, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:10}", HexFmt(bytes))
}

/// Given a number of nodes, returns the maximum number of faulty nodes that can be tolerated: the
/// greatest number less than one third of `n`.
#[inline]
pub fn max_faulty(n: usize) -> usize {
    (n - 1) / 3
}
