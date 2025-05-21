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
    Rem,
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
    FunctionDef(usize, Rc<[usize]>, Rc<[AstNodes]>),
    Call(usize, Vec<AstNodes>),
    Return(Rc<AstNodes>),
    If(Rc<AstNodes>, Rc<[AstNodes]>, Rc<[AstNodes]>),
    For(
        usize,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Rc<AstNodes>,
        Rc<[AstNodes]>,
    ),
    List(Vec<AstNodes>),
    TemplateList(Rc<AstNodes>, Rc<AstNodes>),
    Index(usize, Rc<AstNodes>),
    While(Rc<AstNodes>, Rc<[AstNodes]>),
    Break,
    Continue,
}
