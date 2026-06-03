use core::fmt;

use proc_macro2::Span;

use crate::ast::{
    Arguments, BinOp, Block, ExprKind, FnDeclarationKind, Literal, Mutable, Parameter,
    ParameterKind, Parameters, ParametersKind, Prog, Spanned, StatementKind, Type, UnOp,
};

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node.fmt(f)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::Eq => "==",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
        };
        write!(f, "{}", s)
    }
}

// Your ast Display traits here
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Unit => write!(f, "()"),
        }
    }
}

#[test]
fn display_literal() {
    println!("{}", Literal::Int(3));
    println!("{}", Literal::Bool(false));
    println!("{}", Literal::Unit);
    assert_eq!(format!("{}", Literal::Int(3)), "3");
    assert_eq!(format!("{}", Literal::Bool(false)), "false");
    assert_eq!(format!("{}", Literal::Unit), "()");
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //todo!()
        match self {
            Type::I32 => write!(f, "i32"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "String"),
            Type::Unit => write!(f, "()"),
        }
    }
}

#[test]
fn display_type() {
    assert_eq!(format!("{}", Type::I32), "i32");
    assert_eq!(format!("{}", Type::Bool), "bool");
    assert_eq!(format!("{}", Type::Unit), "()");
    assert_eq!(format!("{}", Type::String), "String");
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Bang => write!(f, "!"),
            UnOp::Neg => write!(f, "-"),
        }
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Ident(i) => write!(f, "{}", i),
            ExprKind::Lit(literal) => write!(f, "{}", literal),
            ExprKind::BinOp(bin_op, expr, expr1) => write!(f, "{} {} {}", expr, bin_op, expr1),
            ExprKind::Par(expr) => write!(f, "({})", expr),
            ExprKind::Call(i, arguments) => write!(f, "{}{}", i, arguments),
            ExprKind::IfThenElse(expr, block, block1) => match block1 {
                Some(b1) => write!(f, "if {} {} else {}", expr, block, b1),
                None => write!(f, "if {} {}", expr, block),
            },
            ExprKind::Block(block) => write!(f, "{}", block),
            ExprKind::UnOp(un_op, expr) => write!(f, "{}{}", un_op, expr),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let statements = self
            .node
            .statements
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(";\n\t");
        write!(
            f,
            "{{\n\t{}{}\n}}",
            statements,
            if self.node.semi { ";" } else { "" }
        )
    }
}

impl fmt::Display for Mutable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            write!(f, "mut ")
        } else {
            write!(f, "")
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}: {}", self.node.mutable, self.node.id, self.node.ty)
    }
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self
            .node
            .0
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({})", parameters)
    }
}

#[test]
fn display_parameters() {
    let params: Vec<Parameter> = vec![
        Spanned::dummy(ParameterKind {
            mutable: Mutable(true),
            id: "testparam".to_string(),
            ty: Type::I32,
        }),
        Spanned::dummy(ParameterKind {
            mutable: Mutable(false),
            id: "testparam2".to_string(),
            ty: Type::String,
        }),
    ];
    let parameters = Parameters::new(ParametersKind(params), Span::call_site());
    println!("ast:\n{}", parameters);
    assert_eq!(
        parameters.to_string(),
        "(mut testparam: i32, testparam2: String)"
    )
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .node
            .0
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({})", args)
    }
}

impl fmt::Display for FnDeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn {}{}{} {}",
            self.id,
            self.parameters,
            match &self.ty {
                Some(t) => format!(" -> {}", t),
                None => "".to_string(),
            },
            self.body
        )
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fns = self
            .0
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", fns)
    }
}

impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            StatementKind::Let(mutable, string, _type, expr) => {
                write!(
                    f,
                    "let {}{}{}{}",
                    mutable,
                    string,
                    match _type {
                        Some(t) => format!(": {}", t),
                        None => "".to_string(),
                    },
                    match expr {
                        Some(e) => format!(" = {}", e),
                        None => "".to_string(),
                    }
                )
            }
            StatementKind::Assign(expr, expr1) => write!(f, "{} = {}", expr, expr1),
            StatementKind::While(expr, block) => {
                write!(f, "while {} {}", expr, block)
            }
            StatementKind::Expr(expr) => write!(f, "{}", expr),
            StatementKind::Fn(fn_declaration) => write! {f, "{}", fn_declaration},
        }
    }
}

#[test]
fn display_if_then_else() {
    let ts: proc_macro2::TokenStream = "
    if a {
        let a : i32 = false;
        0
    } else {
        if a == 5 { b = 8 };
        while b {
            e;
        }
        b
    }
    "
    .parse()
    .unwrap();
    let e: crate::ast::Expr = syn::parse2(ts).unwrap();
    println!("ast:\n{:?}", e);

    println!("pretty:\n{}", e);
}

#[test]
fn display_while() {
    let ts: proc_macro2::TokenStream = "
    while a == 9 {
        let b : i32 = 7;
    }
    "
    .parse()
    .unwrap();
    let e: crate::ast::Statement = syn::parse2(ts).unwrap();
    println!("ast:\n{:?}", e);

    println!("pretty:\n{}", e);
}

#[test]
fn display_expr() {
    println!("{}", ExprKind::Ident("a".to_string()));
    println!("{}", ExprKind::Lit(Literal::Int(7)));
    println!("{}", ExprKind::Lit(Literal::Bool(false)));
    let e = ExprKind::BinOp(
        BinOp::Add,
        Box::new(crate::ast::helpers::expr(ExprKind::Ident("a".to_string()))),
        Box::new(crate::ast::helpers::expr(ExprKind::Lit(Literal::Int(7)))),
    );
    println!("{}", e);
    assert_eq!(format!("{}", e), "a + 7");
}

// As you see it becomes cumbersome to write tests
// if you have to construct the Expr by hand.
//
// Instead we might use our parser

#[test]
fn parse_display_expr() {
    let ts: proc_macro2::TokenStream = "a + 7".parse().unwrap();
    let e: crate::ast::Expr = syn::parse2(ts).unwrap();
    println!("e {}", &e.node);
}

// This one will fail (Display for `if` is not yet implemented).
// Implement it as an optional assignment
//
// Hint: You need to implement Display for Statement and Block

#[test]
fn parse_display_if() {
    let ts: proc_macro2::TokenStream = "if a > 5 {5}".parse().unwrap();
    let e: crate::ast::Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}
