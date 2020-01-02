
$N=750

echo ">> C"
cd c_simd
./build.ps1
./csimd.exe $N
echo ""

echo ">> Rust naive"
cd ..
cd rust_naive
cargo run -q --release -- $N
echo ""

echo ">> Rust simd"
cd ..
cd rust_simd
cargo run -q --release -- $N
echo ""

echo ">> done"
cd ..

