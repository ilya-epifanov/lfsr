# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2020-03-30
### Added
* Direct lookup tables example
### Changed
* Renamed `lfsr_lookup!` to `searching_lfsr_lookup!`

## [0.2.0] - 2019-06-07
### Initial release
* 2-32 bit Galois LFSRs
* Macro to generate Galois XOR LFSR
* Macro to generate reverse lookup functions
* An object-safe LFSR trait
* A non-object-safe LFSRStatic trait for some static information about an LFSR
