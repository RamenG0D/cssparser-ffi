
# build rust code
# cargo build --release

# re-generate c header
# cbindgen --config cbindgen.toml --crate c_cssparser -o test/parser.h
cargo test gen_headers --features headers

# build c++ code
# g++ -o csstest ./test/test.cpp -I./test/ -L./target/release/ -lc_cssparser

# run program
# ./csstest
