use super::*;
use husky_opn_syntax::*;
use husky_print_utils::p;
use husky_text::{TextCharIter, TextRange};
use husky_word::WordDb;
use std::str::FromStr;

pub(crate) struct RangedPretoken {
    pub(crate) range: TextRange,
    pub(crate) token: Pretoken,
}

impl RangedPretoken {
    fn new(i: u32, start: u32, end: u32, token: Pretoken) -> Self {
        RangedPretoken {
            range: husky_text::new_same_line(i, start, end),
            token,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Pretoken {
    Certain(TokenKind),
    Literal(LiteralToken),
    NewLine,
    Ambiguous(AmbiguousPretoken),
    Comment,
    Err(TokenError),
}

impl Into<Pretoken> for IntegerLiteral {
    fn into(self) -> Pretoken {
        Pretoken::Certain(TokenKind::Literal(LiteralToken::Integer(self)))
    }
}

impl Into<Pretoken> for FloatLiteral {
    fn into(self) -> Pretoken {
        Pretoken::Certain(TokenKind::Literal(LiteralToken::Float(self)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AmbiguousPretoken {
    SubOrMinus,
    For,
}

impl From<TokenKind> for Pretoken {
    fn from(kind: TokenKind) -> Self {
        Pretoken::Certain(kind)
    }
}

impl From<Punctuation> for Pretoken {
    fn from(value: Punctuation) -> Self {
        Pretoken::Certain(value.into())
    }
}

impl From<Keyword> for Pretoken {
    fn from(kw: Keyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<StmtKeyword> for Pretoken {
    fn from(kw: StmtKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<TypeKeyword> for Pretoken {
    fn from(kw: TypeKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<LiasonKeyword> for Pretoken {
    fn from(kw: LiasonKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<ConfigKeyword> for Pretoken {
    fn from(kw: ConfigKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<AttributeKeyword> for Pretoken {
    fn from(kw: AttributeKeyword) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl From<WordOpr> for Pretoken {
    fn from(kw: WordOpr) -> Self {
        Pretoken::Certain(kw.into())
    }
}

impl Into<Pretoken> for FormKeyword {
    fn into(self) -> Pretoken {
        Pretoken::Certain(self.into())
    }
}

impl From<Token> for RangedPretoken {
    fn from(value: Token) -> Self {
        Self {
            range: value.range,
            token: Pretoken::Certain(value.kind),
        }
    }
}

pub(crate) struct PretokenStream<'a, 'b> {
    db: &'a dyn TokenDb,
    buffer: String,
    char_iter: TextCharIter<'b>,
}

impl<'a, 'b> PretokenStream<'a, 'b> {
    pub fn new(db: &'a dyn TokenDb, char_iter: TextCharIter<'b>) -> Self {
        let mut buffer = String::new();
        buffer.reserve_exact(100);
        Self {
            db,
            buffer,
            char_iter,
        }
    }
}

impl<'a, 'b: 'a> PretokenStream<'a, 'b> {
    fn skip_whitespaces(&mut self) {
        while let Some(' ') = self.char_iter.peek() {
            self.char_iter.next();
        }
    }

    fn next_word(&mut self) -> Pretoken {
        while let Some(c) = self.char_iter.peek() {
            if is_word_char(c) {
                self.eat_char();
            } else {
                break;
            }
        }
        let _len = self.buffer.len();
        self.take_buffer_word()
    }

    fn next_number(&mut self) -> Pretoken {
        let radix = 10;
        self.eat_chars_with(|c| char::is_digit(c, radix));
        if self.try_eat_char(|c| c == '.').is_some() {
            // parse float type
            self.eat_chars_with(|c| c.is_digit(radix));
            let float_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            let token = match float_suffix {
                "" => FloatLiteral::Unspecified.into(),
                "f8" => todo!(),
                "f16" => todo!(),
                "f32" => todo!(),
                "f64" => todo!(),
                "f128" => todo!(),
                "f256" => todo!(),
                invalid_float_suffix => todo!(),
            };
            self.buffer.clear();
            token
        } else {
            let integer_suffix = self.get_str_slice_with(|c| c.is_alphanumeric());
            let token: Pretoken = match integer_suffix {
                "" => IntegerLiteral::Unspecified.into(),
                "i8" => todo!(),
                "i16" => todo!(),
                "i32" => todo!(),
                "i64" => todo!(),
                "i128" => todo!(),
                "i256" => todo!(),
                "r8" => todo!(),
                "r16" => todo!(),
                "r32" => IntegerLiteral::R32(self.buffer.parse().unwrap()).into(),
                "r64" => IntegerLiteral::R64(self.buffer.parse().unwrap()).into(),
                "r128" => todo!(),
                "r256" => todo!(),
                "u8" => todo!(),
                "u16" => todo!(),
                "u32" => todo!(),
                "u64" => todo!(),
                "u128" => todo!(),
                "u256" => todo!(),
                invalid_integer_suffix => {
                    p!(invalid_integer_suffix);
                    todo!()
                }
            };
            self.buffer.clear();
            token
        }
    }

    fn take_buffer_word(&mut self) -> Pretoken {
        let word = std::mem::take(&mut self.buffer);
        self.new_word(word)
    }

    fn new_word(&self, word: String) -> Pretoken {
        if let Some(token_kind) = new_reserved_word(self.db, &word) {
            // ad hoc
            token_kind
        } else {
            let identifier = self.db.it_ident_owned(word).unwrap();
            TokenKind::Identifier(identifier).into()
        }
    }

    fn take_buffer<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        std::mem::take(&mut self.buffer).parse::<T>().unwrap()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek()
    }

    fn pass_two(&mut self, special: Punctuation) -> Punctuation {
        self.char_iter.next();
        special
    }

    fn eat_char(&mut self) {
        let c = self.char_iter.next().expect("what");
        self.buffer.push(c);
    }

    fn try_eat_char(&mut self, predicate: impl FnOnce(char) -> bool) -> Option<char> {
        if let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eat_chars_with(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.char_iter.peek() {
            if predicate(c) {
                self.eat_char();
            } else {
                break;
            }
        }
    }

    fn get_str_slice_with(&mut self, predicate: impl Fn(char) -> bool) -> &'b str {
        self.char_iter.get_str_slice_with(predicate)
    }

    fn ignore_char(&mut self) {
        let _c = self.char_iter.next().expect("what");
    }

    fn next_punctuation(&mut self, c_start: char) -> Option<Pretoken> {
        Some(
            match c_start {
                '=' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::Comparison(BinaryComparisonPunctuation::Eq),
                    )),
                    _ => Punctuation::BinaryOpr(BinaryPunctuation::Assign(None)),
                },
                ':' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::DeriveAssign),
                    Some(':') => {
                        self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::ScopeResolution))
                    }
                    _ => Punctuation::Colon,
                },
                '(' => Punctuation::Bra(Bracket::Par),
                '[' => Punctuation::Bra(Bracket::Box),
                '{' => Punctuation::Bra(Bracket::Curl),
                ')' => Punctuation::Ket(Bracket::Par),
                ']' => Punctuation::Ket(Bracket::Box),
                '}' => Punctuation::Ket(Bracket::Curl),
                ',' => Punctuation::Comma,
                '@' => Punctuation::At,
                '&' => match self.peek_char() {
                    Some('&') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::ShortcuitLogic(BinaryShortcuitLogicPunctuation::And),
                    )),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::BitAnd),
                    ))),
                    _ => Punctuation::Ambersand,
                },
                '|' => match self.peek_char() {
                    Some('|') => self.pass_two(Punctuation::DoubleVertical),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::BitOr),
                    ))),
                    _ => Punctuation::Vertical,
                },
                '~' => Punctuation::BitNot,
                '.' => Punctuation::Dot,
                ';' => Punctuation::Semicolon,
                '%' => Punctuation::BinaryOpr(BinaryPunctuation::PureClosed(
                    BinaryPureClosedPunctuation::RemEuclid,
                )),

                '-' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::Sub),
                    ))),
                    Some('-') => self.pass_two(Punctuation::Decr),
                    Some('>') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Curry)),
                    _ => return Some(Pretoken::Ambiguous(AmbiguousPretoken::SubOrMinus)),
                },
                '<' => match self.peek_char() {
                    Some('<') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::PureClosed(BinaryPureClosedPunctuation::Shl),
                    )),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::Comparison(BinaryComparisonPunctuation::Leq),
                    )),
                    _ => Punctuation::LAngle,
                },
                '>' => match self.peek_char() {
                    // '>' => self.pass_two(SpecialToken::Shr), // >>
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::Comparison(BinaryComparisonPunctuation::Geq),
                    )),
                    _ => Punctuation::RAngle,
                },
                '*' => match self.peek_char() {
                    Some('*') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::PureClosed(BinaryPureClosedPunctuation::Power),
                    )),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::Mul),
                    ))),
                    _ => Punctuation::BinaryOpr(BinaryPunctuation::PureClosed(
                        BinaryPureClosedPunctuation::Mul,
                    )),
                },
                '/' => match self.peek_char() {
                    Some('/') => unreachable!(),
                    Some('>') => self.pass_two(Punctuation::XmlKet),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::Div),
                    ))),
                    _ => Punctuation::BinaryOpr(BinaryPunctuation::PureClosed(
                        BinaryPureClosedPunctuation::Div,
                    )),
                },
                '+' => match self.peek_char() {
                    Some('+') => self.pass_two(Punctuation::Incr),
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(BinaryPunctuation::Assign(
                        Some(BinaryPureClosedPunctuation::Add),
                    ))),
                    _ => Punctuation::BinaryOpr(BinaryPunctuation::PureClosed(
                        BinaryPureClosedPunctuation::Add,
                    )),
                },
                '!' => match self.peek_char() {
                    Some('=') => self.pass_two(Punctuation::BinaryOpr(
                        BinaryPunctuation::Comparison(BinaryComparisonPunctuation::Neq),
                    )),
                    Some('!') => self.pass_two(Punctuation::DoubleExclamation),
                    _ => Punctuation::Exclamation,
                },
                '?' => Punctuation::QuestionMark,
                '#' => Punctuation::PoundSign,
                c => return Some(Pretoken::Err(TokenError::UnrecognizedChar(c))),
            }
            .into(),
        )
    }

    fn next_string_literal(&mut self) -> TokenResult<Pretoken> {
        let mut s = String::new();
        while let Some(c) = self.char_iter.next() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(c) = self.char_iter.next() {
                        match c {
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            c => return Err(TokenError::UnexpectedCharAfterBackslash),
                        }
                    } else {
                        return Err(TokenError::IncompleteStringLiteral);
                    }
                }
                '\n' => todo!(),
                c => s.push(c),
            }
        }
        Ok(Pretoken::Literal(LiteralToken::String(StringLiteral::new(
            s,
        ))))
    }

    fn next_token_variant(&mut self) -> Option<Pretoken> {
        let c = self.char_iter.next()?;
        if c == '\n' {
            Some(Pretoken::NewLine)
        } else if c == '"' {
            match self.next_string_literal() {
                Ok(v) => Some(v),
                Err(e) => {
                    // skip this line
                    while let Some(c) = self.char_iter.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                    Some(Pretoken::Err(e))
                }
            }
        } else if c == ' ' {
            unreachable!()
        } else if c.is_alphabetic() || c == '_' {
            self.buffer.push(c);
            Some(self.next_word())
        } else if c.is_digit(10) {
            self.buffer.push(c);
            Some(self.next_number())
        } else if c == '/' && self.char_iter.peek() == Some('/') {
            while let Some(c) = self.char_iter.peek() {
                if c == '\n' {
                    break;
                } else {
                    self.char_iter.next();
                }
            }
            Some(Pretoken::Comment)
        } else {
            self.next_punctuation(c)
        }
    }
}

impl<'token_line, 'lex: 'token_line> Iterator for PretokenStream<'token_line, 'lex> {
    type Item = RangedPretoken;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.char_iter.peek()?;
        match c {
            ' ' => {
                self.skip_whitespaces();
                self.next()
            }
            _ => {
                let start = self.char_iter.current_position();
                let variant = self.next_token_variant()?;
                Some(RangedPretoken {
                    range: (start..self.char_iter.current_position()).into(),
                    token: variant,
                })
            }
        }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}