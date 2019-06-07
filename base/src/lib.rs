#![no_std]

/// An object-safe part of the LFSR trait that allows to count up, down and get a current state
pub trait LFSR {
    /** Retrieves the current state of the LFSR */
    fn get_state(&self) -> u32;
    /** Count up */
    fn inc(&mut self);
    /** Count down */
    fn dec(&mut self);
}

/// A non-object-safe part of an LFSR
pub trait LFSRStatic {
    /** Sequence length of this LFSR */
    fn sequence_length() -> u32;
}
