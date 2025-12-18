use rnr::ast::*;
use rnr::env::AnnotatedType;
use rnr::parse::parse;
use rnr::test_util::{assert_type, assert_type_fail};
use rnr::type_check::*;

#[cfg(test)]
mod type_expr {
    #![allow(clippy::all)]

    use super::*;
    #[test]
    fn simple_ident() {
        let mut tc = TypeChecker::new();
        let a_ty = AnnotatedType::new(Type::I32, true, true);
        tc.env.define_binding("test", a_ty);
        assert_eq!(
            tc.check_expr(&Expr::Ident("test".to_string()))
                .expect("ident_failed"),
            Type::I32,
        );
    }

    #[test]
    fn simple_lit() {
        let unit = Expr::Lit(Literal::Unit);
        let int = Expr::Lit(Literal::Int(0));
        let string = Expr::Lit(Literal::String("".to_string()));
        let bool = Expr::Lit(Literal::Bool(false));

        assert_type(&unit, Type::Unit);
        assert_type(&int, Type::I32);
        assert_type(&string, Type::String);
        assert_type(&bool, Type::Bool);
    }

    #[test]
    fn simple_par() {
        let bool = Expr::Lit(Literal::Bool(false));
        let par = Expr::Par(Box::new(bool));

        assert_type(&par, Type::Bool);
    }

    #[test]
    fn simple_call() {
        let c: Block = parse(
            r#"
            {
                fn add2(x: i32) -> i32 {
                    x+2
                }
                add2(0)
            }
            "#,
        );

        assert_type(&c, Type::I32);
    }

    #[test]
    fn simple_call_wrong_arg() {
        let c: Block = parse(
            r#"
            {
                fn add2(x: i32) -> i32 {
                    x+2
                }
                add2("test")
            }
            "#,
        );

        assert_type_fail(&c);
    }
    #[test]
    fn simple_if() {
        let b: Block = parse(
            r#"
            {
                if true {
                ()
                }
            }
            "#,
        );

        assert_type(&b, Type::Unit);
    }

    #[test]
    fn simple_if_rtype_i32() {
        let b: Block = parse(
            r#"{
        let x = if true {0} else {0}; 
        x
        }"#,
        );

        assert_type(&b, Type::I32);
    }

    #[test]
    fn simple_if_rtype_missmatch() {
        let b: Block = parse(
            r#"{
        let x = if true {0} else {true}; 
        x
        }"#,
        );

        assert_type_fail(&b);
    }

    #[test]
    fn simple_if_cond_not_bool() {
        let b: Block = parse(
            r#"{
        let x = if 5 {0} else {0}; 
        x
        }"#,
        );

        assert_type_fail(&b);
    }

    #[test]
    fn simple_block() {
        let b: Block = parse(
            r#"{
            let x = 4;
            x
        }"#,
        );
        assert_type(&b, Type::I32);
    }

    #[test]
    fn simple_block_unit() {
        let b: Block = parse(
            r#"{
            let x = 4;
            let y = "s";
            x;
        }"#,
        );
        assert_type(&b, Type::Unit);
    }

    #[test]
    fn simple_bin() {
        let expected = Type::I32;
        let e: Expr = parse("4+4");
        assert_type(&e, expected);
    }

    #[test]
    fn type_expr_bin_op_fail() {
        let p1: Expr = parse("5+true");
        let p2: Expr = parse("true == false");

        assert_type_fail(&p1);
        assert_type_fail(&p2);
    }

    #[test]
    fn type_expr_un_op_fail() {
        let p1 = &Expr::UnOp(UnOp::Neg, Box::new(Expr::Lit(Literal::Bool(true))));
        let p2 = &Expr::UnOp(UnOp::Bang, Box::new(Expr::Lit(Literal::Int(3))));
        let e3 = &Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Lit(Literal::Int(0))),
            Box::new(Expr::Lit(Literal::Bool(true))),
        );
        let p3 = &Expr::UnOp(UnOp::Bang, Box::new(e3.to_owned()));
        let p4 = &Expr::BinOp(
            BinOp::Eq,
            Box::new(Expr::Lit(Literal::Bool(false))),
            Box::new(Expr::Lit(Literal::Bool(true))),
        );

        assert_type_fail(p1);
        assert_type_fail(p2);
        assert_type_fail(p3);
        assert_type_fail(p4);
    }
}

#[cfg(test)]
mod type_statement {

    use super::*;

    #[test]
    fn double_assign_mut() {
        let p: Block = parse(
            r#"
            {
                let mut x: i32;
                x = 4;
                x = 3;
            }
            "#,
        );
        assert_type(&p, Type::Unit);
    }

    #[test]
    fn double_assign_fail() {
        let p: Block = parse(
            r#"
            {
                let x: i32;
                x = 4;
                x = 3;
            }
            "#,
        );
        assert_type_fail(&p);
    }

    #[test]
    fn print_success() {
        let p: Block = parse(
            r#"
            {
                println!("{}", 1);
            }
            "#,
        );
        assert_type(&p, Type::Unit);
    }

    #[test]
    fn print_fail() {
        let p: Block = parse(
            r#"
            {
                println!("{}", true);
            }
            "#,
        );
        assert_type_fail(&p);
    }
}

#[cfg(test)]
mod type_prog {
    #![allow(clippy::all)]

    use rnr::{test_util::assert_value, vm::Val};

    use super::*;
    #[test]
    fn simple_prog() {
        let p: Prog = parse(
            r#"
            fn add2(x: i32) -> i32 {
                fn add3(x: i32) -> i32 {
                    x + 3
                }
                add3(x) + 2
            }
            fn main() -> i32 {
                add2(0)
            }
            "#,
        );
        assert_type(&p, Type::I32);
        assert_value(&p, Val::Lit(Literal::Int(5)));
    }
    // Call println function
    #[test]
    fn println_function() {
        let block: Block = parse(
            "
            {
                println!(\"Hello, world!\");
                println!(\"Value: {}\", 42);
            }
            ",
        );
        assert_type(&block, Type::Unit);
    }

    // Variable declared in inner block shouldn't be callable from outside
    #[test]
    fn type_fail_inner_block_var() {
        let block: Block = parse(
            r#"
        {
            let a = 1;
            {
                let b = a + 1;
            }
            b
        }
        "#,
        );
        assert_type_fail(&block);
    }

    // Same for functions
    #[test]
    fn type_fail_local_fn_not_visible_outside_block() {
        let prog: Prog = parse(
            "
        fn main() {
            {
                fn f(x: i32) -> i32 { x }
            }
            let a = f(1);
        }
        ",
        );
        assert_type_fail(&prog);
    }
}

#[cfg(test)]
mod peer_review_lab6 {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn test_ex3_if() {
        let prog: Prog = parse(
            r#"
        fn max(a: i32, b: i32) -> i32 {
            if a > b { a } else { b }
        }

        fn main() -> i32 {
            max(3, 7)
            
        }"#,
        );
        assert_type(&prog, Type::I32);
    }

    #[test]
    fn test_ex4_while_sum() {
        let prog: Prog = parse(
            r#"
        fn main() -> i32 {
            let mut i: i32 = 0;
            let mut sum: i32 = 0;
            while i < 5 {
                sum = sum + i;
                i = i + 1;
            }
            sum
        }
        "#,
        );
        assert_type(&prog, Type::I32);
    }

    #[test]
    fn test_ex4_block_expr() {
        let prog: Prog = parse(
            r#"
        fn main() -> i32 {
            let x = {
                let a = 2;
                let b = 4;
                a + b
            };
            let y = -x;
            if !(y < 0) { 1 } else { 0 }
        }
        "#,
        );
        assert_type(&prog, Type::I32);
    }
}
