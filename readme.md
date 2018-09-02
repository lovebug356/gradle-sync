# Gradle-Sync
[![Build Status](https://travis-ci.org/lovebug356/gradle-sync.svg?branch=master)](https://travis-ci.org/lovebug356/gradle-sync)
[![Latest version](https://img.shields.io/crates/v/gradle-sync.svg)](https://crates.io/crates/gradle-sync)

**Small utility to synchronize the gradle version with the cargo version.**

## Usage

Add build dependency in ```Cargo.toml```:

```toml
[build-dependencies]
gradle-sync = 0.1.4

```

Add the following code snipper to ```build.rs```:

```[rust]
extern crate gradle_sync;
use gradle_sync::GradleFile;

fn main() {
    GradleFile::new_and_sync_with_cargo("./app/build.gradle").unwrap();
}

```

## License

gradle-sync is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in gradle-sync by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
