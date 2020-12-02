#!/bin/zsh
set -e
output_dir="./target/release/deps"
input_llvm_filename="oxide_enzyme.ll"
processed_input_llvm_filename="oxide_enzyme_replaced.ll"
output_llvm_filename="processed_oxide_enzyme.ll"
output_dir_n_file=$output_dir/$input_llvm_filename
rm -f "./target/release/deps/oxide_enzyme.ll"
cargo rustc --release --bin oxide_enzyme -- --emit=llvm-ir || true
if [ ! -f $output_dir_n_file  ]; then
    echo "Compilation failed"
    exit 1
fi
echo "Copying!"
cp $output_dir_n_file .
cargo run --release --bin post
./opt $processed_input_llvm_filename -load=LLVMEnzyme-11.dylib -enzyme -enzyme-rust-type -enzyme-loose-types -o $output_llvm_filename -S
clang -m64 $output_llvm_filename -Wl,-dead_strip -nodefaultlibs -lSystem -lresolv -lc -lm $(find $HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib -name '*rlib') -o test.exec
chmod +x ./test.exec
./test.exec

# llvm-as oxide_enzyme_replaced.ll -o oxide_enzyme_replaced.bc