use std::{collections::HashMap, vec};

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
    pub errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
            errors: vec![],
        }
    }
    fn report(&mut self, error: TypeError) -> Type {
        self.errors.push(error);
        Type::error()
    }

    fn try_unify(&mut self, span: Span, got: Type, expected: Type) -> Type {
        match unify(span, got, expected) {
            Ok(ty) => ty,
            Err(err) => {
                self.errors.push(err);
                Type::error()
            }
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Type {
        match &expr.node {
            ExprKind::Ident(l) => match self.env.lookup_binding(l) {
                Some(binding) => binding.ty,
                None => self.report(TypeError::UnknownVariable {
                    name: l.to_owned(),
                    span: expr.span,
                }),
            },
            ExprKind::Lit(literal) => literal.clone().into(),
            ExprKind::BinOp(bin_op, lhs, rhs) => {
                let lhs_type = self.check_expr(lhs);
                let rhs_type = self.check_expr(rhs);
                match bin_op {
                    BinOp::Eq | BinOp::Lt | BinOp::Gt => {
                        let lhs = self.try_unify(expr.span, lhs_type, Type::i32());
                        let rhs = self.try_unify(expr.span, rhs_type, Type::i32());

                        if lhs.is_error() || rhs.is_error() {
                            Type::error()
                        } else {
                            Type::bool()
                        }
                    }
                    _ => {
                        let expected = bin_op.expected_type();
                        let got = self.try_unify(expr.span, lhs_type, rhs_type);
                        self.try_unify(expr.span, got, expected)
                    }
                }
            }
            ExprKind::Par(inner) => self.check_expr(inner),
            ExprKind::Call(id, arguments) => self.check_call(expr, id, arguments),
            ExprKind::IfThenElse(cond, then, opt) => {
                let cond_type = self.check_expr(cond);
                self.try_unify(expr.span, cond_type, Type::bool());
                match opt {
                    Some(else_block) => {
                        let expected = self.check_block(else_block);
                        let got = self.check_block(then);

                        self.try_unify(expr.span, got, expected)
                    }
                    None => {
                        // https://doc.rust-lang.org/reference/expressions/if-expr.html#r-expr.if.result
                        self.check_block(then);
                        Type::unit()
                    }
                }
            }
            ExprKind::Block(block) => self.check_block(block),
            ExprKind::UnOp(un_op, expr) => {
                let expected = un_op.expected_type();
                let got = self.check_expr(expr);
                self.try_unify(expr.span, got, expected)
            }
        }
    }

    pub fn check_stmt(&mut self, stmt: &Statement) -> Type {
        match &stmt.node {
            StatementKind::Let(mutable, id, ty, expr) => match (ty, expr) {
                (None, None) => self.report(TypeError::Uninitialized {
                    name: id.into(),
                    span: stmt.span,
                }),
                (None, Some(e)) => {
                    let inferred_type = self.check_expr(e);
                    self.env.define_binding(
                        id,
                        AnnotatedType::new(inferred_type.clone(), mutable.0, true, stmt.span),
                    );
                    inferred_type
                }
                (Some(t), None) => {
                    self.env.define_binding(
                        id,
                        AnnotatedType::new(t.clone(), mutable.0, false, stmt.span),
                    );
                    t.clone()
                }
                (Some(t), Some(e)) => {
                    let got = self.check_expr(e);
                    self.try_unify(e.span, got, t.clone());
                    self.env.define_binding(
                        id,
                        AnnotatedType::new(t.clone(), mutable.0, true, stmt.span),
                    );
                    t.clone()
                }
            },
            StatementKind::Assign(lhs, rhs) => {
                //check lhs is mutable or unInit and identifier
                let (lhs_at, id) = match &lhs.node {
                    ExprKind::Ident(id) => match self.env.lookup_binding(id) {
                        Some(t) => (t, id),
                        None => {
                            return self.report(TypeError::UnknownVariable {
                                name: id.clone(),
                                span: lhs.span,
                            });
                        }
                    },

                    _ => {
                        return self.report(TypeError::Assignment {
                            kind: crate::error::AssignmentErrorKind::NotIdent,
                            span: lhs.span,
                            decl_span: lhs.span,
                        });
                    }
                };

                let expected = self.check_expr(rhs);
                let ty = self.try_unify(rhs.span, lhs_at.clone().ty, expected);

                let new_at = match (lhs_at.mutable, lhs_at.is_initialized) {
                    (true, _) => AnnotatedType::new(ty, true, true, lhs_at.ty.span),
                    (false, false) => AnnotatedType::new(ty, false, true, lhs_at.ty.span),
                    _ => {
                        return self.report(TypeError::Assignment {
                            kind: crate::error::AssignmentErrorKind::NotMutable,
                            span: lhs.span,
                            decl_span: lhs_at.decl_span,
                        });
                    }
                };

                self.env.define_binding(id, new_at);

                Type::unit()
            }
            StatementKind::While(cond, block) => {
                let got = self.check_expr(cond);
                self.try_unify(cond.span, got, Type::bool());
                self.check_block(block)
            }
            StatementKind::Expr(expr) => self.check_expr(expr),
            StatementKind::Fn(fn_decl) => {
                let _params_type: Vec<AnnotatedType> = fn_decl
                    .node
                    .parameters
                    .node
                    .0
                    .iter()
                    .map(|p| {
                        let a_ty =
                            AnnotatedType::new(p.node.ty.clone(), p.node.mutable.0, true, p.span);
                        self.env.define_binding(&p.node.id, a_ty.clone());
                        a_ty
                    })
                    .collect();

                AnnotatedType::new(
                    fn_decl.node.ty.clone().unwrap_or(Type::unit()),
                    false,
                    true,
                    match &fn_decl.node.ty {
                        Some(rt) => rt.span,
                        None => fn_decl.span,
                    },
                );

                Type::unit()
            }
        }
    }

    pub fn check_block(&mut self, block: &Block) -> Type {
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
            self.check_stmt(stmt); // ignore the type; just check validity
        }

        let last_ty = match block.node.statements.last() {
            Some(stmt) => self.check_stmt(stmt),
            None => Type::unit(),
        };

        self.env.pop_scope();
        if block.node.semi {
            Type::unit()
        } else {
            last_ty
        }
    }

    fn check_call(&mut self, expr: &Expr, id: &str, arguments: &Arguments) -> Type {
        // lookup fn with id
        if id == "println!" {
            //first arg should be string
            match arguments.node.0.first() {
                Some(str_arg) => {
                    let got = self.check_expr(str_arg);
                    self.try_unify(expr.span, got, Type::string())
                }
                None => self.report(TypeError::TypeMismatch {
                    expected: Type::string(),
                    got: Type::unit(),
                    span: expr.span,
                }),
            };
            //rest should i32
            for arg in arguments.node.0.iter().skip(1) {
                let got = self.check_expr(arg);
                self.try_unify(arg.span, got, Type::i32());
            }

            Type::unit()
        } else {
            let Some(fn_decl) = self.env.lookup_function(id) else {
                return self.report(TypeError::UnknownFunction {
                    name: id.to_owned(),
                    span: expr.span,
                });
            };

            //check arity
            if fn_decl.node.parameters.node.0.len() != arguments.node.0.len() {
                return self.report(TypeError::ParameterArityMismatch {
                    id: fn_decl.node.id.clone(),
                    expected: fn_decl.node.parameters.node.0.len(),
                    got: arguments.node.0.len(),
                    call_span: expr.span,
                    fn_span: fn_decl.span,
                });
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
                    return self.report(TypeError::UnknownVariable {
                        name: param.node.id.to_owned(),
                        span: expr.span,
                    });
                };
                let expected = self.check_expr(arg);
                self.try_unify(arg.span, binding.ty, expected);
            }

            fn_decl.node.ty.unwrap_or(Type::unit())
        }
    }

    fn check_fn(&mut self, fn_decl: &FnDeclaration) -> Type {
        let expected = fn_decl.node.ty.clone().unwrap_or(Type::unit());
        // Add param to bindings
        for param in fn_decl.node.parameters.node.0.iter() {
            self.env.define_binding(
                &param.node.id,
                AnnotatedType {
                    ty: param.node.ty.clone(),
                    mutable: param.node.mutable.0,
                    is_initialized: true,
                    decl_span: param.span,
                },
            );
        }

        // eval return type for body
        let got = self.check_block(&fn_decl.node.body);

        // compare body return type and fn return type
        self.try_unify(fn_decl.span, got, expected)
    }

    pub fn check_prog(&mut self, prog: &Prog) -> Type {
        // Go through each fn and put in env check for duplicates
        let mut seen: HashMap<String, Span> = HashMap::new();

        for fn_decl in &prog.0 {
            let id = fn_decl.node.id.clone();

            if let Some(first_span) = seen.get(&id) {
                return self.report(TypeError::Duplicate {
                    kind: crate::error::DuplicateKind::Function,
                    name: id,
                    first_span: *first_span,
                    second_span: fn_decl.span,
                });
            }

            seen.insert(id, fn_decl.span);
            self.env.define_function(fn_decl.to_owned());
        }

        for fn_decl in &prog.0 {
            self.check_fn(fn_decl);
        }

        //valid program needs main
        let main_fn = match self.env.lookup_function("main") {
            Some(f) => f,
            None => {
                return self.report(TypeError::UnknownFunction {
                    name: "main".to_string(),
                    span: proc_macro2::Span::call_site(),
                });
            }
        };

        match main_fn.node.ty {
            Some(t) => t,
            None => Type::unit(),
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

fn unify(span: Span, got: Type, expected: Type) -> Result<Type, TypeError> {
    if got.is_error() || expected.is_error() {
        return Ok(Type::error());
    }
    match got.node == expected.node {
        true => Ok(expected),
        false => Err(TypeError::TypeMismatch {
            expected,
            got,
            span,
        }),
    }
}

impl Eval<Type> for Expr {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        let final_type = type_checker.check_expr(self);
        if type_checker.errors.is_empty() {
            Ok(final_type)
        } else {
            Err(type_checker.errors.first().unwrap().clone().into())
        }
    }
}

impl Eval<Type> for Block {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        let final_type = type_checker.check_block(self);
        if type_checker.errors.is_empty() {
            Ok(final_type)
        } else {
            Err(type_checker.errors.first().unwrap().clone().into())
        }
    }
}

impl Eval<Type> for Prog {
    fn eval(&self) -> Result<Type, Error> {
        let mut type_checker = TypeChecker::new();
        let final_type = type_checker.check_prog(self);
        if type_checker.errors.is_empty() {
            Ok(final_type)
        } else {
            Err(type_checker.errors.first().unwrap().clone().into())
        }
    }
}
