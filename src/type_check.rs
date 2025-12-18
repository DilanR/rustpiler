use std::{collections::HashMap, hash::Hash};

use derive_more::Constructor;

use crate::{
    ast::{Arguments, BinOp, Block, Expr, FnDeclaration, Literal, Prog, Statement, Type},
    ast_traits,
    common::Eval,
    env::{AnnotatedType, TypeEnv},
    error::{Error, TypeError},
    vm::Val,
};

pub struct TypeChecker {
    pub env: TypeEnv,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        match expr {
            Expr::Ident(l) => Ok(self.env.lookup_binding(l)?.ty),
            Expr::Lit(literal) => Ok(literal.clone().into()),
            Expr::BinOp(bin_op, lhs, rhs) => {
                let lhs_type = self.check_expr(lhs)?;
                let rhs_type = self.check_expr(rhs)?;
                match bin_op {
                    BinOp::Eq | BinOp::Lt | BinOp::Gt => {
                        unify(lhs_type, Type::I32)?;
                        unify(rhs_type, Type::I32)?;
                        Ok(Type::Bool)
                    }
                    _ => {
                        let expected = bin_op.expected_type();
                        let got = unify(lhs_type, rhs_type)?;
                        unify(got, expected)
                    }
                }
            }
            Expr::Par(inner) => self.check_expr(inner),
            Expr::Call(id, arguments) => self.check_call(id, arguments),
            Expr::IfThenElse(cond, then, opt) => {
                unify(self.check_expr(cond)?, Type::Bool)?;
                match opt {
                    Some(else_block) => {
                        unify(self.check_block(then)?, self.check_block(else_block)?)
                    }
                    None => self.check_block(then),
                }
            }
            Expr::Block(block) => self.check_block(block),
            Expr::UnOp(un_op, expr) => {
                let expected = un_op.expected_type();
                unify(self.check_expr(expr)?, expected)
            }
        }
    }

    pub fn check_stmt(&mut self, stmt: &Statement) -> Result<Type, Error> {
        match stmt {
            Statement::Let(mutable, id, ty, expr) => match (ty, expr) {
                (None, None) => Err(TypeError::UnInitType(id.to_owned()).into()),
                (None, Some(e)) => {
                    let infered_type = self.check_expr(e)?;
                    self.env.define_binding(
                        id,
                        AnnotatedType::new(infered_type.clone(), mutable.0, true),
                    );
                    Ok(infered_type)
                }
                (Some(t), None) => {
                    self.env
                        .define_binding(id, AnnotatedType::new(t.clone(), mutable.0, false));
                    Ok(t.clone())
                }
                (Some(t), Some(e)) => {
                    unify(self.check_expr(e)?, t.clone())?;
                    self.env
                        .define_binding(id, AnnotatedType::new(t.clone(), mutable.0, true));
                    Ok(t.clone())
                }
            },
            Statement::Assign(lhs, rhs) => {
                //check lhs is mutable or unInit and identifier
                let (lhs_at, id) = match lhs {
                    Expr::Ident(id) => match self.env.lookup_binding(id) {
                        Ok(t) => (t, id),
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    _ => return Err(TypeError::AssignmentToNonIdent(lhs.to_string()).into()),
                };

                let ty = unify(lhs_at.ty, self.check_expr(rhs)?)?;

                let new_at = match (lhs_at.mutable, lhs_at.is_initialized) {
                    (true, _) => AnnotatedType::new(ty, true, true),
                    (false, false) => AnnotatedType::new(ty, false, true),
                    _ => return Err(TypeError::NonMutableAssignment().into()),
                };

                self.env.define_binding(id, new_at);

                Ok(Type::Unit)
            }
            Statement::While(cond, block) => {
                unify(self.check_expr(cond)?, Type::Bool)?;
                Ok(self.check_block(block)?)
            }
            Statement::Expr(expr) => Ok(self.check_expr(expr)?),
            Statement::Fn(fn_decl) => {
                let params_type: Vec<AnnotatedType> = fn_decl
                    .parameters
                    .0
                    .iter()
                    .map(|p| {
                        let a_ty = AnnotatedType::new(p.ty.clone(), p.mutable.0, true);
                        self.env.define_binding(&p.id, a_ty.clone());
                        a_ty
                    })
                    .collect();

                AnnotatedType::new(fn_decl.ty.clone().unwrap_or(Type::Unit), false, true);

                Ok(Type::Unit)
            }
        }
    }

    pub fn check_block(&mut self, block: &Block) -> Result<Type, Error> {
        self.env.push_scope();

        for stmt in &block.statements {
            match stmt {
                Statement::Fn(f) => {
                    let f = f.to_owned();
                    self.env.define_function(f);
                }
                _ => continue,
            };
        }

        for stmt in block.statements.iter().rev().skip(1).rev() {
            self.check_stmt(stmt)?; // ignore the type; just check validity
        }

        let last_ty = match block.statements.last() {
            Some(stmt) => self.check_stmt(stmt)?,
            None => Type::Unit,
        };

        self.env.pop_scope();
        if block.semi {
            Ok(Type::Unit)
        } else {
            Ok(last_ty)
        }
    }

    fn check_call(&mut self, id: &str, arguments: &Arguments) -> Result<Type, Error> {
        // lookup fn with id
        if id == "println!" {
            //first arg should be string
            match arguments.0.first() {
                Some(str_arg) => unify(self.check_expr(str_arg)?, Type::String)?,
                None => {
                    return Err(TypeError::InferenceMismatch {
                        expected: Type::String,
                        got: Type::Unit,
                    }
                    .into());
                }
            };
            //rest should i32
            for arg in arguments.0.iter().skip(1) {
                unify(self.check_expr(arg)?, Type::I32)?;
            }

            Ok(Type::Unit)
        } else {
            let fn_decl = self.env.lookup_function(id)?;
            //check arity
            if fn_decl.parameters.0.len() != arguments.0.len() {
                return Err(Error::ParameterArityMismatch {
                    id: fn_decl.id.clone(),
                    expected: fn_decl.parameters.0.len(),
                    got: arguments.0.len(),
                });
            };

            // check all parameters have a binding and match with arguments
            for (param, arg) in fn_decl.parameters.0.iter().zip(arguments.0.iter()) {
                let binding = self.env.lookup_binding(&param.id)?;
                unify(binding.ty, self.check_expr(arg)?)?;
            }

            Ok(fn_decl.ty.unwrap_or(Type::Unit))
        }
    }

    fn check_fn(&mut self, fn_decl: &FnDeclaration) -> Result<Type, Error> {
        let r_type = fn_decl.ty.clone().unwrap_or(Type::Unit);
        // Add param to bindings
        for param in fn_decl.parameters.0.iter() {
            self.env.define_binding(
                &param.id,
                AnnotatedType {
                    ty: param.ty.clone(),
                    mutable: param.mutable.0,
                    is_initialized: true,
                },
            );
        }

        // eval return type for body
        let body_retrun_type = self.check_block(&fn_decl.body)?;

        // compare body return type and fn return type
        unify(r_type, body_retrun_type)
    }

    pub fn check_prog(&mut self, prog: &Prog) -> Result<Type, Error> {
        // Go through each fn and put in env check for duplicates
        for fn_decl in &prog.0 {
            let id = fn_decl.id.clone();
            if self.env.lookup_function(&id).is_ok() {
                return Err(TypeError::DuplicateFunction(id).into());
            }
            self.env.define_function(fn_decl.to_owned());
        }

        for fn_decl in &prog.0 {
            self.check_fn(fn_decl)?;
        }

        //valid program needs main
        let main = self.env.lookup_function("main")?;

        self.check_fn(&main)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

fn unify(got: Type, expected: Type) -> Result<Type, Error> {
    match got == expected {
        true => Ok(expected),
        false => Err(TypeError::InferenceMismatch { expected, got }.into()),
    }
}

impl Eval<Type> for Expr {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        type_checker.check_expr(self)
    }
}

impl Eval<Type> for Block {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        type_checker.check_block(self)
    }
}

impl Eval<Type> for Prog {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        type_checker.check_prog(self)
    }
}

// Helpers for Val
// Alternatively implement the TryFrom trait
impl Val {
    pub fn get_bool(&self) -> Result<bool, Error> {
        match self {
            Val::Lit(Literal::Bool(b)) => Ok(*b),
            other => Err(Error::TypeMismatch {
                expected: "bool",
                got: other.clone(),
            }),
        }
    }

    pub fn get_int(&self) -> Result<i32, Error> {
        match self {
            Val::Lit(Literal::Int(i)) => Ok(*i),
            other => Err(Error::TypeMismatch {
                expected: "i32",
                got: other.clone(),
            }),
        }
    }

    pub fn get_string(&self) -> Result<String, Error> {
        match self {
            Val::Lit(Literal::String(i)) => Ok(i.clone()),
            other => Err(Error::TypeMismatch {
                expected: "String",
                got: other.clone(),
            }),
        }
    }
    pub fn get_unit(&self) -> Result<(), Error> {
        match self {
            Val::Lit(Literal::Unit) => Ok(()),
            other => Err(Error::TypeMismatch {
                expected: "()",
                got: other.clone(),
            }),
        }
    }
    pub fn check_type(&self, ty: &Type) -> Result<(), Error> {
        match ty {
            Type::I32 => {
                self.get_int()?;
            }
            Type::Bool => {
                self.get_bool()?;
            }
            Type::String => {
                self.get_string()?;
            }
            Type::Unit => {
                self.get_unit()?;
            }
        }
        Ok(())
    }
}
