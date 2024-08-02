
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

# Building

First you need to clone the repository
```bash
git clone https://github.com/RamenG0D/cssparser-ffi.git
```

Then run
```bash
cd cssparser-ffi
```

Then you need to build the project for your platform

- [Linux](#linux)
- [Windows](#windows)
- [Mac](#mac)

# Linux

Just run `run.sh` from the c_cssparser directory.

```bash
./run.sh
```

You should now have an up to date header file inside the test folder named `parser.h` and a static library named `libcss_parser.a` inside the `target/release` directory.

# Windows

Just run `run.bat` from the c_cssparser directory.

This command runs the build script
```powershell
./run.bat
```

You should now have an up to date header file inside the test folder named `parser.h` and a static library named `libcss_parser.a` inside the `target/release` directory.

# Mac

Macos is theoretically supported but entirely untested. If you are on a mac and you have tested this please let me know so I can update this section.
