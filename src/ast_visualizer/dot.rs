use crate::ast::*;
use crate::ast_traits;
// ---------- Public API ----------

pub struct DotGenerator {
    ctx: DotCtx,
}

impl DotGenerator {
    pub fn new() -> Self {
        Self { ctx: DotCtx::new() }
    }

    pub fn generate(&mut self, prog: &Prog) -> String {
        self.visit_prog(prog);
        self.ctx.clone().finish()
    }
}

impl Default for DotGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ---------- Internal Context ----------

type NodeId = usize;

#[derive(Clone)]
struct DotCtx {
    counter: NodeId,
    nodes: Vec<String>,
    edges: Vec<String>,
}

impl DotCtx {
    fn new() -> Self {
        Self {
            counter: 0,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn next_id(&mut self) -> NodeId {
        let id = self.counter;
        self.counter += 1;
        id
    }

    fn add_node(&mut self, id: NodeId, label: &str) {
        self.nodes.push(format!("n{} [label=\"{}\"];", id, label));
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.edges.push(format!("n{} -> n{};", from, to));
    }

    fn finish(self) -> String {
        let mut out = String::from("digraph AST {\n");

        for n in self.nodes {
            out.push_str("  ");
            out.push_str(&n);
            out.push('\n');
        }

        for e in self.edges {
            out.push_str("  ");
            out.push_str(&e);
            out.push('\n');
        }

        out.push('}');
        out
    }
}

// ---------- Traversal (start small) ----------

impl DotGenerator {
    fn visit_prog(&mut self, prog: &Prog) -> NodeId {
        let id = self.ctx.next_id();
        self.ctx.add_node(id, "Prog");

        for func in &prog.0 {
            let child_id = self.visit_fn(func);
            self.ctx.add_edge(id, child_id);
        }

        id
    }

    fn visit_fn(&mut self, f: &FnDeclaration) -> NodeId {
        let id = self.ctx.next_id();

        let label = format!("fn {}", f.id);
        self.ctx.add_node(id, &label);

        let body_id = self.visit_block(&f.body);
        self.ctx.add_edge(id, body_id);

        id
    }

    fn visit_parameters(&mut self, parameters: &Parameters) -> NodeId {
        let id = self.ctx.next_id();
        self.ctx.add_node(id, "Parameters");

        for parameter in &parameters.0 {
            let child_id = self.visit_stmt(stmt);
            self.ctx.add_edge(id, child_id);
        }

        id
    }

    fn visit_block(&mut self, block: &Block) -> NodeId {
        let id = self.ctx.next_id();
        self.ctx.add_node(id, "Block");

        for stmt in &block.statements {
            let child_id = self.visit_stmt(stmt);
            self.ctx.add_edge(id, child_id);
        }

        id
    }

    fn visit_stmt(&mut self, stmt: &Statement) -> NodeId {
        let id = self.ctx.next_id();

        match stmt {
            Statement::Let(_, name, _, expr) => {
                self.ctx.add_node(id, &format!("let {}", name));

                if let Some(e) = expr {
                    let child = self.visit_expr(e);
                    self.ctx.add_edge(id, child);
                }
            }

            Statement::Expr(e) => {
                self.ctx.add_node(id, "Expr");
                let child = self.visit_expr(e);
                self.ctx.add_edge(id, child);
            }

            _ => {
                self.ctx.add_node(id, "stmt");
            }
        }

        id
    }

    fn visit_expr(&mut self, expr: &Expr) -> NodeId {
        let id = self.ctx.next_id();

        match expr {
            Expr::Ident(name) => {
                self.ctx.add_node(id, name);
            }

            Expr::Lit(Literal::Int(i)) => {
                self.ctx.add_node(id, &i.to_string());
            }

            Expr::BinOp(op, lhs, rhs) => {
                self.ctx.add_node(id, &op.to_string());

                let l = self.visit_expr(lhs);
                let r = self.visit_expr(rhs);

                self.ctx.add_edge(id, l);
                self.ctx.add_edge(id, r);
            }

            _ => {
                self.ctx.add_node(id, "Expr");
            }
        }

        id
    }
}
