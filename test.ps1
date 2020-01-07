
$N=1000

echo ">> C"
cd "$PSScriptRoot/c_simd"
./build.ps1
./csimd.exe $N
echo ""

echo ">> Rust naive"
cd "$PSScriptRoot/rust_0naive"
cargo run -q --release -- $N
echo ""

echo ">> Rust opt"
cd "$PSScriptRoot/rust_1opt"
cargo run -q --release -- $N
echo ""

echo ">> Rust simd"
cd "$PSScriptRoot/rust_2simd"
cargo run -q --release -- $N
echo ""

echo ">> Rust parallel"
cd "$PSScriptRoot/rust_3par"
cargo run -q --release -- $N
echo ""

echo ">> Java (parallel)"
cd "$PSScriptRoot/java_jni"
mvn package -DskipTests -q
java -jar .\target\JniRustTest-development.jar $N
echo ""

cd "$PSScriptRoot"
echo ">> done"


## Show assembly
## make sure -Clink-arg=-s in .cargo/config IS NOT enabled (and ideally, -Ctarget-cpu=native IS enabled)
# cargo build --release ; objdump -dS .\target\release\rust_simd.exe > dump.asm

