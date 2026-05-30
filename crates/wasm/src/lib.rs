use compiler::error::{Diagnostic, Error};
use compiler::pipeline;
use compiler::vm::Val;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Serialize)]
struct CompileResult {
    diagnostics: Vec<Diagnostic>,
    output: Option<String>,
}

impl CompileResult {
    fn success(output: Val) -> Self {
        Self {
            diagnostics: Vec::new(),
            output: Some(output.to_string()),
        }
    }

    fn failure(err: Error) -> Self {
        Self {
            diagnostics: vec![err.into()],
            output: None,
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

    CompileResult::success(result)
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
            1 + 2
        }
        "#,
    );

    assert!(result.diagnostics.is_empty());
    assert_eq!(result.output.as_deref(), Some("3"));
}
