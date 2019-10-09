export RUSTFLAGS="-Clink-arg=-fuse-ld=gold"
cargo afl build
