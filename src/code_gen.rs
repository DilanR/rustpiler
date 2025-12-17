use std::collections::HashMap;

use crate::{
    ast::*,
    common::Eval,
    error::{Error, TypeError},
};
use mips::{
    asm::*,
    instr::Instr,
    instrs::Instrs,
    rf::{self, Reg::*},
    vm::Mips,
};

// Backend targeting the Mips architecture
// Stack Machine implementation
#[derive(Debug, Clone)]
pub struct CodeGenEnv {
    instructions: Vec<Instr>,
    values: Vec<HashMap<String, i16>>,
    labels: Vec<HashMap<String, String>>,
    stack_offset: i16,
    fn_local_count: i16,
    label_count: i16,
}

impl CodeGenEnv {
    #![allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            values: vec![HashMap::new()],
            labels: vec![HashMap::new()],
            stack_offset: -4,
            fn_local_count: 0,
            label_count: 0,
        }
    }

    fn enter_scope(&mut self) {
        self.values.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.values.pop();
    }

    fn add_instr(&mut self, instr: Instr) {
        self.instructions.push(instr);
    }

    fn add_nop_with_label(&mut self, label: &str) {
        let label = addu(zero, zero, zero).label(label);
        self.instructions.push(label)
    }

    fn push_to_stack(&mut self, rt: rf::Reg) {
        self.instructions.push(addiu(sp, sp, -4));
        self.instructions.push(sw(rt, 0, sp));
    }

    fn pop_from_stack(&mut self, rt: rf::Reg) {
        self.instructions.push(lw(rt, 0, sp));
        self.instructions.push(addiu(sp, sp, 4));
    }

    fn push_lit_to_stack(&mut self, lit: &Literal) -> Result<(), Error> {
        match lit {
            Literal::Bool(b) => self.add_instr(addi(t0, zero, *b as i16)),
            Literal::Int(i) => self.add_instr(addi(t0, zero, *i as i16)),
            Literal::Unit => self.add_instr(add(t0, zero, zero)),
            _ => return Err(Error::UndefinedOperation()),
        };
        self.push_to_stack(t0);
        Ok(())
    }

    pub fn lookup_label(&self, id: &str) -> Result<String, Error> {
        for scope in self.labels.iter().rev() {
            if let Some(label) = scope.get(id) {
                return Ok(label.clone());
            }
        }
        Err(Error::UndefinedFunction(id.to_string()))
    }

    pub fn lookup_value_offset(&self, id: &str) -> Result<i16, Error> {
        for scope in self.values.iter().rev() {
            if let Some(offset) = scope.get(id) {
                return Ok(*offset);
            }
        }
        Err(Error::UndefinedValue(id.to_string()))
    }

    fn define_value_offset(&mut self, ident: &str) {
        // Put val on top of current stack
        self.values
            .last_mut()
            .unwrap()
            .insert(ident.to_string(), self.stack_offset);

        // make room
        self.stack_offset += -4;
        self.fn_local_count += 1;
    }

    pub fn define_function_label(&mut self, id: &str) {
        self.label_count += 1;
        let label = if id != "main" {
            format!("fn_{}_{}", self.label_count, id)
        } else {
            id.to_string()
        };
        self.labels
            .last_mut()
            .unwrap()
            .insert(id.to_string(), label);
    }

    pub fn define_label(&mut self, id: &str) -> String {
        let label = format!("label_{}_{}", self.label_count, id);
        self.labels
            .last_mut()
            .unwrap()
            .insert(id.to_string(), label.clone());
        self.label_count += 1;
        label
    }

    fn emit_label(&mut self, id: &str) -> String {
        let label = self.define_label(id);
        self.add_nop_with_label(&label);
        label
    }
}

pub struct CodegenVm {
    env: CodeGenEnv,
}

impl CodegenVm {
    pub fn new() -> Self {
        Self {
            env: CodeGenEnv::new(),
        }
    }
    pub fn run_instrs_get_t0_as_i32(&self, instrs: Vec<Instr>) -> i32 {
        let mut mips = Mips::new(Instrs::new_from_slice(&instrs));
        mips.run().ok();
        mips.rf.get(t0) as i32
    }

    pub fn run(&self) -> Mips {
        let mut mips = Mips::new(Instrs::new_from_slice(&self.env.instructions));
        mips.run().ok();
        mips
    }

    fn push_unit(&mut self) {
        self.env.add_instr(addi(t0, zero, 0));
        self.env.push_to_stack(t0);
    }

    fn push_unit_if_semi(&mut self, block: &Block) {
        if block.semi {
            self.push_unit();
        }
    }

    fn emit_fn_prologue(&mut self) -> usize {
        self.env.push_to_stack(ra);
        self.env.push_to_stack(fp);
        self.env.add_instr(addi(fp, sp, 0));
        let alloc_idx = self.env.instructions.len();
        // patched after body once locals are known
        self.env.add_instr(addiu(sp, sp, 0));
        alloc_idx
    }

    fn emit_fn_epilogue(&mut self, is_main: bool, local_alloc_idx: usize) {
        let frame_size = self.env.fn_local_count * 4;
        self.env.instructions[local_alloc_idx] = addiu(sp, sp, -frame_size);
        self.env.add_instr(addi(sp, fp, 0));
        self.env.pop_from_stack(fp);
        self.env.pop_from_stack(ra);
        if is_main {
            self.env.add_instr(halt());
        } else {
            self.env.add_instr(jr(ra));
        }
    }

    pub fn codegen_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Ident(id) => {
                let offset = self.env.lookup_value_offset(id)?;
                self.env.add_instr(lw(t0, offset, fp));
                self.env.push_to_stack(t0);
                Ok(())
            }
            Expr::Lit(lit) => self.env.push_lit_to_stack(lit),
            Expr::BinOp(bin_op, lhs, rhs) => {
                self.codegen_expr(lhs)?;
                self.codegen_expr(rhs)?;
                self.codegen_expr_binop(*bin_op, lhs, rhs)
            }
            Expr::Par(expr) => self.codegen_expr(expr),
            Expr::Call(id, args) => self.codegen_call(id, args),
            Expr::IfThenElse(cond, then, else_then) => {
                self.codegen_expr_if_then_else(cond, then, else_then)
            }
            Expr::Block(block) => self.codegen_block_expr(block),
            Expr::UnOp(un_op, expr) => {
                self.codegen_expr(expr)?;
                self.codegen_expr_unop(*un_op, expr)
            }
        }
    }

    fn codegen_expr_binop(&mut self, bin_op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<(), Error> {
        self.env.pop_from_stack(t1);
        self.env.pop_from_stack(t0);
        match bin_op {
            BinOp::Add => self.env.add_instr(add(t0, t0, t1)),
            BinOp::Sub => self.env.add_instr(sub(t0, t0, t1)),
            BinOp::Mul => unimplemented!("sll not implemented"),
            BinOp::Div => unimplemented!("srl not implemented"),
            BinOp::And => self.env.add_instr(and(t0, t0, t1)),
            BinOp::Or => self.env.add_instr(or(t0, t0, t1)),
            BinOp::Eq => {
                let equal_label = self.env.define_label("equal");
                let done_label = self.env.define_label("done");
                // t0 == t1 then t0 = 1 else 0
                self.env.add_instr(beq_label(t0, t1, &equal_label));
                self.env.add_instr(addu(t0, zero, zero));
                self.env.add_instr(b_label(&done_label));
                self.env.add_nop_with_label(&equal_label);
                self.env.add_instr(addiu(t0, zero, 1));
                self.env.add_nop_with_label(&done_label);
            }
            BinOp::Lt => self.env.add_instr(slt(t0, t0, t1)),
            BinOp::Gt => self.env.add_instr(slt(t0, t1, t0)),
        }
        self.env.push_to_stack(t0);
        Ok(())
    }

    fn codegen_expr_unop(&mut self, un_op: UnOp, expr: &Expr) -> Result<(), Error> {
        self.env.pop_from_stack(t0);
        match un_op {
            UnOp::Neg => self.env.add_instr(subu(t0, zero, t0)),
            UnOp::Bang => self.env.add_instr(xori(t0, t0, 1)),
        }
        self.env.push_to_stack(t0);
        Ok(())
    }

    fn codegen_block(&mut self, block: &Block) -> Result<(), Error> {
        self.env.enter_scope();

        let n = block.statements.len();
        for (i, stmt) in block.statements.iter().enumerate() {
            let is_last = i + 1 == n;
            match stmt {
                // Last expression and no trailing semicolon => expression block
                Statement::Expr(expr) if is_last && !block.semi => {
                    self.codegen_expr(expr)?;
                }
                // Expression used as statement => discard result
                Statement::Expr(expr) => {
                    self.codegen_expr(expr)?;
                    self.env.pop_from_stack(t0);
                }
                _ => self.codegen_stmt(stmt)?,
            }
        }

        self.env.exit_scope();
        Ok(())
    }

    fn codegen_block_expr(&mut self, block: &Block) -> Result<(), Error> {
        self.codegen_block(block)?;
        if block.semi || block.statements.is_empty() {
            self.push_unit();
        }
        Ok(())
    }

    fn codegen_stmt(&mut self, stmt: &Statement) -> Result<(), Error> {
        match stmt {
            Statement::Let(_, ident, _, expr) => {
                // if None = Unit ie 0
                match expr {
                    Some(e) => {
                        self.codegen_expr(e)?;
                        self.env.pop_from_stack(t0);
                    }
                    None => self.env.add_instr(addiu(t0, zero, 0)),
                };

                // put val on stack of values
                self.env.define_value_offset(ident);

                // get offset and store
                let offset = self.env.lookup_value_offset(ident)?;
                self.env.add_instr(sw(t0, offset, fp));
            }
            Statement::Assign(ident, rhs) => {
                // eval rhs and put in reg t0
                self.codegen_expr(rhs)?;
                self.env.pop_from_stack(t0);

                // check if ident is valid and assign
                if let Expr::Ident(var) = ident {
                    let offset = self.env.lookup_value_offset(var)?;
                    self.env.add_instr(sw(t0, offset, fp));
                } else {
                    return Err(TypeError::AssignmentToNonIdent(ident.to_string()).into());
                };
            }
            Statement::While(cond, block) => {
                let while_start_label = self.env.emit_label("while_start");
                let while_end_label = self.env.define_label("while_end");
                // codegen condition
                self.codegen_expr(cond)?;
                self.env.pop_from_stack(t0);
                self.env.add_instr(beq_label(t0, zero, &while_end_label));
                // codegen block if condition false
                self.codegen_block(block)?;
                self.env.add_instr(b_label(&while_start_label));
                self.env.add_nop_with_label(&while_end_label);
            }
            Statement::Expr(expr) => self.codegen_expr(expr)?,
            Statement::Fn(fn_declaration) => {
                // make fn visible in current scope and skip over body at runtime
                self.env.define_function_label(&fn_declaration.id);
                let after_fn_label = self.env.define_label("after_local_fn");
                self.env.add_instr(b_label(&after_fn_label));
                self.codegen_fn_decl(fn_declaration)?;
                self.env.add_nop_with_label(&after_fn_label);
            }
        }
        Ok(())
    }

    fn codegen_fn_decl(&mut self, fn_decl: &FnDeclaration) -> Result<(), Error> {
        // Save outer scope for ease of use of nested functions
        let saved_values = std::mem::take(&mut self.env.values);
        let saved_stack_offset = self.env.stack_offset;
        let saved_fn_local_count = self.env.fn_local_count;

        self.env.values = vec![HashMap::new()];
        self.env.stack_offset = -4;
        self.env.fn_local_count = 0;

        // params need to be above fp
        for (i, param) in fn_decl.parameters.0.iter().enumerate() {
            let offset = 8 + (i as i16) * 4;
            self.env
                .values
                .last_mut()
                .unwrap()
                .insert(param.id.clone(), offset);
        }

        // TODO: fetch correct label for shadowing
        // label for bal
        let fn_label = self.env.lookup_label(&fn_decl.id)?;
        self.env.add_nop_with_label(&fn_label);
        let local_alloc_idx = self.emit_fn_prologue();

        self.codegen_block(&fn_decl.body)?;
        let returns_unit = matches!(fn_decl.ty, Some(Type::Unit) | None);
        if !returns_unit {
            self.env.pop_from_stack(t0);
        } else if fn_decl.id != "main" {
            self.push_unit();
            self.env.pop_from_stack(t0);
        }

        self.emit_fn_epilogue(fn_decl.id == "main", local_alloc_idx);

        // restore outer scope
        self.env.values = saved_values;
        self.env.stack_offset = saved_stack_offset;
        self.env.fn_local_count = saved_fn_local_count;
        Ok(())
    }

    fn codegen_expr_if_then_else(
        &mut self,
        cond: &Expr,
        then: &Block,
        else_then: &Option<Block>,
    ) -> Result<(), Error> {
        let else_then_label = self.env.define_label("else_then");
        let end_if_label = self.env.define_label("end_if");
        self.codegen_expr(cond)?;
        self.env.pop_from_stack(t0);
        self.env.add_instr(beq_label(t0, zero, &else_then_label));
        self.codegen_block(then)?;
        self.push_unit_if_semi(then);
        self.env.add_instr(b_label(&end_if_label));
        self.env.add_nop_with_label(&else_then_label);
        if let Some(else_then) = else_then {
            self.codegen_block(else_then)?;
            self.push_unit_if_semi(else_then);
        } else {
            self.push_unit();
        }
        self.env.add_nop_with_label(&end_if_label);
        Ok(())
    }

    fn codegen_call(&mut self, id: &str, args: &Arguments) -> Result<(), Error> {
        //check fn has label
        let label = self.env.lookup_label(id)?;
        self.codegen_call_impl(&label, args)
    }

    fn codegen_call_impl(&mut self, label: &str, args: &Arguments) -> Result<(), Error> {
        let argc = args.0.len() as i16;

        self.env.push_to_stack(ra);
        for arg in args.0.iter() {
            self.codegen_expr(arg)?;
        }

        self.env.add_instr(bal_label(label));

        if argc > 0 {
            self.env.add_instr(addiu(sp, sp, argc * 4));
        }

        self.env.pop_from_stack(ra);
        self.env.push_to_stack(t0);
        Ok(())
    }

    pub fn codegen_prog(&mut self, prog: &Prog) -> Result<(i32, Vec<Instr>), Error> {
        for fn_decl in prog.0.iter() {
            self.env.define_function_label(&fn_decl.id);
        }

        let main_label = self.env.lookup_label("main")?;
        self.env.add_instr(b_label(&main_label));
        for fn_decl in &prog.0 {
            self.codegen_fn_decl(fn_decl)?;
        }

        let mips = self.run();
        let result = mips.rf.get(t0) as i32;
        Ok((result, self.env.instructions.clone()))
    }
}

impl Default for CodegenVm {
    fn default() -> Self {
        Self::new()
    }
}

impl Eval<Vec<Instr>> for Expr {
    fn eval(&self) -> Result<Vec<Instr>, Error> {
        let mut cg = CodegenVm::new();
        cg.codegen_expr(self)?;
        Ok(cg.env.instructions.clone())
    }
}

impl Eval<Vec<Instr>> for Block {
    fn eval(&self) -> Result<Vec<Instr>, Error> {
        let mut cg = CodegenVm::new();
        cg.codegen_block_expr(self)?;
        Ok(cg.env.instructions.clone())
    }
}

impl Eval<Vec<Instr>> for Prog {
    fn eval(&self) -> Result<Vec<Instr>, Error> {
        let mut cg = CodegenVm::new();
        for fn_decl in self.0.iter() {
            cg.env.define_function_label(&fn_decl.id);
        }

        let main_label = cg.env.lookup_label("main")?;
        cg.env.add_instr(b_label(&main_label));
        for fn_decl in &self.0 {
            cg.codegen_fn_decl(fn_decl)?;
        }
        Ok(cg.env.instructions.clone())
    }
}
