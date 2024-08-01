#![allow(nonstandard_style)]
use std::{ffi::{c_char, c_float, c_int, CString}, fmt::Debug};

use safer_ffi::{derive_ReprC, ffi_export, prelude::repr_c};

type cstr = *const c_char;

macro_rules! to_cstr {
    ($value:expr) => {
        CString::from_raw($value).to_str().unwrap_or_default().to_string()
    };
}

#[cfg(feature = "headers")]
#[test]
pub fn gen_headers() {
    safer_ffi::headers::builder()
        .to_file("test/parser.h")
        .expect("Failed to write to file")
        .generate()
        .expect("Failed to generate headers");
}

// The C struct representations for their rust counterparts
#[derive_ReprC]
#[repr(u8)]
#[derive(Debug)]
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
    /// The value does not include the `/ *` `* /` markers.
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

#[derive_ReprC]
#[repr(opaque)]
pub struct TokenValue(Value);

macro_rules! value_getter {
    ($name:ident, $type:ty) => {
        #[safer_ffi::ffi_export]
        pub fn $name(value: &TokenValue) -> $type {
            unsafe{value.0.$name}
        }
    };
}

macro_rules! make_getters {
    ($($name:ident, $type:ty;)*) => {
        $(value_getter!($name, $type);)*
    };
}

make_getters! {
    ident, cstr;
    at_keyword, cstr;
    hash, cstr;
    id_hash, cstr;
    quoted_string, cstr;
    unquoted_url, cstr;
    comment, cstr;
    function, cstr;
    percentage, Percentage;
    dimension, Dimension;
    number, Number;
    whitespace, cstr;
    bad_string, cstr;
    bad_url, cstr;
    delim, c_char;
}

macro_rules! cstr {
    ($value:expr) => {
        CString::new($value.to_string()).unwrap().into_raw()
    };
}

impl<'i> From<cssparser::Token<'i>> for TokenValue {
    fn from(value: cssparser::Token<'i>) -> Self {
        match value {
            cssparser::Token::Ident(ident) => Self(Value::new_ident(cstr!(ident))),
            cssparser::Token::AtKeyword(at_keyword) => Self(Value::new_at_keyword(cstr!(at_keyword))),
            cssparser::Token::Hash(hash) => Self(Value::new_hash(cstr!(hash))),
            cssparser::Token::IDHash(id_hash) => Self(Value::new_id_hash(cstr!(id_hash))),
            cssparser::Token::QuotedString(quoted_string) => Self(Value::new_quoted_string(cstr!(quoted_string))),
            cssparser::Token::UnquotedUrl(unquoted_url) => Self(Value::new_unquoted_url(cstr!(unquoted_url))),
            cssparser::Token::Comment(comment) => Self(Value::new_comment(cstr!(comment))),
            cssparser::Token::Function(function) => Self(Value::new_function(cstr!(function))),
            cssparser::Token::Percentage {
                has_sign,
                int_value,
                unit_value,
            } => Self(Value::new_percentage(
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
                value,
                int_value,
                unit,
            } => Self(Value::new_dimension(
                Dimension {
                    has_sign,
                    value,
                    int_value: match int_value {
                        Some(value) => &value,
                        None => std::ptr::null(),
                    },
                    unit: cstr!(unit),
                }
            )),
            cssparser::Token::Number {
                has_sign,
                value,
                int_value,
            } => Self(Value::new_number(
                Number {
                    has_sign,
                    value,
                    int_value: match int_value {
                        Some(value) => &value,
                        None => std::ptr::null(),
                    },
                }
            )),
            cssparser::Token::Delim(delim) => Self(Value::new_delim(delim as c_char)),
            cssparser::Token::WhiteSpace(whitespace) => Self(Value::new_whitespace(cstr!(whitespace))),
            cssparser::Token::BadString(bad_string) => Self(Value::new_bad_string(cstr!(bad_string))),
            cssparser::Token::BadUrl(bad_url) => Self(Value::new_bad_url(cstr!(bad_url))),
            cssparser::Token::CDC => Self(Value::empty()),
            cssparser::Token::CDO => Self(Value::empty()),
            cssparser::Token::CloseCurlyBracket => Self(Value::empty()),
            cssparser::Token::CloseParenthesis => Self(Value::empty()),
            cssparser::Token::CloseSquareBracket => Self(Value::empty()),
            cssparser::Token::Colon => Self(Value::empty()),
            cssparser::Token::Comma => Self(Value::empty()),
            cssparser::Token::CurlyBracketBlock => Self(Value::empty()),
            cssparser::Token::DashMatch => Self(Value::empty()),
            cssparser::Token::IncludeMatch => Self(Value::empty()),
            cssparser::Token::ParenthesisBlock => Self(Value::empty()),
            cssparser::Token::PrefixMatch => Self(Value::empty()),
            cssparser::Token::Semicolon => Self(Value::empty()),
            cssparser::Token::SquareBracketBlock => Self(Value::empty()),
            cssparser::Token::SubstringMatch => Self(Value::empty()),
            cssparser::Token::SuffixMatch => Self(Value::empty()),
        }
    }
}

impl TokenValue {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn get_ident(&self) -> cstr {
        unsafe { self.0.ident }
    }

    pub fn get_at_keyword(&self) -> cstr {
        unsafe { self.0.at_keyword }
    }

    pub fn get_hash(&self) -> cstr {
        unsafe { self.0.hash }
    }

    pub fn get_quoted_string(&self) -> cstr {
        unsafe { self.0.quoted_string }
    }

    pub fn get_unquoted_url(&self) -> cstr {
        unsafe { self.0.unquoted_url }
    }

    pub fn get_comment(&self) -> cstr {
        unsafe { self.0.comment }
    }

    pub fn get_function(&self) -> cstr {
        unsafe { self.0.function }
    }

    pub fn get_percentage(&self) -> Percentage {
        unsafe { self.0.percentage }
    }

    pub fn get_dimension(&self) -> Dimension {
        unsafe { self.0.dimension }
    }

    pub fn get_number(&self) -> Number {
        unsafe { self.0.number }
    }

    pub fn get_whitespace(&self) -> cstr {
        unsafe { self.0.whitespace }
    }

    pub fn get_bad_string(&self) -> cstr {
        unsafe { self.0.bad_string }
    }

    pub fn get_bad_url(&self) -> cstr {
        unsafe { self.0.bad_url }
    }

    pub fn get_delim(&self) -> c_char {
        unsafe { self.0.delim }
    }

    pub fn get_id_hash(&self) -> cstr {
        unsafe { self.0.id_hash }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.0.empty == () }
    }
}

#[repr(C)]
pub union Value {
    /// The value of an [`Ident`](TokenType::Ident) token.
    ident: cstr,

    /// The value of an [`AtKeyword`](TokenType::AtKeyword) token.
    at_keyword: cstr,

    /// The value of a [`Hash`](TokenType::Hash) token.
    hash: cstr,

    /// The value of a [`QuotedString`](TokenType::QuotedString) token.
    quoted_string: cstr,

    /// The value of a [`UnquotedUrl`](TokenType::UnquotedUrl) token.
    unquoted_url: cstr,

    /// The value of a [`Comment`](TokenType::Comment) token.
    comment: cstr,

    /// The value of a [`Function`](TokenType::Function) token.
    function: cstr,

    /// The value of a [`Percentage`](TokenType::Percentage) token.
    percentage: Percentage,

    /// The value of a [`Dimension`](TokenType::Dimension) token.
    dimension: Dimension,

    /// The value of a [`Number`](TokenType::Number) token.
    number: Number,

    /// The value of a [`Whitespace`](TokenType::WhiteSpace) token
    whitespace: cstr,

    /// The value of a [`BadString`](TokenType::BadString) token.
    bad_string: cstr,

    /// The value of a [`BadUrl`](TokenType::BadUrl) token.
    bad_url: cstr,

    /// The value of a [`Delim`](TokenType::Delim) token.
    delim: c_char,

    /// The value of a [`IDHash`](TokenType::IDHash) token.
    id_hash: cstr,

    /// The value of any token that does not have a value.
    empty: (),
}

impl Value {
    pub fn empty() -> Self {
        Self { empty: () }
    }

    pub fn new_ident(ident: cstr) -> Self {
        Self { ident, }
    }

    pub fn new_at_keyword(at_keyword: cstr) -> Self {
        Self { at_keyword, }
    }

    pub fn new_hash(hash: cstr) -> Self {
        Self { hash, }
    }

    pub fn new_quoted_string(quoted_string: cstr) -> Self {
        Self { quoted_string, }
    }

    pub fn new_unquoted_url(unquoted_url: cstr) -> Self {
        Self { unquoted_url, }
    }

    pub fn new_comment(comment: cstr) -> Self {
        Self { comment, }
    }

    pub fn new_function(function: cstr) -> Self {
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

    pub fn new_whitespace(whitespace: cstr) -> Self {
        Self { whitespace, }
    }

    pub fn new_bad_string(bad_string: cstr) -> Self {
        Self { bad_string, }
    }

    pub fn new_bad_url(bad_url: cstr) -> Self {
        Self { bad_url, }
    }

    pub fn new_delim(delim: c_char) -> Self {
        Self { delim }
    }

    pub fn new_id_hash(id_hash: cstr) -> Self {
        Self { id_hash, }
    }
}

#[no_mangle]
pub fn get_token_type(token: &cssparser::Token) -> TokenType {
    match token {
        cssparser::Token::Ident(_) => TokenType::Ident,
        cssparser::Token::AtKeyword(_) => TokenType::AtKeyword,
        cssparser::Token::Hash(_) => TokenType::Hash,
        cssparser::Token::IDHash(_) => TokenType::IDHash,
        cssparser::Token::QuotedString(_) => TokenType::QuotedString,
        cssparser::Token::UnquotedUrl(_) => TokenType::UnquotedUrl,
        cssparser::Token::Comment(_) => TokenType::Comment,
        cssparser::Token::Function(_) => TokenType::Function,
        cssparser::Token::Percentage { .. } => TokenType::Percentage,
        cssparser::Token::Dimension { .. } => TokenType::Dimension,
        cssparser::Token::Number { .. } => TokenType::Number,
        cssparser::Token::Delim(_) => TokenType::Delim,
        cssparser::Token::Colon => TokenType::Colon,
        cssparser::Token::Semicolon => TokenType::Semicolon,
        cssparser::Token::Comma => TokenType::Comma,
        cssparser::Token::IncludeMatch => TokenType::IncludeMatch,
        cssparser::Token::DashMatch => TokenType::DashMatch,
        cssparser::Token::PrefixMatch => TokenType::PrefixMatch,
        cssparser::Token::SuffixMatch => TokenType::SuffixMatch,
        cssparser::Token::SubstringMatch => TokenType::SubstringMatch,
        cssparser::Token::CDO => TokenType::CDO,
        cssparser::Token::CDC => TokenType::CDC,
        cssparser::Token::ParenthesisBlock => TokenType::ParenthesisBlock,
        cssparser::Token::SquareBracketBlock => TokenType::SquareBracketBlock,
        cssparser::Token::CurlyBracketBlock => TokenType::CurlyBracketBlock,
        cssparser::Token::BadUrl(_) => TokenType::BadUrl,
        cssparser::Token::BadString(_) => TokenType::BadString,
        cssparser::Token::CloseParenthesis => TokenType::CloseParenthesis,
        cssparser::Token::CloseSquareBracket => TokenType::CloseSquareBracket,
        cssparser::Token::CloseCurlyBracket => TokenType::CloseCurlyBracket,
        cssparser::Token::WhiteSpace(_) => TokenType::WhiteSpace,
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Number {
    has_sign: bool,
    value: std::ffi::c_float,
    int_value: *const c_int,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Dimension {
    has_sign: bool,
    value: c_float,
    int_value: *const c_int,
    unit: cstr,
}

impl Clone for Dimension {
    fn clone(&self) -> Self {
        Self {
            has_sign: self.has_sign,
            value: self.value,
            int_value: self.int_value,
            unit: self.unit,
        }
    }
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Percentage {
    has_sign: bool,
    unit_value: f32,
    int_value: *const c_int,
}

#[derive_ReprC]
#[repr(C)]
pub struct Token {
    pub token_type: TokenType,
    /// This is an opaque pointer to the actual value of the token
    /// to get the value, you need to pass this to the appropriate function get_* function
    /// depending on the token type
    /// ```cpp
    /// switch (token.token_type) {
    ///    case TokenType::Ident: {
    ///       auto value = token.value.get_ident();
    ///    } break;
    ///    case TokenType::AtKeyword: {
    ///      auto value = token.value.get_at_keyword();
    ///    } break;
    ///    etc...
    /// }
    /// ```
    pub value: repr_c::Box<TokenValue>,
}

#[ffi_export]
pub fn value_as_string(value: &TokenValue, token_type: &TokenType) -> repr_c::String {
    let value = match token_type {
        TokenType::Ident => unsafe { to_cstr!(value.get_ident().cast_mut()) },
        TokenType::AtKeyword => unsafe { to_cstr!(value.get_at_keyword().cast_mut()) },
        TokenType::Hash => unsafe { to_cstr!(value.get_hash().cast_mut()) },
        TokenType::IDHash => unsafe { to_cstr!(value.get_id_hash().cast_mut()) },
        TokenType::QuotedString => unsafe { to_cstr!(value.get_quoted_string().cast_mut()) },
        TokenType::UnquotedUrl => unsafe { to_cstr!(value.get_unquoted_url().cast_mut()) },
        TokenType::Comment => unsafe { to_cstr!(value.get_comment().cast_mut()) },
        TokenType::Function => unsafe { to_cstr!(value.get_function().cast_mut()) },
        TokenType::WhiteSpace => unsafe { to_cstr!(value.get_whitespace().cast_mut()) },
        TokenType::BadString => unsafe { to_cstr!(value.get_bad_string().cast_mut()) },
        TokenType::BadUrl => unsafe { to_cstr!(value.get_bad_url().cast_mut()) },
        TokenType::Percentage => format!("{:?}", value.get_percentage()),
        TokenType::Dimension => format!("{:?}", value.get_dimension()),
        TokenType::Number => format!("{:?}", value.get_number()),
        TokenType::Delim => format!("{:?}", value.get_delim()),
        TokenType::Colon => ":".to_string(),
        TokenType::Semicolon => ";".to_string(),
        TokenType::Comma => ",".to_string(),
        TokenType::IncludeMatch => "~=".to_string(),
        TokenType::DashMatch => "|=".to_string(),
        TokenType::PrefixMatch => "^=".to_string(),
        TokenType::SuffixMatch => "$=".to_string(),
        TokenType::SubstringMatch => "*=".to_string(),
        TokenType::CDO => "<!--".to_string(),
        TokenType::CDC => "-->".to_string(),
        TokenType::ParenthesisBlock => "(".to_string(),
        TokenType::SquareBracketBlock => "[".to_string(),
        TokenType::CurlyBracketBlock => "{".to_string(),
        TokenType::CloseParenthesis => ")".to_string(),
        TokenType::CloseSquareBracket => "]".to_string(),
        TokenType::CloseCurlyBracket => "}".to_string(),
    };
    value.into()
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = value_as_string(&self.value, &self.token_type).to_string();
        write!(f, "Token {{ token_type: {:?}, value: {:?} }}", self.token_type, value)
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: TokenValue) -> Token {
        Self { token_type, value: Box::new(value).into() }
    }
}

impl<'a> From<&cssparser::Token<'a>> for Token {
    fn from(value: &cssparser::Token<'a>) -> Self {
        Token::from(value.clone())
    }
}

impl<'a> From<cssparser::Token<'a>> for Token {
    fn from(value: cssparser::Token<'a>) -> Self {
        Token::new(get_token_type(&value), TokenValue::from(value))
    }
}

#[no_mangle]
pub fn parse<'a>(parser: &'a mut cssparser::Parser, tokens: &mut Vec<Token>) -> Result<(), cssparser::ParseError<'a, ()>> {
    let token = parser.next()?.clone();
    if token == cssparser::Token::CurlyBracketBlock || token == cssparser::Token::SquareBracketBlock || token == cssparser::Token::ParenthesisBlock {
        parser.parse_nested_block::<_, _, ()>(|p| {
            while let Ok(token) = p.next() {
                // println!("Token: {:?}", token);
                tokens.push(Token::from(token));
            }
            Ok(())
        }).expect("Failed to parse nested block");
    }
    // println!("Token: {:?}", token);
    tokens.push(Token::from(token));
    Ok(())
}

#[ffi_export]
pub fn css_parse<'i>(input: *const i8) -> safer_ffi::Vec<Token> {
    let input = unsafe { std::ffi::CStr::from_ptr(input) };
    let mut input = cssparser::ParserInput::new(match input.to_str() {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {:?}", e);
            return safer_ffi::Vec::EMPTY;
        }
    });
    let mut parser = cssparser::Parser::new(&mut input);
    let mut tokens = Vec::new();

    loop {
        match parse(&mut parser, &mut tokens) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    for token in tokens.iter() {
        println!("{:#?}", token);
    }

    tokens.into()
}
