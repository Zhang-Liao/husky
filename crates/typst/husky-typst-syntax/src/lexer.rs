use ecow::{eco_format, EcoString};
use unicode_ident::{is_xid_continue, is_xid_start};
use unicode_script::{Script, UnicodeScript};
use unicode_segmentation::UnicodeSegmentation;
use unscanny::Scanner;

use crate::TypstSyntaxKind;

/// Splits up a string of source code into tokens.
#[derive(Clone)]
pub(super) struct Lexer<'s> {
    /// The underlying scanner.
    s: Scanner<'s>,
    /// The mode the lexer is in. This determines which kinds of tokens it
    /// produces.
    mode: LexMode,
    /// Whether the last token contained a newline.
    newline: bool,
    /// An error for the last token.
    error: Option<EcoString>,
}

/// What kind of tokens to emit.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum LexMode {
    /// Text and markup.
    TypstMarkup,
    /// Math atoms, operators, etc.
    Math,
    /// Keywords, literals and operators.
    Code,
}

impl<'s> Lexer<'s> {
    /// Create a new lexer with the given mode and a prefix to offset column
    /// calculations.
    pub fn new(text: &'s str, mode: LexMode) -> Self {
        Self {
            s: Scanner::new(text),
            mode,
            newline: false,
            error: None,
        }
    }

    /// Get the current lexing mode.
    pub fn mode(&self) -> LexMode {
        self.mode
    }

    /// Change the lexing mode.
    pub fn set_mode(&mut self, mode: LexMode) {
        self.mode = mode;
    }

    /// The index in the string at which the last token ends and next token
    /// will start.
    pub fn cursor(&self) -> usize {
        self.s.cursor()
    }

    /// Jump to the given index in the string.
    pub fn jump(&mut self, index: usize) {
        self.s.jump(index);
    }

    /// Whether the last token contained a newline.
    pub fn newline(&self) -> bool {
        self.newline
    }

    /// Take out the last error, if any.
    pub fn take_error(&mut self) -> Option<EcoString> {
        self.error.take()
    }
}

impl Lexer<'_> {
    /// Construct a full-positioned syntax error.
    fn error(&mut self, message: impl Into<EcoString>) -> TypstSyntaxKind {
        self.error = Some(message.into());
        TypstSyntaxKind::Error
    }
}

/// Shared.
impl Lexer<'_> {
    pub fn next(&mut self) -> TypstSyntaxKind {
        self.newline = false;
        self.error = None;
        let start = self.s.cursor();
        match self.s.eat() {
            Some(c) if is_space(c, self.mode) => self.whitespace(start, c),
            Some('/') if self.s.eat_if('/') => self.line_comment(),
            Some('/') if self.s.eat_if('*') => self.block_comment(),
            Some('*') if self.s.eat_if('/') => self.error("unexpected end of block comment"),

            Some(c) => match self.mode {
                LexMode::TypstMarkup => self.markup(start, c),
                LexMode::Math => self.math(start, c),
                LexMode::Code => self.code(start, c),
            },

            None => TypstSyntaxKind::Eof,
        }
    }

    fn whitespace(&mut self, start: usize, c: char) -> TypstSyntaxKind {
        let more = self.s.eat_while(|c| is_space(c, self.mode));
        let newlines = match c {
            ' ' if more.is_empty() => 0,
            _ => count_newlines(self.s.from(start)),
        };

        self.newline = newlines > 0;
        if self.mode == LexMode::TypstMarkup && newlines >= 2 {
            TypstSyntaxKind::Parbreak
        } else {
            TypstSyntaxKind::Space
        }
    }

    fn line_comment(&mut self) -> TypstSyntaxKind {
        self.s.eat_until(is_newline);
        TypstSyntaxKind::LineComment
    }

    fn block_comment(&mut self) -> TypstSyntaxKind {
        let mut state = '_';
        let mut depth = 1;

        // Find the first `*/` that does not correspond to a nested `/*`.
        while let Some(c) = self.s.eat() {
            state = match (state, c) {
                ('*', '/') => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    '_'
                }
                ('/', '*') => {
                    depth += 1;
                    '_'
                }
                _ => c,
            }
        }

        TypstSyntaxKind::BlockComment
    }
}

/// TypstMarkup.
impl Lexer<'_> {
    fn markup(&mut self, start: usize, c: char) -> TypstSyntaxKind {
        match c {
            '\\' => self.backslash(),
            '`' => self.raw(),
            'h' if self.s.eat_if("ttp://") => self.link(),
            'h' if self.s.eat_if("ttps://") => self.link(),
            '<' if self.s.at(is_id_continue) => self.label(),
            '@' => self.ref_marker(),

            '.' if self.s.eat_if("..") => TypstSyntaxKind::Shorthand,
            '-' if self.s.eat_if("--") => TypstSyntaxKind::Shorthand,
            '-' if self.s.eat_if('-') => TypstSyntaxKind::Shorthand,
            '-' if self.s.eat_if('?') => TypstSyntaxKind::Shorthand,
            '-' if self.s.at(char::is_numeric) => TypstSyntaxKind::Shorthand,
            '*' if !self.in_word() => TypstSyntaxKind::Star,
            '_' if !self.in_word() => TypstSyntaxKind::Underscore,

            '#' => TypstSyntaxKind::Hash,
            '[' => TypstSyntaxKind::LeftDelimiter,
            ']' => TypstSyntaxKind::RightDelimiter,
            '\'' => TypstSyntaxKind::SmartQuote,
            '"' => TypstSyntaxKind::SmartQuote,
            '$' => TypstSyntaxKind::Dollar,
            '~' => TypstSyntaxKind::Shorthand,
            ':' => TypstSyntaxKind::Colon,
            '=' => {
                self.s.eat_while('=');
                if self.space_or_end() {
                    TypstSyntaxKind::HeadingMarker
                } else {
                    self.text()
                }
            }
            '-' if self.space_or_end() => TypstSyntaxKind::ListMarker,
            '+' if self.space_or_end() => TypstSyntaxKind::EnumMarker,
            '/' if self.space_or_end() => TypstSyntaxKind::TermMarker,
            '0'..='9' => self.numbering(start),

            _ => self.text(),
        }
    }

    fn backslash(&mut self) -> TypstSyntaxKind {
        if self.s.eat_if("u{") {
            let hex = self.s.eat_while(char::is_ascii_alphanumeric);
            if !self.s.eat_if('}') {
                return self.error("unclosed Unicode escape sequence");
            }

            if u32::from_str_radix(hex, 16)
                .ok()
                .and_then(std::char::from_u32)
                .is_none()
            {
                return self.error(eco_format!("invalid Unicode codepoint: {}", hex));
            }

            return TypstSyntaxKind::Escape;
        }

        if self.s.done() || self.s.at(char::is_whitespace) {
            TypstSyntaxKind::Linebreak
        } else {
            self.s.eat();
            TypstSyntaxKind::Escape
        }
    }

    fn raw(&mut self) -> TypstSyntaxKind {
        let mut backticks = 1;
        while self.s.eat_if('`') {
            backticks += 1;
        }

        if backticks == 2 {
            return TypstSyntaxKind::Raw;
        }

        let mut found = 0;
        while found < backticks {
            match self.s.eat() {
                Some('`') => found += 1,
                Some(_) => found = 0,
                None => break,
            }
        }

        if found != backticks {
            return self.error("unclosed raw text");
        }

        TypstSyntaxKind::Raw
    }

    fn link(&mut self) -> TypstSyntaxKind {
        let (link, balanced) = link_prefix(self.s.after());
        self.s.jump(self.s.cursor() + link.len());

        if !balanced {
            return self.error(
                "automatic links cannot contain unbalanced brackets, \
                 use the `link` function instead",
            );
        }

        TypstSyntaxKind::Link
    }

    fn numbering(&mut self, start: usize) -> TypstSyntaxKind {
        self.s.eat_while(char::is_ascii_digit);

        let read = self.s.from(start);
        if self.s.eat_if('.') && self.space_or_end() && read.parse::<usize>().is_ok() {
            return TypstSyntaxKind::EnumMarker;
        }

        self.text()
    }

    fn ref_marker(&mut self) -> TypstSyntaxKind {
        self.s
            .eat_while(|c| is_id_continue(c) || matches!(c, ':' | '.'));

        // Don't include the trailing characters likely to be part of text.
        while matches!(self.s.scout(-1), Some('.' | ':')) {
            self.s.uneat();
        }

        TypstSyntaxKind::RefMarker
    }

    fn label(&mut self) -> TypstSyntaxKind {
        let label = self
            .s
            .eat_while(|c| is_id_continue(c) || matches!(c, ':' | '.'));
        if label.is_empty() {
            return self.error("label cannot be empty");
        }

        if !self.s.eat_if('>') {
            return self.error("unclosed label");
        }

        TypstSyntaxKind::Label
    }

    fn text(&mut self) -> TypstSyntaxKind {
        macro_rules! table {
            ($(|$c:literal)*) => {
                static TABLE: [bool; 128] = {
                    let mut t = [false; 128];
                    $(t[$c as usize] = true;)*
                    t
                };
            };
        }

        table! {
            | ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\\' | '/'
            | '[' | ']' | '~' | '-' | '.' | '\'' | '"' | '*' | '_'
            | ':' | 'h' | '`' | '$' | '<' | '>' | '@' | '#'
        };

        loop {
            self.s.eat_until(|c: char| {
                TABLE
                    .get(c as usize)
                    .copied()
                    .unwrap_or_else(|| c.is_whitespace())
            });

            // Continue with the same text node if the thing would become text
            // anyway.
            let mut s = self.s;
            match s.eat() {
                Some(' ') if s.at(char::is_alphanumeric) => {}
                Some('/') if !s.at(['/', '*']) => {}
                Some('-') if !s.at(['-', '?']) => {}
                Some('.') if !s.at("..") => {}
                Some('h') if !s.at("ttp://") && !s.at("ttps://") => {}
                Some('@') if !s.at(is_id_start) => {}
                _ => break,
            }

            self.s = s;
        }

        TypstSyntaxKind::Text
    }

    fn in_word(&self) -> bool {
        let wordy = |c: Option<char>| {
            c.map_or(false, |c| {
                c.is_alphanumeric()
                    && !matches!(
                        c.script(),
                        Script::Han | Script::Hiragana | Script::Katakana
                    )
            })
        };
        let prev = self.s.scout(-2);
        let next = self.s.peek();
        wordy(prev) && wordy(next)
    }

    fn space_or_end(&self) -> bool {
        self.s.done() || self.s.at(char::is_whitespace)
    }
}

/// Math.
impl Lexer<'_> {
    fn math(&mut self, start: usize, c: char) -> TypstSyntaxKind {
        match c {
            '\\' => self.backslash(),
            '"' => self.string(),

            '-' if self.s.eat_if(">>") => TypstSyntaxKind::Shorthand,
            '-' if self.s.eat_if('>') => TypstSyntaxKind::Shorthand,
            '-' if self.s.eat_if("->") => TypstSyntaxKind::Shorthand,
            ':' if self.s.eat_if('=') => TypstSyntaxKind::Shorthand,
            ':' if self.s.eat_if(":=") => TypstSyntaxKind::Shorthand,
            '!' if self.s.eat_if('=') => TypstSyntaxKind::Shorthand,
            '.' if self.s.eat_if("..") => TypstSyntaxKind::Shorthand,
            '[' if self.s.eat_if('|') => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("==>") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("-->") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("--") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("-<") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("->") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("<-") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("<<") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("=>") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("==") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if("~~") => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if('=') => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if('<') => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if('-') => TypstSyntaxKind::Shorthand,
            '<' if self.s.eat_if('~') => TypstSyntaxKind::Shorthand,
            '>' if self.s.eat_if("->") => TypstSyntaxKind::Shorthand,
            '>' if self.s.eat_if(">>") => TypstSyntaxKind::Shorthand,
            '=' if self.s.eat_if("=>") => TypstSyntaxKind::Shorthand,
            '=' if self.s.eat_if('>') => TypstSyntaxKind::Shorthand,
            '=' if self.s.eat_if(':') => TypstSyntaxKind::Shorthand,
            '>' if self.s.eat_if('=') => TypstSyntaxKind::Shorthand,
            '>' if self.s.eat_if('>') => TypstSyntaxKind::Shorthand,
            '|' if self.s.eat_if("->") => TypstSyntaxKind::Shorthand,
            '|' if self.s.eat_if("=>") => TypstSyntaxKind::Shorthand,
            '|' if self.s.eat_if(']') => TypstSyntaxKind::Shorthand,
            '|' if self.s.eat_if('|') => TypstSyntaxKind::Shorthand,
            '~' if self.s.eat_if("~>") => TypstSyntaxKind::Shorthand,
            '~' if self.s.eat_if('>') => TypstSyntaxKind::Shorthand,
            '*' | '-' => TypstSyntaxKind::Shorthand,

            '#' => TypstSyntaxKind::Hash,
            '_' => TypstSyntaxKind::Underscore,
            '$' => TypstSyntaxKind::Dollar,
            '/' => TypstSyntaxKind::Slash,
            '^' => TypstSyntaxKind::Hat,
            '\'' => TypstSyntaxKind::Prime,
            '&' => TypstSyntaxKind::MathAlignPoint,
            '√' | '∛' | '∜' => TypstSyntaxKind::Root,

            // Identifiers.
            c if is_math_id_start(c) && self.s.at(is_math_id_continue) => {
                self.s.eat_while(is_math_id_continue);
                TypstSyntaxKind::MathIdent
            }

            // Other math atoms.
            _ => self.math_text(start, c),
        }
    }

    fn math_text(&mut self, start: usize, c: char) -> TypstSyntaxKind {
        // Keep numbers and grapheme clusters together.
        if c.is_numeric() {
            self.s.eat_while(char::is_numeric);
            let mut s = self.s;
            if s.eat_if('.') && !s.eat_while(char::is_numeric).is_empty() {
                self.s = s;
            }
        } else {
            let len = self
                .s
                .get(start..self.s.string().len())
                .graphemes(true)
                .next()
                .map_or(0, str::len);
            self.s.jump(start + len);
        }
        TypstSyntaxKind::Text
    }
}

/// Code.
impl Lexer<'_> {
    fn code(&mut self, start: usize, c: char) -> TypstSyntaxKind {
        match c {
            '`' => self.raw(),
            '<' if self.s.at(is_id_continue) => self.label(),
            '0'..='9' => self.number(start, c),
            '.' if self.s.at(char::is_ascii_digit) => self.number(start, c),
            '"' => self.string(),

            '=' if self.s.eat_if('=') => TypstSyntaxKind::EqEq,
            '!' if self.s.eat_if('=') => TypstSyntaxKind::ExclEq,
            '<' if self.s.eat_if('=') => TypstSyntaxKind::LtEq,
            '>' if self.s.eat_if('=') => TypstSyntaxKind::GtEq,
            '+' if self.s.eat_if('=') => TypstSyntaxKind::PlusEq,
            '-' | '\u{2212}' if self.s.eat_if('=') => TypstSyntaxKind::HyphEq,
            '*' if self.s.eat_if('=') => TypstSyntaxKind::StarEq,
            '/' if self.s.eat_if('=') => TypstSyntaxKind::SlashEq,
            '.' if self.s.eat_if('.') => TypstSyntaxKind::Dots,
            '=' if self.s.eat_if('>') => TypstSyntaxKind::Arrow,

            '{' => TypstSyntaxKind::LeftBrace,
            '}' => TypstSyntaxKind::RightBrace,
            '[' => TypstSyntaxKind::LeftDelimiter,
            ']' => TypstSyntaxKind::RightDelimiter,
            '(' => TypstSyntaxKind::LeftParen,
            ')' => TypstSyntaxKind::RightParen,
            '$' => TypstSyntaxKind::Dollar,
            ',' => TypstSyntaxKind::Comma,
            ';' => TypstSyntaxKind::Semicolon,
            ':' => TypstSyntaxKind::Colon,
            '.' => TypstSyntaxKind::Dot,
            '+' => TypstSyntaxKind::Plus,
            '-' | '\u{2212}' => TypstSyntaxKind::Minus,
            '*' => TypstSyntaxKind::Star,
            '/' => TypstSyntaxKind::Slash,
            '=' => TypstSyntaxKind::Eq,
            '<' => TypstSyntaxKind::Lt,
            '>' => TypstSyntaxKind::Gt,

            c if is_id_start(c) => self.ident(start),

            c => self.error(eco_format!("the character `{c}` is not valid in code")),
        }
    }

    fn ident(&mut self, start: usize) -> TypstSyntaxKind {
        self.s.eat_while(is_id_continue);
        let ident = self.s.from(start);

        let prev = self.s.get(0..start);
        if !prev.ends_with(['.', '@']) || prev.ends_with("..") {
            if let Some(keyword) = keyword(ident) {
                return keyword;
            }
        }

        if ident == "_" {
            TypstSyntaxKind::Underscore
        } else {
            TypstSyntaxKind::Ident
        }
    }

    fn number(&mut self, mut start: usize, c: char) -> TypstSyntaxKind {
        // Handle alternative integer bases.
        let mut base = 10;
        if c == '0' {
            if self.s.eat_if('b') {
                base = 2;
            } else if self.s.eat_if('o') {
                base = 8;
            } else if self.s.eat_if('x') {
                base = 16;
            }
            if base != 10 {
                start = self.s.cursor();
            }
        }

        // Read the first part (integer or fractional depending on `first`).
        self.s.eat_while(if base == 16 {
            char::is_ascii_alphanumeric
        } else {
            char::is_ascii_digit
        });

        // Read the fractional part if not already done.
        // Make sure not to confuse a range for the decimal separator.
        if c != '.'
            && !self.s.at("..")
            && !self.s.scout(1).map_or(false, is_id_start)
            && self.s.eat_if('.')
            && base == 10
        {
            self.s.eat_while(char::is_ascii_digit);
        }

        // Read the exponent.
        if !self.s.at("em") && self.s.eat_if(['e', 'E']) && base == 10 {
            self.s.eat_if(['+', '-']);
            self.s.eat_while(char::is_ascii_digit);
        }

        // Read the suffix.
        let suffix_start = self.s.cursor();
        if !self.s.eat_if('%') {
            self.s.eat_while(char::is_ascii_alphanumeric);
        }

        let number = self.s.get(start..suffix_start);
        let suffix = self.s.from(suffix_start);

        let kind = if i64::from_str_radix(number, base).is_ok() {
            TypstSyntaxKind::Int
        } else if base == 10 && number.parse::<f64>().is_ok() {
            TypstSyntaxKind::Float
        } else {
            return self.error(match base {
                2 => eco_format!("invalid binary number: 0b{}", number),
                8 => eco_format!("invalid octal number: 0o{}", number),
                16 => eco_format!("invalid hexadecimal number: 0x{}", number),
                _ => eco_format!("invalid number: {}", number),
            });
        };

        if suffix.is_empty() {
            return kind;
        }

        if !matches!(
            suffix,
            "pt" | "mm" | "cm" | "in" | "deg" | "rad" | "em" | "fr" | "%"
        ) {
            return self.error(eco_format!("invalid number suffix: {}", suffix));
        }

        TypstSyntaxKind::Numeric
    }

    fn string(&mut self) -> TypstSyntaxKind {
        let mut escaped = false;
        self.s.eat_until(|c| {
            let stop = c == '"' && !escaped;
            escaped = c == '\\' && !escaped;
            stop
        });

        if !self.s.eat_if('"') {
            return self.error("unclosed string");
        }

        TypstSyntaxKind::Str
    }
}

/// Try to parse an identifier into a keyword.
fn keyword(ident: &str) -> Option<TypstSyntaxKind> {
    Some(match ident {
        "none" => TypstSyntaxKind::None,
        "auto" => TypstSyntaxKind::Auto,
        "true" => TypstSyntaxKind::Bool,
        "false" => TypstSyntaxKind::Bool,
        "not" => TypstSyntaxKind::Not,
        "and" => TypstSyntaxKind::And,
        "or" => TypstSyntaxKind::Or,
        "let" => TypstSyntaxKind::Let,
        "set" => TypstSyntaxKind::Set,
        "show" => TypstSyntaxKind::Show,
        "if" => TypstSyntaxKind::If,
        "else" => TypstSyntaxKind::Else,
        "for" => TypstSyntaxKind::For,
        "in" => TypstSyntaxKind::In,
        "while" => TypstSyntaxKind::While,
        "break" => TypstSyntaxKind::Break,
        "continue" => TypstSyntaxKind::Continue,
        "return" => TypstSyntaxKind::Return,
        "import" => TypstSyntaxKind::Import,
        "include" => TypstSyntaxKind::Include,
        "as" => TypstSyntaxKind::As,
        _ => return None,
    })
}

/// Whether a character will become a Space token in Typst
#[inline]
fn is_space(character: char, mode: LexMode) -> bool {
    match mode {
        LexMode::TypstMarkup => matches!(character, ' ' | '\t') || is_newline(character),
        _ => character.is_whitespace(),
    }
}

/// Whether a character is interpreted as a newline by Typst.
#[inline]
pub fn is_newline(character: char) -> bool {
    matches!(
        character,
        // Line Feed, Vertical Tab, Form Feed, Carriage Return.
        '\n' | '\x0B' | '\x0C' | '\r' |
        // Next Line, Line Separator, Paragraph Separator.
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

/// Extracts a prefix of the text that is a link and also returns whether the
/// parentheses and brackets in the link were balanced.
pub fn link_prefix(text: &str) -> (&str, bool) {
    let mut s = unscanny::Scanner::new(text);
    let mut brackets = Vec::new();

    #[rustfmt::skip]
    s.eat_while(|c: char| {
        match c {
            | '0' ..= '9'
            | 'a' ..= 'z'
            | 'A' ..= 'Z'
            | '!' | '#' | '$' | '%' | '&' | '*' | '+'
            | ',' | '-' | '.' | '/' | ':' | ';' | '='
            | '?' | '@' | '_' | '~' | '\'' => true,
            '[' => {
                brackets.push(b'[');
                true
            }
            '(' => {
                brackets.push(b'(');
                true
            }
            ']' => brackets.pop() == Some(b'['),
            ')' => brackets.pop() == Some(b'('),
            _ => false,
        }
    });

    // Don't include the trailing characters likely to be part of text.
    while matches!(s.scout(-1), Some('!' | ',' | '.' | ':' | ';' | '?' | '\'')) {
        s.uneat();
    }

    (s.before(), brackets.is_empty())
}

/// Split text at newlines.
pub fn split_newlines(text: &str) -> Vec<&str> {
    let mut s = Scanner::new(text);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut end = 0;

    while let Some(c) = s.eat() {
        if is_newline(c) {
            if c == '\r' {
                s.eat_if('\n');
            }

            lines.push(&text[start..end]);
            start = s.cursor();
        }
        end = s.cursor();
    }

    lines.push(&text[start..]);
    lines
}

/// Count the number of newlines in text.
fn count_newlines(text: &str) -> usize {
    let mut newlines = 0;
    let mut s = Scanner::new(text);
    while let Some(c) = s.eat() {
        if is_newline(c) {
            if c == '\r' {
                s.eat_if('\n');
            }
            newlines += 1;
        }
    }
    newlines
}

/// Whether a string is a valid Typst identifier.
///
/// In addition to what is specified in the [Unicode Standard][uax31], we allow:
/// - `_` as a starting character,
/// - `_` and `-` as continuing characters.
///
/// [uax31]: http://www.unicode.org/reports/tr31/
#[inline]
pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    chars
        .next()
        .map_or(false, |c| is_id_start(c) && chars.all(is_id_continue))
}

/// Whether a character can start an identifier.
#[inline]
pub fn is_id_start(c: char) -> bool {
    is_xid_start(c) || c == '_'
}

/// Whether a character can continue an identifier.
#[inline]
pub fn is_id_continue(c: char) -> bool {
    is_xid_continue(c) || c == '_' || c == '-'
}

/// Whether a character can start an identifier in math.
#[inline]
fn is_math_id_start(c: char) -> bool {
    is_xid_start(c)
}

/// Whether a character can continue an identifier in math.
#[inline]
fn is_math_id_continue(c: char) -> bool {
    is_xid_continue(c) && c != '_'
}