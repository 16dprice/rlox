echo "building..."
RUSTFLAGS="-Awarnings" cargo build -q

echo "installing..."
sudo cp -r ./target/debug/rlox /bin/rlox-dbg

echo "done"
