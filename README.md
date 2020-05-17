# %CUSTOM_PLUGIN_SERVICE_NAME%

[![pipeline status][pipeline]][git-link]
[![coverage report][coverage]][git-link]

## Summary

Welcome to Rust Hello World application. This example allows you to become familiar with Rust and Mia-Platform.

## Local Development

Install [rust](https://www.rust-lang.org/tools/install).

### Test
```bash
cargo check
cargo test -- --nocapture
```

### Coverage

Install [grcov](https://github.com/mozilla/grcov)
```bash
cargo install grcov
```

Set up environment:
```bash
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
```

Calculate coverage
```bash
cargo +nightly build
cargo +nightly test
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
open target/debug/coverage/index.html
```

### Build
```bash
cargo build
cargo build --release
```

### Run

```bash
target/debug/rust-hello-world
```

and go to [http://localhost:3000/]()

If you want to setup env vars justi change file *exportenv.sh* and execute

```bash
. ./exportenv.sh
```

before launching the *rust-hello-world* application.

---------------------------

[pipeline]: https://git.tools.mia-platform.eu/clients/mia-platform/demo/services/rust-hello-world/badges/master/pipeline.svg
[coverage]: https://git.tools.mia-platform.eu/clients/mia-platform/demo/services/rust-hello-world/badges/master/coverage.svg
[git-link]: https://git.tools.mia-platform.eu/clients/mia-platform/demo/services/rust-hello-world/commits/master
