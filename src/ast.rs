use alloc::{rc::Rc, string::String, vec::Vec};
use dashu_int::IBig;

/// This is the AST nodes definition.
#[derive(Debug, Clone)]
pub enum AstNodes {
    Assign(String, Option<Rc<AstNodes>>, Rc<AstNodes>),
    CompileUnit(Vec<AstNodes>),
    BinaryOp(Rc<AstNodes>, &'static str, Rc<AstNodes>),
    UnaryOp(&'static str, Rc<AstNodes>),
    Number(IBig),
    VarDef(String, Rc<AstNodes>),
    ConstDef(String, Rc<AstNodes>),
    ReadVar(String),
    FunctionDef(String, Vec<String>, Vec<AstNodes>),
    Call(String, Vec<AstNodes>),
    Return(Rc<AstNodes>),
    If(Rc<AstNodes>, Vec<AstNodes>, Vec<AstNodes>),
    For(
        String,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Vec<AstNodes>,
    ),
    List(Vec<AstNodes>),
    TemplateList(Rc<AstNodes>, Rc<AstNodes>),
    Index(String, Rc<AstNodes>),
    While(Rc<AstNodes>, Vec<AstNodes>),
    Break,
    Continue,
}
