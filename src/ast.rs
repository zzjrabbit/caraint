use alloc::{rc::Rc, vec::Vec};
use dashu_int::IBig;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Ge,
    LShift,
    Gt,
    Le,
    RShift,
    Lt,
    Or,
    And,
    Mod,
}

/// This is the AST nodes definition.
#[derive(Debug, Clone)]
pub enum AstNodes {
    Assign(usize, Option<Rc<AstNodes>>, Rc<AstNodes>),
    CompileUnit(Vec<AstNodes>),
    BinaryOp(Rc<AstNodes>, Op, Rc<AstNodes>),
    UnaryOp(Op, Rc<AstNodes>),
    Number(IBig),
    VarDef(usize, Rc<AstNodes>),
    ConstDef(usize, Rc<AstNodes>),
    ReadVar(usize),
    FunctionDef(usize, Vec<usize>, Vec<AstNodes>),
    Call(usize, Vec<AstNodes>),
    Return(Rc<AstNodes>),
    If(Rc<AstNodes>, Vec<AstNodes>, Vec<AstNodes>),
    For(
        usize,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Vec<AstNodes>,
    ),
    List(Vec<AstNodes>),
    TemplateList(Rc<AstNodes>, Rc<AstNodes>),
    Index(usize, Rc<AstNodes>),
    While(Rc<AstNodes>, Vec<AstNodes>),
    Break,
    Continue,
}
