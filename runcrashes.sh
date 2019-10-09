export RUSTFLAGS="-Clink-arg=-fuse-ld=gold"
cargo test --package CryptoFuzz --bin CryptoFuzz evalute_crashes -- --exact --nocapture
