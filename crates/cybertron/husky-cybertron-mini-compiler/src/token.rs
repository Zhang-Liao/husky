pub mod delimiter;
pub mod ident;
pub mod keyword;
pub mod literal;
pub mod opr;
pub mod separator;

use self::{ident::Ident, keyword::Keyword, literal::Literal, opr::Opr};
use crate::token::separator::Separator;
use crate::*;
use delimiter::{Delimiter, LeftDelimiter, RightDelimiter};
use husky_cybertron::seq::Seq;
use husky_text_protocol::char::TextCharIter;

#[enum_class::from_variants]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Literal(Literal),
    Keyword(Keyword),
    Ident(Ident),
    Opr(Opr),
    LeftDelimiter(LeftDelimiter),
    RightDelimiter(RightDelimiter),
    Separator(Separator),
}

pub enum Convexity {
    Convex,
    Concave,
}

impl Token {
    pub fn right_convexity(self) -> Convexity {
        match self {
            Token::Literal(_) => Convexity::Convex,
            Token::Keyword(_) => Convexity::Concave,
            Token::Ident(_) => Convexity::Convex,
            Token::Opr(opr) => match opr {
                Opr::Prefix(_) => Convexity::Concave,
                Opr::Binary(_) => Convexity::Concave,
                Opr::Suffix(_) => Convexity::Convex,
            },
            Token::LeftDelimiter(_) => Convexity::Concave,
            Token::RightDelimiter(_) => Convexity::Convex,
            Token::Separator(_) => Convexity::Concave,
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Literal(lit) => write!(f, "`{}`", lit),
            Token::Keyword(kw) => write!(f, "`{}`", kw.data()),
            Token::Ident(ident) => write!(f, "`{}`", ident.data()),
            Token::Opr(opr) => write!(f, "`{}`", opr.data()),
            Token::LeftDelimiter(delimiter) => write!(f, "`{}`", delimiter.data()),
            Token::RightDelimiter(delimiter) => write!(f, "`{}`", delimiter.data()),
            Token::Separator(separator) => write!(f, "`{}`", separator.data()),
        }
    }
}

pub fn tokenize(input: &str) -> Seq<Token> {
    Seq::new(Tokenizer::new(input).collect())
}

struct Tokenizer<'a> {
    chars: TextCharIter<'a>,
    last_token_right_convexity: Convexity,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: TextCharIter::new(input),
            last_token_right_convexity: Convexity::Concave,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_aux();
        if let Some(token) = token {
            self.last_token_right_convexity = token.right_convexity();
        }
        token
    }
}

/// # actions
impl<'a> Tokenizer<'a> {
    fn next_aux(&mut self) -> Option<Token> {
        match self.chars.next()? {
            ' ' => self.next(),
            '+' => match self.chars.peek() {
                Some('+') => {
                    self.chars.next();
                    Some(Opr::INCR.into())
                }
                _ => match self.last_token_right_convexity {
                    Convexity::Convex => Some(Opr::ADD.into()),
                    Convexity::Concave => Some(Opr::PLUS.into()),
                },
            },
            '-' => match self.chars.peek() {
                Some('-') => {
                    self.chars.next();
                    Some(Opr::DECR.into())
                }
                _ => match self.last_token_right_convexity {
                    Convexity::Convex => Some(Opr::SUB.into()),
                    Convexity::Concave => Some(Opr::MINUS.into()),
                },
            },
            '*' => Some(Opr::MUL.into()),
            '/' => Some(Opr::DIV.into()),
            '=' => Some(Opr::ASSIGN.into()),
            c if c.is_alphabetic() || c == '_' => Some(self.next_keyword_or_ident(c)),
            c if c.is_numeric() => Some(self.next_numeric_literal(c)),
            c => todo!(),
        }
    }

    fn next_keyword_or_ident(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        s += self
            .chars
            .next_str_slice_while(|c| c.is_alphanumeric() || c == '_');
        if let Some(keyword) = Keyword::new(&s) {
            return keyword.into();
        }
        Ident::new(s).into()
    }

    fn next_numeric_literal(&mut self, c: char) -> Token {
        let mut s = String::from(c);
        s += self.chars.next_str_slice_while(|c| c.is_numeric());
        let i: i32 = s.parse().unwrap();
        Token::Literal(Literal::Int(i))
    }
}

#[test]
fn tokenize_works() {
    fn t(input: &str, expect: Expect) {
        expect.assert_debug_eq(&tokenize(input));
    }
    t(
        "hello",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        "hello ",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        " hello ",
        expect![[r#"
        [`hello`]
    "#]],
    );
    t(
        " let hello = world",
        expect![[r#"
            [`let`, `hello`, `=`, `world`]
        "#]],
    );
    t(
        " let hello = 1",
        expect![[r#"
            [`let`, `hello`, `=`, `1`]
        "#]],
    );
    t(
        " let hello = world + 1",
        expect![[r#"
            [`let`, `hello`, `=`, `world`, `+(add)`, `1`]
        "#]],
    );
    t(
        "-1",
        expect![[r#"
            [`-(minus)`, `1`]
        "#]],
    );
    t(
        "1+-1",
        expect![[r#"
            [`1`, `+(add)`, `-(minus)`, `1`]
        "#]],
    );
    t(
        "1-1",
        expect![[r#"
            [`1`, `-(sub)`, `1`]
        "#]],
    );
    t(
        "x++",
        expect![[r#"
            [`x`, `++`]
        "#]],
    );
    t(
        "x--",
        expect![[r#"
            [`x`, `--`]
        "#]],
    );
}
