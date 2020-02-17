#!/usr/bin/env bash

N=1000
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

#echo ">> C"
#cd "$DIR/c_simd"
#./build.ps1
#./csimd.exe $N
#echo ""

echo ">> Rust naive"
cd "$DIR/rust_0naive"
cargo run -q --release -- $N
echo ""

echo ">> Rust index"
cd "$DIR/rust_1index"
cargo run -q --release -- $N
echo ""

echo ">> Rust opt"
cd "$DIR/rust_2unroll"
cargo run -q --release -- $N
echo ""

echo ">> Rust simd"
cd "$DIR/rust_3simd"
cargo run -q --release -- $N
echo ""

echo ">> Rust parallel"
cd "$DIR/rust_4par"
cargo run -q --release -- $N
echo ""

#echo ">> Java (parallel)"
#cd "$DIR/java_jni"
#mvn package -DskipTests -q
#Measure-Command { java -jar .\target\JniRustTest-development.jar $N }
#echo ""

cd "$DIR"
echo ">> done"


## Show assembly
## make sure -Clink-arg=-s in .cargo/config IS NOT enabled (and ideally, -Ctarget-cpu=native IS enabled)
# cargo build --release ; objdump -dS .\target\release\rust_simd.exe > dump.asm



