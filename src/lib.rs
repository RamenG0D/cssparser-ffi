#![allow(nonstandard_style)]
use safer_ffi::{derive_ReprC, ffi_export, prelude::repr_c};
use token_types::{get_token_type, TokenType};
use token_union::TokenValue;
use std::fmt::Debug;

pub type cstr = repr_c::String;

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
pub mod token_union;
pub mod token_types;

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

#[inline]
fn token2str(token: &Token) -> String {
    match token.token_type {
        TokenType::Ident => token.value.get_ident().into(),
        TokenType::AtKeyword => token.value.get_at_keyword().into(),
        TokenType::Hash => token.value.get_hash().into(),
        TokenType::IDHash => token.value.get_id_hash().into(),
        TokenType::QuotedString => token.value.get_quoted_string().into(),
        TokenType::UnquotedUrl => token.value.get_unquoted_url().into(),
        TokenType::Comment => token.value.get_comment().into(),
        TokenType::Function => token.value.get_function().into(),
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
        TokenType::WhiteSpace => token.value.get_whitespace().into(),
        TokenType::BadString => token.value.get_bad_string().into(),
        TokenType::BadUrl => token.value.get_bad_url().into(),
        TokenType::Delim => token.value.get_delim().to_string(),
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
    }
}

#[ffi_export]
pub fn token_to_string(token: &Token) -> repr_c::String {
    token2str(token).into()
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}(\"{}\")", self.token_type, token2str(self))
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
        match token {
            cssparser::Token::Function(_) => {
                tokens = parse_inner(parser, tokens);
            },
            cssparser::Token::CurlyBracketBlock => {
                tokens = parse_inner(parser, tokens);
                // add a closing token
                tokens.push(cssparser::Token::CloseCurlyBracket.into());
            },
            cssparser::Token::SquareBracketBlock => {
                tokens = parse_inner(parser, tokens);
                // add a closing token
                tokens.push(cssparser::Token::CloseSquareBracket.into());
            },
            cssparser::Token::ParenthesisBlock => {
                tokens = parse_inner(parser, tokens);
                // add a closing token
                tokens.push(cssparser::Token::CloseParenthesis.into());
            },
            _ => {},
        }
    }
    Ok(tokens)
}

#[ffi_export]
pub fn free_tokens(tokens: safer_ffi::Vec<Token>) {
    let v: Vec<Token> = tokens.into();
    drop(v);
}

#[ffi_export]
pub fn parse_css<'i>(input: *const safer_ffi::c_char) -> safer_ffi::Vec<Token> {
    let input = unsafe { std::ffi::CStr::from_ptr(input as *const _).to_str().expect("Failed to convert input to string") };
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
