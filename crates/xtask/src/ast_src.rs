//! Defines input for code generation process.

pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) contextual_keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        // Sync with Field::method_name
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        (".", "DOT"),
        (":", "COLON"),
        ("?", "QMARK"),
        ("=", "EQ"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "ASTERISK"),
        ("/", "SLASH"),
        ("%", "PERCENT"),
        ("&", "AMP"),
        ("&&", "AMP2"),
        ("|", "PIPE"),
        ("||", "PIPE2"),
        ("^", "CARET"),
        ("!", "BANG"),
        ("+=", "PLUS_EQ"),
        ("-=", "MINUS_EQ"),
        ("*=", "ASTERISK_EQ"),
        ("/=", "SLASH_EQ"),
        ("%=", "MODULUS_EQ"),
        ("&=", "AMP_EQ"),
        ("|=", "PIPE_EQ"),
        ("&&=", "AMP2_EQ"),
        ("||=", "PIPE2_EQ"),
        ("^=", "CARET_EQ"),
        ("==", "EQ_EQ"),
        ("!=", "BANG_EQ"),
        ("<=", "L_ANGLE_EQ"),
        (">=", "R_ANGLE_EQ"),
    ],
    keywords: &[
        "fn", "self", "while", "for", "in", "continue", "break", "return", "if", "else", "let",
        "true", "false", "import", "as"
    ],
    contextual_keywords: &[],
    literals: &[
        "INT_NUMBER",
        "FLOAT_NUMBER",
        "CHAR",
        "STR",
        "MULTILINE_STR",
    ],
    tokens: &[
        "ERROR",
        "IDENT",
        "WHITESPACE",
        "LIFETIME",
        "COMMENT",
        "BLOCK_COMMENT",
        "SHEBANG",
        "NEWLINE",
    ],
    nodes: &[
        "ROOT",
        "IMPORT_STMT",
        "IMPORT_TARGET",
        "NAME",
        "NAME_IDENT",
        "LIT_VAL",
        "FN_DEF",
        "BLOCK",
        "STMT",
        "EXPR_STMT",
        "BIN_EXPR",
        "TUPLE_EXPR",
        "ARR_EXPR",
        "PRIMARY_EXPR",
        "FN_CALL_EXPR",
        "METHOD_CALL_EXPR",
        "FN_CALL_ARGS",
        "PREFIX_UNARY_EXPR",
        "INDEX_EXPR",
        "INDEX_EXPR_BRACKETS",
        "MEMBER_ACCESS_EXPR",
        "IF_BRANCH",
        "ELSE_IF_BRANCH",
        "ELSE_BRANCH",
        "IF_EXPR",
        "WHILE_STMT",
        "FOR_STMT",
        "FOR_IN_EXPR",
        "BREAK_STMT",
        "CONTINUE_STMT",
        "RETURN_STMT",
        "DECLARATION_STMT",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}
