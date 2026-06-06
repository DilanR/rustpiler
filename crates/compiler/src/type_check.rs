use proc_macro2::Span;

use crate::{
    ast::{
        Arguments, BinOp, Block, Expr, ExprKind, FnDeclaration, Prog, Statement, StatementKind,
        Type,
    },
    common::Eval,
    env::{AnnotatedType, TypeEnv},
    error::{Error, TypeError},
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
        match &expr.node {
            ExprKind::Ident(l) => match self.env.lookup_binding(l) {
                Some(binding) => Ok(binding.ty),
                None => Err(TypeError::UnknownVariable {
                    name: l.to_owned(),
                    span: expr.span,
                }
                .into()),
            },
            ExprKind::Lit(literal) => Ok(literal.clone().into()),
            ExprKind::BinOp(bin_op, lhs, rhs) => {
                let lhs_type = self.check_expr(lhs)?;
                let rhs_type = self.check_expr(rhs)?;
                match bin_op {
                    BinOp::Eq | BinOp::Lt | BinOp::Gt => {
                        unify(expr.span, lhs_type, Type::i32())?;
                        unify(expr.span, rhs_type, Type::i32())?;
                        Ok(Type::bool())
                    }
                    _ => {
                        let expected = bin_op.expected_type();
                        let got = unify(expr.span, lhs_type, rhs_type)?;
                        unify(expr.span, got, expected)
                    }
                }
            }
            ExprKind::Par(inner) => self.check_expr(inner),
            ExprKind::Call(id, arguments) => self.check_call(expr, id, arguments),
            ExprKind::IfThenElse(cond, then, opt) => {
                unify(expr.span, self.check_expr(cond)?, Type::bool())?;
                match opt {
                    Some(else_block) => unify(
                        expr.span,
                        self.check_block(then)?,
                        self.check_block(else_block)?,
                    ),
                    None => {
                        // https://doc.rust-lang.org/reference/expressions/if-expr.html#r-expr.if.result
                        self.check_block(then)?;
                        Ok(Type::unit())
                    }
                }
            }
            ExprKind::Block(block) => self.check_block(block),
            ExprKind::UnOp(un_op, expr) => {
                let expected = un_op.expected_type();
                unify(expr.span, self.check_expr(expr)?, expected)
            }
        }
    }

    pub fn check_stmt(&mut self, stmt: &Statement) -> Result<Type, Error> {
        match &stmt.node {
            StatementKind::Let(mutable, id, ty, expr) => match (ty, expr) {
                (None, None) => Err(TypeError::Uninitialized {
                    name: id.into(),
                    span: stmt.span,
                }
                .into()),
                (None, Some(e)) => {
                    let inferred_type = self.check_expr(e)?;
                    self.env.define_binding(
                        id,
                        AnnotatedType::new(inferred_type.clone(), mutable.0, true),
                    );
                    Ok(inferred_type)
                }
                (Some(t), None) => {
                    self.env
                        .define_binding(id, AnnotatedType::new(t.clone(), mutable.0, false));
                    Ok(t.clone())
                }
                (Some(t), Some(e)) => {
                    unify(e.span, self.check_expr(e)?, t.clone())?;
                    self.env
                        .define_binding(id, AnnotatedType::new(t.clone(), mutable.0, true));
                    Ok(t.clone())
                }
            },
            StatementKind::Assign(lhs, rhs) => {
                //check lhs is mutable or unInit and identifier
                let (lhs_at, id) = match &lhs.node {
                    ExprKind::Ident(id) => match self.env.lookup_binding(id) {
                        Some(t) => (t, id),
                        None => {
                            return Err(TypeError::UnknownVariable {
                                name: id.clone(),
                                span: lhs.span,
                            }
                            .into());
                        }
                    },

                    _ => {
                        return Err(TypeError::Assignment {
                            kind: crate::error::AssignmentErrorKind::NotIdent,
                            span: lhs.span,
                        }
                        .into());
                    }
                };

                let ty = unify(rhs.span, lhs_at.ty, self.check_expr(rhs)?)?;

                let new_at = match (lhs_at.mutable, lhs_at.is_initialized) {
                    (true, _) => AnnotatedType::new(ty, true, true),
                    (false, false) => AnnotatedType::new(ty, false, true),
                    _ => {
                        return Err(TypeError::Assignment {
                            kind: crate::error::AssignmentErrorKind::NotMutable,
                            span: lhs.span,
                        }
                        .into());
                    }
                };

                self.env.define_binding(id, new_at);

                Ok(Type::unit())
            }
            StatementKind::While(cond, block) => {
                unify(cond.span, self.check_expr(cond)?, Type::bool())?;
                Ok(self.check_block(block)?)
            }
            StatementKind::Expr(expr) => Ok(self.check_expr(expr)?),
            StatementKind::Fn(fn_decl) => {
                let _params_type: Vec<AnnotatedType> = fn_decl
                    .node
                    .parameters
                    .node
                    .0
                    .iter()
                    .map(|p| {
                        let a_ty = AnnotatedType::new(p.node.ty.clone(), p.node.mutable.0, true);
                        self.env.define_binding(&p.node.id, a_ty.clone());
                        a_ty
                    })
                    .collect();

                AnnotatedType::new(fn_decl.node.ty.clone().unwrap_or(Type::unit()), false, true);

                Ok(Type::unit())
            }
        }
    }

    pub fn check_block(&mut self, block: &Block) -> Result<Type, Error> {
        self.env.push_scope();

        for stmt in &block.node.statements {
            match &stmt.node {
                StatementKind::Fn(f) => {
                    let f = f.to_owned();
                    self.env.define_function(f);
                }
                _ => continue,
            };
        }

        for stmt in block.node.statements.iter().rev().skip(1).rev() {
            self.check_stmt(stmt)?; // ignore the type; just check validity
        }

        let last_ty = match block.node.statements.last() {
            Some(stmt) => self.check_stmt(stmt)?,
            None => Type::unit(),
        };

        self.env.pop_scope();
        if block.node.semi {
            Ok(Type::unit())
        } else {
            Ok(last_ty)
        }
    }

    fn check_call(&mut self, expr: &Expr, id: &str, arguments: &Arguments) -> Result<Type, Error> {
        // lookup fn with id
        if id == "println!" {
            //first arg should be string
            match arguments.node.0.first() {
                Some(str_arg) => unify(expr.span, self.check_expr(str_arg)?, Type::string())?,
                None => {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::string(),
                        got: Type::unit(),
                        span: expr.span,
                    }
                    .into());
                }
            };
            //rest should i32
            for arg in arguments.node.0.iter().skip(1) {
                unify(arg.span, self.check_expr(arg)?, Type::i32())?;
            }

            Ok(Type::unit())
        } else {
            let Some(fn_decl) = self.env.lookup_function(id) else {
                return Err(TypeError::UnknownFunction {
                    name: id.to_owned(),
                    span: expr.span,
                }
                .into());
            };

            //check arity
            if fn_decl.node.parameters.node.0.len() != arguments.node.0.len() {
                return Err(TypeError::ParameterArityMismatch {
                    id: fn_decl.node.id.clone(),
                    expected: fn_decl.node.parameters.node.0.len(),
                    got: arguments.node.0.len(),
                    span: expr.span,
                }
                .into());
            };

            // check all parameters have a binding and match with arguments
            for (param, arg) in fn_decl
                .node
                .parameters
                .node
                .0
                .iter()
                .zip(arguments.node.0.iter())
            {
                let Some(binding) = self.env.lookup_binding(&param.node.id) else {
                    return Err(TypeError::UnknownVariable {
                        name: param.node.id.to_owned(),
                        span: expr.span,
                    }
                    .into());
                };
                unify(arg.span, binding.ty, self.check_expr(arg)?)?;
            }

            Ok(fn_decl.node.ty.unwrap_or(Type::unit()))
        }
    }

    fn check_fn(&mut self, fn_decl: &FnDeclaration) -> Result<Type, Error> {
        let r_type = fn_decl.node.ty.clone().unwrap_or(Type::unit());
        // Add param to bindings
        for param in fn_decl.node.parameters.node.0.iter() {
            self.env.define_binding(
                &param.node.id,
                AnnotatedType {
                    ty: param.node.ty.clone(),
                    mutable: param.node.mutable.0,
                    is_initialized: true,
                },
            );
        }

        // eval return type for body
        let body_return_type = self.check_block(&fn_decl.node.body)?;

        // compare body return type and fn return type
        // TODO change sig for unify
        unify(fn_decl.span, r_type, body_return_type)
    }

    pub fn check_prog(&mut self, prog: &Prog) -> Result<Type, Error> {
        // Go through each fn and put in env check for duplicates
        let mut seen = std::collections::HashSet::new();

        for fn_decl in &prog.0 {
            let id = fn_decl.node.id.clone();
            if !seen.insert(id.clone()) {
                return Err(TypeError::Duplicate {
                    kind: crate::error::DuplicateKind::Function,
                    name: id,
                    span: fn_decl.span,
                }
                .into());
            }
            self.env.define_function(fn_decl.to_owned());
        }

        for fn_decl in &prog.0 {
            self.check_fn(fn_decl)?;
        }

        //valid program needs main
        let main = match self.env.lookup_function("main") {
            Some(f) => f,
            None => {
                return Err(TypeError::UnknownFunction {
                    name: "main".to_string(),
                    span: proc_macro2::Span::call_site(),
                }
                .into());
            }
        };

        self.check_fn(&main)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

fn unify(span: Span, got: Type, expected: Type) -> Result<Type, Error> {
    match got.node == expected.node {
        true => Ok(expected),
        false => Err(TypeError::TypeMismatch {
            expected,
            got,
            span,
        }
        .into()),
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
