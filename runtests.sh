export RUSTFLAGS="-Clink-arg=-fuse-ld=gold"
cargo test --package CryptoFuzz --bin CryptoFuzz test_in_data -- --exact --nocapture
