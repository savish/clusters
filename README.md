# Clusters

[![Build Status](https://travis-ci.com/savish/clusters.svg?branch=master)](https://travis-ci.com/savish/clusters)

Useful traits for clustering algorithms.

This library can be used as a basis for implementing a wide variety of 'distance' based clustering algorithms, such as [DBSCAN](1).

## Usage

The library is written entirely in Rust. To use it in your project, add the latest stable version of this crate to your `Cargo.toml` dependencies.

**Cargo.toml**

```toml
[dependencies]
clusters = "0.1"
```

In your project, import the library using the following code:

```rust
extern crate clusters;
use clusters::{Algorithm, Proximity}
```

Note that the above code makes the `Algorithm` and `Proximity` traits available for usage in the given scope, without needing the `clusters::` qualification. For instance:

```rust
struct DBSCAN { ... }

impl Algorithm for DBSCAN {...}
```

## Examples

**TODO**

## Tests

**TODO**

## Versioning

This project uses SemVer for versioning. For the versions available, see the releases tagged on this repository.

## Authors

_Primary:_ Alan K <mailto:afksavish@gmail.com> @savish

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Contributing

Please see [CONTRIBUTING.md](2) for the process of contributing to this repo.

[1]: https://en.wikipedia.org/wiki/DBSCAN
[2]: ./CONTRIBUTING.md
