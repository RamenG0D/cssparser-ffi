#include <cstdio>
#include "parser.h"
#include <string>
#include <iostream>

// #define file_input

#ifndef file_input
#define INPUT R"( \
    .checkbox-wrapper * { \
        -webkit-tap-highlight-color: transparent; \
        outline: none; \
    } \
    .checkbox-wrapper input[type="checkbox"] { \
        display: none; \
    } \
    .checkbox-wrapper label { \
        --size: 50px; \
        --shadow: calc(var(--size) * .07) calc(var(--size) * .1); \
        position: relative; \
        display: block; \
        width: var(--size); \
        height: var(--size); \
        margin: 0 auto; \
        background-color: #4158D0; \
        background-image: linear-gradient(43deg, #4158D0 0%, #C850C0 46%, #FFCC70 100%); \
        border-radius: 50%; \
        box-shadow: 0 var(--shadow) #ffbeb8; \
        cursor: pointer; \
        transition: 0.2s ease transform, 0.2s ease background-color, \
            0.2s ease box-shadow; \
        overflow: hidden; \
        z-index: 1; \
    } \
    .checkbox-wrapper label:before { \
    content: ""; \
    position: absolute; \
    top: 50%; \
    right: 0; \
    left: 0; \
    width: calc(var(--size) * .7); \
    height: calc(var(--size) * .7); \
    margin: 0 auto; \
    background-color: #fff; \
    transform: translateY(-50%); \
    border-radius: 50%; \
    box-shadow: inset 0 var(--shadow) #ffbeb8; \
    transition: 0.2s ease width, 0.2s ease height; \
    } \
 \
    .checkbox-wrapper label:hover:before { \
    width: calc(var(--size) * .55); \
    height: calc(var(--size) * .55); \
    box-shadow: inset 0 var(--shadow) #ff9d96; \
    } \
 \
    .checkbox-wrapper label:active { \
    transform: scale(0.9); \
    } \
 \
    .checkbox-wrapper .tick_mark { \
    position: absolute; \
    top: -1px; \
    right: 0; \
    left: calc(var(--size) * -.05); \
    width: calc(var(--size) * .6); \
    height: calc(var(--size) * .6); \
    margin: 0 auto; \
    margin-left: calc(var(--size) * .14); \
    transform: rotateZ(-40deg); \
    } \
 \
    .checkbox-wrapper .tick_mark:before, \
    .checkbox-wrapper .tick_mark:after { \
    content: ""; \
    position: absolute; \
    background-color: #fff; \
    border-radius: 2px; \
    opacity: 0; \
    transition: 0.2s ease transform, 0.2s ease opacity; \
    } \
 \
    .checkbox-wrapper .tick_mark:before { \
    left: 0; \
    bottom: 0; \
    width: calc(var(--size) * .1); \
    height: calc(var(--size) * .3); \
    box-shadow: -2px 0 5px rgba(0, 0, 0, 0.23); \
    transform: translateY(calc(var(--size) * -.68)); \
    } \
 \
    .checkbox-wrapper .tick_mark:after { \
    left: 0; \
    bottom: 0; \
    width: 100%; \
    height: calc(var(--size) * .1); \
    box-shadow: 0 3px 5px rgba(0, 0, 0, 0.23); \
    transform: translateX(calc(var(--size) * .78)); \
    } \
 \
    .checkbox-wrapper input[type="checkbox"]:checked + label { \
    background-color: #4158D0; \
    background-image: linear-gradient(43deg, #4158D0 0%, #C850C0 46%, #FFCC70 100%); \
    box-shadow: rgba(0, 0, 0, 0.3) 0px 19px 38px, rgba(0, 0, 0, 0.22) 0px 15px 12px; \
    } \
 \
    .checkbox-wrapper input[type="checkbox"]:checked + label:before { \
    width: 0; \
    height: 0; \
    } \
 \
    .checkbox-wrapper input[type="checkbox"]:checked + label .tick_mark:before, \
    .checkbox-wrapper input[type="checkbox"]:checked + label .tick_mark:after { \
    transform: translate(0); \
    opacity: 1; \
    } \
)"
#else
const char* input() {
    FILE* f = fopen("test.css", "r");
    if (f == NULL) {
        printf("Error: could not open file\n");
        return NULL;
    }

    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);

    char* input = (char*)malloc(fsize);
    if (input == NULL) {
        printf("Error: could not allocate memory\n");
        fclose(f);
        return NULL;
    }

    fread(input, 1, fsize, f);
    fclose(f);

    return input;
}
#define INPUT input()
#endif

int main(void) {
    // we use a string literal as input for the parser
    // and a multi-line string for better readability
    const char* css_input = R"(
        checkbox:test {
        }
    )";

    Vec_Token_t tokens = css_parse((const int8_t*)css_input);
    if(tokens.ptr == nullptr) {
      printf("Error: tokens array is NULL\n");
      return 1;
    }
    printf("Tokens array recieved successfully\n");
    printf("Number of tokens: %ld\n", tokens.len);

    printf("Token: ");
    if(tokens.ptr->token_type == TokenType::TOKEN_TYPE_IDENT) {
      Vec_uint8_t i = value_as_string(&tokens.ptr->value, &tokens.ptr->token_type);
      std::string s((const char*)i.ptr, (const size_t)i.len);
      std::cout << "Ident(\"" << s << "\")" << std::endl;
    }

    return 0;
}
