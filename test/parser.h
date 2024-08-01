#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


enum class TokenType : uint8_t {
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
  IDHash,
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
  Colon,
  /// A `;` `<semicolon-token>`
  Semicolon,
  /// A `,` `<comma-token>`
  Comma,
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
};

using c_str = const char*;

struct Percentage {
  bool has_sign;
  float unit_value;
  const int *int_value;
};

struct Dimension {
  bool has_sign;
  float value;
  const int *int_value;
  c_str unit;
};

struct Number {
  bool has_sign;
  float value;
  const int *int_value;
};

union Value {
  /// The value of an [`Ident`](TokenType::Ident) token.
  c_str ident;
  /// The value of an [`AtKeyword`](TokenType::AtKeyword) token.
  c_str at_keyword;
  /// The value of a [`Hash`](TokenType::Hash) token.
  c_str hash;
  /// The value of a [`QuotedString`](TokenType::QuotedString) token.
  c_str quoted_string;
  /// The value of a [`UnquotedUrl`](TokenType::UnquotedUrl) token.
  c_str unquoted_url;
  /// The value of a [`Comment`](TokenType::Comment) token.
  c_str comment;
  /// The value of a [`Function`](TokenType::Function) token.
  c_str function;
  /// The value of a [`Percentage`](TokenType::Percentage) token.
  Percentage percentage;
  /// The value of a [`Dimension`](TokenType::Dimension) token.
  Dimension dimension;
  /// The value of a [`Number`](TokenType::Number) token.
  Number number;
  /// The value of a [`Whitespace`](TokenType::WhiteSpace) token
  c_str whitespace;
  /// The value of a [`BadString`](TokenType::BadString) token.
  c_str bad_string;
  /// The value of a [`BadUrl`](TokenType::BadUrl) token.
  c_str bad_url;
  /// The value of a [`Delim`](TokenType::Delim) token.
  char delim;
  /// The value of a [`IDHash`](TokenType::IDHash) token.
  c_str id_hash;
};

struct Token {
  TokenType token_type;
  Value value;
};


extern "C" {

size_t css_parse(Token *tokens, const char *input);

} // extern "C"
