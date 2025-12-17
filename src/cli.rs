use std::path::{Path, PathBuf};

use anyhow::Ok;
use clap::{Parser, arg};
use mips::instr::Instr;
use mips::rf::Reg;
use mips::vm::Mips;

use crate::ast::Prog;
use crate::code_gen::CodegenVm;
use crate::common::{Eval, codegen_instrs};
use crate::type_check::TypeChecker;
use crate::vm::VM;
use crate::{parse, type_check};

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
    #[arg(short = 't', long = "type-check", alias = "type_check")]
    pub type_check: bool,

    /// Run the virtual machine on generated code.
    #[arg(short = 'v', long = "virtual-machine", alias = "vm")]
    pub virtual_machine: bool,

    /// Perform code generation.
    #[arg(short = 'c', long = "code-gen", alias = "code_gen")]
    pub code_gen: bool,

    /// Write generated assembly to a file.
    #[arg(long = "asm", value_name = "PATH")]
    pub asm: Option<PathBuf>,

    /// Execute generated code using the mips VM.
    #[arg(short = 'r', long = "run")]
    pub run: bool,
}

impl Cli {
    /// Parse arguments from the process invocation.
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Execute compiler actions based on parsed options.
    pub fn execute(&self) -> anyhow::Result<()> {
        let parsed_string = parse_input(&self.input)?;
        let parsed_ast = parse::try_parse::<Prog>(&parsed_string)?;

        if let Some(ast_path) = &self.ast_path {
            emit_ast(ast_path, &parsed_ast)?;
        }

        if self.type_check {
            run_type_check(&parsed_ast)?;
        }

        if self.virtual_machine {
            run_vm(&parsed_ast)?;
        }

        if self.run || self.asm.is_some() || self.code_gen {
            let instrs = codegen_instrs::<Prog>(&parsed_string)?;
            if self.run {
                run_generated(instrs.clone())?;
            }

            if let Some(asm_path) = &self.asm {
                let mut asm_output = File::create(asm_path)?;
                for instr in &instrs {
                    asm_output.write_all(format!("{:#?}", instr).as_bytes())?;
                }
            }

            if self.code_gen {
                generate_code(instrs)?;
            }
        };
        Ok(())
    }
}

use std::fs::File;
use std::io::{Write, read_to_string};
fn parse_input(path: &Path) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let result = read_to_string(file)?;
    Ok(result)
}

fn emit_ast(path: &Path, ast: &Prog) -> anyhow::Result<()> {
    let mut ast_output = File::create(path)?;
    ast_output.write_all(format!("{:#?}", ast).as_bytes())?;
    Ok(())
}

fn run_type_check(ast: &Prog) -> anyhow::Result<()> {
    let mut type_checker = TypeChecker::new();
    let result = type_checker
        .check_prog(ast)
        .map_err(|op| println!("{}", op));
    println!("Type success: {}", result.unwrap());

    Ok(())
}

fn generate_code(instrs: Vec<Instr>) -> anyhow::Result<()> {
    let result = CodegenVm::new().run_instrs_get_t0_as_i32(instrs);
    println!("CodeGen result: {}", result);
    Ok(())
}

fn run_vm(ast: &Prog) -> anyhow::Result<()> {
    let mut vm = VM::new();
    let result = vm.eval_prog(ast).map_err(|op| println!("{}", op));
    println!("VM success: {:?}", result.unwrap());

    Ok(())
}

fn run_generated(instrs: Vec<Instr>) -> anyhow::Result<()> {
    let mut mips = Mips::new(mips::instrs::Instrs(instrs));

    let _ = mips.run();
    let result = mips.rf.get(Reg::t0);
    println!("Mips VM result: {}", result);
    Ok(())
}
