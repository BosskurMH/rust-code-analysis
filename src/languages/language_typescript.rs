// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Typescript {
    End = 0,
    Identifier = 1,
    HashBangLine = 2,
    Export = 3,
    STAR = 4,
    Default = 5,
    Type = 6,
    EQ = 7,
    As = 8,
    Namespace = 9,
    LBRACE = 10,
    COMMA = 11,
    RBRACE = 12,
    Typeof = 13,
    Import2 = 14,
    From = 15,
    With = 16,
    Var = 17,
    Let = 18,
    Const = 19,
    BANG = 20,
    Else = 21,
    If = 22,
    Switch = 23,
    For = 24,
    LPAREN = 25,
    RPAREN = 26,
    Await = 27,
    In = 28,
    Of = 29,
    While = 30,
    Do = 31,
    Try = 32,
    Break = 33,
    Continue = 34,
    Debugger = 35,
    Return = 36,
    Throw = 37,
    SEMI = 38,
    COLON = 39,
    Case = 40,
    Catch = 41,
    Finally = 42,
    Yield = 43,
    LBRACK = 44,
    RBRACK = 45,
    GlimmerTemplateContent = 46,
    GlimmerOpeningTag = 47,
    GlimmerClosingTag = 48,
    GT = 49,
    DOT = 50,
    DQUOTE = 51,
    SQUOTE = 52,
    Class2 = 53,
    Async = 54,
    Function = 55,
    EQGT = 56,
    QMARKDOT = 57,
    New = 58,
    Using = 59,
    PLUSEQ = 60,
    DASHEQ = 61,
    STAREQ = 62,
    SLASHEQ = 63,
    PERCENTEQ = 64,
    CARETEQ = 65,
    AMPEQ = 66,
    PIPEEQ = 67,
    GTGTEQ = 68,
    GTGTGTEQ = 69,
    LTLTEQ = 70,
    STARSTAREQ = 71,
    AMPAMPEQ = 72,
    PIPEPIPEEQ = 73,
    QMARKQMARKEQ = 74,
    DOTDOTDOT = 75,
    AMPAMP = 76,
    PIPEPIPE = 77,
    GTGT = 78,
    GTGTGT = 79,
    LTLT = 80,
    AMP = 81,
    CARET = 82,
    PIPE = 83,
    PLUS = 84,
    DASH = 85,
    SLASH = 86,
    PERCENT = 87,
    STARSTAR = 88,
    LT = 89,
    LTEQ = 90,
    EQEQ = 91,
    EQEQEQ = 92,
    BANGEQ = 93,
    BANGEQEQ = 94,
    GTEQ = 95,
    QMARKQMARK = 96,
    Instanceof = 97,
    TILDE = 98,
    Void = 99,
    Delete = 100,
    PLUSPLUS = 101,
    DASHDASH = 102,
    StringFragment = 103,
    StringFragment2 = 104,
    EscapeSequence = 105,
    Comment = 106,
    BQUOTE = 107,
    DOLLARLBRACE = 108,
    SLASH2 = 109,
    RegexPattern = 110,
    RegexFlags = 111,
    Number = 112,
    PrivatePropertyIdentifier = 113,
    Target = 114,
    This = 115,
    Super = 116,
    True = 117,
    False = 118,
    Null = 119,
    Undefined = 120,
    AT = 121,
    Static = 122,
    Readonly = 123,
    Get = 124,
    Set = 125,
    QMARK = 126,
    Declare = 127,
    Public = 128,
    Private = 129,
    Protected = 130,
    Override = 131,
    Module2 = 132,
    Any = 133,
    Number2 = 134,
    Boolean = 135,
    String2 = 136,
    Symbol = 137,
    Object2 = 138,
    Abstract = 139,
    Accessor = 140,
    Satisfies = 141,
    Require = 142,
    Extends = 143,
    Implements = 144,
    Global = 145,
    Interface = 146,
    Enum = 147,
    DASHQMARKCOLON = 148,
    PLUSQMARKCOLON = 149,
    QMARKCOLON = 150,
    Asserts2 = 151,
    Infer = 152,
    Is = 153,
    Keyof = 154,
    Uniquesymbol = 155,
    Unknown = 156,
    Never = 157,
    LBRACEPIPE = 158,
    PIPERBRACE = 159,
    AutomaticSemicolon = 160,
    StringFragment3 = 161,
    QMARK2 = 162,
    HtmlComment = 163,
    FunctionSignatureAutomaticSemicolon = 164,
    ErrorRecovery = 165,
    Program = 166,
    ExportStatement = 167,
    NamespaceExport = 168,
    ExportClause = 169,
    ExportSpecifier = 170,
    ModuleExportName = 171,
    Declaration = 172,
    Import = 173,
    ImportStatement = 174,
    ImportClause = 175,
    FromClause = 176,
    NamespaceImport = 177,
    NamedImports = 178,
    ImportSpecifier = 179,
    ImportAttribute = 180,
    Statement = 181,
    ExpressionStatement = 182,
    VariableDeclaration = 183,
    LexicalDeclaration = 184,
    VariableDeclarator = 185,
    StatementBlock = 186,
    ElseClause = 187,
    IfStatement = 188,
    SwitchStatement = 189,
    ForStatement = 190,
    ForInStatement = 191,
    ForHeader = 192,
    WhileStatement = 193,
    DoStatement = 194,
    TryStatement = 195,
    WithStatement = 196,
    BreakStatement = 197,
    ContinueStatement = 198,
    DebuggerStatement = 199,
    ReturnStatement = 200,
    ThrowStatement = 201,
    EmptyStatement = 202,
    LabeledStatement = 203,
    SwitchBody = 204,
    SwitchCase = 205,
    SwitchDefault = 206,
    CatchClause = 207,
    FinallyClause = 208,
    ParenthesizedExpression = 209,
    Expression = 210,
    PrimaryExpression = 211,
    YieldExpression = 212,
    Object = 213,
    ObjectPattern = 214,
    AssignmentPattern = 215,
    ObjectAssignmentPattern = 216,
    Array = 217,
    ArrayPattern = 218,
    GlimmerTemplate = 219,
    NestedIdentifier = 220,
    Class = 221,
    ClassDeclaration = 222,
    ClassHeritage = 223,
    FunctionExpression = 224,
    FunctionDeclaration = 225,
    GeneratorFunction = 226,
    GeneratorFunctionDeclaration = 227,
    ArrowFunction = 228,
    CallSignature2 = 229,
    FormalParameter = 230,
    OptionalChain = 231,
    CallExpression = 232,
    NewExpression = 233,
    AwaitExpression = 234,
    MemberExpression = 235,
    SubscriptExpression = 236,
    AssignmentExpression = 237,
    AugmentedAssignmentLhs = 238,
    AugmentedAssignmentExpression = 239,
    Initializer = 240,
    DestructuringPattern = 241,
    SpreadElement = 242,
    TernaryExpression = 243,
    BinaryExpression = 244,
    UnaryExpression = 245,
    UpdateExpression = 246,
    SequenceExpression = 247,
    String = 248,
    TemplateString = 249,
    TemplateSubstitution = 250,
    Regex = 251,
    MetaProperty = 252,
    Arguments = 253,
    Decorator = 254,
    MemberExpression2 = 255,
    CallExpression2 = 256,
    ClassBody = 257,
    FormalParameters = 258,
    ClassStaticBlock = 259,
    Pattern = 260,
    RestPattern = 261,
    MethodDefinition = 262,
    Pair = 263,
    PairPattern = 264,
    PropertyName = 265,
    ComputedPropertyName = 266,
    PublicFieldDefinition = 267,
    ImportIdentifier = 268,
    NonNullExpression = 269,
    MethodSignature = 270,
    AbstractMethodSignature = 271,
    FunctionSignature = 272,
    TypeAssertion = 273,
    AsExpression = 274,
    SatisfiesExpression = 275,
    InstantiationExpression = 276,
    ImportRequireClause = 277,
    ExtendsClause = 278,
    ExtendsClauseSingle = 279,
    ImplementsClause = 280,
    AmbientDeclaration = 281,
    AbstractClassDeclaration = 282,
    Module = 283,
    InternalModule = 284,
    Module3 = 285,
    ImportAlias = 286,
    NestedTypeIdentifier = 287,
    InterfaceDeclaration = 288,
    ExtendsTypeClause = 289,
    EnumDeclaration = 290,
    EnumBody = 291,
    EnumAssignment = 292,
    TypeAliasDeclaration = 293,
    AccessibilityModifier = 294,
    OverrideModifier = 295,
    RequiredParameter = 296,
    OptionalParameter = 297,
    ParameterName = 298,
    OmittingTypeAnnotation = 299,
    AddingTypeAnnotation = 300,
    OptingTypeAnnotation = 301,
    TypeAnnotation = 302,
    MemberExpression3 = 303,
    CallExpression3 = 304,
    Asserts = 305,
    AssertsAnnotation = 306,
    Type2 = 307,
    RequiredParameter2 = 308,
    OptionalParameter2 = 309,
    OptionalType = 310,
    RestType = 311,
    TupleTypeMember = 312,
    ConstructorType = 313,
    PrimaryType = 314,
    TemplateType = 315,
    TemplateLiteralType = 316,
    InferType = 317,
    ConditionalType = 318,
    GenericType = 319,
    TypePredicate = 320,
    TypePredicateAnnotation = 321,
    MemberExpression4 = 322,
    SubscriptExpression2 = 323,
    CallExpression4 = 324,
    InstantiationExpression2 = 325,
    TypeQuery = 326,
    IndexTypeQuery = 327,
    LookupType = 328,
    MappedTypeClause = 329,
    LiteralType = 330,
    UnaryExpression2 = 331,
    ExistentialType = 332,
    FlowMaybeType = 333,
    ParenthesizedType = 334,
    PredefinedType = 335,
    TypeArguments = 336,
    ObjectType = 337,
    CallSignature = 338,
    PropertySignature = 339,
    TypeParameters = 340,
    TypeParameter = 341,
    DefaultType = 342,
    Constraint = 343,
    ConstructSignature = 344,
    IndexSignature = 345,
    ArrayType = 346,
    TupleType = 347,
    ReadonlyType = 348,
    UnionType = 349,
    IntersectionType = 350,
    FunctionType = 351,
    ProgramRepeat1 = 352,
    ExportStatementRepeat1 = 353,
    ExportClauseRepeat1 = 354,
    NamedImportsRepeat1 = 355,
    VariableDeclarationRepeat1 = 356,
    SwitchBodyRepeat1 = 357,
    ObjectRepeat1 = 358,
    ObjectPatternRepeat1 = 359,
    ArrayRepeat1 = 360,
    ArrayPatternRepeat1 = 361,
    GlimmerTemplateRepeat1 = 362,
    SequenceExpressionRepeat1 = 363,
    StringRepeat1 = 364,
    StringRepeat2 = 365,
    TemplateStringRepeat1 = 366,
    ClassBodyRepeat1 = 367,
    FormalParametersRepeat1 = 368,
    ExtendsClauseRepeat1 = 369,
    ImplementsClauseRepeat1 = 370,
    ExtendsTypeClauseRepeat1 = 371,
    EnumBodyRepeat1 = 372,
    TemplateLiteralTypeRepeat1 = 373,
    ObjectTypeRepeat1 = 374,
    TypeParametersRepeat1 = 375,
    TupleTypeRepeat1 = 376,
    InterfaceBody = 377,
    PropertyIdentifier = 378,
    ShorthandPropertyIdentifier = 379,
    ShorthandPropertyIdentifierPattern = 380,
    StatementIdentifier = 381,
    ThisType = 382,
    TypeIdentifier = 383,
    Error = 384,
}

impl From<Typescript> for &'static str {
    #[inline(always)]
    fn from(tok: Typescript) -> Self {
        match tok {
            Typescript::End => "end",
            Typescript::Identifier => "identifier",
            Typescript::HashBangLine => "hash_bang_line",
            Typescript::Export => "export",
            Typescript::STAR => "*",
            Typescript::Default => "default",
            Typescript::Type => "type",
            Typescript::EQ => "=",
            Typescript::As => "as",
            Typescript::Namespace => "namespace",
            Typescript::LBRACE => "{",
            Typescript::COMMA => ",",
            Typescript::RBRACE => "}",
            Typescript::Typeof => "typeof",
            Typescript::Import2 => "import",
            Typescript::From => "from",
            Typescript::With => "with",
            Typescript::Var => "var",
            Typescript::Let => "let",
            Typescript::Const => "const",
            Typescript::BANG => "!",
            Typescript::Else => "else",
            Typescript::If => "if",
            Typescript::Switch => "switch",
            Typescript::For => "for",
            Typescript::LPAREN => "(",
            Typescript::RPAREN => ")",
            Typescript::Await => "await",
            Typescript::In => "in",
            Typescript::Of => "of",
            Typescript::While => "while",
            Typescript::Do => "do",
            Typescript::Try => "try",
            Typescript::Break => "break",
            Typescript::Continue => "continue",
            Typescript::Debugger => "debugger",
            Typescript::Return => "return",
            Typescript::Throw => "throw",
            Typescript::SEMI => ";",
            Typescript::COLON => ":",
            Typescript::Case => "case",
            Typescript::Catch => "catch",
            Typescript::Finally => "finally",
            Typescript::Yield => "yield",
            Typescript::LBRACK => "[",
            Typescript::RBRACK => "]",
            Typescript::GlimmerTemplateContent => "_glimmer_template_content",
            Typescript::GlimmerOpeningTag => "glimmer_opening_tag",
            Typescript::GlimmerClosingTag => "glimmer_closing_tag",
            Typescript::GT => ">",
            Typescript::DOT => ".",
            Typescript::DQUOTE => "\"",
            Typescript::SQUOTE => "'",
            Typescript::Class2 => "class",
            Typescript::Async => "async",
            Typescript::Function => "function",
            Typescript::EQGT => "=>",
            Typescript::QMARKDOT => "?.",
            Typescript::New => "new",
            Typescript::Using => "using",
            Typescript::PLUSEQ => "+=",
            Typescript::DASHEQ => "-=",
            Typescript::STAREQ => "*=",
            Typescript::SLASHEQ => "/=",
            Typescript::PERCENTEQ => "%=",
            Typescript::CARETEQ => "^=",
            Typescript::AMPEQ => "&=",
            Typescript::PIPEEQ => "|=",
            Typescript::GTGTEQ => ">>=",
            Typescript::GTGTGTEQ => ">>>=",
            Typescript::LTLTEQ => "<<=",
            Typescript::STARSTAREQ => "**=",
            Typescript::AMPAMPEQ => "&&=",
            Typescript::PIPEPIPEEQ => "||=",
            Typescript::QMARKQMARKEQ => "??=",
            Typescript::DOTDOTDOT => "...",
            Typescript::AMPAMP => "&&",
            Typescript::PIPEPIPE => "||",
            Typescript::GTGT => ">>",
            Typescript::GTGTGT => ">>>",
            Typescript::LTLT => "<<",
            Typescript::AMP => "&",
            Typescript::CARET => "^",
            Typescript::PIPE => "|",
            Typescript::PLUS => "+",
            Typescript::DASH => "-",
            Typescript::SLASH => "/",
            Typescript::PERCENT => "%",
            Typescript::STARSTAR => "**",
            Typescript::LT => "<",
            Typescript::LTEQ => "<=",
            Typescript::EQEQ => "==",
            Typescript::EQEQEQ => "===",
            Typescript::BANGEQ => "!=",
            Typescript::BANGEQEQ => "!==",
            Typescript::GTEQ => ">=",
            Typescript::QMARKQMARK => "??",
            Typescript::Instanceof => "instanceof",
            Typescript::TILDE => "~",
            Typescript::Void => "void",
            Typescript::Delete => "delete",
            Typescript::PLUSPLUS => "++",
            Typescript::DASHDASH => "--",
            Typescript::StringFragment => "string_fragment",
            Typescript::StringFragment2 => "string_fragment",
            Typescript::EscapeSequence => "escape_sequence",
            Typescript::Comment => "comment",
            Typescript::BQUOTE => "`",
            Typescript::DOLLARLBRACE => "${",
            Typescript::SLASH2 => "/",
            Typescript::RegexPattern => "regex_pattern",
            Typescript::RegexFlags => "regex_flags",
            Typescript::Number => "number",
            Typescript::PrivatePropertyIdentifier => "private_property_identifier",
            Typescript::Target => "target",
            Typescript::This => "this",
            Typescript::Super => "super",
            Typescript::True => "true",
            Typescript::False => "false",
            Typescript::Null => "null",
            Typescript::Undefined => "undefined",
            Typescript::AT => "@",
            Typescript::Static => "static",
            Typescript::Readonly => "readonly",
            Typescript::Get => "get",
            Typescript::Set => "set",
            Typescript::QMARK => "?",
            Typescript::Declare => "declare",
            Typescript::Public => "public",
            Typescript::Private => "private",
            Typescript::Protected => "protected",
            Typescript::Override => "override",
            Typescript::Module2 => "module",
            Typescript::Any => "any",
            Typescript::Number2 => "number",
            Typescript::Boolean => "boolean",
            Typescript::String2 => "string",
            Typescript::Symbol => "symbol",
            Typescript::Object2 => "object",
            Typescript::Abstract => "abstract",
            Typescript::Accessor => "accessor",
            Typescript::Satisfies => "satisfies",
            Typescript::Require => "require",
            Typescript::Extends => "extends",
            Typescript::Implements => "implements",
            Typescript::Global => "global",
            Typescript::Interface => "interface",
            Typescript::Enum => "enum",
            Typescript::DASHQMARKCOLON => "-?:",
            Typescript::PLUSQMARKCOLON => "+?:",
            Typescript::QMARKCOLON => "?:",
            Typescript::Asserts2 => "asserts",
            Typescript::Infer => "infer",
            Typescript::Is => "is",
            Typescript::Keyof => "keyof",
            Typescript::Uniquesymbol => "unique symbol",
            Typescript::Unknown => "unknown",
            Typescript::Never => "never",
            Typescript::LBRACEPIPE => "{|",
            Typescript::PIPERBRACE => "|}",
            Typescript::AutomaticSemicolon => "_automatic_semicolon",
            Typescript::StringFragment3 => "string_fragment",
            Typescript::QMARK2 => "?",
            Typescript::HtmlComment => "html_comment",
            Typescript::FunctionSignatureAutomaticSemicolon => {
                "_function_signature_automatic_semicolon"
            }
            Typescript::ErrorRecovery => "__error_recovery",
            Typescript::Program => "program",
            Typescript::ExportStatement => "export_statement",
            Typescript::NamespaceExport => "namespace_export",
            Typescript::ExportClause => "export_clause",
            Typescript::ExportSpecifier => "export_specifier",
            Typescript::ModuleExportName => "_module_export_name",
            Typescript::Declaration => "declaration",
            Typescript::Import => "import",
            Typescript::ImportStatement => "import_statement",
            Typescript::ImportClause => "import_clause",
            Typescript::FromClause => "_from_clause",
            Typescript::NamespaceImport => "namespace_import",
            Typescript::NamedImports => "named_imports",
            Typescript::ImportSpecifier => "import_specifier",
            Typescript::ImportAttribute => "import_attribute",
            Typescript::Statement => "statement",
            Typescript::ExpressionStatement => "expression_statement",
            Typescript::VariableDeclaration => "variable_declaration",
            Typescript::LexicalDeclaration => "lexical_declaration",
            Typescript::VariableDeclarator => "variable_declarator",
            Typescript::StatementBlock => "statement_block",
            Typescript::ElseClause => "else_clause",
            Typescript::IfStatement => "if_statement",
            Typescript::SwitchStatement => "switch_statement",
            Typescript::ForStatement => "for_statement",
            Typescript::ForInStatement => "for_in_statement",
            Typescript::ForHeader => "_for_header",
            Typescript::WhileStatement => "while_statement",
            Typescript::DoStatement => "do_statement",
            Typescript::TryStatement => "try_statement",
            Typescript::WithStatement => "with_statement",
            Typescript::BreakStatement => "break_statement",
            Typescript::ContinueStatement => "continue_statement",
            Typescript::DebuggerStatement => "debugger_statement",
            Typescript::ReturnStatement => "return_statement",
            Typescript::ThrowStatement => "throw_statement",
            Typescript::EmptyStatement => "empty_statement",
            Typescript::LabeledStatement => "labeled_statement",
            Typescript::SwitchBody => "switch_body",
            Typescript::SwitchCase => "switch_case",
            Typescript::SwitchDefault => "switch_default",
            Typescript::CatchClause => "catch_clause",
            Typescript::FinallyClause => "finally_clause",
            Typescript::ParenthesizedExpression => "parenthesized_expression",
            Typescript::Expression => "expression",
            Typescript::PrimaryExpression => "primary_expression",
            Typescript::YieldExpression => "yield_expression",
            Typescript::Object => "object",
            Typescript::ObjectPattern => "object_pattern",
            Typescript::AssignmentPattern => "assignment_pattern",
            Typescript::ObjectAssignmentPattern => "object_assignment_pattern",
            Typescript::Array => "array",
            Typescript::ArrayPattern => "array_pattern",
            Typescript::GlimmerTemplate => "glimmer_template",
            Typescript::NestedIdentifier => "nested_identifier",
            Typescript::Class => "class",
            Typescript::ClassDeclaration => "class_declaration",
            Typescript::ClassHeritage => "class_heritage",
            Typescript::FunctionExpression => "function_expression",
            Typescript::FunctionDeclaration => "function_declaration",
            Typescript::GeneratorFunction => "generator_function",
            Typescript::GeneratorFunctionDeclaration => "generator_function_declaration",
            Typescript::ArrowFunction => "arrow_function",
            Typescript::CallSignature2 => "_call_signature",
            Typescript::FormalParameter => "_formal_parameter",
            Typescript::OptionalChain => "optional_chain",
            Typescript::CallExpression => "call_expression",
            Typescript::NewExpression => "new_expression",
            Typescript::AwaitExpression => "await_expression",
            Typescript::MemberExpression => "member_expression",
            Typescript::SubscriptExpression => "subscript_expression",
            Typescript::AssignmentExpression => "assignment_expression",
            Typescript::AugmentedAssignmentLhs => "_augmented_assignment_lhs",
            Typescript::AugmentedAssignmentExpression => "augmented_assignment_expression",
            Typescript::Initializer => "_initializer",
            Typescript::DestructuringPattern => "_destructuring_pattern",
            Typescript::SpreadElement => "spread_element",
            Typescript::TernaryExpression => "ternary_expression",
            Typescript::BinaryExpression => "binary_expression",
            Typescript::UnaryExpression => "unary_expression",
            Typescript::UpdateExpression => "update_expression",
            Typescript::SequenceExpression => "sequence_expression",
            Typescript::String => "string",
            Typescript::TemplateString => "template_string",
            Typescript::TemplateSubstitution => "template_substitution",
            Typescript::Regex => "regex",
            Typescript::MetaProperty => "meta_property",
            Typescript::Arguments => "arguments",
            Typescript::Decorator => "decorator",
            Typescript::MemberExpression2 => "member_expression",
            Typescript::CallExpression2 => "call_expression",
            Typescript::ClassBody => "class_body",
            Typescript::FormalParameters => "formal_parameters",
            Typescript::ClassStaticBlock => "class_static_block",
            Typescript::Pattern => "pattern",
            Typescript::RestPattern => "rest_pattern",
            Typescript::MethodDefinition => "method_definition",
            Typescript::Pair => "pair",
            Typescript::PairPattern => "pair_pattern",
            Typescript::PropertyName => "_property_name",
            Typescript::ComputedPropertyName => "computed_property_name",
            Typescript::PublicFieldDefinition => "public_field_definition",
            Typescript::ImportIdentifier => "_import_identifier",
            Typescript::NonNullExpression => "non_null_expression",
            Typescript::MethodSignature => "method_signature",
            Typescript::AbstractMethodSignature => "abstract_method_signature",
            Typescript::FunctionSignature => "function_signature",
            Typescript::TypeAssertion => "type_assertion",
            Typescript::AsExpression => "as_expression",
            Typescript::SatisfiesExpression => "satisfies_expression",
            Typescript::InstantiationExpression => "instantiation_expression",
            Typescript::ImportRequireClause => "import_require_clause",
            Typescript::ExtendsClause => "extends_clause",
            Typescript::ExtendsClauseSingle => "_extends_clause_single",
            Typescript::ImplementsClause => "implements_clause",
            Typescript::AmbientDeclaration => "ambient_declaration",
            Typescript::AbstractClassDeclaration => "abstract_class_declaration",
            Typescript::Module => "module",
            Typescript::InternalModule => "internal_module",
            Typescript::Module3 => "_module",
            Typescript::ImportAlias => "import_alias",
            Typescript::NestedTypeIdentifier => "nested_type_identifier",
            Typescript::InterfaceDeclaration => "interface_declaration",
            Typescript::ExtendsTypeClause => "extends_type_clause",
            Typescript::EnumDeclaration => "enum_declaration",
            Typescript::EnumBody => "enum_body",
            Typescript::EnumAssignment => "enum_assignment",
            Typescript::TypeAliasDeclaration => "type_alias_declaration",
            Typescript::AccessibilityModifier => "accessibility_modifier",
            Typescript::OverrideModifier => "override_modifier",
            Typescript::RequiredParameter => "required_parameter",
            Typescript::OptionalParameter => "optional_parameter",
            Typescript::ParameterName => "_parameter_name",
            Typescript::OmittingTypeAnnotation => "omitting_type_annotation",
            Typescript::AddingTypeAnnotation => "adding_type_annotation",
            Typescript::OptingTypeAnnotation => "opting_type_annotation",
            Typescript::TypeAnnotation => "type_annotation",
            Typescript::MemberExpression3 => "member_expression",
            Typescript::CallExpression3 => "call_expression",
            Typescript::Asserts => "asserts",
            Typescript::AssertsAnnotation => "asserts_annotation",
            Typescript::Type2 => "type",
            Typescript::RequiredParameter2 => "required_parameter",
            Typescript::OptionalParameter2 => "optional_parameter",
            Typescript::OptionalType => "optional_type",
            Typescript::RestType => "rest_type",
            Typescript::TupleTypeMember => "_tuple_type_member",
            Typescript::ConstructorType => "constructor_type",
            Typescript::PrimaryType => "primary_type",
            Typescript::TemplateType => "template_type",
            Typescript::TemplateLiteralType => "template_literal_type",
            Typescript::InferType => "infer_type",
            Typescript::ConditionalType => "conditional_type",
            Typescript::GenericType => "generic_type",
            Typescript::TypePredicate => "type_predicate",
            Typescript::TypePredicateAnnotation => "type_predicate_annotation",
            Typescript::MemberExpression4 => "member_expression",
            Typescript::SubscriptExpression2 => "subscript_expression",
            Typescript::CallExpression4 => "call_expression",
            Typescript::InstantiationExpression2 => "instantiation_expression",
            Typescript::TypeQuery => "type_query",
            Typescript::IndexTypeQuery => "index_type_query",
            Typescript::LookupType => "lookup_type",
            Typescript::MappedTypeClause => "mapped_type_clause",
            Typescript::LiteralType => "literal_type",
            Typescript::UnaryExpression2 => "unary_expression",
            Typescript::ExistentialType => "existential_type",
            Typescript::FlowMaybeType => "flow_maybe_type",
            Typescript::ParenthesizedType => "parenthesized_type",
            Typescript::PredefinedType => "predefined_type",
            Typescript::TypeArguments => "type_arguments",
            Typescript::ObjectType => "object_type",
            Typescript::CallSignature => "call_signature",
            Typescript::PropertySignature => "property_signature",
            Typescript::TypeParameters => "type_parameters",
            Typescript::TypeParameter => "type_parameter",
            Typescript::DefaultType => "default_type",
            Typescript::Constraint => "constraint",
            Typescript::ConstructSignature => "construct_signature",
            Typescript::IndexSignature => "index_signature",
            Typescript::ArrayType => "array_type",
            Typescript::TupleType => "tuple_type",
            Typescript::ReadonlyType => "readonly_type",
            Typescript::UnionType => "union_type",
            Typescript::IntersectionType => "intersection_type",
            Typescript::FunctionType => "function_type",
            Typescript::ProgramRepeat1 => "program_repeat1",
            Typescript::ExportStatementRepeat1 => "export_statement_repeat1",
            Typescript::ExportClauseRepeat1 => "export_clause_repeat1",
            Typescript::NamedImportsRepeat1 => "named_imports_repeat1",
            Typescript::VariableDeclarationRepeat1 => "variable_declaration_repeat1",
            Typescript::SwitchBodyRepeat1 => "switch_body_repeat1",
            Typescript::ObjectRepeat1 => "object_repeat1",
            Typescript::ObjectPatternRepeat1 => "object_pattern_repeat1",
            Typescript::ArrayRepeat1 => "array_repeat1",
            Typescript::ArrayPatternRepeat1 => "array_pattern_repeat1",
            Typescript::GlimmerTemplateRepeat1 => "glimmer_template_repeat1",
            Typescript::SequenceExpressionRepeat1 => "sequence_expression_repeat1",
            Typescript::StringRepeat1 => "string_repeat1",
            Typescript::StringRepeat2 => "string_repeat2",
            Typescript::TemplateStringRepeat1 => "template_string_repeat1",
            Typescript::ClassBodyRepeat1 => "class_body_repeat1",
            Typescript::FormalParametersRepeat1 => "formal_parameters_repeat1",
            Typescript::ExtendsClauseRepeat1 => "extends_clause_repeat1",
            Typescript::ImplementsClauseRepeat1 => "implements_clause_repeat1",
            Typescript::ExtendsTypeClauseRepeat1 => "extends_type_clause_repeat1",
            Typescript::EnumBodyRepeat1 => "enum_body_repeat1",
            Typescript::TemplateLiteralTypeRepeat1 => "template_literal_type_repeat1",
            Typescript::ObjectTypeRepeat1 => "object_type_repeat1",
            Typescript::TypeParametersRepeat1 => "type_parameters_repeat1",
            Typescript::TupleTypeRepeat1 => "tuple_type_repeat1",
            Typescript::InterfaceBody => "interface_body",
            Typescript::PropertyIdentifier => "property_identifier",
            Typescript::ShorthandPropertyIdentifier => "shorthand_property_identifier",
            Typescript::ShorthandPropertyIdentifierPattern => {
                "shorthand_property_identifier_pattern"
            }
            Typescript::StatementIdentifier => "statement_identifier",
            Typescript::ThisType => "this_type",
            Typescript::TypeIdentifier => "type_identifier",
            Typescript::Error => "ERROR",
        }
    }
}

impl From<u16> for Typescript {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Typescript == u16
impl PartialEq<u16> for Typescript {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

// u16 == Typescript
impl PartialEq<Typescript> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Typescript) -> bool {
        *x == *self
    }
}
