use compiler::test_util::assert_parse_value;

#[cfg(test)]
mod expr_tests {
    use super::*;
    use compiler::ast::Expr;

    #[test]
    fn simple_arithmetic() {
        assert_parse_value::<Expr>("1 + 1", 2);
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;
    use compiler::ast::Block;

    #[test]
    fn bool_and() {
        assert_parse_value::<Block>(
            r#"
            {
                let a = true && false;
                a
            }
        "#,
            false,
        );
    }

    #[test]
    fn bool_bang_nested() {
        assert_parse_value::<Block>(
            r#"
            {
                let a = (!true) && false;
                a
            }
        "#,
            false,
        );
    }

    #[test]
    fn bool_bang_right() {
        assert_parse_value::<Block>(
            r#"
            {
                let a = true && !false;
                a
            }
        "#,
            true,
        );
    }

    #[test]
    fn let_block() {
        assert_parse_value::<Block>(
            r#"
            {
                let a: i32 = 1;
                let b: i32 = 2;
                a + b
            }
        "#,
            3,
        );
    }

    #[test]
    fn let_shadowing() {
        assert_parse_value::<Block>(
            r#"
            {
                let a: i32 = 1;
                let b: i32 = 2;
                let a: i32 = 3;
                let b: i32 = 4;
                a + b
            }
        "#,
            7,
        );
    }

    #[test]
    fn nested_block() {
        assert_parse_value::<Block>(
            r#"
            {
                let a = 1;
                let b = {
                    let b = a;
                    b * 2
                };
                b
            }
        "#,
            2,
        );
    }

    #[test]
    fn assignment() {
        assert_parse_value::<Block>(
            r#"
            {
                let mut a: i32 = 1;
                a = a + 2;
                a
            }
        "#,
            3,
        );
    }

    #[test]
    fn if_expression() {
        assert_parse_value::<Block>(
            r#"
            {
                let mut a: i32 = 1;
                a = if a > 0 { a + 1 } else { a - 2 };
                a
            }
        "#,
            2,
        );
    }

    #[test]
    fn while_loop() {
        assert_parse_value::<Block>(
            r#"
            {
                let a = 2;
                let b = 0;
                while a > 0 {
                    a = a - 1;
                    b = b + 1;
                }
                b
            }
        "#,
            2,
        );
    }

    #[test]
    fn complex_shadowing() {
        assert_parse_value::<Block>(
            r#"
            {
                let a: i32 = 1 + 2;
                let a: i32 = 2 + a;
                if true {
                    a = a - 1;
                    let a: i32 = 0;
                    a = a + 1
                } else {
                    a = a - 1
                };
                a
            }
        "#,
            4,
        );
    }
}

#[cfg(test)]
mod prog_tests {
    use super::*;
    use compiler::ast::{Literal, Prog};
    use compiler::vm::Val;

    #[test]
    fn simple_program() {
        assert_parse_value::<Prog>(
            r#"
            fn main() {
                let a = 1;
                a
            }
        "#,
            1,
        );
    }

    #[test]
    fn local_function() {
        assert_parse_value::<Prog>(
            r#"
            fn main() {
                fn f(i: i32, j: i32) -> i32 {
                    i + j
                }
                let a = f(1, 2);
                println!("a = {} and another a = {}", a, a);
            }
        "#,
            Val::Lit(Literal::Unit),
        );
    }

    #[test]
    fn global_fn_shadowing() {
        assert_parse_value::<Prog>(
            r#"
            fn a() -> i32 {
                b()
            }

            fn b() -> i32 {
                42
            }

            fn main() {
                fn b() -> i32 {
                    99
                }
                a()
            }
        "#,
            42, // expected, still exposes bug
        );
    }

    #[test]
    fn nested_fn_shadowing() {
        assert_parse_value::<Prog>(
            r#"
            fn a() -> i32 {
                fn b() -> i32 {
                    100
                }
                b()
            }

            fn main() {
                a()
            }
        "#,
            100,
        );
    }
}
