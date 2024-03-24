# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [Unreleased] - yyyy-mm-dd

### Added

### Changed

### Removed

### Fixed

## [0.2.0] - 2024-03-25

### Added

- All the wrapper structures now implement `Deref` of the wrapped `leap-sys` structures
- All the reference wrapper structures now implement `Clone` and `Copy`
- The opt-in features `glam` and `nalgebra` add conversion method for vectors and quaternions to these libraries.

### Changed

- Improved the examples to avoid common timeout unwrap()
- The vectors and quaternion are not anymore copied
- **Breaking** All the non-owning wrappers (most of them) are now named with `Ref` as a suffix.
    `Hand` becomes `HandRef`, `Digit` becomes `DigitRef`, etc.

### Removed

- **Breaking** All the trivial accessor functions were removed, in favour of the `Deref` trait
  `vector.x()` becomes `vector.x`, `hand.pinch_distance()` becomes `hand.pinch_distance`, etc.

### Fixed

## [0.1.4] - 2023-12-28

### Added

- Palm direction vector

## [0.1.2] - 2023-09-23

### Changed

- Documentation and examples improvements

## [0.1.1] - 2023-01-30

### Added

- Linux support

## [0.1.0] - 2022-08-29

### Added

- Events and transforms
- Coverage for the *Ex() methods
- Tracking mode event

### Fixed

- Orion build

## [0.0.x] - 2022-07-xx

Initial versions.
