# RustExercise
RustExercise

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustc main.rs
./main

# temp_convert example
git clone temp_convert
cd temp_convert

cargo init
cargo build
cargo run -- 100 C

cargo test