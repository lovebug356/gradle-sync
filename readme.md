# Gradle-Sync
[![Build Status](https://travis-ci.org/lovebug356/gradle-sync.svg?branch=master)](https://travis-ci.org/lovebug356/gradle-sync)
[![Build status](https://ci.appveyor.com/api/projects/status/86s3teekhbj2a25h?svg=true)](https://ci.appveyor.com/project/lovebug356/gradle-sync)
[![codecov](https://codecov.io/gh/lovebug356/gradle-sync/branch/master/graph/badge.svg)](https://codecov.io/gh/lovebug356/gradle-sync)
[![Latest version](https://img.shields.io/crates/v/gradle-sync.svg)](https://crates.io/crates/gradle-sync)

**Small utility to synchronize the gradle version with the cargo version.**

## Usage

First, add build dependency in ```Cargo.toml```:

```toml
[build-dependencies]
gradle-sync = "0.1.4"
```

and secondly, add the following code snippet to ```build.rs``` (with a reference to the ```build.gradle``` file):

```rust
extern crate gradle_sync;
use gradle_sync::BuildGradleFile;
use gradle_sync::GradlePropertiesFile;

fn main() {
    BuildGradleFile::new("./app/build.gradle").unwrap()
      .sync_with_cargo().unwrap();
    GradlePropertiesFile::new("./gradle.properties").unwrap()
      .sync_with_cargo().unwrap();
}

```

When you now build your project, the version specified in ```Cargo.toml``` file is used as ```versionName``` in the ```build.gradle``` file. If required, the ```versionCode``` is also incremented.

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
