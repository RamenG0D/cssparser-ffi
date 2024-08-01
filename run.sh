
# build rust code
cargo build --release

# re-generate c header
# cbindgen --config cbindgen.toml --crate c_cssparser -o test/parser.h
cargo test gen_headers --features headers

# build c++ code
g++ -o csstest test/test.cpp -Itest/ -Ltarget/release/ -Ltarget/debug/ -lc_cssparser

# run program
./csstest
