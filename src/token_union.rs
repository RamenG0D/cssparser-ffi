#![allow(nonstandard_style)]
use std::ffi::{c_char, c_float, c_int};

use safer_ffi::{derive_ReprC, prelude::repr_c};
use paste::paste;
use crate::cstr;

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
