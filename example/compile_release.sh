#!/bin/zsh
set -e
output_dir="../target/release/deps"
input_llvm_filename="example.ll"
processed_input_llvm_filename="example_replaced.ll"
output_llvm_filename="final_example.ll"
output_dir_n_file=$output_dir/$input_llvm_filename
#rm -f "../target/release/deps/example.ll"
cargo rustc --release -- --emit=llvm-ir -C embed-bitcode=yes -C lto=fat || true
if [ ! -f $output_dir_n_file  ]; then
    echo "Compilation failed"
    exit 1
fi
echo "Copying!"
cp $output_dir_n_file .
echo "Success (ignore Rust error above)!"
#clang -m64 $output_llvm_filename -Wl,-dead_strip -nodefaultlibs -lSystem -lresolv -lc -lm $(find $HOME/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib -name '*rlib') -o test.exec
#chmod +x ./test.exec
#./test.exec

# llvm-as example_replaced.ll -o example_replaced.bc