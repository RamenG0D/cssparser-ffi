
# C CssParser

This rust library aims to provide a proper api to interop with cssparser, another rust library focused on parsing css, for C/C++ applications.

## Usage

```cpp
// THESE ARE THE REQUIRED!
#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")
// they are essentially a library include
// so you can remove these if your project is big all you need to do is add
// `-luserenv -lws2_32 -lntdll` to your linker flags (otherwise you will get linker errors)

#include <iostream>
#include <string>

#include "css_parser.h"

int main(void) {
    const char* css = "body { background-color: #f00; }";
    Vec_Tokens_t tokens = parse_css((int8_t*)css);

    // iterate over all tokens
    for (int i = 0; i < tokens.length; i++) {
        Token_t token = tokens.tokens[i];
        // do something with the token
    }

    // free the tokens
    free_tokens(tokens);

    return 0;
}

```
