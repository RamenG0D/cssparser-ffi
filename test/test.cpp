#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")

#include "parser.h"
#include <string>
#include <iostream>
#include <fstream>

std::string input() {
    // open a file the cpp way
    std::ifstream file("test/test.css", std::ios::in);

    // check if the file is open
    if(!file.is_open()) {
        std::cout << "Error: Failed to open file" << std::endl;
        return nullptr;
    }

    // read the file
    std::string input;

    // read in the whole file using its length
    file.seekg(0, std::ios::end);
    input.resize(file.tellg());
    file.seekg(0, std::ios::beg);

    // read the file's text into the string
    file.read(&input[0], input.size());

    // close the file
    file.close();

    return input;
}

int main(void) {
    // we use a string literal as input for the parser
    // and a multi-line string for better readability
    // std::string in = input();
    const int8_t* css_input = (int8_t*)R"(
        .class {
            color: red;
            background-color: blue;
            var-thing: var(--thing);
        }
    )";
    // (int8_t*)in.c_str();

    Vec_Token_t tokens = css_parse(css_input);
    if(tokens.ptr == nullptr) {
        std::cout << "Error: Failed to parse tokens array" << std::endl;
        return 1;
    }
    std::cout << "Tokens array recieved successfully\n" << std::endl;
    std::cout << "Number of tokens: " << tokens.len << std::endl;

    for(size_t i = 0; i < tokens.len; i++) {
        Token_t token = tokens.ptr[i];
        auto value = value_as_string(token.value, token.token_type);
        std::string s((const char*)value.ptr, value.len);
        std::cout << "Token: Value(\"" << s << "\")" << std::endl;
    }

    return 0;
}
