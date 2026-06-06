use compiler::ast::*;
use compiler::env::AnnotatedType;
use compiler::test_util::*;
use compiler::type_check::TypeChecker;

#[cfg(test)]
mod type_expr {
    #![allow(clippy::all)]

    use super::*;

    #[test]
    fn simple_ident() {
        let mut tc = TypeChecker::new();
        let a_ty = AnnotatedType::new(Type::i32(), true, true);
        tc.env.define_binding("test", a_ty);
        assert_eq!(
            tc.check_expr(&ExprKind::Ident("test".to_string()).into())
                .expect("ident_failed"),
            Type::i32(),
        );
    }

    #[test]
    fn simple_lit() {
        let unit: Expr = ExprKind::Lit(Literal::Unit).into();
        let int: Expr = ExprKind::Lit(Literal::Int(0)).into();
        let string: Expr = ExprKind::Lit(Literal::String("".to_string())).into();
        let bool: Expr = ExprKind::Lit(Literal::Bool(false)).into();

        assert_type(&unit, Type::unit());
        assert_type(&int, Type::i32());
        assert_type(&string, Type::string());
        assert_type(&bool, Type::bool());
    }

    #[test]
    fn simple_par() {
        let par: Expr = ExprKind::Par(Box::new(ExprKind::Lit(Literal::Bool(false)).into())).into();
        assert_type(&par, Type::bool());
    }

    #[test]
    fn simple_call() {
        assert_parse_type::<Block>(
            r#"
            {
                fn add2(x: i32) -> i32 {
                    x+2
                }
                add2(0)
            }
            "#,
            Type::i32(),
        );
    }

    #[test]
    fn simple_call_wrong_arg() {
        assert_parse_type_fail::<Block>(
            r#"
            {
                fn add2(x: i32) -> i32 {
                    x+2
                }
                add2("test")
            }
            "#,
        );
    }

    #[test]
    fn simple_if() {
        assert_parse_type::<Block>(
            r#"
            {
                if true { () }
            }
            "#,
            Type::unit(),
        );
    }

    #[test]
    fn simple_if_rtype_i32() {
        assert_parse_type::<Block>(
            r#"{
                let x = if true {0} else {0}; 
                x
            }"#,
            Type::i32(),
        );
    }

    #[test]
    fn simple_if_rtype_missmatch() {
        assert_parse_type_fail::<Block>(
            r#"{
                let x = if true {0} else {true}; 
                x
            }"#,
        );
    }

    #[test]
    fn simple_if_cond_not_bool() {
        assert_parse_type_fail::<Block>(
            r#"{
                let x = if 5 {0} else {0}; 
                x
            }"#,
        );
    }

    #[test]
    fn simple_block() {
        assert_parse_type::<Block>(
            r#"{
                let x = 4;
                x
            }"#,
            Type::i32(),
        );
    }

    #[test]
    fn simple_block_unit() {
        assert_parse_type::<Block>(
            r#"{
                let x = 4;
                let y = "s";
                x;
            }"#,
            Type::unit(),
        );
    }

    #[test]
    fn simple_bin() {
        assert_parse_type::<Expr>("4+4", Type::i32());
    }

    #[test]
    fn type_expr_bin_op_fail() {
        assert_parse_type_fail::<Expr>("5+true");
        assert_parse_type_fail::<Expr>("true == false");
    }

    #[test]
    fn type_expr_un_op_fail() {
        let p1: Expr = ExprKind::UnOp(
            UnOp::Neg,
            Box::new(ExprKind::Lit(Literal::Bool(true)).into()),
        )
        .into();

        let p2: Expr =
            ExprKind::UnOp(UnOp::Bang, Box::new(ExprKind::Lit(Literal::Int(3)).into())).into();

        assert_type_fail(&p1);
        assert_type_fail(&p2);
    }
}

#[cfg(test)]
mod type_statement {
    use super::*;

    #[test]
    fn double_assign_mut() {
        assert_parse_type::<Block>(
            r#"
            {
                let mut x: i32;
                x = 4;
                x = 3;
            }
            "#,
            Type::unit(),
        );
    }

    #[test]
    fn double_assign_fail() {
        assert_parse_type_fail::<Block>(
            r#"
            {
                let x: i32;
                x = 4;
                x = 3;
            }
            "#,
        );
    }

    #[test]
    fn print_success() {
        assert_parse_type::<Block>(
            r#"
            {
                println!("{}", 1);
            }
            "#,
            Type::unit(),
        );
    }

    #[test]
    fn print_fail() {
        assert_parse_type_fail::<Block>(
            r#"
            {
                println!("{}", true);
            }
            "#,
        );
    }
}

#[cfg(test)]
mod type_prog {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn simple_prog() {
        assert_parse_eval::<Prog>(
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
            5,
        );
    }

    #[test]
    fn println_function() {
        assert_parse_type::<Block>(
            r#"
            {
                println!("Hello, world!");
                println!("Value: {}", 42);
            }
            "#,
            Type::unit(),
        );
    }

    #[test]
    fn type_fail_inner_block_var() {
        assert_parse_type_fail::<Block>(
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
    }

    #[test]
    fn type_fail_local_fn_not_visible_outside_block() {
        assert_parse_type_fail::<Prog>(
            r#"
            fn main() {
                {
                    fn f(x: i32) -> i32 { x }
                }
                let a = f(1);
            }
            "#,
        );
    }
}

#[cfg(test)]
mod peer_review_lab6 {
    #![allow(clippy::all)]
    use super::*;

    #[test]
    fn test_ex3_if() {
        assert_parse_type::<Prog>(
            r#"
            fn max(a: i32, b: i32) -> i32 {
                if a > b { a } else { b }
            }

            fn main() -> i32 {
                max(3, 7)
            }"#,
            Type::i32(),
        );
    }

    #[test]
    fn test_ex4_while_sum() {
        assert_parse_type::<Prog>(
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
            Type::i32(),
        );
    }

    #[test]
    fn test_ex4_block_expr() {
        assert_parse_type::<Prog>(
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
            Type::i32(),
        );
    }
}

#[cfg(test)]
mod type_failures_deep {
    use super::*;

    #[test]
    fn fail_missing_return_expr() {
        assert_parse_type_fail::<Prog>(
            r#"
            fn main() -> i32 {
                let x = 5;
            }
        "#,
        );
    }

    #[test]
    fn fail_trailing_semicolon_kills_return() {
        assert_parse_type_fail::<Prog>(
            r#"
            fn main() -> i32 {
                5;
            }
        "#,
        );
    }

    #[test]
    fn fail_unit_expected_but_expr_present() {
        assert_parse_type_fail::<Prog>(
            r#"
            fn main() -> () {
                5
            }
        "#,
        );
    }

    #[test]
    fn fail_if_branch_mismatch() {
        assert_parse_type_fail::<Block>(
            r#"
            {
                if true { 1 } else { false }
            }
        "#,
        );
    }

    #[test]
    fn fail_if_missing_else_used_as_value() {
        assert_parse_type_fail::<Block>(
            r#"
            {
                let x: i32 = if true { 5 };
                x
            }
        "#,
        );
    }

    #[test]
    fn fail_if_semicolon_erases_value() {
        assert_parse_type_fail::<Prog>(
            r#"
            fn main() -> i32 {
                if true { 5 } else { 6 };
            }
        "#,
        );
    }
}
