use proc_macro2::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }

    pub fn dummy(node: T) -> Self {
        Self {
            node,
            span: Span::call_site(),
        }
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32,
    Bool,
    String,
    Unit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mutable(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(pub Vec<Parameter>);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub mutable: Mutable,
    pub id: String,
    pub ty: Type,
}

pub type FnDeclaration = Spanned<FnDeclarationKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclarationKind {
    pub id: String,
    pub parameters: Parameters,
    pub ty: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prog(pub Vec<FnDeclaration>);

pub type Statement = Spanned<StatementKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Let(Mutable, String, Option<Type>, Option<Expr>),
    Assign(Expr, Expr),
    While(Expr, Block),
    Expr(Expr),
    Fn(FnDeclaration),
}

pub type Block = Spanned<BlockKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockKind {
    pub statements: Vec<Statement>,
    pub semi: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(pub Vec<Expr>);

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Ident(String),
    Lit(Literal),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Par(Box<Expr>),
    Call(String, Arguments),
    IfThenElse(Box<Expr>, Block, Option<Block>),
    Block(Block),
    UnOp(UnOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Int(i32),
    String(String),
    Unit,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Lt,
    Gt,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnOp {
    Neg,
    Bang,
}

// Expression priority according to https://doc.rust-lang.org/reference/expressions.html
impl BinOp {
    pub fn priority(&self) -> u8 {
        match self {
            BinOp::Mul => 5,
            BinOp::Div => 5,
            BinOp::Add => 4,
            BinOp::Sub => 4,
            BinOp::Eq => 3,
            BinOp::Lt => 3,
            BinOp::Gt => 3,
            BinOp::And => 2,
            BinOp::Or => 1,
        }
    }
    pub fn expected_type(&self) -> Type {
        match self {
            BinOp::Add => Type::I32,
            BinOp::Sub => Type::I32,
            BinOp::Mul => Type::I32,
            BinOp::Div => Type::I32,
            BinOp::And => Type::Bool,
            BinOp::Or => Type::Bool,
            BinOp::Eq => Type::I32,
            BinOp::Lt => Type::I32,
            BinOp::Gt => Type::I32,
        }
    }
}
impl UnOp {
    pub fn priority(&self) -> u8 {
        match self {
            UnOp::Neg => 6,
            UnOp::Bang => 6,
        }
    }
    pub fn expected_type(&self) -> Type {
        match self {
            UnOp::Neg => Type::I32,
            UnOp::Bang => Type::Bool,
        }
    }
}

impl From<Expr> for Statement {
    fn from(expr: Expr) -> Self {
        Spanned::new(StatementKind::Expr(expr.clone()), expr.span)
    }
}

impl From<Expr> for Block {
    fn from(expr: Expr) -> Self {
        Self {
            node: BlockKind {
                statements: vec![Statement::from(expr.clone())],
                semi: false,
            },

            span: expr.span,
        }
    }
}

impl From<Statement> for Block {
    fn from(stmt: Statement) -> Self {
        Self {
            node: BlockKind {
                statements: vec![stmt.clone()],
                semi: false,
            },
            span: stmt.span,
        }
    }
}
