// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Python {
    End = 0,
    Identifier = 1,
    SEMI = 2,
    Import = 3,
    DOT = 4,
    From = 5,
    Future = 6,
    LPAREN = 7,
    RPAREN = 8,
    COMMA = 9,
    As = 10,
    STAR = 11,
    Print = 12,
    GTGT = 13,
    Assert = 14,
    COLONEQ = 15,
    Match = 16,
    Return = 17,
    Del = 18,
    Raise = 19,
    Pass = 20,
    Break = 21,
    Continue = 22,
    If = 23,
    COLON = 24,
    Elif = 25,
    Else = 26,
    Case = 27,
    Async = 28,
    For = 29,
    In = 30,
    While = 31,
    Try = 32,
    Except = 33,
    Finally = 34,
    With = 35,
    Def = 36,
    DASHGT = 37,
    STARSTAR = 38,
    Global = 39,
    Nonlocal = 40,
    Exec = 41,
    Class = 42,
    AT = 43,
    LBRACK = 44,
    RBRACK = 45,
    EQ = 46,
    Not = 47,
    And = 48,
    Or = 49,
    PLUS = 50,
    DASH = 51,
    SLASH = 52,
    PERCENT = 53,
    SLASHSLASH = 54,
    PIPE = 55,
    AMP = 56,
    CARET = 57,
    LTLT = 58,
    TILDE = 59,
    LT = 60,
    LTEQ = 61,
    EQEQ = 62,
    BANGEQ = 63,
    GTEQ = 64,
    GT = 65,
    LTGT = 66,
    Is = 67,
    Lambda3 = 68,
    PLUSEQ = 69,
    DASHEQ = 70,
    STAREQ = 71,
    SLASHEQ = 72,
    ATEQ = 73,
    SLASHSLASHEQ = 74,
    PERCENTEQ = 75,
    STARSTAREQ = 76,
    GTGTEQ = 77,
    LTLTEQ = 78,
    AMPEQ = 79,
    CARETEQ = 80,
    PIPEEQ = 81,
    Yield2 = 82,
    Ellipsis = 83,
    LBRACE = 84,
    RBRACE = 85,
    LBRACELBRACE = 86,
    RBRACERBRACE = 87,
    EscapeSequence = 88,
    NotEscapeSequence = 89,
    FormatSpecifierToken1 = 90,
    TypeConversion = 91,
    Integer = 92,
    Float = 93,
    Await2 = 94,
    True = 95,
    False = 96,
    None = 97,
    Comment = 98,
    Newline = 99,
    Indent = 100,
    Dedent = 101,
    DQUOTE = 102,
    StringContent = 103,
    DQUOTE2 = 104,
    Module = 105,
    Statement = 106,
    SimpleStatements = 107,
    ImportStatement = 108,
    ImportPrefix = 109,
    RelativeImport = 110,
    FutureImportStatement = 111,
    ImportFromStatement = 112,
    ImportList = 113,
    AliasedImport = 114,
    WildcardImport = 115,
    PrintStatement = 116,
    Chevron = 117,
    AssertStatement = 118,
    ExpressionStatement = 119,
    NamedExpression = 120,
    NamedExpresssionLhs = 121,
    ReturnStatement = 122,
    DeleteStatement = 123,
    RaiseStatement = 124,
    PassStatement = 125,
    BreakStatement = 126,
    ContinueStatement = 127,
    IfStatement = 128,
    ElifClause = 129,
    ElseClause = 130,
    MatchStatement = 131,
    CaseClause = 132,
    ForStatement = 133,
    WhileStatement = 134,
    TryStatement = 135,
    ExceptClause = 136,
    FinallyClause = 137,
    WithStatement = 138,
    WithClause = 139,
    WithItem = 140,
    FunctionDefinition = 141,
    Parameters = 142,
    LambdaParameters = 143,
    ListSplat = 144,
    DictionarySplat = 145,
    GlobalStatement = 146,
    NonlocalStatement = 147,
    ExecStatement = 148,
    ClassDefinition = 149,
    ParenthesizedListSplat = 150,
    ArgumentList = 151,
    DecoratedDefinition = 152,
    Decorator = 153,
    Block = 154,
    ExpressionList = 155,
    DottedName = 156,
    Parameters2 = 157,
    Patterns = 158,
    Parameter = 159,
    Pattern = 160,
    TuplePattern = 161,
    ListPattern = 162,
    DefaultParameter = 163,
    TypedDefaultParameter = 164,
    ListSplatPattern = 165,
    DictionarySplatPattern = 166,
    AsPattern = 167,
    ExpressionWithinForInClause = 168,
    Expression = 169,
    PrimaryExpression = 170,
    NotOperator = 171,
    BooleanOperator = 172,
    BinaryOperator = 173,
    UnaryOperator = 174,
    ComparisonOperator = 175,
    Lambda = 176,
    Lambda2 = 177,
    Assignment = 178,
    AugmentedAssignment = 179,
    PatternList = 180,
    RightHandSide = 181,
    Yield = 182,
    Attribute = 183,
    Subscript = 184,
    Slice = 185,
    Call = 186,
    TypedParameter = 187,
    Type = 188,
    KeywordArgument = 189,
    List = 190,
    Set = 191,
    Tuple = 192,
    Dictionary = 193,
    Pair = 194,
    ListComprehension = 195,
    DictionaryComprehension = 196,
    SetComprehension = 197,
    GeneratorExpression = 198,
    ComprehensionClauses = 199,
    ParenthesizedExpression = 200,
    CollectionElements = 201,
    ForInClause = 202,
    IfClause = 203,
    ConditionalExpression = 204,
    ConcatenatedString = 205,
    String = 206,
    Interpolation = 207,
    EscapeInterpolation = 208,
    FormatSpecifier = 209,
    FormatExpression = 210,
    Await = 211,
    PositionalSeparator = 212,
    KeywordSeparator = 213,
    ModuleRepeat1 = 214,
    SimpleStatementsRepeat1 = 215,
    ImportPrefixRepeat1 = 216,
    ImportListRepeat1 = 217,
    PrintStatementRepeat1 = 218,
    AssertStatementRepeat1 = 219,
    IfStatementRepeat1 = 220,
    MatchStatementRepeat1 = 221,
    MatchStatementRepeat2 = 222,
    CaseClauseRepeat1 = 223,
    TryStatementRepeat1 = 224,
    WithClauseRepeat1 = 225,
    GlobalStatementRepeat1 = 226,
    ArgumentListRepeat1 = 227,
    DecoratedDefinitionRepeat1 = 228,
    DottedNameRepeat1 = 229,
    ParametersRepeat1 = 230,
    PatternsRepeat1 = 231,
    ComparisonOperatorRepeat1 = 232,
    SubscriptRepeat1 = 233,
    DictionaryRepeat1 = 234,
    ComprehensionClausesRepeat1 = 235,
    CollectionElementsRepeat1 = 236,
    ForInClauseRepeat1 = 237,
    ConcatenatedStringRepeat1 = 238,
    StringRepeat1 = 239,
    FormatSpecifierRepeat1 = 240,
    AsPatternTarget = 241,
    CasePattern = 242,
    Error = 243,
}

impl From<Python> for &'static str {
    #[inline(always)]
    fn from(tok: Python) -> Self {
        match tok {
            Python::End => "end",
            Python::Identifier => "identifier",
            Python::SEMI => ";",
            Python::Import => "import",
            Python::DOT => ".",
            Python::From => "from",
            Python::Future => "__future__",
            Python::LPAREN => "(",
            Python::RPAREN => ")",
            Python::COMMA => ",",
            Python::As => "as",
            Python::STAR => "*",
            Python::Print => "print",
            Python::GTGT => ">>",
            Python::Assert => "assert",
            Python::COLONEQ => ":=",
            Python::Match => "match",
            Python::Return => "return",
            Python::Del => "del",
            Python::Raise => "raise",
            Python::Pass => "pass",
            Python::Break => "break",
            Python::Continue => "continue",
            Python::If => "if",
            Python::COLON => ":",
            Python::Elif => "elif",
            Python::Else => "else",
            Python::Case => "case",
            Python::Async => "async",
            Python::For => "for",
            Python::In => "in",
            Python::While => "while",
            Python::Try => "try",
            Python::Except => "except",
            Python::Finally => "finally",
            Python::With => "with",
            Python::Def => "def",
            Python::DASHGT => "->",
            Python::STARSTAR => "**",
            Python::Global => "global",
            Python::Nonlocal => "nonlocal",
            Python::Exec => "exec",
            Python::Class => "class",
            Python::AT => "@",
            Python::LBRACK => "[",
            Python::RBRACK => "]",
            Python::EQ => "=",
            Python::Not => "not",
            Python::And => "and",
            Python::Or => "or",
            Python::PLUS => "+",
            Python::DASH => "-",
            Python::SLASH => "/",
            Python::PERCENT => "%",
            Python::SLASHSLASH => "//",
            Python::PIPE => "|",
            Python::AMP => "&",
            Python::CARET => "^",
            Python::LTLT => "<<",
            Python::TILDE => "~",
            Python::LT => "<",
            Python::LTEQ => "<=",
            Python::EQEQ => "==",
            Python::BANGEQ => "!=",
            Python::GTEQ => ">=",
            Python::GT => ">",
            Python::LTGT => "<>",
            Python::Is => "is",
            Python::Lambda3 => "lambda",
            Python::PLUSEQ => "+=",
            Python::DASHEQ => "-=",
            Python::STAREQ => "*=",
            Python::SLASHEQ => "/=",
            Python::ATEQ => "@=",
            Python::SLASHSLASHEQ => "//=",
            Python::PERCENTEQ => "%=",
            Python::STARSTAREQ => "**=",
            Python::GTGTEQ => ">>=",
            Python::LTLTEQ => "<<=",
            Python::AMPEQ => "&=",
            Python::CARETEQ => "^=",
            Python::PIPEEQ => "|=",
            Python::Yield2 => "yield",
            Python::Ellipsis => "ellipsis",
            Python::LBRACE => "{",
            Python::RBRACE => "}",
            Python::LBRACELBRACE => "{{",
            Python::RBRACERBRACE => "}}",
            Python::EscapeSequence => "escape_sequence",
            Python::NotEscapeSequence => "_not_escape_sequence",
            Python::FormatSpecifierToken1 => "format_specifier_token1",
            Python::TypeConversion => "type_conversion",
            Python::Integer => "integer",
            Python::Float => "float",
            Python::Await2 => "await",
            Python::True => "true",
            Python::False => "false",
            Python::None => "none",
            Python::Comment => "comment",
            Python::Newline => "_newline",
            Python::Indent => "_indent",
            Python::Dedent => "_dedent",
            Python::DQUOTE => "\"",
            Python::StringContent => "_string_content",
            Python::DQUOTE2 => "\"",
            Python::Module => "module",
            Python::Statement => "_statement",
            Python::SimpleStatements => "_simple_statements",
            Python::ImportStatement => "import_statement",
            Python::ImportPrefix => "import_prefix",
            Python::RelativeImport => "relative_import",
            Python::FutureImportStatement => "future_import_statement",
            Python::ImportFromStatement => "import_from_statement",
            Python::ImportList => "_import_list",
            Python::AliasedImport => "aliased_import",
            Python::WildcardImport => "wildcard_import",
            Python::PrintStatement => "print_statement",
            Python::Chevron => "chevron",
            Python::AssertStatement => "assert_statement",
            Python::ExpressionStatement => "expression_statement",
            Python::NamedExpression => "named_expression",
            Python::NamedExpresssionLhs => "_named_expresssion_lhs",
            Python::ReturnStatement => "return_statement",
            Python::DeleteStatement => "delete_statement",
            Python::RaiseStatement => "raise_statement",
            Python::PassStatement => "pass_statement",
            Python::BreakStatement => "break_statement",
            Python::ContinueStatement => "continue_statement",
            Python::IfStatement => "if_statement",
            Python::ElifClause => "elif_clause",
            Python::ElseClause => "else_clause",
            Python::MatchStatement => "match_statement",
            Python::CaseClause => "case_clause",
            Python::ForStatement => "for_statement",
            Python::WhileStatement => "while_statement",
            Python::TryStatement => "try_statement",
            Python::ExceptClause => "except_clause",
            Python::FinallyClause => "finally_clause",
            Python::WithStatement => "with_statement",
            Python::WithClause => "with_clause",
            Python::WithItem => "with_item",
            Python::FunctionDefinition => "function_definition",
            Python::Parameters => "parameters",
            Python::LambdaParameters => "lambda_parameters",
            Python::ListSplat => "list_splat",
            Python::DictionarySplat => "dictionary_splat",
            Python::GlobalStatement => "global_statement",
            Python::NonlocalStatement => "nonlocal_statement",
            Python::ExecStatement => "exec_statement",
            Python::ClassDefinition => "class_definition",
            Python::ParenthesizedListSplat => "parenthesized_list_splat",
            Python::ArgumentList => "argument_list",
            Python::DecoratedDefinition => "decorated_definition",
            Python::Decorator => "decorator",
            Python::Block => "block",
            Python::ExpressionList => "expression_list",
            Python::DottedName => "dotted_name",
            Python::Parameters2 => "_parameters",
            Python::Patterns => "_patterns",
            Python::Parameter => "parameter",
            Python::Pattern => "pattern",
            Python::TuplePattern => "tuple_pattern",
            Python::ListPattern => "list_pattern",
            Python::DefaultParameter => "default_parameter",
            Python::TypedDefaultParameter => "typed_default_parameter",
            Python::ListSplatPattern => "list_splat_pattern",
            Python::DictionarySplatPattern => "dictionary_splat_pattern",
            Python::AsPattern => "as_pattern",
            Python::ExpressionWithinForInClause => "_expression_within_for_in_clause",
            Python::Expression => "expression",
            Python::PrimaryExpression => "primary_expression",
            Python::NotOperator => "not_operator",
            Python::BooleanOperator => "boolean_operator",
            Python::BinaryOperator => "binary_operator",
            Python::UnaryOperator => "unary_operator",
            Python::ComparisonOperator => "comparison_operator",
            Python::Lambda => "lambda",
            Python::Lambda2 => "lambda",
            Python::Assignment => "assignment",
            Python::AugmentedAssignment => "augmented_assignment",
            Python::PatternList => "pattern_list",
            Python::RightHandSide => "_right_hand_side",
            Python::Yield => "yield",
            Python::Attribute => "attribute",
            Python::Subscript => "subscript",
            Python::Slice => "slice",
            Python::Call => "call",
            Python::TypedParameter => "typed_parameter",
            Python::Type => "type",
            Python::KeywordArgument => "keyword_argument",
            Python::List => "list",
            Python::Set => "set",
            Python::Tuple => "tuple",
            Python::Dictionary => "dictionary",
            Python::Pair => "pair",
            Python::ListComprehension => "list_comprehension",
            Python::DictionaryComprehension => "dictionary_comprehension",
            Python::SetComprehension => "set_comprehension",
            Python::GeneratorExpression => "generator_expression",
            Python::ComprehensionClauses => "_comprehension_clauses",
            Python::ParenthesizedExpression => "parenthesized_expression",
            Python::CollectionElements => "_collection_elements",
            Python::ForInClause => "for_in_clause",
            Python::IfClause => "if_clause",
            Python::ConditionalExpression => "conditional_expression",
            Python::ConcatenatedString => "concatenated_string",
            Python::String => "string",
            Python::Interpolation => "interpolation",
            Python::EscapeInterpolation => "_escape_interpolation",
            Python::FormatSpecifier => "format_specifier",
            Python::FormatExpression => "format_expression",
            Python::Await => "await",
            Python::PositionalSeparator => "positional_separator",
            Python::KeywordSeparator => "keyword_separator",
            Python::ModuleRepeat1 => "module_repeat1",
            Python::SimpleStatementsRepeat1 => "_simple_statements_repeat1",
            Python::ImportPrefixRepeat1 => "import_prefix_repeat1",
            Python::ImportListRepeat1 => "_import_list_repeat1",
            Python::PrintStatementRepeat1 => "print_statement_repeat1",
            Python::AssertStatementRepeat1 => "assert_statement_repeat1",
            Python::IfStatementRepeat1 => "if_statement_repeat1",
            Python::MatchStatementRepeat1 => "match_statement_repeat1",
            Python::MatchStatementRepeat2 => "match_statement_repeat2",
            Python::CaseClauseRepeat1 => "case_clause_repeat1",
            Python::TryStatementRepeat1 => "try_statement_repeat1",
            Python::WithClauseRepeat1 => "with_clause_repeat1",
            Python::GlobalStatementRepeat1 => "global_statement_repeat1",
            Python::ArgumentListRepeat1 => "argument_list_repeat1",
            Python::DecoratedDefinitionRepeat1 => "decorated_definition_repeat1",
            Python::DottedNameRepeat1 => "dotted_name_repeat1",
            Python::ParametersRepeat1 => "_parameters_repeat1",
            Python::PatternsRepeat1 => "_patterns_repeat1",
            Python::ComparisonOperatorRepeat1 => "comparison_operator_repeat1",
            Python::SubscriptRepeat1 => "subscript_repeat1",
            Python::DictionaryRepeat1 => "dictionary_repeat1",
            Python::ComprehensionClausesRepeat1 => "_comprehension_clauses_repeat1",
            Python::CollectionElementsRepeat1 => "_collection_elements_repeat1",
            Python::ForInClauseRepeat1 => "for_in_clause_repeat1",
            Python::ConcatenatedStringRepeat1 => "concatenated_string_repeat1",
            Python::StringRepeat1 => "string_repeat1",
            Python::FormatSpecifierRepeat1 => "format_specifier_repeat1",
            Python::AsPatternTarget => "as_pattern_target",
            Python::CasePattern => "case_pattern",
            Python::Error => "ERROR",
        }
    }
}

impl From<u16> for Python {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Python == u16
impl PartialEq<u16> for Python {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

// u16 == Python
impl PartialEq<Python> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Python) -> bool {
        *x == *self
    }
}
