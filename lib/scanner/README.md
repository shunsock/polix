# Polix Scanner

## About

The Polix Scanner is a scanner of the Polix language.

```
let f: fn = (x: int): int { return x + 1; }
```

Following code will be scanned into:

```shell
[
    KeywordLet,
    Identifier("f"),
    SeparatorColon,
    KeywordFn,
    SingleEqual,
    DelimiterParenthesisLeft,
    Identifier("x"),
    SeparatorColon,
    KeywordTypeInteger,
    DelimiterParenthesisRight,
    SeparatorColon,
    KeywordTypeInteger,
    DelimiterBraceLeft,
    KeywordOperationReturn,
    Identifier("x"),
    SinglePlus,
    LiteralInteger(1),
    SeparatorSemicolon,
    DelimiterBraceRight,
    Eof, 
]
```

This is the list of all the tokens that the scanner can return

```rust
enum TokenType {
    DelimiterAngleLeft,
    DelimiterAngleRight,
    DelimiterBraceLeft,
    DelimiterBraceRight,
    DelimiterBracketLeft,
    DelimiterBracketRight,
    DelimiterParenthesisLeft,
    DelimiterParenthesisRight,
    DoubleEqual,
    DoubleSlash,
    Eof,
    Identifiers(String),
    KeywordAnd,
    KeywordAs,
    KeywordElse,
    KeywordIf,
    KeywordInterface,
    KeywordLet,
    KeywordMatch,
    KeywordMutable,
    KeywordNot,
    KeywordOperationBreak,
    KeywordOperationContinue,
    KeywordOperationFor,
    KeywordOperationLoop,
    KeywordOperationReturn,
    KeywordOr,
    KeywordTypeBoolean,
    KeywordTypeFloat,
    KeywordTypeFunction,
    KeywordTypeHash,
    KeywordTypeInteger,
    KeywordTypeList,
    KeywordTypeNone,
    KeywordTypeOption,
    KeywordTypeStruct,
    LiteralBoolean(bool),
    LiteralCharacter(char),
    LiteralFloat(f64),
    LiteralInteger(i32),
    LiteralNone,
    LiteralString(String),
    SeparatorColon,
    SeparatorComma,
    SeparatorDot,
    SeparatorSemicolon,
    SingleAsterisk,
    SingleEqual,
    SingleMinus,
    SinglePercent,
    SinglePlus,
    SingleSlash,
}
```
