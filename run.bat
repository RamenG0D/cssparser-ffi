@REM build the library (statically)
cargo build --release

@REM generate the header needed to use the library
cargo test gen_headers --features headers

@REM build the cpp example
clang++ -o ./target/release/csstest.exe ./test/test.cpp -I./test/ -L./target/release/ -l./c_cssparser

@REM run the cpp example
CALL "./target/release/csstest.exe"
