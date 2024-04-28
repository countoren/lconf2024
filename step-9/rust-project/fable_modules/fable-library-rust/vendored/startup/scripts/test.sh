#!/nix/store/4vzal97iq3dmrgycj8r0gflrh51p8w1s-bash-5.2p26/bin/bash
# run like `bash scripts/test.sh`. not used in CI, but mimics it.

set -euvx

cargo run --manifest-path=testcases/smokebin/Cargo.toml --all-features
cargo build --manifest-path=testcases/dylib/Cargo.toml
cargo run --manifest-path=testcases/dylib_runner/Cargo.toml -- testcases/target/debug dylibtest
cargo build --manifest-path=testcases/paniclib/Cargo.toml

bash scripts/test_panic_afterbuild.sh
