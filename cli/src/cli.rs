use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};
use clap::Parser;

use compiler::{
    pipeline::{code_gen, frontend, interpret},
    vm::Val,
};

#[derive(Debug, Parser)]
#[command(name = "rustpiler")]
#[command(about = "Frontend, interpreter, and codegen pipeline")]
pub struct Cli {
    /// Input source file
    #[arg(
        short = 'i',
        long = "input",
        value_name = "PATH",
        default_value = "examples/ex1.rnr"
    )]
    pub input: PathBuf,

    /// Dump parsed AST
    #[arg(short = 'a', long = "ast", value_name = "PATH")]
    pub ast: Option<PathBuf>,

    /// Run interpreter
    #[arg(short = 'r', long = "run")]
    pub run: bool,

    /// Generate assembly
    #[arg(short = 'c', long = "codegen")]
    pub codegen: bool,

    /// Output assembly path
    #[arg(long = "asm", value_name = "PATH")]
    pub asm: Option<PathBuf>,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        let raw = parse_file(&self.input)?;

        let prog = frontend(&raw).map_err(|err| anyhow::anyhow!("Frontend failed:\n{err}"))?;

        if let Some(path) = &self.ast {
            emit_ast(path, &prog)?;
        }

        if self.run {
            let result =
                interpret(&prog).map_err(|err| anyhow::anyhow!("Interpreter failed:\n{err}"))?;

            print_interpret_result(&result);
        }

        if self.codegen || self.asm.is_some() {
            let (result, asm) =
                code_gen(&prog).map_err(|err| anyhow::anyhow!("Code generation failed:\n{err}"))?;

            println!("{}", result);

            if let Some(path) = &self.asm {
                write_asm(path, &asm)?;
            }
        }

        Ok(())
    }
}

fn parse_file(path: &Path) -> anyhow::Result<String> {
    let display_path = path.display();

    let metadata = path
        .metadata()
        .with_context(|| format!("Input file not found: {display_path}"))?;

    if !metadata.is_file() {
        bail!("Input path is not a file: {display_path}");
    }

    let result = read_to_string(path).with_context(|| format!("Failed to read {display_path}"))?;

    let mut cleaned = String::new();

    for line in result.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("//") {
            continue;
        }

        let code = match line.find("//") {
            Some(index) => &line[..index],
            None => line,
        };

        let code = code.trim_end();

        if code.is_empty() {
            continue;
        }

        if !cleaned.is_empty() {
            cleaned.push('\n');
        }

        cleaned.push_str(code);
    }

    Ok(cleaned)
}

fn emit_ast(path: &Path, ast: &impl std::fmt::Debug) -> anyhow::Result<()> {
    let mut output =
        File::create(path).with_context(|| format!("Failed to create {}", path.display()))?;

    output
        .write_all(format!("{:#?}", ast).as_bytes())
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn write_asm(path: &Path, asm: &[String]) -> anyhow::Result<()> {
    let mut output =
        File::create(path).with_context(|| format!("Failed to create {}", path.display()))?;

    output
        .write_all(asm.join("\n").as_bytes())
        .with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(())
}

fn print_interpret_result(result: &Val) {
    println!("Interpreter result: {result:?}");
}
