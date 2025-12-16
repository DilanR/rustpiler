use mips::instr::Instr;
use mips::instrs::Instrs;
use mips::rf::Reg::t0;
use mips::vm::Mips;
use rnr::ast::*;
use rnr::common::codegen_instrs;

fn run_expr_i32(src: &str) -> i32 {
    let instrs = codegen_instrs::<Expr>(src).unwrap();
    run_instrs(instrs).rf.get(t0) as i32
}

fn run_block_i32(src: &str) -> i32 {
    let instrs = codegen_instrs::<Block>(src).unwrap();
    run_instrs(instrs).rf.get(t0) as i32
}

fn run_prog_i32(src: &str) -> i32 {
    let instrs = codegen_instrs::<Prog>(src).unwrap();
    run_instrs(instrs).rf.get(t0) as i32
}

fn run_instrs(instrs: Vec<Instr>) -> Mips {
    let mut mips = Mips::new(Instrs::new_from_slice(&instrs));
    mips.run().ok();
    mips
}

#[cfg(test)]
mod codegen_expr_test {

    use super::*;

    #[test]
    fn binop_tests_success() {
        assert_eq!(run_expr_i32("1+2+3"), 6);
        assert_eq!(run_expr_i32("1+2-3"), 0);
        assert_eq!(run_expr_i32("4==4"), true.into());
        assert_eq!(run_expr_i32("true && false"), false.into());
        assert_eq!(run_expr_i32("true || false"), true.into());
        assert_eq!(run_expr_i32("1<2"), true.into());
        assert_eq!(run_expr_i32("1>2"), false.into());
        assert_eq!(run_expr_i32("4-(1+2)"), 1);
    }

    #[test]
    fn unop_tests_success() {
        assert_eq!(run_expr_i32("!true"), 0);
        assert_eq!(run_expr_i32("!!(2>1)"), 1);
    }

    #[test]
    fn if_then_else_test() {
        assert_eq!(run_expr_i32("if 1>2 {1} else {2}"), 2);
    }
}

#[cfg(test)]
mod codegen_block_test {

    use super::*;

    #[test]
    fn block_expr_test() {
        assert_eq!(run_block_i32("{1+2+3}"), 6);
    }

    #[test]
    fn blocks_tests() {
        assert_eq!(run_block_i32("{let x = 5; let y = 3; x}"), 5);
        assert_eq!(run_block_i32("{let x = 5; x = 3; let y = 3; x}"), 3);
        assert_eq!(run_block_i32("{1}"), 1);
    }

    #[test]
    fn while_tests() {
        assert_eq!(run_block_i32("{let x = 2;while x > 1 {x = x-1}; x}"), 1);
    }

    #[test]
    fn test_block_if_else() {
        assert_eq!(run_block_i32("{if true { 1 } else { 0 } }"), 1);
    }
    #[test]
    fn while_inside_if() {
        let block = r#"
        { 
            let x = 0;
            if true { 
                while x < 3 { x = x + 1 };
            } else { x = 100 };
            x
        }"#;
        assert_eq!(run_block_i32(block), 3);
    }
    #[test]
    fn if_assign_then_branch() {
        assert_eq!(
            run_block_i32("{ let x = 0; if true { x = 7 } else { x = 3 }; x }"),
            7
        );
    }
}

#[cfg(test)]
mod peer_review_alexander_pettersson {
    use super::*;

    #[test]
    fn codegen_block_let_simple() {
        // Tests: basic let-binding + reading locals inside main.
        let src = r#"
        fn main() -> i32 {
            let a: i32 = 1;
            let b: i32 = 2;
            a + b
        }
        "#;
        assert_eq!(run_prog_i32(src), 3);
    }

    #[test]
    fn codegen_block_let_shadow() {
        // Tests: variable shadowing should create new bindings.
        let src = r#"
        fn main() -> i32 {
            let a = 1;
            let b = 2;
            let a = 3;
            let b = 4;
            a + b
        }
        "#;
        assert_eq!(run_prog_i32(src), 7);
    }

    #[test]
    fn codegen_local_block_scoping() {
        // Tests: inner block scopes and variable isolation.
        let src = r#"
        fn main() -> i32 {
            let a = 1;
            let b = {
                let b = a;
                b + 2
            };
            b
        }
        "#;
        assert_eq!(run_prog_i32(src), 3);
    }

    #[test]
    fn codegen_local_fn_call() {
        // Tests: calling a function defined inside main.
        let src = r#"
        fn main() -> i32 {
            fn add(i: i32, j: i32) -> i32 { i + j }
            add(10, 32)
        }
        "#;
        assert_eq!(run_prog_i32(src), 42);
    }

    #[test]
    fn codegen_int_comparisons_and_bool_ops() {
        // Tests: <, >, == and boolean AND evaluation.
        let src = r#"
        fn main() -> i32 {
            let b1 = 1 < 2;
            let b2 = 2 > 1;
            let b3 = 1 == 1;
            if b1 && b2 && b3 { 1 } else { 0 }
        }
        "#;
        assert_eq!(run_prog_i32(src), 1);
    }

    #[test]
    fn codegen_while_loop() {
        // Tests: while-loop execution and variable updates.
        let src = r#"
        fn main() -> i32 {
            let mut a = 2;
            let mut b = 0;
            while a > 0 {
                a = a - 1;
                b = b + 1;
            }
            b
        }
        "#;
        assert_eq!(run_prog_i32(src), 2);
    }

    #[test]
    fn codegen_recursive_sum() {
        // Tests: recursive calls + correct parameter passing.
        let src = r#"
        fn sum(n: i32) -> i32 {
            if n == 0 { 0 } else { n + sum(n - 1) }
        }
        fn main() -> i32 {
            sum(4)
        }
        "#;
        assert_eq!(run_prog_i32(src), 10);
    }
}

#[test]
fn recursion_sum_yassine_taharaste() {
    let src = "
        fn main() { sum(3) }
        fn sum(n: i32) -> i32 { if n == 0 { 0 } else { n + sum(n-1) } }
    ";
    assert_eq!(run_prog_i32(src), 6);
}
#[cfg(test)]
mod codegen_prog_test {
    use super::*;

    #[test]
    fn test_program_codegen() {
        let src = "
            fn main() {
                add2(0)
            }
            fn add2(x: i32) -> i32 {
                x+2 
            }

            ";
        assert_eq!(run_prog_i32(src), 2)
    }
    #[test]
    fn test_program_nested_call() {
        let src = "
            fn main() {
                add2(0)
            }
            fn add2(x: i32) -> i32 {
                  add1(add1(x))
        
            }
            fn add1(x: i32) -> i32 {
                x+1
            }
            ";
        assert_eq!(run_prog_i32(src), 2)
    }
    #[test]
    fn test_program_two_args() {
        let src = "
        fn main() {
            add(10, 32)
        }

        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        ";
        assert_eq!(run_prog_i32(src), 42);
    }

    #[test]
    fn test_program_call_with_locals() {
        let src = "
        fn main() {
            foo(3)
        }

        fn foo(x: i32) -> i32 {
            let y = x + 2;    // y = 5
            let z = y + 1;    // z = 6
            z
        }
        ";
        assert_eq!(run_prog_i32(src), 6);
    }

    #[test]
    fn test_program_if_inside_function() {
        let src = "
        fn main() {
            positive(5)
        }

        fn positive(x: i32) -> i32 {
            if x > 0 { 1 } else { 0 }
        }
        ";
        assert_eq!(run_prog_i32(src), 1);
    }

    #[test]
    fn test_program_if_else_on_param() {
        let src = "
        fn main() {
            posneg(-3)
        }

        fn posneg(x: i32) -> i32 {
            if x > 0 { 1 } else { -1 }
        }
        ";
        assert_eq!(run_prog_i32(src), -1);
    }

    #[test]
    fn test_program_while_in_function() {
        let src = "
        fn main() {
            countdown(3)
        }

        fn countdown(x: i32) -> i32 {
            while x > 0 {
                x = x - 1;
            };
            x
        }
        ";
        assert_eq!(run_prog_i32(src), 0);
    }

    #[test]
    fn test_program_nested_blocks_in_function() {
        let src = "
        fn main() {
            calc(2)
        }

        fn calc(x: i32) -> i32 {
            {
                let y = x + 3;
                {
                    let z = y + 4;
                    z
                }
            }
        }
        ";
        assert_eq!(run_prog_i32(src), 9);
    }

    #[test]
    fn test_program_unit_returning_function() {
        let src = "
        fn main() {
            do_nothing();
            ()
        }

        fn do_nothing() -> () {
            let x = 10;
        }
        ";
        assert_eq!(run_prog_i32(src), 0);
    }

    #[test]
    fn test_program_call_defined_later() {
        let src = "
        fn main() {
            mul2(4)
        }

        // defined AFTER main to test forward references
        fn mul2(x: i32) -> i32 {
            x + x
        }
        ";
        assert_eq!(run_prog_i32(src), 8);
    }

    #[test]
    fn test_program_local_shadowing() {
        let src = "
        fn main() {
            shadow(5)
        }

        fn shadow(x: i32) -> i32 {
            let x = x + 1;     // shadows parameter x
            let x = x + 1;     // shadows again
            x                  // should be 7
        }
        ";
        assert_eq!(run_prog_i32(src), 7);
    }
    #[test]
    fn test_program_empty_block_and_unit_behavior() {
        let src = "
        fn main() {
            {};
            ()
        }
        ";
        assert_eq!(run_prog_i32(src), 0); // unit = 0
    }
}

#[cfg(test)]
pub mod codegen_ai_tests {
    use super::*;

    #[test]
    fn block_trailing_semi_pushes_unit() {
        // A block ending with a semicolon should leave Unit (0) on the stack.
        assert_eq!(run_block_i32("{ let x = 1; }"), 0);
    }

    #[test]
    fn while_with_false_initial_condition_skips_body() {
        let src = r#"
        fn main() -> i32 {
            let mut x = 5;
            while x < 0 { x = x - 1; }
            x
        }
        "#;
        assert_eq!(run_prog_i32(src), 5);
    }

    #[test]
    fn unit_returning_fn_call_does_not_corrupt_stack() {
        let src = r#"
        fn ping() -> () { let y = 10; }
        fn main() -> i32 {
            ping();
            9
        }
        "#;
        assert_eq!(run_prog_i32(src), 9);
    }

    #[test]
    fn inner_shadow_does_not_leak_out_of_block() {
        let src = r#"
        fn main() -> i32 {
            let a = 1;
            {
                let a = 2;
                a; // inner value ignored after scope
            };
            a
        }
        "#;
        assert_eq!(run_prog_i32(src), 1);
    }

    #[test]
    fn nested_local_function_is_scoped_and_callable() {
        let src = r#"
        fn main() -> i32 {
            fn outer(x: i32) -> i32 {
                fn inner(y: i32) -> i32 { y + 1 }
                inner(x) + 2
            }
            outer(3)
        }
        "#;
        assert_eq!(run_prog_i32(src), 6);
    }
    #[test]
    fn nested_local_function_is_scoped_and_callable_with_shadowing() {
        let src = r#"
        fn main() -> i32 {
            fn inner() {};
            fn outer(x: i32) -> i32 {
                fn inner(y: i32) -> i32 { y + 1 }
                inner(x) + 2
            }
            outer(3)

        }
        "#;
        assert_eq!(run_prog_i32(src), 6);
    }
}
