#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")

#include "parser.h"
#include <string>
#include <iostream>
#include <fstream>

void showcase(void);

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

    // get the total file size
    file.seekg(0, std::ios::end);
    input.resize(file.tellg()); // resize the string to the file size
    file.seekg(0, std::ios::beg);

    // read the file's text into the string
    file.read(const_cast<char*>(input.c_str()), input.size());

    // close the file
    file.close();

    return input;
}

int main(void) {
    // we use a string literal as input for the parser
    // and a multi-line string for better readability
    std::string css = input();
    // just use a char* for the input behind the scenes
    // so this is also perfectly valid.
    //
    // const int8_t* css_input = (int8_t*)R"(
    //     /* This is a comment */
    //     .class {
    //         color: red;
    //         background-color: blue;
    //         custom-property: 1;
    //         var-property: var(--custom-property);
    //     }
    // )";

    Vec_Token_t tokens = parse_css(css.c_str());
    if(tokens.ptr == nullptr) {
        std::cout << "Error: Failed to parse tokens array" << std::endl;
        return 1;
    }
    std::cout << "Tokens array recieved successfully\n" << std::endl;
    std::cout << "Number of tokens: " << tokens.len << std::endl;

    for(size_t i = 0; i < tokens.len; i++) {
        debug_token(&tokens.ptr[i]);
    }

    // free the tokens
    free_tokens(tokens);

    showcase();

    return 0;
}

// how to get a tokens value
void showcase(void) {
    std::string css = R"(
/* This is a comment */
.class {
    color: red;
    custom-property: 1;
    var-property: var(--custom-property);
})";
    std::cout << "CSS:" << std::endl << css << std::endl << std::endl;
    // get the tokens array
    Vec_Token_t tokens = parse_css(css.c_str());

    // get the first token
    Token_t token = tokens.ptr[1];

    // to get a token's value, you switch on the token's type, which is used to determine the function to call
    // Example:
    switch (token.token_type) {
        case TokenType::TOKEN_TYPE_IDENT:{
            // get the value of the token
            Vec_uint8_t value = ident(token.value);
            // do something with the value
            std::cout
            << "Ident value: "
            << std::string((const char*)value.ptr, value.len)
            << std::endl;
        } break;
        case TokenType::TOKEN_TYPE_HASH:{
            // get the value of the token
            Vec_uint8_t value = hash(token.value);
            // do something with the value
        } break;
        case TokenType::TOKEN_TYPE_FUNCTION:{
            // get the value of the token
            Vec_uint8_t value = function(token.value);
            // do something with the value
        } break;
        case TokenType::TOKEN_TYPE_NUMBER:{
            // get the value of the token
            Number_t value = number(token.value);
            // do something with the value
        } break;
        default:{
            // base case
            // error or unknown, etc...
        } break;
    }

    // free the tokens
    free_tokens(tokens);
}
