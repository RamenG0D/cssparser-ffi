use std::ffi::{c_char, c_float, c_int, CString};

#[allow(non_camel_case_types)]
type c_str = *const c_char;

// The C struct representations for their rust counterparts
#[repr(u8)]
pub enum TokenType {
    /// A [`<ident-token>`](https://drafts.csswg.org/css-syntax/#ident-token-diagram)
    Ident,

    /// A [`<at-keyword-token>`](https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram)
    ///
    /// The value does not include the `@` marker.
    AtKeyword,

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "unrestricted"
    ///
    /// The value does not include the `#` marker.
    Hash,

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "id"
    ///
    /// The value does not include the `#` marker.
    IDHash, // Hash that is a valid ID selector.

    /// A [`<string-token>`](https://drafts.csswg.org/css-syntax/#string-token-diagram)
    ///
    /// The value does not include the quotes.
    QuotedString,

    /// A [`<url-token>`](https://drafts.csswg.org/css-syntax/#url-token-diagram)
    ///
    /// The value does not include the `url(` `)` markers.  Note that `url( <string-token> )` is represented by a
    /// `Function` token.
    UnquotedUrl,

    /// A `<delim-token>`
    Delim,

    /// A [`<number-token>`](https://drafts.csswg.org/css-syntax/#number-token-diagram)
    Number,

    /// A [`<percentage-token>`](https://drafts.csswg.org/css-syntax/#percentage-token-diagram)
    Percentage,

    /// A [`<dimension-token>`](https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
    Dimension,

    /// A [`<whitespace-token>`](https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
    WhiteSpace,

    /// A comment.
    ///
    /// The CSS Syntax spec does not generate tokens for comments,
    /// But we do, because we can (borrowed &str makes it cheap).
    ///
    /// The value does not include the `/*` `*/` markers.
    Comment,

    /// A `:` `<colon-token>`
    Colon, // :

    /// A `;` `<semicolon-token>`
    Semicolon, // ;

    /// A `,` `<comma-token>`
    Comma, // ,

    /// A `~=` [`<include-match-token>`](https://drafts.csswg.org/css-syntax/#include-match-token-diagram)
    IncludeMatch,

    /// A `|=` [`<dash-match-token>`](https://drafts.csswg.org/css-syntax/#dash-match-token-diagram)
    DashMatch,

    /// A `^=` [`<prefix-match-token>`](https://drafts.csswg.org/css-syntax/#prefix-match-token-diagram)
    PrefixMatch,

    /// A `$=` [`<suffix-match-token>`](https://drafts.csswg.org/css-syntax/#suffix-match-token-diagram)
    SuffixMatch,

    /// A `*=` [`<substring-match-token>`](https://drafts.csswg.org/css-syntax/#substring-match-token-diagram)
    SubstringMatch,

    /// A `<!--` [`<CDO-token>`](https://drafts.csswg.org/css-syntax/#CDO-token-diagram)
    CDO,

    /// A `-->` [`<CDC-token>`](https://drafts.csswg.org/css-syntax/#CDC-token-diagram)
    CDC,

    /// A [`<function-token>`](https://drafts.csswg.org/css-syntax/#function-token-diagram)
    ///
    /// The value (name) does not include the `(` marker.
    Function,

    /// A `<(-token>`
    ParenthesisBlock,

    /// A `<[-token>`
    SquareBracketBlock,

    /// A `<{-token>`
    CurlyBracketBlock,

    /// A `<bad-url-token>`
    ///
    /// This token always indicates a parse error.
    BadUrl,

    /// A `<bad-string-token>`
    ///
    /// This token always indicates a parse error.
    BadString,

    /// A `<)-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseParenthesis,

    /// A `<]-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseSquareBracket,

    /// A `<}-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseCurlyBracket,
}

#[repr(C)]
pub union Value {
    /// The value of an [`Ident`](TokenType::Ident) token.
    ident: c_str,

    /// The value of an [`AtKeyword`](TokenType::AtKeyword) token.
    at_keyword: c_str,

    /// The value of a [`Hash`](TokenType::Hash) token.
    hash: c_str,

    /// The value of a [`QuotedString`](TokenType::QuotedString) token.
    quoted_string: c_str,

    /// The value of a [`UnquotedUrl`](TokenType::UnquotedUrl) token.
    unquoted_url: c_str,

    /// The value of a [`Comment`](TokenType::Comment) token.
    comment: c_str,

    /// The value of a [`Function`](TokenType::Function) token.
    function: c_str,

    /// The value of a [`Percentage`](TokenType::Percentage) token.
    percentage: Percentage,

    /// The value of a [`Dimension`](TokenType::Dimension) token.
    dimension: Dimension,

    /// The value of a [`Number`](TokenType::Number) token.
    number: Number,

    /// The value of a [`Whitespace`](TokenType::WhiteSpace) token
    whitespace: c_str,

    /// The value of a [`BadString`](TokenType::BadString) token.
    bad_string: c_str,

    /// The value of a [`BadUrl`](TokenType::BadUrl) token.
    bad_url: c_str,

    /// The value of a [`Delim`](TokenType::Delim) token.
    delim: c_char,

    /// The value of a [`IDHash`](TokenType::IDHash) token.
    id_hash: c_str,

    /// The value of any token that does not have a value.
    empty: (),
}

impl Value {
    pub fn empty() -> Self {
        Self { empty: () }
    }

    pub fn new_ident(ident: c_str) -> Self {
        Self { ident, }
    }

    pub fn new_at_keyword(at_keyword: c_str) -> Self {
        Self { at_keyword, }
    }

    pub fn new_hash(hash: c_str) -> Self {
        Self { hash, }
    }

    pub fn new_quoted_string(quoted_string: c_str) -> Self {
        Self { quoted_string, }
    }

    pub fn new_unquoted_url(unquoted_url: c_str) -> Self {
        Self { unquoted_url, }
    }

    pub fn new_comment(comment: c_str) -> Self {
        Self { comment, }
    }

    pub fn new_function(function: c_str) -> Self {
        Self { function, }
    }

    pub fn new_percentage(percentage: Percentage) -> Self {
        Self { percentage, }
    }

    pub fn new_dimension(dimension: Dimension) -> Self {
        Self { dimension, }
    }

    pub fn new_number(number: Number) -> Self {
        Self { number, }
    }

    pub fn new_whitespace(whitespace: c_str) -> Self {
        Self { whitespace, }
    }

    pub fn new_bad_string(bad_string: c_str) -> Self {
        Self { bad_string, }
    }

    pub fn new_bad_url(bad_url: c_str) -> Self {
        Self { bad_url, }
    }

    pub fn new_delim(delim: c_char) -> Self {
        Self { delim }
    }

    pub fn new_id_hash(id_hash: c_str) -> Self {
        Self { id_hash, }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Number {
    has_sign: bool,
    value: std::ffi::c_float,
    int_value: *const c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Dimension {
    has_sign: bool,
    value: c_float,
    int_value: *const c_int,
    unit: c_str,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Percentage {
    has_sign: bool,
    unit_value: f32,
    int_value: *const c_int,
}

#[repr(C)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Value,
}

impl Token {
    pub fn new(token_type: TokenType, value: Value) -> Token {
        Self { token_type, value }
    }
}

impl<'a> From<&cssparser::Token<'a>> for Token {
    fn from(value: &cssparser::Token<'a>) -> Self {
        Token::from(value.clone())
    }
}

impl<'a> From<cssparser::Token<'a>> for Token {
    fn from(value: cssparser::Token<'a>) -> Self {
        match value {
            cssparser::Token::Ident(ident) => Token::new(TokenType::Ident, Value::new_ident(
                CString::new(ident.to_string()).expect("Failed to convert ident to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::AtKeyword(at_keyword) => Token::new(TokenType::AtKeyword, Value::new_at_keyword(
                CString::new(at_keyword.to_string()).expect("Failed to convert at keyword to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::Hash(hash) => Token::new(TokenType::Hash, Value::new_hash(
                CString::new(hash.to_string()).expect("Failed to convert hash to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::IDHash(id_hash) => Token::new(TokenType::IDHash, Value::new_id_hash(
                CString::new(id_hash.to_string()).expect("Failed to convert id hash to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::QuotedString(quoted_string) => Token::new(TokenType::QuotedString, Value::new_quoted_string(
                CString::new(quoted_string.to_string()).expect("Failed to convert quoted string to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::UnquotedUrl(unquoted_url) => Token::new(TokenType::UnquotedUrl, Value::new_unquoted_url(
                CString::new(unquoted_url.to_string()).expect("Failed to convert unquoted url to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::Comment(comment) => Token::new(TokenType::Comment, Value::new_comment(
                CString::new(comment.to_string()).expect("Failed to convert comment to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::Function(function) => Token::new(TokenType::Function, Value::new_function(
                CString::new(function.to_string()).expect("Failed to convert function to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::Percentage {
                has_sign,
                int_value,
                unit_value,
            } => Token::new(TokenType::Percentage, Value::new_percentage(
                Percentage {
                    has_sign,
                    int_value: match int_value {
                        Some(value) => &value,
                        None => std::ptr::null(),
                    },
                    unit_value,
                }
            )),
            cssparser::Token::Dimension {
                has_sign,
                int_value,
                unit,
                value,
            } => Token::new(TokenType::Dimension, Value::new_dimension(
                Dimension {
                    has_sign,
                    int_value: match int_value {
                        Some(value) => &value,
                        None => std::ptr::null(),
                    },
                    unit: CString::new(unit.to_string()).expect("Failed to convert unit to CString").into_boxed_c_str().as_ptr(),
                    value,
                }
            )),
            cssparser::Token::Number {
                has_sign,
                int_value,
                value,
            } => Token::new(TokenType::Number, Value::new_number(
                Number {
                    has_sign,
                    int_value: match int_value {
                        Some(value) => &value,
                        None => std::ptr::null(),
                    },
                    value,
                }
            )),
            cssparser::Token::WhiteSpace(whitespace) => Token::new(TokenType::WhiteSpace, Value::new_whitespace(
                CString::new(whitespace.to_string()).expect("Failed to convert whitespace to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::BadString(bad_string) => Token::new(TokenType::BadString, Value::new_bad_string(
                CString::new(bad_string.to_string()).expect("Failed to convert bad string to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::BadUrl(bad_url) => Token::new(TokenType::BadUrl, Value::new_bad_url(
                CString::new(bad_url.to_string()).expect("Failed to convert bad url to CString").into_boxed_c_str().as_ptr(),
            )),
            cssparser::Token::Delim(delim) => Token::new(TokenType::Delim, Value::new_delim(delim as c_char)),
            cssparser::Token::Colon => Token::new(TokenType::Colon, Value::empty()),
            cssparser::Token::Semicolon => Token::new(TokenType::Semicolon, Value::empty()),
            cssparser::Token::Comma => Token::new(TokenType::Comma, Value::empty()),
            cssparser::Token::IncludeMatch => Token::new(TokenType::IncludeMatch, Value::empty()),
            cssparser::Token::DashMatch => Token::new(TokenType::DashMatch, Value::empty()),
            cssparser::Token::PrefixMatch => Token::new(TokenType::PrefixMatch, Value::empty()),
            cssparser::Token::SuffixMatch => Token::new(TokenType::SuffixMatch, Value::empty()),
            cssparser::Token::SubstringMatch => Token::new(TokenType::SubstringMatch, Value::empty()),
            cssparser::Token::CDO => Token::new(TokenType::CDO, Value::empty()),
            cssparser::Token::CDC => Token::new(TokenType::CDC, Value::empty()),
            cssparser::Token::ParenthesisBlock => Token::new(TokenType::ParenthesisBlock, Value::empty()),
            cssparser::Token::SquareBracketBlock => Token::new(TokenType::SquareBracketBlock, Value::empty()),
            cssparser::Token::CurlyBracketBlock => Token::new(TokenType::CurlyBracketBlock, Value::empty()),
            cssparser::Token::CloseParenthesis => Token::new(TokenType::CloseParenthesis, Value::empty()),
            cssparser::Token::CloseSquareBracket => Token::new(TokenType::CloseSquareBracket, Value::empty()),
            cssparser::Token::CloseCurlyBracket => Token::new(TokenType::CloseCurlyBracket, Value::empty()),
        }
    }
}

#[no_mangle]
#[allow(unused_variables, unused_assignments)]
pub extern "C" fn css_parse<'i>(mut tokens: *mut Token, input: *const c_char) -> usize {
    let input = unsafe { std::ffi::CStr::from_ptr(input) };
    let mut input = cssparser::ParserInput::new(input.to_str().unwrap());
    let mut parser = cssparser::Parser::new(&mut input);
    let mut _tokens = Vec::new();
    while let Ok(token) = parser.next() {
        _tokens.push(Token::from(token));
    }

    // Convert the vector to a pointer
    tokens = _tokens.as_mut_ptr();

    // Return the length of the vector
    _tokens.len()
}
