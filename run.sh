# build rust code
cargo build --release

# re-generate c header
cargo test gen_headers --features headers

# build c++ code
g++ -o ./target/release/csstest ./test/test.cpp -I./test/ -L./target/release/ -lc_cssparser

# run program
./target/release/csstest
