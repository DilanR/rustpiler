//#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(double_negations)]
#![allow(clippy::assign_op_pattern)]

use compiler::ast::*;
use compiler::parse::*;
use syn::Result;

use compiler::test_util::*;

#[cfg(test)]
mod parse_lit {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn parse_lit_int() {
        let lit: Literal = parse("1");
        assert_eq!(lit, Literal::Int(1));
    }

    #[test]
    fn parse_lit_neg_int() {
        let lit: Literal = parse("-1");
        assert_eq!(lit, Literal::Int(-1));
    }

    #[test]
    fn parse_lit_bool_false() {
        let lit: Literal = parse("false");
        assert_eq!(lit, Literal::Bool(false));
    }

    #[test]
    fn parse_lit_string() {
        let lit: Literal = parse("\"abba\"");
        assert_eq!(lit, Literal::String("abba".to_string()));
    }

    #[test]
    fn parse_lit_fail() {
        assert_parse_fail::<Literal>("a");
        assert_parse_fail::<Literal>("-");
        assert_parse_fail::<Literal>("'hello'");
    }
}

#[cfg(test)]
mod parse_binop {

    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn add() {
        let op: BinOp = parse("+");
        assert_eq!(op, BinOp::Add);
    }

    #[test]
    fn sub() {
        let op: BinOp = parse("-");
        assert_eq!(op, BinOp::Sub);
    }

    #[test]
    fn mul() {
        let op: BinOp = parse("*");
        assert_eq!(op, BinOp::Mul);
    }

    #[test]
    fn div() {
        let op: BinOp = parse("/");
        assert_eq!(op, BinOp::Div);
    }

    #[test]
    fn and() {
        let op: BinOp = parse("&&");
        assert_eq!(op, BinOp::And);
    }

    #[test]
    fn or() {
        let op: BinOp = parse("||");
        assert_eq!(op, BinOp::Or);
    }

    #[test]
    fn eq() {
        let op: BinOp = parse("==");
        assert_eq!(op, BinOp::Eq);
    }

    #[test]
    fn lt() {
        let op: BinOp = parse("<");
        assert_eq!(op, BinOp::Lt);
    }

    #[test]
    fn gt() {
        let op: BinOp = parse(">");
        assert_eq!(op, BinOp::Gt);
    }

    #[test]
    fn parse_op_fail() {
        assert_parse_fail::<BinOp>("1");
        assert_parse_fail::<BinOp>("x");
        assert_parse_fail::<BinOp>(".");
    }
}

#[cfg(test)]
mod parse_unop {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn bang() {
        let op: UnOp = parse("!");
        assert_eq!(op, UnOp::Bang);
    }

    #[test]
    fn neg() {
        let op: UnOp = parse("-");
        assert_eq!(op, UnOp::Neg);
    }

    #[test]
    fn parse_unop_fail() {
        assert_parse_fail::<UnOp>(".");
        assert_parse_fail::<UnOp>("/");
        assert_parse_fail::<UnOp>("x");
        assert_parse_fail::<UnOp>("1");
        assert_parse_fail::<UnOp>("i32");
        assert_parse_fail::<UnOp>("true");
        assert_parse_fail::<UnOp>("()");
    }
}

#[cfg(test)]
mod parse_expr {
    #![allow(clippy::all)]
    use super::*;

    // NOTE: Most tests that involve expressions have been moved to the `expr` module in
    // the `tests/integration_tests.rs` file.
    // Here we just focus on checking that parsing does not panic, and also checking that binary
    // operator expressions are correctly parsed (in terms of associativity and precedence).
    // In particular, these tests do not evaluate expressions!

    #[test]
    fn literal() {
        parse::<Expr>("123");
        parse::<Expr>("true");
        parse::<Expr>("()");
        parse::<Expr>("\"hello world\"");
    }

    #[test]
    fn binary_op() {
        parse::<Expr>("1 + 2");
        parse::<Expr>("1 - 2");
        parse::<Expr>("1 * 2");
        parse::<Expr>("1 / 2");
        parse::<Expr>("1 + 2 + 3");
        parse::<Expr>("1 * 2 - 3 / 1");
        parse::<Expr>("(1) + (2)");
        parse::<Expr>("(1 + 2) + (3 + 4)");
        parse::<Expr>("(1 * 2) - (3 / 4)");
        parse::<Expr>("true || false");
        parse::<Expr>("true && false");
        parse::<Expr>("true && (false || true) && false");
    }

    #[test]
    fn binary_op_comparisons() {
        parse::<Expr>("1 == 2");
        parse::<Expr>("1 < 2");
        parse::<Expr>("1 > 2");
        parse::<Expr>("1 + (2 * 3) == (2 - 3) / 4");
        parse::<Expr>("1 + 2 < 2 * 3");
        parse::<Expr>("1 + 2 > (2 - 1)");
        parse::<Expr>("true == true");
        parse::<Expr>("true || false == true && (false || true)");
    }

    #[test]
    fn unary_op() {
        parse::<Expr>("-(1)");
        parse::<Expr>("-(1+2)");
        parse::<Expr>("-(2 * 3 / 2)");
        parse::<Expr>("!true");
        parse::<Expr>("!true && true");
        parse::<Expr>("!true || !false");
        parse::<Expr>("!(true && !true)");
    }

    #[test]
    fn identifier() {
        parse::<Expr>("my_variable");
        parse::<Expr>("var");
        parse::<Expr>("var123");
        parse::<Expr>("_var_123_");
    }

    #[test]
    fn operators_and_identifiers() {
        parse::<Expr>("-my_variable");
        parse::<Expr>("!var");
        parse::<Expr>("321 + var123 - 123");
        parse::<Expr>("(1 - _var_123_ * 2) == a && (!b || true) || !a");
    }

    // Trying to parse these expressions should fail.

    #[test]
    fn fail_tests() {
        assert_parse_fail::<Expr>("12 34");
        assert_parse_fail::<Expr>("+");
        assert_parse_fail::<Expr>("1+");
        assert_parse_fail::<Expr>("1++2");
        assert_parse_fail::<Expr>("1+2+3+4+");
        assert_parse_fail::<Expr>("(1+2+3+4");
        assert_parse_fail::<Expr>("1+2+3+4)");
        assert_parse_fail::<Expr>("1)+2+(3+4");
        assert_parse_fail::<Expr>("3(1+2)");
        assert_parse_fail::<Expr>("(1+2)3");
        assert_parse_fail::<Expr>("(1+2)(3+4)");
        assert_parse_fail::<Expr>("12!34");
        assert_parse_fail::<Expr>("true ! false");
        assert_parse_fail::<Expr>("(2 * 4) - ");
    }

    // Some helpers for building Expr ASTs.

    fn add<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Expr {
        Spanned::dummy(ExprKind::bin_op(BinOp::Add, left.into(), right.into()))
    }

    fn mul<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Expr {
        Spanned::dummy(ExprKind::bin_op(BinOp::Mul, left.into(), right.into()))
    }

    fn or<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Expr {
        Spanned::dummy(ExprKind::bin_op(BinOp::Or, left.into(), right.into()))
    }

    fn and<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Expr {
        Spanned::dummy(ExprKind::bin_op(BinOp::And, left.into(), right.into()))
    }

    fn eq<T1: Into<Expr>, T2: Into<Expr>>(left: T1, right: T2) -> Expr {
        Spanned::dummy(ExprKind::bin_op(BinOp::Eq, left.into(), right.into()))
    }

    pub fn paren(_expr: Expr) -> Expr {
        Spanned::dummy(ExprKind::Par(Box::new(_expr)))
    }

    // Here are some test cases that directly examine the AST that is built from the expressions to
    // make sure that precedence and associativity are handled correctly.

    #[test]
    fn precedence_and_associativity_1() {
        let expr: Expr = parse("1+2+3");
        let expected = add(add(1, 2), 3);
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_2() {
        let expr: Expr = parse("1+2*3");
        let expected = add(1, mul(2, 3));
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_3() {
        let expr: Expr = parse("1+2*3+4");
        let expected = add(add(1, mul(2, 3)), 4);
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_4() {
        let expr: Expr = parse("1+2*3 == 4");
        let expected = eq(add(1, mul(2, 3)), 4);
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_5() {
        let expr: Expr = parse("1+2*3 == 1+2*3");
        let expected = eq(add(1, mul(2, 3)), add(1, mul(2, 3)));
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_6() {
        let expr: Expr = parse("1*2+3*4+5*6");
        let expected = add(add(mul(1, 2), mul(3, 4)), mul(5, 6));
        assert_eq!(expr, expected);
    }

    #[test]
    fn precedence_and_associativity_7() {
        let expr: Expr = parse("1+2 * 3+4 == 5*(6+7) + 8*9");
        let left = add(add(1, mul(2, 3)), 4);
        let right = add(mul(5, paren(add(6, 7))), mul(8, 9));
        let expected = eq(left, right);
        assert_eq!(expr, expected);
    }

    // NOTE: priorities: `==` > `&&` > `||`
    // (Comparisons take precedence over and/or!)

    #[test] // 1 2 3
    fn precedence_and_associativity_123() {
        let expr: Expr = parse("true || true && true == true");
        let expected = or(true, and(true, eq(true, true)));
        assert_eq!(expr, expected);
    }

    #[test] // 1 3 2
    fn precedence_and_associativity_132() {
        let expr: Expr = parse("true || true == true && true");
        let expected = or(true, and(eq(true, true), true));
        assert_eq!(expr, expected);
    }

    #[test] // 2 1 3
    fn precedence_and_associativity_213() {
        let expr: Expr = parse("true && true || true == true");
        let expected = or(and(true, true), eq(true, true));
        assert_eq!(expr, expected);
    }

    #[test] // 2 3 1
    fn precedence_and_associativity_231() {
        let expr: Expr = parse("true && true == true || true");
        let expected = or(and(true, eq(true, true)), true);
        assert_eq!(expr, expected);
    }

    #[test] // 3 1 2
    fn precedence_and_associativity_312() {
        let expr: Expr = parse("true == true || true && true");
        let expected = or(eq(true, true), and(true, true));
        assert_eq!(expr, expected);
    }

    #[test] // 3 2 1
    fn precedence_and_associativity_321() {
        let expr: Expr = parse("true == true && true || true");
        let expected = or(and(eq(true, true), true), true);
        assert_eq!(expr, expected);
    }
}

#[cfg(test)]
mod parse_block {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn test_block_expr_fail() {
        let ts: proc_macro2::TokenStream = "{ let a = }".parse().unwrap();
        let stmt: Result<Statement> = syn::parse2(ts);
        println!("stmt {:?}", stmt);
        assert!(stmt.is_err());
    }

    #[test]
    fn test_block_semi() {
        let ts: proc_macro2::TokenStream = "
        {
            let a : i32 = 1;
            a = 5;
            a + 5;
        }"
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.node.statements.len(), 3);
        assert!(bl.node.semi);
    }

    #[test]
    fn test_block_no_semi() {
        let ts: proc_macro2::TokenStream = "
        {
            let a : i32 = 1;
            a = 5;
            a + 5
        }"
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.node.statements.len(), 3);
        assert!(!bl.node.semi);
    }

    #[test]
    fn test_block_fn() {
        let ts: proc_macro2::TokenStream = "
        {
            let a : i32 = 1;
            fn t() {}
            a = 5;
            a + 5
        }"
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.node.statements.len(), 4);
        assert!(!bl.node.semi);
    }

    #[test]
    fn test_block_while() {
        let ts: proc_macro2::TokenStream = "
        {
            let a : i32 = 1;
            while true {}
            a = 5;
            a + 5
        }"
        .parse()
        .unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.node.statements.len(), 4);
        assert!(!bl.node.semi);
    }

    #[test]
    fn test_block2() {
        let ts: proc_macro2::TokenStream = "{ let b : bool = false; b = true }".parse().unwrap();
        let bl: Block = syn::parse2(ts).unwrap();
        println!("bl {:?}", bl);
        assert_eq!(bl.node.statements.len(), 2);
        assert!(!bl.node.semi);
    }

    #[test]
    fn test_expr_block() {
        let ts: proc_macro2::TokenStream = "
        {
            12
        }
        "
        .parse()
        .unwrap();
        println!("{:?}", ts);
        let e: Expr = syn::parse2(ts).unwrap();
        println!("e {:?}", e);
    }

    #[test]
    fn test_block_fail() {
        let ts: proc_macro2::TokenStream = "{ let a = 1 a = 5 }".parse().unwrap();
        let bl: Result<Block> = syn::parse2(ts);
        println!("bl {:?}", bl);

        assert!(bl.is_err());
    }
}

#[cfg(test)]
mod parse_prog {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn test_prog() {
        let ts: proc_macro2::TokenStream = "
        fn a(a: i32) { let b = a; }
        fn b() -> i32 { 3 }

        fn main() {

        }
        "
        .parse()
        .unwrap();
        let pr: Result<Prog> = syn::parse2(ts);
        println!("prog\n{:?}", pr.unwrap());
    }

    #[ignore = "Requires references to implemented"]
    #[test]
    fn test_ref_de_ref() {
        let ts: proc_macro2::TokenStream = "
        fn main() {
            let a = &1;
            let mut a = &mut 1;
            *a = *a + 1;
            println!(\"{}\", *a);
        }
        "
        .parse()
        .unwrap();
        let pr: Result<Prog> = syn::parse2(ts);
        println!("prog\n{:?}", pr.unwrap());
    }
}

#[cfg(test)]
mod parse_if {
    #![allow(clippy::all)]
    use super::*;

    // This test is not really a test of our parser
    // Added just a reference to how Rust would treat the nesting.
    #[test]
    #[allow(unused_must_use)]
    fn test_if_then_else_nested_rust() {
        if false {
            2;
        } else {
            if true {
                3 + 5;
            }
        };
    }

    // This test is not really a test of our parser
    // Added just a reference to how Rust would treat the nesting.
    #[test]
    #[allow(unused_must_use)]
    fn test_if_then_else_nested_rust2() {
        if false {
            2;
        } else if true {
            3 + 5;
        };
    }

    // NOTE: These tests just parse some if-expressions and just (implicitly) check that there are
    // no panics.

    #[test]
    fn test_if_then_else_nested2() {
        let src = "
        if false {
            2;
        } else if true {
            3 + 5;
        }";
        let e: Expr = parse(src);
    }

    #[test]
    fn test_if_then_else_nested() {
        let src = "
        if false {
            2;
        } else {
            if true {
                3 + 5;
            }
        }";
        let e: Expr = parse(src);
    }

    #[test]
    fn test_if_then_else_nested3() {
        let src = "
        if false {
            2;
        } else if true {
            3 + 5;
        } else if false {
            let a : i32 = 0;
        } else {
            5
        }
        ";
        let e: Expr = parse(src);
    }

    #[test]
    fn test_expr_if_then_else() {
        let src = "if a > 0 {1} else {2}";
        let e: Expr = parse(src);
    }
}

#[cfg(test)]
mod parse_type {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn parse_type_i32() {
        let typ: Type = parse("i32");
        assert_eq!(typ, Type::i32());
    }

    #[test]
    fn parse_type_bool() {
        let typ: Type = parse("bool");
        assert_eq!(typ, Type::bool());
    }

    #[test]
    fn parse_type_unit() {
        let typ: Type = parse("()");
        assert_eq!(typ, Type::unit());
    }

    #[test]
    fn parse_type_fail() {
        assert_parse_fail::<Type>("u32");
        assert_parse_fail::<Type>("I32");
        assert_parse_fail::<Type>("123");
        assert_parse_fail::<Type>("boolean");
        assert_parse_fail::<Type>("Bool");
        assert_parse_fail::<Type>("true");
        assert_parse_fail::<Type>("false");
    }
}

#[cfg(test)]
mod parse_fn_calls {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn args() {
        parse::<Arguments>("(1)");
        parse::<Arguments>("(a)");
        parse::<Arguments>("(a, b)");
        parse::<Arguments>("(a + 1, b * 2)");
        parse::<Arguments>("(1, 2, 3 + 4)");
    }

    #[test]
    fn function_call() {
        parse::<Expr>("foo()");
        parse::<Expr>("foo(1)");
        parse::<Expr>("foo(true)");
        parse::<Expr>("foo(true, false)");
        parse::<Expr>("foo(true || false)");
        parse::<Expr>("foo(1 + 2)");
        parse::<Expr>("foo(1, 2)");
        parse::<Expr>("foo(1, 2 + 2)");
        parse::<Expr>("foo(my_variable)");
        parse::<Expr>("foo(a, b, c)");
        parse::<Expr>("foo(\"passing a string\")");
        parse::<Expr>("ident({1}, {let a = 6; a },)");
    }

    #[test]
    fn function_call_extra_comma() {
        parse::<Expr>("foo(1,)");
        parse::<Expr>("foo(1, 2,)");
        parse::<Expr>("foo(1, 2 + 2,)");
        parse::<Expr>("foo(a,)");
        parse::<Expr>("foo(a, b, c,)");
        parse::<Expr>("foo(true, false,)");
    }

    #[test]
    fn fail_tests() {
        assert_parse_fail::<Expr>("foo(,)");
        assert_parse_fail::<Expr>("foo(+)");
        assert_parse_fail::<Expr>("foo(1+)");
        assert_parse_fail::<Expr>("foo(2 * 4, -)");
    }
}

#[cfg(test)]
mod parse_fn_declaration {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn param() {
        parse::<Parameter>("a: i32");
        parse::<Parameter>("b: bool");
    }

    #[test]
    fn params() {
        parse::<Parameters>("(a: i32)");
        parse::<Parameters>("(a: i32,)");
        parse::<Parameters>("(b: bool)");
        parse::<Parameters>("(a: i32, b: bool)");
        parse::<Parameters>("(a: i32, b: bool,)");
    }

    #[test]
    fn fn_no_type() {
        parse::<FnDeclaration>("fn foo() {}");
        parse::<FnDeclaration>("fn foo(a: i32, b: bool) {}");
    }

    #[test]
    fn fn_with_type() {
        parse::<FnDeclaration>("fn foo() -> i32 {}");
        parse::<FnDeclaration>("fn foo(a: i32, b: bool) -> i32 {}");
        parse::<FnDeclaration>("fn foo() -> () {}");
        parse::<FnDeclaration>("fn foo(a: i32, b: bool) -> () {}");
        parse::<FnDeclaration>("fn foo() -> bool {}");
        parse::<FnDeclaration>("fn foo(a: i32, b: bool) -> bool {}");
    }

    #[test]
    fn test_println() {
        let src = "println!(\"{}\", 1)";
        let expr: Expr = parse(src);
    }

    // Trying to parse these function declarations should fail.

    #[test]
    fn fail_tests() {
        assert_parse_fail::<Parameter>("123");
        assert_parse_fail::<Parameter>("i32");
        assert_parse_fail::<Parameter>("a = i32");
        assert_parse_fail::<FnDeclaration>("fn 123() {}");
        assert_parse_fail::<FnDeclaration>("fn foo(a, b: i32) {}");
        assert_parse_fail::<FnDeclaration>("fn foo(): i32 {}");
    }
}

#[cfg(test)]
mod parse_statement {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn test_statement_let_ty_expr() {
        let stmt: Statement = parse("let a: i32 = 2");
        let expected = Spanned::dummy(StatementKind::Let(
            Mutable(false),
            "a".to_string(),
            Some(Type::i32()),
            Some(Spanned::dummy(ExprKind::Lit(Literal::Int(2)))),
        ));
        assert_eq!(stmt, expected);
    }

    #[test]
    fn test_statement_let_mut_ty_expr() {
        let stmt: Statement = parse("let mut a: i32 = 2");
        let expected = Spanned::dummy(StatementKind::Let(
            Mutable(true),
            "a".to_string(),
            Some(Type::i32()),
            Some(Spanned::dummy(ExprKind::Lit(Literal::Int(2)))),
        ));
        assert_eq!(stmt, expected);
    }

    #[test]
    fn test_statement_let() {
        let stmt: Statement = parse("let a");
        let expected = Spanned::dummy(StatementKind::Let(
            Mutable(false),
            "a".to_string(),
            None,
            None,
        ));
        assert_eq!(stmt, expected);
    }

    #[test]
    fn test_statement_assign() {
        let stmt: Statement = parse("a = false");
        let expected = Spanned::dummy(StatementKind::Assign(
            Spanned::dummy(ExprKind::Ident("a".to_string())),
            Spanned::dummy(ExprKind::Lit(Literal::Bool(false))),
        ));
        assert_eq!(stmt, expected);
    }

    #[test]
    fn test_statement_while() {
        let stmt: Statement = parse("while a {}");
        let expected = Spanned::dummy(StatementKind::While(
            Spanned::dummy(ExprKind::Ident("a".to_string())),
            Spanned::dummy(BlockKind {
                statements: vec![],
                semi: false,
            }),
        ));
        assert_eq!(stmt, expected);
    }

    #[test]
    fn test_statement_expr() {
        let stmt: Statement = parse("a");
        println!("stmt {:?}", stmt);
        assert_eq!(
            stmt,
            Spanned::dummy(StatementKind::Expr(Spanned::dummy(ExprKind::Ident(
                "a".to_string()
            ))))
        );
    }

    // Trying to parse these statements should fail.

    #[test]
    fn fail_tests() {
        assert_parse_fail::<Statement>("let a i32;");
        assert_parse_fail::<Statement>("let a: I32;");
        assert_parse_fail::<Statement>("let a: i32 == 3;");
        assert_parse_fail::<Statement>("let 123;");
        assert_parse_fail::<Statement>("let 123: i32;");
        assert_parse_fail::<Statement>("123_var = 3;");
        assert_parse_fail::<Statement>("while true { let x }");
        assert_parse_fail::<Statement>("while {}");
        // NOTE: we could also test something like "123 = 3", but we will want to allow the
        // left-hand side to be an expression (such as `xs[0] = 3`). So checking what kinds of
        // expressions are allowed on the left of an assignment would require a bit more work and
        // is probably best done by the type checker or VM.
    }
}

#[cfg(test)]
#[cfg(test)]
mod span_tests {
    use super::*;

    #[test]
    fn span_exists_on_expr() {
        let expr: Expr = parse("1 + 2");

        let _ = expr.span;

        if let ExprKind::BinOp(_, left, right) = &expr.node {
            let _ = left.span;
            let _ = right.span;
        } else {
            panic!("expected binop");
        }
    }

    #[test]
    fn span_binop_joinable() {
        let expr: Expr = parse("1 + 2");

        if let ExprKind::BinOp(_, left, right) = &expr.node {
            let joined = left.span.join(right.span);
            assert!(joined.is_some());
        } else {
            panic!("expected binop");
        }
    }

    #[test]
    fn span_paren_differs_from_inner() {
        let expr: Expr = parse("(1)");

        if let ExprKind::Par(inner) = &expr.node {
            assert_ne!(format!("{:?}", expr.span), format!("{:?}", inner.span));
        } else {
            panic!("expected paren");
        }
    }

    #[test]
    fn span_nested_binop_joinable() {
        let expr: Expr = parse("1 + 2 * 3");

        if let ExprKind::BinOp(_, _, right) = &expr.node {
            if let ExprKind::BinOp(_, r_left, r_right) = &right.node {
                assert!(r_left.span.join(r_right.span).is_some());
            } else {
                panic!("expected nested binop");
            }
        } else {
            panic!("expected binop");
        }
    }

    #[test]
    fn span_literal_exists() {
        let expr: Expr = parse("42");

        match &expr.node {
            ExprKind::Lit(_) => {
                let _ = expr.span;
            }
            _ => panic!("expected literal"),
        }
    }

    #[test]
    fn span_unop_exists() {
        let expr: Expr = parse("!true");

        if let ExprKind::UnOp(_, inner) = &expr.node {
            let _ = expr.span;
            let _ = inner.span;
        } else {
            panic!("expected unary op");
        }
    }

    #[test]
    fn span_if_expr_exists() {
        let expr: Expr = parse("if true {1} else {2}");

        if let ExprKind::IfThenElse(cond, _then_block, else_block) = &expr.node {
            let _ = expr.span;
            let _ = cond.span;

            if let Some(block) = else_block {
                let _ = block;
            }
        } else {
            panic!("expected if expression");
        }
    }

    #[test]
    fn span_block_expr_exists() {
        let expr: Expr = parse("{ 1 }");

        if let ExprKind::Block(block) = &expr.node {
            let _ = expr.span;
            assert!(block.node.statements.len() == 1);
        } else {
            panic!("expected block");
        }
    }

    #[test]
    fn span_ident_exists() {
        let expr: Expr = parse("abc");

        if let ExprKind::Ident(_) = &expr.node {
            let _ = expr.span;
        } else {
            panic!("expected ident");
        }
    }

    #[test]
    fn span_call_exists() {
        let expr: Expr = parse("foo(1, 2)");

        if let ExprKind::Call(_, args) = &expr.node {
            let _ = expr.span;

            for arg in &args.node.0 {
                let _ = arg.span;
            }
        } else {
            panic!("expected call");
        }
    }
}
