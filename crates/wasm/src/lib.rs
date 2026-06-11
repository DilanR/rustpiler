use compiler::{ast::AstNode, error::Diagnostics, pipeline, vm::Val};
use web_sys::wasm_bindgen;

use serde::Serialize;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[derive(Debug, Serialize)]
struct CompileResult {
    diagnostics: Diagnostics,
    result: Option<String>,
    stdout: Option<String>,
    ast: Option<AstNode>,
    time_ms: Option<f64>,
}

impl CompileResult {
    fn success(result: (Val, String), ast: AstNode) -> Self {
        Self {
            diagnostics: Diagnostics(vec![]),
            result: Some(result.0.to_string()),
            stdout: Some(result.1),
            ast: Some(ast),
            time_ms: None,
        }
    }

    fn failure(diagnostics: Diagnostics, ast: Option<AstNode>) -> Self {
        Self {
            diagnostics,
            result: None,
            stdout: None,
            ast,
            time_ms: None,
        }
    }
}

fn compile_impl(source: &str) -> CompileResult {
    let prog = match pipeline::frontend(source) {
        Ok(p) => p,
        Err(e) => {
            let diagnostics = e.1.iter().map(|e| e.clone().into()).collect();
            return CompileResult::failure(diagnostics, e.0.map(|p| AstNode::from(&p)));
        }
    };

    let result = match pipeline::interpret(&prog) {
        Ok(val) => val,
        Err(e) => {
            return CompileResult::failure(
                Diagnostics(vec![e.clone().into()]),
                Some(AstNode::from(&prog)),
            );
        }
    };

    CompileResult::success(result, AstNode::from(&prog))
}

#[wasm_bindgen]
pub fn compile(source: &str) -> JsValue {
    // std::Instant wont work here, need information on the browser
    let performance = web_sys::window().unwrap().performance().unwrap();

    let start = performance.now();

    let mut result = compile_impl(source);

    result.time_ms = Some(performance.now() - start);

    match serde_wasm_bindgen::to_value(&result) {
        Ok(v) => {
            web_sys::console::log_1(&"serialization finished".into());
            v
        }
        Err(e) => {
            web_sys::console::log_1(&format!("serialization failed: {e}").into());

            JsValue::NULL
        }
    }
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

#[test]
fn compile_result_serializes() {
    let result = compile_impl(
        r#"
        fn main() -> i32 {
            1
        }
        "#,
    );

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
