use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use clap::{Parser, arg};
use mips::instr::Instr;
use mips::rf::Reg;
use mips::vm::Mips;
use regex::Regex;

use crate::ast::Prog;
use crate::code_gen::CodegenVm;
use crate::common::Eval;
use crate::parse;
use crate::type_check::TypeChecker;
use crate::vm::VM;

/// Command-line interface for the RnR compiler.
#[derive(Debug, Parser)]
#[command(name = "rnr", about = "Run and configure the RnR compiler pipeline")]
pub struct Cli {
    /// Input program to compile.
    #[arg(
        short = 'i',
        long = "input",
        value_name = "PATH",
        default_value = "examples/ex1.rnr"
    )]
    pub input: PathBuf,

    /// Dump the parsed AST to a file.
    #[arg(short = 'a', long = "ast", value_name = "PATH")]
    pub ast_path: Option<PathBuf>,

    /// Run the type checker.
    #[arg(short = 't', long = "type_check")]
    pub type_check: bool,

    /// Run the virtual machine on generated code.
    #[arg(short = 'v', long = "virtual_machine", alias = "vm")]
    pub virtual_machine: bool,

    /// Perform code generation.
    #[arg(short = 'c', long = "code_gen")]
    pub code_gen: bool,

    /// Write generated assembly to a file.
    #[arg(long = "asm", value_name = "PATH")]
    pub asm: Option<PathBuf>,

    /// Read .asm file [unimplemented].
    #[arg(long = "load-asm", value_name = "PATH")]
    pub load_asm: Option<PathBuf>,

    /// Execute generated code using the mips VM.
    #[arg(short = 'r')]
    pub run: bool,
}

impl Cli {
    /// Parse arguments from the process invocation.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Execute compiler actions based on parsed options.
    pub fn execute(&self) -> anyhow::Result<()> {
        let parsed_string = parse_file(&self.input)?;
        let parsed_ast = parse::try_parse::<Prog>(&parsed_string)
            .with_context(|| format!("Failed to parse {}", self.input.display()))?;

        if let Some(ast_path) = &self.ast_path {
            emit_ast(ast_path, &parsed_ast)?;
        }

        if self.type_check {
            run_type_check(&parsed_ast)?;
        }

        if self.virtual_machine {
            run_vm(&parsed_ast)?;
        }

        if self.run || self.asm.is_some() || self.code_gen || self.load_asm.is_some() {
            let instrs: Vec<Instr> = self.get_asm(&parsed_ast)?;

            if self.run {
                run_generated(&instrs)?
            }

            if let Some(asm_path) = &self.asm {
                let _ = write_asm(asm_path, &instrs);
            }
        }
        Ok(())
    }

    fn get_asm(&self, parsed_ast: &Prog) -> anyhow::Result<Vec<Instr>> {
        if !(self.run || self.asm.is_some()) {
            println!("Warning: Generated code is not used")
        }
        match (self.code_gen, self.load_asm.clone()) {
            (true, None) => Ok(parsed_ast.eval().context("Code generation failed")?),

            (false, None) => Err(anyhow::anyhow!("Error: Flag -c (--code-gen) is required")),

            (false, Some(p)) => unimplemented!("load-asm"),

            (true, Some(_)) => Err(anyhow::anyhow!(
                "Error: Flag --code-gen and --load-asm are mutually exclusive"
            )),
        }
    }
}

use std::fs::File;
use std::io::{Write, read_to_string};
fn parse_file(path: &Path) -> anyhow::Result<String> {
    let display_path = path.display();
    let metadata = path
        .metadata()
        .with_context(|| format!("Input file not found: {display_path}"))?;
    if !metadata.is_file() {
        bail!("Input path is not a file: {display_path}");
    }

    let file = File::open(path).with_context(|| format!("Failed to open {display_path}"))?;
    let result = read_to_string(file).with_context(|| format!("Failed to read {display_path}"))?;
    let mut cleaned = String::new();

    // exclude `//` in code to facilitate comments should be done in parser!
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

fn emit_ast(path: &Path, ast: &Prog) -> anyhow::Result<()> {
    let mut ast_output =
        File::create(path).with_context(|| format!("Failed to create {}", path.display()))?;
    ast_output
        .write_all(format!("{:#?}", ast).as_bytes())
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn run_type_check(ast: &Prog) -> anyhow::Result<()> {
    let mut type_checker = TypeChecker::new();
    let result = match type_checker.check_prog(ast) {
        Ok(result) => result,
        Err(err) => return Err(anyhow::anyhow!("Type check failed: {err}")),
    };
    println!("Type check success: {:?}", result);
    Ok(())
}

fn run_vm(ast: &Prog) -> anyhow::Result<()> {
    let mut vm = VM::new();
    let result = vm.eval_prog(ast).context("VM execution failed")?;
    println!("VM success: {:?}", result);
    Ok(())
}

fn run_generated(instrs: &[Instr]) -> anyhow::Result<()> {
    let mut mips = Mips::new(mips::instrs::Instrs(instrs.to_vec()));

    match mips.run() {
        Ok(()) => {}
        // Skip Halt error
        Err(mips::error::Error::Halt) => {}
        Err(err) => {
            return Err(anyhow::anyhow!("MIPS VM execution failed: {err:?}"));
        }
    }
    let result = mips.rf.get(Reg::t0);
    println!("Mips VM result: {}", result);
    Ok(())
}

fn write_asm(asm_path: &PathBuf, instrs: &[Instr]) -> anyhow::Result<()> {
    let mut asm_output = File::create(asm_path)
        .with_context(|| format!("Failed to create {}", asm_path.display()))?;
    let parsed_instrs = parse_instrs(instrs);
    for instr in &parsed_instrs {
        asm_output
            .write_all(format!("{instr}\n").as_bytes())
            .with_context(|| format!("Failed to write {}", asm_path.display()))?;
    }
    Ok(())
}

fn parse_instrs(instrs: &[Instr]) -> Vec<String> {
    // Instr is in external crate, fmt::display cannot be implemented
    // Printing the debug of Instr gave a good understanding of the structure.
    // Note: Used llm to get help with this implementation
    let op_re =
        Regex::new(r"(?s)op:\s*(?:Type\w+)\(\s*(?P<opcode>\w+)\s*,(?P<params>.*?)\)\s*,\s*label:")
            .expect("valid opcode regex");
    let op_no_params_re =
        Regex::new(r"op:\s*(?P<opcode>\w+)\s*,\s*label:").expect("valid opcode regex");
    let label_re =
        Regex::new(r#"label:\s*Some\(\s*"(?P<label>[^"]+)""#).expect("valid label regex");
    // Pull registers, immediate, and label refs out of the params list.
    let param_re = Regex::new(
        r#"Label\(\s*"(?P<label>[^"]+)"|U16\(\s*(?P<u16>-?\d+)\s*,?\s*\)|(?P<reg>[A-Za-z_][A-Za-z0-9_]*)"#,
    )
    .expect("valid param regex");

    instrs
        .iter()
        .map(|instr| {
            let text = format!("{:#?}", instr);
            // Label is stored separately, keep it aligned in output.
            let label = label_re
                .captures(&text)
                .and_then(|caps| caps.name("label"))
                .map(|m| m.as_str().to_string());

            let mut opcode = String::new();
            let mut params: Vec<String> = Vec::new();
            if let Some(caps) = op_re.captures(&text) {
                opcode = caps
                    .name("opcode")
                    .map(|m| m.as_str())
                    .unwrap_or_default()
                    .to_string();
                // Keep params in order for "Op rs rd rt" formatting.
                let params_text = caps.name("params").map(|m| m.as_str()).unwrap_or("");
                for caps in param_re.captures_iter(params_text) {
                    if let Some(m) = caps.name("label") {
                        params.push(m.as_str().to_string());
                    } else if let Some(m) = caps.name("u16") {
                        params.push(m.as_str().to_string());
                    } else if let Some(m) = caps.name("reg") {
                        params.push(m.as_str().to_string());
                    }
                }
            } else if let Some(caps) = op_no_params_re.captures(&text) {
                // Fallback for ops like Halt with no params.
                opcode = caps
                    .name("opcode")
                    .map(|m| m.as_str())
                    .unwrap_or_default()
                    .to_string();
            }

            let op_and_params = if params.is_empty() {
                opcode.clone()
            } else {
                format!("{} {}", opcode, params.join(" "))
            };

            let label_width = 10;
            if let Some(label) = label {
                format!("{label:<label_width$} {op_and_params}")
            } else {
                // Indent to match labeled instructions.
                format!("{: <label_width$} {op_and_params}", "")
            }
        })
        .collect()
}
