
$N=1000

echo ">> Rust naive"
cd "$DIR/rust_0naive"
cargo run -q --release -- $N
echo ""

echo ">> Rust index"
cd "$DIR/rust_1index"
cargo run -q --release -- $N
echo ""

echo ">> Rust unroll"
cd "$DIR/rust_2unroll"
cargo run -q --release -- $N
echo ""

echo ">> Rust layout"
cd "$DIR/rust_3layout"
cargo run -q --release -- $N
echo ""

echo ">> Rust indep"
cd "$DIR/rust_4indep"
cargo run -q --release -- $N
echo ""

echo ">> Rust simd"
cd "$DIR/rust_5simd"
cargo run -q --release -- $N
echo ""

echo ">> Rust parallel"
cd "$DIR/rust_6par"
cargo run -q --release -- $N
echo ""

cd "$PSScriptRoot"
echo ">> done"


## Show assembly
## make sure -Clink-arg=-s in .cargo/config IS NOT enabled (and ideally, -Ctarget-cpu=native IS enabled)
# cargo build --release ; objdump -dS .\target\release\rust_simd.exe > dump.asm

