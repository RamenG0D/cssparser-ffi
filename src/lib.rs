#![allow(nonstandard_style)]
use std::{ffi::{c_char, c_float, c_int}, fmt::Debug};
use safer_ffi::{derive_ReprC, ffi_export, prelude::repr_c};

type cstr = repr_c::String;

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
#[derive(Debug, Clone, Copy)]
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

use paste::paste;

#[derive_ReprC]
#[repr(opaque)]
pub struct TokenValue(Value);

macro_rules! value_getter {
    ($name:ident, $type:ty) => {
        #[safer_ffi::ffi_export]
        pub fn $name(value: &TokenValue) -> $type {
            paste!{value.[<get_ $name>]()}
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

impl<'i> From<cssparser::Token<'i>> for TokenValue {
    fn from(value: cssparser::Token<'i>) -> Self {
        match value {
            cssparser::Token::Ident(ident) => Self(Value::new_ident(ident.to_string().into())),
            cssparser::Token::AtKeyword(at_keyword) => Self(Value::new_at_keyword(at_keyword.to_string().into())),
            cssparser::Token::Hash(hash) => Self(Value::new_hash(hash.to_string().into())),
            cssparser::Token::IDHash(id_hash) => Self(Value::new_id_hash(id_hash.to_string().into())),
            cssparser::Token::QuotedString(quoted_string) => Self(Value::new_quoted_string(quoted_string.to_string().into())),
            cssparser::Token::UnquotedUrl(unquoted_url) => Self(Value::new_unquoted_url(unquoted_url.to_string().into())),
            cssparser::Token::Comment(comment) => Self(Value::new_comment(comment.to_string().into())),
            cssparser::Token::Function(function) => Self(Value::new_function(function.to_string().into())),
            cssparser::Token::Percentage {
                has_sign,
                int_value,
                unit_value,
            } => Self(Value::new_percentage(
                Percentage {
                    has_sign,
                    int_value: int_value.map(|v| Box::new(v).into()),
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
                    int_value: int_value.map(|v| Box::new(v).into()),
                    unit: unit.to_string().into(),
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
                    int_value: int_value.map(|v| Box::new(v).into()),
                }
            )),
            cssparser::Token::Delim(delim) => Self(Value::new_delim(delim as c_char)),
            cssparser::Token::WhiteSpace(whitespace) => Self(Value::new_whitespace(whitespace.to_string().into())),
            cssparser::Token::BadString(bad_string) => Self(Value::new_bad_string(bad_string.to_string().into())),
            cssparser::Token::BadUrl(bad_url) => Self(Value::new_bad_url(bad_url.to_string().into())),
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

    fn pull_str(s: &std::mem::ManuallyDrop<repr_c::String>) -> repr_c::String {
        (**s).clone()
    }

    pub fn get_ident(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.ident })
    }

    pub fn get_at_keyword(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.at_keyword })
    }

    pub fn get_percentage(&self) -> Percentage {
        let value = unsafe { &self.0.percentage };
        Percentage {
            has_sign: value.has_sign,
            unit_value: value.unit_value,
            int_value: value.int_value.as_ref().map(|v| (*v).clone()),
        }
    }

    pub fn get_dimension(&self) -> Dimension {
        let value = unsafe { &self.0.dimension };
        Dimension {
            has_sign: value.has_sign,
            value: value.value,
            int_value: value.int_value.as_ref().map(|v| (*v).clone()),
            unit: value.unit.clone(),
        }
    }

    pub fn get_number(&self) -> Number {
        let value = unsafe { &self.0.number };
        Number {
            has_sign: value.has_sign,
            value: value.value,
            int_value: value.int_value.as_ref().map(|v| (*v).clone()),
        }
    }

    pub fn get_whitespace(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.whitespace })
    }

    pub fn get_comment(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.comment })
    }

    pub fn get_function(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.function })
    }

    pub fn get_bad_string(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.bad_string })
    }

    pub fn get_bad_url(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.bad_url })
    }

    pub fn get_delim(&self) -> c_char {
        unsafe { self.0.delim }
    }

    pub fn get_hash(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.hash })
    }

    pub fn get_id_hash(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.id_hash })
    }

    pub fn get_quoted_string(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.quoted_string })
    }

    pub fn get_unquoted_url(&self) -> cstr {
        Self::pull_str(unsafe { &self.0.unquoted_url })
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.0.empty == () }
    }
}

#[repr(C)]
pub union Value {
    /// The value of an [`Ident`](TokenType::Ident) token.
    ident: std::mem::ManuallyDrop<cstr>,

    /// The value of an [`AtKeyword`](TokenType::AtKeyword) token.
    at_keyword: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`Hash`](TokenType::Hash) token.
    hash: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`QuotedString`](TokenType::QuotedString) token.
    quoted_string: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`UnquotedUrl`](TokenType::UnquotedUrl) token.
    unquoted_url: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`Comment`](TokenType::Comment) token.
    comment: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`Function`](TokenType::Function) token.
    function: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`Percentage`](TokenType::Percentage) token.
    percentage: std::mem::ManuallyDrop<Percentage>,

    /// The value of a [`Dimension`](TokenType::Dimension) token.
    dimension: std::mem::ManuallyDrop<Dimension>,

    /// The value of a [`Number`](TokenType::Number) token.
    number: std::mem::ManuallyDrop<Number>,

    /// The value of a [`Whitespace`](TokenType::WhiteSpace) token
    whitespace: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`BadString`](TokenType::BadString) token.
    bad_string: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`BadUrl`](TokenType::BadUrl) token.
    bad_url: std::mem::ManuallyDrop<cstr>,

    /// The value of a [`Delim`](TokenType::Delim) token.
    delim: c_char,

    /// The value of a [`IDHash`](TokenType::IDHash) token.
    id_hash: std::mem::ManuallyDrop<cstr>,

    /// The value of any token that does not have a value.
    empty: (),
}

impl Value {
    pub fn empty() -> Self {
        Self { empty: () }
    }

    pub fn new_ident(ident: cstr) -> Self {
        Self { ident: std::mem::ManuallyDrop::new(ident) }
    }

    pub fn new_at_keyword(at_keyword: cstr) -> Self {
        Self { at_keyword: std::mem::ManuallyDrop::new(at_keyword) }
    }

    pub fn new_hash(hash: cstr) -> Self {
        Self { hash: std::mem::ManuallyDrop::new(hash) }
    }

    pub fn new_quoted_string(quoted_string: cstr) -> Self {
        Self { quoted_string: std::mem::ManuallyDrop::new(quoted_string) }
    }

    pub fn new_unquoted_url(unquoted_url: cstr) -> Self {
        Self { unquoted_url: std::mem::ManuallyDrop::new(unquoted_url) }
    }

    pub fn new_comment(comment: cstr) -> Self {
        Self { comment: std::mem::ManuallyDrop::new(comment) }
    }

    pub fn new_function(function: cstr) -> Self {
        Self { function: std::mem::ManuallyDrop::new(function) }
    }

    pub fn new_percentage(percentage: Percentage) -> Self {
        Self { percentage: std::mem::ManuallyDrop::new(percentage) }
    }

    pub fn new_dimension(dimension: Dimension) -> Self {
        Self { dimension: std::mem::ManuallyDrop::new(dimension) }
    }

    pub fn new_number(number: Number) -> Self {
        Self { number: std::mem::ManuallyDrop::new(number) }
    }

    pub fn new_whitespace(whitespace: cstr) -> Self {
        Self { whitespace: std::mem::ManuallyDrop::new(whitespace) }
    }

    pub fn new_bad_string(bad_string: cstr) -> Self {
        Self { bad_string: std::mem::ManuallyDrop::new(bad_string) }
    }

    pub fn new_bad_url(bad_url: cstr) -> Self {
        Self { bad_url: std::mem::ManuallyDrop::new(bad_url) }
    }

    pub fn new_delim(delim: c_char) -> Self {
        Self { delim }
    }

    pub fn new_id_hash(id_hash: cstr) -> Self {
        Self { id_hash: std::mem::ManuallyDrop::new(id_hash) }
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
#[derive(Debug)]
pub struct Number {
    has_sign: bool,
    value: std::ffi::c_float,
    int_value: Option<repr_c::Box<c_int>>,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct Dimension {
    has_sign: bool,
    value: c_float,
    int_value: Option<repr_c::Box<c_int>>,
    unit: cstr,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug)]
pub struct Percentage {
    has_sign: bool,
    unit_value: f32,
    int_value: Option<repr_c::Box<c_int>>,
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
pub fn token_to_string(token: &Token) -> repr_c::String {
    let value = match token.token_type {
        TokenType::Ident => token.value.get_ident(),
        TokenType::AtKeyword => token.value.get_at_keyword(),
        TokenType::Hash => token.value.get_hash(),
        TokenType::IDHash => token.value.get_id_hash(),
        TokenType::QuotedString => token.value.get_quoted_string(),
        TokenType::UnquotedUrl => token.value.get_unquoted_url(),
        TokenType::Comment => token.value.get_comment(),
        TokenType::Function => token.value.get_function(),
        TokenType::Percentage => {
            let percent = token.value.get_percentage();
            let value = format!("{:?}", percent);
            value.into()
        },
        TokenType::Dimension => {
            let dim = token.value.get_dimension();
            let value = format!("{:?}", dim);
            value.into()
        },
        TokenType::Number => {
            let num = token.value.get_number();
            let value = format!("{:?}", num);
            value.into()
        },
        TokenType::WhiteSpace => token.value.get_whitespace(),
        TokenType::BadString => token.value.get_bad_string(),
        TokenType::BadUrl => token.value.get_bad_url(),
        TokenType::Delim => token.value.get_delim().to_string().into(),
        TokenType::Colon => ":".to_string().into(),
        TokenType::Semicolon => ";".to_string().into(),
        TokenType::Comma => ",".to_string().into(),
        TokenType::IncludeMatch => "~=".to_string().into(),
        TokenType::DashMatch => "|=".to_string().into(),
        TokenType::PrefixMatch => "^=".to_string().into(),
        TokenType::SuffixMatch => "$=".to_string().into(),
        TokenType::SubstringMatch => "*=".to_string().into(),
        TokenType::CDO => "<!--".to_string().into(),
        TokenType::CDC => "-->".to_string().into(),
        TokenType::ParenthesisBlock => "(".to_string().into(),
        TokenType::SquareBracketBlock => "[".to_string().into(),
        TokenType::CurlyBracketBlock => "{".to_string().into(),
        TokenType::CloseParenthesis => ")".to_string().into(),
        TokenType::CloseSquareBracket => "]".to_string().into(),
        TokenType::CloseCurlyBracket => "}".to_string().into(),
    };

    value
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = token_to_string(self).to_string();
        write!(f, "{:?}(\"{}\")", self.token_type, value)
    }
}

#[ffi_export]
pub fn debug_token(token: &Token) {
    println!("{:?}", token);
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
pub fn parse<'a>(parser: &mut cssparser::Parser, mut tokens: Vec<Token>) -> Result<Vec<Token>, cssparser::ParseError<'a, Vec<Token>>> {
    let parse_inner = |parser: &mut cssparser::Parser, tokens: Vec<Token>| parser.parse_nested_block::<_, Vec<Token>, Vec<Token>>(|p| parse(p, tokens)).expect("Failed to parse nested block");
    while let Ok(token) = parser.next().cloned() {
        tokens.push(Token::from(&token));
        if let cssparser::Token::Function(_) = token {
            tokens = parse_inner(parser, tokens);
        } else if token == cssparser::Token::CurlyBracketBlock ||
        token == cssparser::Token::SquareBracketBlock ||
        token == cssparser::Token::ParenthesisBlock {
            tokens = parse_inner(parser, tokens);
        }
    }
    Ok(tokens)
}

#[ffi_export]
pub fn free_tokens(tokens: safer_ffi::Vec<Token>) {
    let _: Vec<Token> = tokens.into();
}

#[ffi_export]
pub fn parse_css<'i>(input: *const c_char) -> safer_ffi::Vec<Token> {
    let input = unsafe { std::ffi::CStr::from_ptr(input).to_str().expect("Failed to convert input to string") };
    let mut input = cssparser::ParserInput::new(input);
    let mut parser = cssparser::Parser::new(&mut input);

    let mut tokens = None;
    loop {
        let toks = match parse(&mut parser, Vec::new()) {
            Ok(v) => v,
            Err(v) => {
                match v.kind {
                    cssparser::ParseErrorKind::Basic(_) => break,
                    cssparser::ParseErrorKind::Custom(v) => {
                        tokens = Some(v);
                        break;
                    },
                }
            },
        };

        if toks.is_empty() {
            break;
        }

        tokens = Some(toks);
    }

    let tokens: safer_ffi::Vec<Token> = tokens.expect("Failed to parse tokens").into();

    tokens
}
