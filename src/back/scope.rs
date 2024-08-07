#[derive(Debug, Clone)]
pub enum Symbol {
    Const(String, i64),
    Var(String, i64),
}

impl Symbol {
    pub fn get_id(&self) -> String {
        match self {
            Symbol::Const(id, _) => id,
            Symbol::Var(id, _) => id,
        }
        .clone()
    }

    pub fn get_value(&self) -> i64 {
        match self {
            Symbol::Const(_, value) => *value,
            Symbol::Var(_, value) => *value,
        }
    }

    pub fn try_assign(&mut self, value: i64) {
        match self {
            Symbol::Const(id, _) => panic!("Cannot assign to a constant {id}!"),
            Symbol::Var(_, old_value) => *old_value = value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn get(&self, id: &str) -> Option<&Symbol> {
        self.symbols.iter().find(|s| s.get_id() == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Symbol> {
        self.symbols.iter_mut().find(|s| s.get_id() == id)
    }
}
