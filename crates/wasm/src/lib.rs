use compiler::ast::AstNode;
use compiler::error::{Diagnostic, Error};
use compiler::pipeline;
use compiler::vm::Val;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize)]
struct CompileResult {
    diagnostics: Vec<Diagnostic>,
    result: Option<String>,
    stdout: Option<String>,
    ast: Option<AstNode>,
}

impl CompileResult {
    fn success(result: (Val, String), ast: AstNode) -> Self {
        Self {
            diagnostics: Vec::new(),
            result: Some(result.0.to_string()),
            stdout: Some(result.1),
            ast: Some(ast),
        }
    }

    fn failure(err: Error) -> Self {
        Self {
            diagnostics: vec![err.into()],
            result: None,
            stdout: None,
            ast: None,
        }
    }
}

fn compile_impl(source: &str) -> CompileResult {
    let prog = match pipeline::frontend(source) {
        Ok(p) => p,
        Err(e) => return CompileResult::failure(e),
    };

    let result = match pipeline::interpret(&prog) {
        Ok(val) => val,
        Err(e) => return CompileResult::failure(e),
    };

    CompileResult::success(result, AstNode::from(&prog))
}

#[wasm_bindgen]
pub fn compile(source: &str) -> JsValue {
    serde_wasm_bindgen::to_value(&compile_impl(source)).expect("CompileResult should serialize")
}

#[test]
fn wasm_compile_success() {
    let result = compile_impl(
        r#"
        fn main() -> i32 {
            println!("test");
            println!("stdout");
            1 + 2
        }
        "#,
    );

    assert!(result.diagnostics.is_empty());
    assert_eq!(result.result.as_deref(), Some("3"));
    assert_eq!(result.stdout.as_deref(), Some("test\nstdout\n"));
}
