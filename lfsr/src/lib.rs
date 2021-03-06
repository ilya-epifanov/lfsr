#![no_std]
#![allow(clippy::needless_doctest_main)]
//! ```rust
//! use lfsr::{direct_lfsr_lookup, searching_lfsr_lookup};
//! use lfsr::galois;
//! use lfsr::LFSR;
//!
//! searching_lfsr_lookup!(galois32_lookup, galois::Galois32, 10, 20, 5);
//! searching_lfsr_lookup!(galois32_lookup_big, galois::Galois32, 99_999_000, 100_001_000, 100);
//! direct_lfsr_lookup!(galois16_direct_lookup, galois::Galois16);
//!
//! # fn main() {
//! // Searching lookup
//!
//! let mut lfsr = galois::Galois32::default();
//! assert_eq!(galois32_lookup(&lfsr), None);
//!
//! for _ in 0..9 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois32_lookup(&lfsr), None);
//!
//! for _ in 9..10 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois32_lookup(&lfsr), Some(10));
//!
//! for _ in 10..17 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois32_lookup(&lfsr), Some(17));
//!
//! for _ in 9..20 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois32_lookup(&lfsr), None);
//!
//! // Direct lookup
//!
//! let mut lfsr = galois::Galois16::default();
//! assert_eq!(galois16_direct_lookup(&lfsr), 0);
//!
//! for _ in 0..10 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois16_direct_lookup(&lfsr), 10);
//!
//! for _ in 10..11 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois16_direct_lookup(&lfsr), 11);
//!
//! for _ in 11..65534 {
//!     lfsr.inc();
//! }
//! assert_eq!(galois16_direct_lookup(&lfsr), 65534);
//!
//! // wrap around
//! lfsr.inc();
//! assert_eq!(galois16_direct_lookup(&lfsr), 0);
//! # }
//! ```

pub use lfsr_base::*;
pub use lfsr_instances::*;
pub use lfsr_macro_generate::galois_lfsr;
pub use lfsr_macro_lookup::{direct_lfsr_lookup, searching_lfsr_lookup};

#[cfg(test)]
mod tests {
    use super::*;

    searching_lfsr_lookup!(galois32_lookup, galois::Galois32, 10, 20, 5);
    direct_lfsr_lookup!(galois16_direct_lookup, galois::Galois16);

    #[test]
    fn lookup_out_of_range() {
        let mut lfsr = galois::Galois32::default();
        assert_eq!(galois32_lookup(&lfsr), None);

        for _ in 0..9 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), None);

        for _ in 9..20 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), None);
    }

    #[test]
    fn lookup_inside_range() {
        let mut lfsr = galois::Galois32::default();

        for _ in 0..10 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), Some(10));

        for _ in 10..11 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), Some(11));

        for _ in 11..15 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), Some(15));
        for _ in 15..19 {
            lfsr.inc();
        }
        assert_eq!(galois32_lookup(&lfsr), Some(19));
    }

    #[test]
    fn lookup_direct() {
        let mut lfsr = galois::Galois16::default();

        assert_eq!(galois16_direct_lookup(&lfsr), 0);

        for _ in 0..10 {
            lfsr.inc();
        }
        assert_eq!(galois16_direct_lookup(&lfsr), 10);

        for _ in 10..11 {
            lfsr.inc();
        }
        assert_eq!(galois16_direct_lookup(&lfsr), 11);

        for _ in 11..15 {
            lfsr.inc();
        }
        assert_eq!(galois16_direct_lookup(&lfsr), 15);

        for _ in 15..19 {
            lfsr.inc();
        }
        assert_eq!(galois16_direct_lookup(&lfsr), 19);

        for _ in 19..65534 {
            lfsr.inc();
        }
        assert_eq!(galois16_direct_lookup(&lfsr), 65534);

        // wrap around
        lfsr.inc();
        assert_eq!(galois16_direct_lookup(&lfsr), 0);
    }

    #[test]
    fn galois32_first_states() {
        let mut lfsr = galois::Galois32::default();
        assert_eq!(lfsr.state, 0b00000000000000000000000000000001);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b10100011000000000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b01010001100000000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00101000110000000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00010100011000000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00001010001100000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000101000110000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000010100011000000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000001010001100000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000101000110000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000010100011000000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000001010001100000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000101000110000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000010100011000000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000001010001100000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000101000110000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000010100011000000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000001010001100000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000101000110000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000010100011000000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000001010001100000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000000101000110000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000000010100011000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000000001010001100);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000000000101000110);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00000000000000000000000010100011);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b10100011000000000000000001010001);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b11110010100000000000000000101000);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b01111001010000000000000000010100);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00111100101000000000000000001010);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b00011110010100000000000000000101);
        lfsr.inc();
        assert_eq!(lfsr.state, 0b10101100001010000000000000000010);
    }
}
