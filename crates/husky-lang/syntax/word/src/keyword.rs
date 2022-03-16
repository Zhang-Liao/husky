use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Keyword {
    Config(ConfigKeyword),
    Routine(RoutineKeyword),
    Type(TypeKeyword),
    Stmt(StmtKeyword),
    Def,
    Use,
    Mod,
}

impl Deref for Keyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Keyword::Config(keyword) => keyword.deref(),
            Keyword::Routine(keyword) => keyword.deref(),
            Keyword::Type(keyword) => keyword.deref(),
            Keyword::Stmt(keyword) => keyword.deref(),
            Keyword::Use => "use",
            Keyword::Mod => "mod",
            Keyword::Def => "def",
        }
    }
}

impl From<ConfigKeyword> for Keyword {
    fn from(kw: ConfigKeyword) -> Self {
        Self::Config(kw)
    }
}

impl From<RoutineKeyword> for Keyword {
    fn from(kw: RoutineKeyword) -> Self {
        Keyword::Routine(kw)
    }
}

impl From<TypeKeyword> for Keyword {
    fn from(kw: TypeKeyword) -> Self {
        Keyword::Type(kw)
    }
}

impl From<StmtKeyword> for Keyword {
    fn from(stmt: StmtKeyword) -> Self {
        Keyword::Stmt(stmt)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ConfigKeyword {
    Dataset,
}

impl Deref for ConfigKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            ConfigKeyword::Dataset => "dataset",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RoutineKeyword {
    Main,
    Test,
    Proc,
    Func,
}

impl Deref for RoutineKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            RoutineKeyword::Main => "main",
            RoutineKeyword::Test => "test",
            RoutineKeyword::Proc => "proc",
            RoutineKeyword::Func => "func",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeKeyword {
    Struct,
    Rename,
    Enum,
    Props,
}

impl Deref for TypeKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            TypeKeyword::Struct => "struct",
            TypeKeyword::Rename => "rename",
            TypeKeyword::Enum => "enum",
            TypeKeyword::Props => "props",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum StmtKeyword {
    Let,
    Var,
    If,
    Elif,
    Else,
    Switch,
    Match,
    Case,
    DeFault,
    For,
    ForExt,
    While,
    Do,
    Break,
    Return,
    Assert,
}

impl Deref for StmtKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            StmtKeyword::Let => "let",
            StmtKeyword::Var => "var",
            StmtKeyword::If => "if",
            StmtKeyword::Elif => "elif",
            StmtKeyword::Else => "else",
            StmtKeyword::Switch => "switch",
            StmtKeyword::Match => "match",
            StmtKeyword::Case => "case",
            StmtKeyword::DeFault => "default",
            StmtKeyword::For => "for",
            StmtKeyword::ForExt => "forext",
            StmtKeyword::While => "while",
            StmtKeyword::Do => "do",
            StmtKeyword::Break => "break",
            StmtKeyword::Return => "return",
            StmtKeyword::Assert => "assert",
        }
    }
}
