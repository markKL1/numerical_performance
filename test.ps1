
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


## Show assembly
## make sure -Clink-arg=-s in .cargo/config IS NOT enabled (and ideally, -Ctarget-cpu=native IS enabled)
# cargo build --release ; objdump -dS .\target\release\rust_simd.exe > dump.asm

