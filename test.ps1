
echo ">> C"
cd c_simd
./build.ps1
./csimd.exe 100

echo ">> Rust"
cd ..
cd rust_simd
cargo run -- 100

echo ">> done"
cd ..

