use crate::ast::{BinOp, Expr, ExprKind, Literal, Spanned, Type, UnOp};

pub fn expr(kind: ExprKind) -> Expr {
    Spanned {
        node: kind,
        span: proc_macro2::Span::call_site(), // dummy span for tests
    }
}

impl From<ExprKind> for Expr {
    fn from(kind: ExprKind) -> Self {
        Spanned {
            node: kind,
            span: proc_macro2::Span::call_site(),
        }
    }
}

/// Anything that can be converted into a Literal (like i32, bool, etc) can also be converted into an Expr.
impl<T: Into<Literal>> From<T> for ExprKind {
    fn from(x: T) -> Self {
        ExprKind::Lit(x.into())
    }
}

impl<T: Into<Literal>> From<T> for Expr {
    fn from(x: T) -> Self {
        expr(ExprKind::Lit(x.into()))
    }
}

/// Anything that can be converted to a Literal can also be converted to a Type.
impl<T: Into<Literal>> From<T> for Type {
    fn from(x: T) -> Self {
        let lit: Literal = x.into();
        match lit {
            Literal::Unit => Type::Unit,
            Literal::Bool(_) => Type::Bool,
            Literal::Int(_) => Type::I32,
            Literal::String(_) => Type::String,
        }
    }
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl From<()> for Literal {
    fn from(_: ()) -> Self {
        Literal::Unit
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

// Utility functions/traits for your AST here.
impl ExprKind {
    pub fn bin_op(o: BinOp, left: Expr, right: Expr) -> Self {
        ExprKind::BinOp(o, Box::new(left), Box::new(right))
    }

    pub fn un_op(o: UnOp, right: Expr) -> Self {
        ExprKind::UnOp(o, Box::new(right))
    }
}

impl ExprKind {
    fn as_str(&self) -> &'static str {
        match self {
            ExprKind::Ident(_) => "Ident",
            ExprKind::Lit(_) => "Lit",
            ExprKind::BinOp(_, _, _) => "BinOp",
            ExprKind::Par(_) => "Par",
            ExprKind::Call(_, _) => "Call",
            ExprKind::IfThenElse(_, _, _) => "IfThenElse",
            ExprKind::Block(_) => "Block",
            ExprKind::UnOp(_, _) => "UnOp",
        }
    }
    pub fn label(&self) -> String {
        match self {
            ExprKind::Ident(name) => format!("Ident({})", name),
            ExprKind::Lit(lit) => format!("Lit({})", lit),
            _ => self.as_str().to_string(),
        }
    }
}
