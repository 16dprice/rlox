RUSTFLAGS="-Awarnings" cargo build -q
sudo cp -r ./target/debug/rlox /bin/rlox-dbg
