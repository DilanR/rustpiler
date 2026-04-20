use derive_more::Constructor;

use crate::ast::*;
use crate::vm::Val;
use std::collections::HashMap;

pub type ValueScope = HashMap<String, Val>;
pub type FnScope = HashMap<String, FnDeclaration>;

pub struct Env {
    pub values: Vec<ValueScope>,
    pub functions: Vec<FnScope>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            values: vec![HashMap::new()],
            functions: vec![HashMap::new()],
        }
    }
    pub fn push_scope(&mut self) {
        self.values.push(HashMap::new());
        self.functions.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.values.pop();
        self.functions.pop();
    }
    pub fn lookup_value(&self, name: &str) -> Option<Val> {
        for scope in self.values.iter().rev() {
            if let Some(v) = scope.get(name) {
                return match v {
                    Val::Mut(inner) => Some(*inner.clone()),
                    _ => Some(v.clone()),
                };
            }
        }
        None
    }

    pub fn lookup_value_mut(&mut self, name: &str) -> Option<&mut Val> {
        for scope in self.values.iter_mut().rev() {
            if scope.contains_key(name) {
                return scope.get_mut(name);
            }
        }
        None
    }

    pub fn define_value(&mut self, id: &str, val: Val) {
        self.values.last_mut().unwrap().insert(id.to_owned(), val);
    }

    pub fn assign_value(&mut self, id: &str, new_val: Val) -> bool {
        for scope in self.values.iter_mut().rev() {
            if let Some(existing) = scope.get_mut(id) {
                match existing {
                    Val::Mut(inner) => {
                        **inner = new_val;
                    }
                    _ => {
                        *existing = new_val;
                    }
                }
                return true;
            }
        }

        false
    }

    pub fn lookup_fn(&self, name: &str) -> Option<(usize, FnDeclaration)> {
        for (i, scope) in self.functions.iter().enumerate().rev() {
            if let Some(f) = scope.get(name) {
                return Some((i, f.clone()));
            }
        }
        None
    }

    pub fn define_fn(&mut self, f: FnDeclaration) {
        self.functions.last_mut().unwrap().insert(f.id.clone(), f);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Constructor)]
pub struct AnnotatedType {
    pub ty: Type,
    pub mutable: bool,
    pub is_initialized: bool,
}

pub struct TypeEnv {
    bindings: Vec<HashMap<String, AnnotatedType>>,
    functions: Vec<HashMap<String, FnDeclaration>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            bindings: vec![HashMap::new()],
            functions: vec![HashMap::new()],
        }
    }
    pub fn push_scope(&mut self) {
        self.bindings.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.bindings.pop();
    }

    pub fn lookup_binding(&self, id: &str) -> Option<AnnotatedType> {
        for scope in self.bindings.iter().rev() {
            if let Some(binding) = scope.get(id) {
                return Some(binding.clone());
            }
        }
        None
    }

    pub fn lookup_function(&self, id: &str) -> Option<FnDeclaration> {
        for scope in self.functions.iter().rev() {
            if let Some(func) = scope.get(id) {
                return Some(func.clone());
            }
        }
        None
    }

    pub fn define_binding(&mut self, id: &str, a_ty: AnnotatedType) {
        self.bindings
            .last_mut()
            .unwrap()
            .insert(id.to_owned(), a_ty);
    }

    pub fn define_function(&mut self, fn_decl: FnDeclaration) {
        self.functions
            .last_mut()
            .unwrap()
            .insert(fn_decl.id.clone(), fn_decl);
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}
