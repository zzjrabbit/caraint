use num_bigint::BigInt;

/// This is the AST nodes definition.
#[derive(Debug, Clone)]
pub enum AstNodes {
    Assign(String, Option<Box<AstNodes>>, Box<AstNodes>),
    CompileUnit(Vec<Box<AstNodes>>),
    BinaryOp(Box<AstNodes>, char, Box<AstNodes>),
    UnaryOp(char, Box<AstNodes>),
    Number(BigInt),
    VarDef(String, Box<AstNodes>),
    ConstDef(String, Box<AstNodes>),
    ReadVar(String),
    FunctionDef(String, Vec<String>, Vec<Box<AstNodes>>),
    Call(String, Vec<Box<AstNodes>>),
    Return(Box<AstNodes>),
    If(Box<AstNodes>, Vec<Box<AstNodes>>, Vec<Box<AstNodes>>),
    For(String, Box<AstNodes>, Box<AstNodes>, Vec<Box<AstNodes>>),
    List(Vec<Box<AstNodes>>),
    TemplateList(Box<AstNodes>,Box<AstNodes>),
    Index(String,Box<AstNodes>)
}
