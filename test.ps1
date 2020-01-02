
$N=900

echo ">> C"
cd "$PSScriptRoot/c_simd"
./build.ps1
./csimd.exe $N
echo ""

echo ">> Rust naive"
cd "$PSScriptRoot/rust_naive"
cargo run -q --release -- $N
echo ""

echo ">> Rust simd"
cd "$PSScriptRoot/rust_opt"
cargo run -q --release -- $N
echo ""

cd "$PSScriptRoot"
echo ">> done"
