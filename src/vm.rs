use crate::ast::{
    Arguments, BinOp, Block, Expr, FnDeclaration, Literal, Mutable, Prog, Statement, Type, UnOp,
};
use crate::common::Eval;
use crate::env;
use crate::error::{Error, VmError};
use crate::intrinsics::vm_println;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Lit(Literal),
    UnInit,
    Mut(Box<Val>),
}

// Helper for Op
impl BinOp {
    // Evaluate operator to literal
    pub fn eval(&self, left: Val, right: Val) -> Result<Val, Error> {
        match self {
            BinOp::Add => Ok(Val::Lit(Literal::Int(i32::from(left) + i32::from(right)))),
            BinOp::Sub => Ok(Val::Lit(Literal::Int(i32::from(left) - i32::from(right)))),
            BinOp::Mul => Ok(Val::Lit(Literal::Int(i32::from(left) * i32::from(right)))),
            BinOp::Div => Ok(Val::Lit(Literal::Int(i32::from(left) / i32::from(right)))),
            BinOp::And => Ok(Val::Lit(Literal::Bool(
                bool::from(left) && bool::from(right),
            ))),
            BinOp::Or => Ok(Val::Lit(Literal::Bool(
                bool::from(left) || bool::from(right),
            ))),
            BinOp::Eq => Ok(Val::Lit(Literal::Bool(left == right))),
            BinOp::Lt => Ok(Val::Lit(Literal::Bool(i32::from(left) < i32::from(right)))),
            BinOp::Gt => Ok(Val::Lit(Literal::Bool(i32::from(left) > i32::from(right)))),
        }
    }
}

impl Eval<Val> for Expr {
    fn eval(&self) -> Result<Val, Error> {
        let mut vm = VM::new();
        vm.eval_expr(self)
    }
}

impl Eval<Val> for Block {
    fn eval(&self) -> Result<Val, Error> {
        let mut vm = VM::new();
        vm.eval_block(self)
    }
}

impl Eval<Val> for Prog {
    fn eval(&self) -> Result<Val, Error> {
        let mut vm = VM::new();
        vm.eval_prog(self)
    }
}

impl From<Literal> for Val {
    fn from(lit: Literal) -> Self {
        Val::Lit(lit)
    }
}

impl From<String> for Val {
    fn from(str: String) -> Self {
        Val::Lit(Literal::String(str))
    }
}

impl From<Val> for Literal {
    fn from(val: Val) -> Literal {
        match val {
            Val::Lit(lit) => lit,
            _ => panic!("expected literal value"),
        }
    }
}

impl From<i32> for Val {
    fn from(val: i32) -> Self {
        Val::Lit(val.into())
    }
}

impl From<bool> for Val {
    fn from(val: bool) -> Self {
        Val::Lit(val.into())
    }
}

impl From<()> for Val {
    fn from(val: ()) -> Self {
        Val::Lit(val.into())
    }
}

impl From<Val> for i32 {
    fn from(val: Val) -> Self {
        match val {
            Val::Lit(Literal::Int(i)) => i,
            _ => panic!("cannot get int from {:?}", val),
        }
    }
}
impl From<Val> for bool {
    fn from(val: Val) -> Self {
        match val {
            Val::Lit(Literal::Bool(b)) => b,
            _ => panic!("cannot get bool from {:?}", val),
        }
    }
}

impl From<Literal> for bool {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Bool(b) => b,
            _ => panic!("cannot get bool from {:?}", lit),
        }
    }
}

// A VM will execute a single program. it needs to keep check of scope.
pub struct VM {
    env: env::Env,
}

impl VM {
    #![allow(clippy::new_without_default)]
    pub fn new() -> Self {
        VM {
            env: env::Env::default(),
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Val, Error> {
        match expr {
            Expr::Lit(lit) => Ok(Val::from(lit.clone())),
            Expr::Par(expr) => self.eval_expr(expr),
            Expr::Ident(id) => self
                .env
                .lookup_value(id)
                .ok_or(VmError::NoValFound(id.to_owned()).into()),
            Expr::BinOp(op, left, right) => {
                let left: Val = self.eval_expr(left)?;
                let right: Val = self.eval_expr(right)?;
                let result = BinOp::eval(op, left, right)?;
                Ok(result)
            }
            Expr::Call(fn_name, arguments) => self.eval_expr_call(fn_name, arguments),
            Expr::IfThenElse(expr, block, block1) => {
                self.eval_expr_if_then_else(expr, block, block1)
            }
            Expr::Block(block) => self.eval_block(block),
            Expr::UnOp(un_op, expr) => {
                let right = self.eval_expr(expr)?;
                let result = match un_op {
                    UnOp::Neg => Literal::Int(-i32::from(right)),
                    UnOp::Bang => Literal::Bool(!bool::from(right)),
                };
                Ok(Val::from(result))
            }
        }
    }

    pub fn eval_expr_call(&mut self, ident: &String, args: &Arguments) -> Result<Val, Error> {
        //first consider functions always in scope, in our case println!
        let collected_args = args
            .0
            .iter()
            .map(|arg| self.eval_expr(arg))
            .collect::<Result<Vec<Val>, Error>>()?;
        if ident == "println!" {
            let literals = collected_args
                .into_iter()
                .filter_map(|v| match v {
                    Val::Lit(literal) => Some(literal),
                    _ => None,
                })
                .collect();

            let (_, intrinsic) = vm_println();
            let res = intrinsic(literals);

            // TODO: proper eval of println!
            return Ok(Val::Lit(res));
        }
        let (def_depth, func) = self
            .env
            .lookup_fn(ident)
            .ok_or(VmError::NoFunctionFound(ident.to_owned()))?; // check arity of both matches in future match types aswell
        // check arity
        if func.parameters.0.len() != collected_args.len() {
            return Err(Error::ParameterArityMismatch {
                id: ident.to_owned(),
                expected: func.parameters.0.len(),
                got: collected_args.len(),
            });
        }

        let mut new_val_scope: HashMap<String, Val> = HashMap::new();
        //match callers args to parameters
        for (param, arg) in func.parameters.0.iter().zip(collected_args) {
            let val = if param.mutable.0 {
                Val::Mut(Box::new(arg))
            } else {
                arg
            };

            new_val_scope.insert(param.id.clone(), val);
        }

        // New vm for local scope
        let mut fn_vm = VM::new();

        fn_vm.env.functions = self.env.functions[..=def_depth].to_vec();

        fn_vm.env.values.push(new_val_scope);

        fn_vm.eval_block(&func.body)
    }

    pub fn eval_type(&mut self, expr: &Expr, ty: &Option<Type>) -> Result<Val, Error> {
        let val = self.eval_expr(expr)?;
        match ty {
            Some(t) => val.check_type(t)?,
            None => return Ok(val),
        }
        Ok(val)
    }

    pub fn eval_stmt(&mut self, stmt: &Statement) -> Result<Val, Error> {
        match stmt {
            Statement::Let(mutable, id, ty, expr) => self.eval_stmt_let(mutable, id, ty, expr),
            Statement::Assign(lhs, rhs) => self.eval_stmt_assign(lhs, rhs),
            Statement::While(cond, block) => self.eval_stmt_while(cond, block),
            Statement::Expr(expr) => self.eval_expr(expr),
            Statement::Fn(fn_declaration) => self.eval_stmt_fn(fn_declaration),
        }
    }

    pub fn eval_stmt_let(
        &mut self,
        mutable: &Mutable,
        id: &str,
        ty: &Option<Type>,
        expr: &Option<Expr>,
    ) -> Result<Val, Error> {
        let val = match expr {
            Some(e) => self.eval_expr(e)?,
            None => Val::UnInit,
        };

        self.env.define_value(
            id,
            if mutable.0 {
                Val::Mut(Box::new(val.clone()))
            } else {
                val.clone()
            },
        );

        Ok(Val::Lit(Literal::Unit))
    }

    pub fn eval_stmt_assign(&mut self, lhs: &Expr, rhs: &Expr) -> Result<Val, Error> {
        let rhs_val = self.eval_expr(rhs)?;
        if let Expr::Ident(id) = lhs {
            self.env.assign_value(id, rhs_val)?;
            Ok(Val::from(()))
        } else {
            Err(VmError::IllegalAssignment(lhs.to_string()).into())
        }
    }

    pub fn eval_block(&mut self, block: &Block) -> Result<Val, Error> {
        // push no scope for fn and value
        self.env.push_scope();

        // add all functions to scope such they are evaluable
        for stmt in &block.statements {
            match stmt {
                Statement::Fn(f) => {
                    let this = &mut self.env;
                    let f = f.to_owned();
                    this.functions.last_mut().unwrap().insert(f.id.clone(), f);
                }
                _ => continue,
            };
        }

        let mut last_val = Val::Lit(Literal::Unit);

        for stmt in &block.statements {
            last_val = self.eval_stmt(stmt)?;
        }

        self.env.pop_scope();

        if block.semi {
            Ok(Val::Lit(Literal::Unit))
        } else {
            Ok(last_val)
        }
    }

    fn eval_stmt_fn(&mut self, fn_declaration: &FnDeclaration) -> Result<Val, Error> {
        // add fn to fn_scope in current scope
        let f = fn_declaration.to_owned();
        self.env
            .functions
            .last_mut()
            .unwrap()
            .insert(f.id.clone(), f);
        Ok(Val::Lit(Literal::Unit))
    }

    fn eval_stmt_while(&mut self, cond: &Expr, block: &Block) -> Result<Val, Error> {
        while self.eval_expr(cond)?.get_bool()? {
            self.eval_block(block)?;
        }
        //while block always returns Unit
        Ok(Val::Lit(Literal::Unit))
    }

    pub fn eval_prog(&mut self, prog: &Prog) -> Result<Val, Error> {
        //Add all function to root-level scope
        for func in &prog.0 {
            {
                let this = &mut self.env;
                let f = func.to_owned();
                this.functions.last_mut().unwrap().insert(f.id.clone(), f);
            };
        }

        // check if valid program ie main exists
        let main_fn = match self.env.lookup_fn("main") {
            Some(main_fn) => main_fn.to_owned(),
            None => return Err(VmError::NoMainFound.into()),
        };

        //main should have no args and rtype, only parse body
        self.eval_block(&main_fn.1.body)
    }

    fn eval_expr_if_then_else(
        &mut self,
        expr: &Expr,
        block: &Block,
        block1: &Option<Block>,
    ) -> Result<Val, Error> {
        let cond = self.eval_expr(expr)?;
        if cond.get_bool()? {
            Ok(self.eval_block(block)?)
        } else {
            match block1 {
                Some(else_block) => Ok(self.eval_block(else_block)?),
                None => Ok(Val::Lit(Literal::Unit)),
            }
        }
    }
}
