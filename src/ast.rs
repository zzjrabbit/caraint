use alloc::{rc::Rc, string::String, vec::Vec};

use num_bigint::BigInt;

/// This is the AST nodes definition.
#[derive(Debug, Clone)]
pub enum AstNodes {
    Assign(String, Option<Rc<AstNodes>>, Rc<AstNodes>),
    CompileUnit(Vec<Rc<AstNodes>>),
    BinaryOp(Rc<AstNodes>, &'static str, Rc<AstNodes>),
    UnaryOp(&'static str, Rc<AstNodes>),
    Number(BigInt),
    VarDef(String, Rc<AstNodes>),
    ConstDef(String, Rc<AstNodes>),
    ReadVar(String),
    FunctionDef(String, Vec<String>, Vec<Rc<AstNodes>>),
    Call(String, Vec<Rc<AstNodes>>),
    Return(Rc<AstNodes>),
    If(Rc<AstNodes>, Vec<Rc<AstNodes>>, Vec<Rc<AstNodes>>),
    For(
        String,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Vec<Rc<AstNodes>>,
    ),
    List(Vec<Rc<AstNodes>>),
    TemplateList(Rc<AstNodes>, Rc<AstNodes>),
    Index(String, Rc<AstNodes>),
    While(Rc<AstNodes>, Vec<Rc<AstNodes>>),
    Break,
    Continue,
}
