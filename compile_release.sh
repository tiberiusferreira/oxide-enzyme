#!/bin/zsh
set -e
output_dir="./target/release/deps"
output_file="oxide_enzyme.ll"
output_dir_n_file=$output_dir/$output_file
rm -f "./target/release/deps/oxide_enzyme.ll"
cargo rustc --release -- --emit=llvm-ir || true
if [ ! -f $output_dir_n_file  ]; then
    echo "Compilation failed"
    exit 1
fi
cp $output_dir_n_file .
opt ${output_dir}/oxide_enzyme.ll -load=LLVMEnzyme-11.dylib -enzyme -o $output_file -S
clang -m64 $output_file -Wl,-dead_strip -nodefaultlibs -lSystem -lresolv -lc -lm $(find /Users/tiberiodarferreira/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib -name '*rlib') -o test.exec
./test.exec