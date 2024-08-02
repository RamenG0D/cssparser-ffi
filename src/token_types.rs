#![allow(nonstandard_style)]
use safer_ffi::derive_ReprC;

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

impl From<cssparser::Token<'_>> for TokenType {
    fn from(value: cssparser::Token) -> Self {
        match value {
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
}

impl From<&cssparser::Token<'_>> for TokenType {
    fn from(value: &cssparser::Token) -> Self {
        value.to_owned().into()
    }
}

#[no_mangle]
pub fn get_token_type(token: &cssparser::Token) -> TokenType {
    token.into()
}
