use core::source_code::{Line, Position};
use core::token::Token;
use core::token::TokenKind;
use core::token::{BinaryOperation, Control, Declaration, Keyword, Literal, Type, PrimitiveType};

pub fn recognize_keyword(value: String, line: Line, position: Position) -> Option<Token> {
    match value.as_str() {
        "true" => Some(Token::new(
            TokenKind::Literal(Literal::Boolean(true)),
            line,
            position,
        )),
        "false" => Some(Token::new(
            TokenKind::Literal(Literal::Boolean(false)),
            line,
            position,
        )),
        "none" => Some(Token::new(
            TokenKind::Literal(Literal::None),
            line,
            position,
        )),
        "int" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Integer)),
            line,
            position,
        )),
        "float" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Float)),
            line,
            position,
        )),
        "bool" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::Boolean)),
            line,
            position,
        )),
        "string" => Some(Token::new(
            TokenKind::Type(Type::Primitive(PrimitiveType::String)),
            line,
            position,
        )),
        "Option" => Some(Token::new(
            TokenKind::Type(Type::Option),
            line,
            position,
        )),
        "List" => Some(Token::new(
            TokenKind::Type(Type::List),
            line,
            position,
        )),
        "if" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::If)),
            line,
            position,
        )),
        "loop" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::Loop)),
            line,
            position,
        )),
        "match" => Some(Token::new(
            TokenKind::Keyword(Keyword::Control(Control::Match)),
            line,
            position,
        )),
        "as" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::As)),
            line,
            position,
        )),
        "define" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Define)),
            line,
            position,
        )),
        "fn" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Function)),
            line,
            position,
        )),
        "let" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Let)),
            line,
            position,
        )),
        "module" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Module)),
            line,
            position,
        )),
        "new" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::New)),
            line,
            position,
        )),
        "return" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Return)),
            line,
            position,
        )),
        "struct" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Struct)),
            line,
            position,
        )),
        "use" => Some(Token::new(
            TokenKind::Keyword(Keyword::Declaration(Declaration::Use)),
            line,
            position,
        )),
        "==" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::DoubleEqual)),
            line,
            position,
        )),
        "!=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::NotEqual)),
            line,
            position,
        )),
        ">" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::GreaterThan)),
            line,
            position,
        )),
        ">=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(
                BinaryOperation::GreaterThanOrEqual,
            )),
            line,
            position,
        )),
        "<" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::LessThan)),
            line,
            position,
        )),
        "<=" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::LessThanOrEqual)),
            line,
            position,
        )),
        "&&" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::And)),
            line,
            position,
        )),
        "||" => Some(Token::new(
            TokenKind::Keyword(Keyword::BinaryOperation(BinaryOperation::Or)),
            line,
            position,
        )),
        _ => None,
    }
}
