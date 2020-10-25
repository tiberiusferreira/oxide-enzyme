#!/bin/zsh
set -e
output_dir="./target/debug/deps"
rm -f "./target/debug/deps/oxide_enzyme.ll"
cargo rustc -- --emit=llvm-ir || true
if [ ! -f "./target/debug/deps/oxide_enzyme.ll"  ]; then
    echo "Compilation failed"
    exit 1
fi
opt ${output_dir}/oxide_enzyme.ll -load=LLVMEnzyme-11.dylib -enzyme -o ${output_dir}/output.ll -S
clang -m64 ${output_dir}/output.ll -Wl,-dead_strip -nodefaultlibs -lSystem -lresolv -lc -lm $(find /Users/tiberiodarferreira/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib -name '*rlib') -o test.exec
./test.exec