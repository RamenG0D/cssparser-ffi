#include <cstdio>
#include "parser.h"

#define file_input

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

#ifndef file_input
#define INPUT R"( \
    .test1 { \
        color: blue; \
    } \
    .test2 { \
        color: red; \
    } \
)"
#else
#define INPUT input()
#endif

int main(void) {
    // we use a string literal as input for the parser
    // and a multi-line string for better readability
    const char* css_input = INPUT;

    Token* tokens;
    size_t tlen = css_parse(tokens, css_input);
    if (tokens == NULL) {
        printf("Error: tokens array is NULL\n");
        return 1;
    }
    printf("Tokens array recieved successfully\n");
    printf("Number of tokens: %ld\n", tlen);

    return 0;
}
