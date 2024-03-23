# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [Unreleased] - yyyy-mm-dd

### Added

- All the wrapper structures now implement `Deref` of the wrapped `leap-sys` structures

### Changed

- Improved the examples to avoid common timeout unwrap()
- The vectors and quaternion are not anymore copied

### Removed

- **Breaking** All the trivial accessor functions were removed, in favour of the `Deref` trait.

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
