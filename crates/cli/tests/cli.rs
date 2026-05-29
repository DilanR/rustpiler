use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn interpret_simple_program() {
    let dir = tempdir().unwrap();

    let source = dir.path().join("test.rnr");

    fs::write(
        &source,
        r#"
        fn main() -> i32 {
            1 + 2
        }
        "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args(["--input", source.to_str().unwrap(), "--run"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Interpreter result"));
}

#[test]
fn fail_typecheck_simple_program() {
    let dir = tempdir().unwrap();

    let source = dir.path().join("test.rnr");

    fs::write(
        &source,
        r#"
        fn main() { // should return unit
            1 + 2 // not unit
        }
        "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args(["--input", source.to_str().unwrap(), "--run"]);

    cmd.assert().failure().stderr(predicate::str::contains(
        "
Type mismatch: expected i32, got ()
",
    ));
}

#[test]
fn missing_input_file() {
    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args(["--input", "does/not/exist.rnr", "--run"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Input file not found"));
}

#[test]
fn invalid_syntax_fails() {
    let dir = tempdir().unwrap();

    let source = dir.path().join("invalid.rnr");

    fs::write(
        &source,
        r#"
        fn main( {
        "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args(["--input", source.to_str().unwrap(), "--run"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Frontend failed"));
}

#[test]
fn emits_ast_file() {
    let dir = tempdir().unwrap();

    let source = dir.path().join("test.rnr");
    let ast = dir.path().join("out.ast");

    fs::write(
        &source,
        r#"
        fn main() -> i32 {
            123
        }
        "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args([
        "--input",
        source.to_str().unwrap(),
        "--ast",
        ast.to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(ast.exists());

    let content = fs::read_to_string(ast).unwrap();

    assert!(!content.is_empty());
}

#[test]
fn generates_asm() {
    let dir = tempdir().unwrap();

    let source = dir.path().join("test.rnr");
    let asm = dir.path().join("out.asm");

    fs::write(
        &source,
        r#"
        fn main() -> i32 {
            5
        }
        "#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.args([
        "--input",
        source.to_str().unwrap(),
        "--codegen",
        "--asm",
        asm.to_str().unwrap(),
    ]);

    cmd.assert().success();

    assert!(asm.exists());

    let content = fs::read_to_string(asm).unwrap();

    assert!(!content.is_empty());
}

#[test]
fn invalid_flag_fails() {
    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.arg("--does-not-exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}
