# Polix Transformer

## About

The Polix Transformer is a module that takes the list of tokens from the scanner and transforms it for preparing the parser to parse the code.

```
let f: fn = (x: int): int { return x + 1; };
```

Following code will be scanned into:

```shell
[
    Identifier("let"),
    Identifier("f"),
    SeparatorColon,
    Identifier("fn"),
    SingleEqual,
    DelimiterParenthesisLeft,
    Identifier("x"),
    SeparatorColon,
    Identifier("int"),
    DelimiterParenthesisRight,
    SeparatorColon,
    Identifier("int"),
    DelimiterBraceLeft,
    Identifier("return"),
    Identifier("x"),
    SinglePlus,
    Identifier("1"),
    SeparatorSemicolon,
    DelimiterBraceRight,
    SeparatorSemicolon,
]
```

This module will transform the above list of tokens into:

```shell
[
    KeywordLet,
    Variable("f"),
    SeparatorColon,
    KeywordFn,
    SingleEqual,
    DelimiterParenthesisLeft,
    Variable("x"),
    SeparatorColon,
    KeywordTypeInteger,
    DelimiterParenthesisRight,
    SeparatorColon,
    KeywordTypeInteger,
    DelimiterBraceLeft,
    KeywordOperationReturn,
    Variable("x"),
    SinglePlus,
    LiteralInteger(1),
    SeparatorSemicolon,
    DelimiterBraceRight,
    SeparatorSemicolon,
]
```
