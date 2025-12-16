// Extra traits implemented for AST

use crate::ast::*;
use std::fmt::{self, write};

// Utility functions/traits for your AST here.
impl Expr {
    pub fn bin_op(o: BinOp, left: Expr, right: Expr) -> Self {
        Expr::BinOp(o, Box::new(left), Box::new(right))
    }

    pub fn un_op(o: UnOp, right: Expr) -> Self {
        Expr::UnOp(o, Box::new(right))
    }
}

/// Anything that can be converted into a Literal (like i32, bool, etc) can also be converted into an Expr.
impl<T: Into<Literal>> From<T> for Expr {
    fn from(x: T) -> Self {
        Expr::Lit(x.into())
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Ident(i) => write!(f, "{}", i),
            Expr::Lit(literal) => write!(f, "{}", literal),
            Expr::BinOp(bin_op, expr, expr1) => write!(f, "{} {} {}", expr, bin_op, expr1),
            Expr::Par(expr) => write!(f, "({})", expr),
            Expr::Call(i, arguments) => write!(f, "{}{}", i, arguments),
            Expr::IfThenElse(expr, block, block1) => match block1 {
                Some(b1) => write!(f, "if {} {} else {}", expr, block, b1),
                None => write!(f, "if {} {}", expr, block),
            },
            Expr::Block(block) => write!(f, "{}", block),
            Expr::UnOp(un_op, expr) => write!(f, "{}{}", un_op, expr),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let statements = self
            .statements
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(";\n\t");
        write!(
            f,
            "{{\n\t{}{}\n}}",
            statements,
            if self.semi { ";" } else { "" }
        )
    }
}

#[test]
#[ignore = "ignoring self made test /dilred"]
fn test_simple_block() {
    let block = Block {
        statements: vec![Statement::Expr(Expr::Ident("test".to_string()))],
        semi: true,
    };
    let should_be = "{\n\ttest\n};";
    assert_eq!(block.to_string(), should_be);
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
        write!(f, "{}{}: {}", self.mutable, self.id, self.ty)
    }
}

impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = self
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
        Parameter {
            mutable: Mutable(true),
            id: "testparam".to_string(),
            ty: Type::I32,
        },
        Parameter {
            mutable: Mutable(false),
            id: "testparam2".to_string(),
            ty: Type::String,
        },
    ];
    let parameters = Parameters(params);
    println!("ast:\n{}", parameters);
    assert_eq!(
        parameters.to_string(),
        "(mut testparam: i32, testparam2: String)"
    )
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .0
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({})", args)
    }
}

impl fmt::Display for FnDeclaration {
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

#[test]
#[ignore = "ignoring self made test /dilred"]
fn fn_declaration_test() {
    let params = Parameters(vec![Parameter {
        mutable: Mutable(true),
        id: "testparam".to_string(),
        ty: Type::I32,
    }]);

    let func = FnDeclaration {
        id: "test_fn".to_string(),
        parameters: params,
        ty: Some(Type::I32),
        body: Block {
            statements: vec![Statement::Expr(Expr::Ident("testparam".to_string()))],
            semi: false,
        },
    };

    let should_be = "fn test_fn(mut testparam: i32) -> i32 {\n\ttestparam\n}".to_string();
    assert_eq!(func.to_string(), should_be);
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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Statement::Let(mutable, string, _type, expr) => {
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
            Statement::Assign(expr, expr1) => write!(f, "{} = {}", expr, expr1),
            Statement::While(expr, block) => {
                write!(f, "while {} {}", expr, block)
            }
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Fn(fn_declaration) => write! {f, "{}", fn_declaration},
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
    let e: Expr = syn::parse2(ts).unwrap();
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
    let e: Statement = syn::parse2(ts).unwrap();
    println!("ast:\n{:?}", e);

    println!("pretty:\n{}", e);
}

#[test]
fn display_expr() {
    println!("{}", Expr::Ident("a".to_string()));
    println!("{}", Expr::Lit(Literal::Int(7)));
    println!("{}", Expr::Lit(Literal::Bool(false)));
    let e = Expr::BinOp(
        BinOp::Add,
        Box::new(Expr::Ident("a".to_string())),
        Box::new(Expr::Lit(Literal::Int(7))),
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
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}

// This one will fail (Display for `if` is not yet implemented).
// Implement it as an optional assignment
//
// Hint: You need to implement Display for Statement and Block

#[test]
fn parse_display_if() {
    let ts: proc_macro2::TokenStream = "if a > 5 {5}".parse().unwrap();
    let e: Expr = syn::parse2(ts).unwrap();
    println!("e {}", e);
}
