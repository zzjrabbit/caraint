/// This is the AST nodes definition.
#[derive(Debug, Clone)]
pub enum AstNodes {
    Assign(String, Box<AstNodes>),
    CompileUnit(Vec<Box<AstNodes>>),
    BinaryOp(Box<AstNodes>, char, Box<AstNodes>),
    UnaryOp(char, Box<AstNodes>),
    Number(i64),
    VarDef(String, Box<AstNodes>),
    ConstDef(String, Box<AstNodes>),
    ReadVar(String),
}
