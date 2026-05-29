use compiler::pipeline;
use compiler::vm::Val;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize)]
struct CompileResult {
    diagnostic: Option<compiler::error::Diagnostic>,
    result: String,
}

impl CompileResult {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|e| format!("Error serializing: {}", e))
    }
}

#[wasm_bindgen]
pub fn compile(source: &str) -> String {
    let prog = match pipeline::frontend(source) {
        Ok(prog) => prog,
        Err(e) => {
            return CompileResult {
                diagnostic: Some(e.into()),
                result: "Compilation Error".to_string(),
            }
            .to_json();
        }
    };

    let result: Val = match pipeline::interpret(&prog) {
        Ok(prog) => prog,
        Err(e) => {
            return CompileResult {
                diagnostic: Some(e.into()),
                result: "Interpret Error".to_string(),
            }
            .to_json();
        }
    };

    CompileResult {
        diagnostic: None,
        result: result.to_string(),
    }
    .to_json()
}

#[test]
fn wasm_compile_success() {
    let result = compile(
        r#"
        fn main() -> i32 {
            1 + 2
        }
        "#,
    );

    assert!(result.contains("3"));
}
