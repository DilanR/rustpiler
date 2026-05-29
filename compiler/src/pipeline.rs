use mips::{instr::Instr, rf::Reg, vm::Mips};
use regex::Regex;

use crate::{
    ast::Prog,
    common::Eval,
    error::{CodeGenError, Error},
    parse::try_parse,
    type_check::TypeChecker,
    vm::{VM, Val},
};

pub fn frontend(raw: &str) -> Result<Prog, Error> {
    let prog: Prog = try_parse(raw)?;
    let mut type_checker = TypeChecker::new();
    match type_checker.check_prog(&prog) {
        Ok(_) => Ok(prog),
        Err(err) => Err(err),
    }
}

pub fn interpret(prog: &Prog) -> Result<Val, Error> {
    let mut vm = VM::new();
    vm.eval_prog(prog)
}

pub fn code_gen(ast: &Prog) -> Result<(u32, Vec<String>), Error> {
    let instrs: Vec<Instr> = ast.eval()?;
    let mut mips = Mips::new(mips::instrs::Instrs(instrs.to_vec()));

    match mips.run() {
        Ok(()) => {}
        // Skip Halt error
        Err(mips::error::Error::Halt) => {}
        Err(_) => return Err(CodeGenError::Mips.into()),
    }
    let result = mips.rf.get(Reg::t0);
    println!("Mips VM result: {}", result);
    Ok((result, parse_instrs(instrs)))
}

fn parse_instrs(instrs: Vec<Instr>) -> Vec<String> {
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
