
$N=1000

echo ">> C"
cd c_simd
./build.ps1
./csimd.exe $N
echo ""

echo ">> Rust"
cd ..
cd rust_simd
cargo run -q --release -- $N >$null
echo ""

echo ">> done"
cd ..

