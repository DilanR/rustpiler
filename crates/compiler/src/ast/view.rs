use serde::Serialize;

use crate::{
    ast::{
        Arguments, Block, Expr, ExprKind::*, FnDeclaration, Parameter, Parameters, Prog, Statement,
        StatementKind::*, Type,
    },
    error::ErrRange,
};

// Feature for frontend, making Ast svgs on hover of code.
// If span is None assume all subtrees with children None belongs to 'local' Ast
#[derive(Debug, Serialize)]
pub struct AstNode {
    label: String,
    span: Option<ErrRange>,
    children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(label: impl Into<String>, span: Option<ErrRange>, children: Vec<AstNode>) -> Self {
        Self {
            label: label.into(),
            span,
            children,
        }
    }
}

impl From<&Prog> for AstNode {
    fn from(prog: &Prog) -> Self {
        AstNode::new(
            "Prog".to_string(),
            None,
            prog.0.iter().map(AstNode::from).collect(),
        )
    }
}

impl From<&FnDeclaration> for AstNode {
    fn from(value: &FnDeclaration) -> Self {
        let rtype = value.node.ty.as_ref().unwrap_or(&Type::Unit);
        AstNode::new(
            format!("fn {}", value.node.id),
            Some(value.span.into()),
            vec![
                (&value.node.parameters).into(),
                (&value.node.body).into(),
                rtype.into(),
            ],
        )
    }
}

impl From<&Block> for AstNode {
    fn from(value: &Block) -> Self {
        AstNode::new(
            if value.node.semi {
                "Block (semi)"
            } else {
                "Block"
            },
            Some(value.span.into()),
            value.node.statements.iter().map(AstNode::from).collect(),
        )
    }
}

impl From<&Statement> for AstNode {
    fn from(value: &Statement) -> Self {
        match &value.node {
            Let(mutable, name, ty, expr) => {
                let mut children = Vec::new();

                if let Some(ty) = ty {
                    children.push(AstNode::from(ty));
                }

                if let Some(expr) = expr {
                    children.push(AstNode::from(expr));
                }

                AstNode::new(
                    format!("Let({}{})", if mutable.0 { "mut " } else { "" }, name),
                    Some(value.span.into()),
                    children,
                )
            }

            Assign(lhs, rhs) => AstNode::new(
                "Assign",
                Some(value.span.into()),
                vec![AstNode::from(lhs), AstNode::from(rhs)],
            ),

            While(cond, body) => AstNode::new(
                "While",
                Some(value.span.into()),
                vec![AstNode::from(cond), AstNode::from(body)],
            ),

            Expr(expr) => expr.into(),

            Fn(func) => AstNode::from(func),
        }
    }
}

impl From<&Expr> for AstNode {
    fn from(value: &Expr) -> Self {
        match &value.node {
            Ident(ident) => {
                AstNode::new(format!("Ident({})", ident), Some(value.span.into()), vec![])
            }
            Lit(literal) => AstNode::new(
                format!("Literal({})", literal),
                Some(value.span.into()),
                vec![],
            ),
            BinOp(bin_op, spanned, spanned1) => AstNode::new(
                format!("BinOp({})", bin_op),
                Some(value.span.into()),
                vec![spanned.as_ref().into(), spanned1.as_ref().into()],
            ),
            Par(spanned) => AstNode::new(
                "Par()",
                Some(value.span.into()),
                vec![spanned.as_ref().into()],
            ),
            Call(ident, arguments) => AstNode::new(
                format!("Call({})", ident),
                Some(value.span.into()),
                vec![arguments.into()],
            ),
            IfThenElse(cond, then_block, else_block) => {
                let mut children = vec![AstNode::from(cond.as_ref()), AstNode::from(then_block)];

                if let Some(else_block) = else_block {
                    children.push(AstNode::from(else_block));
                }

                AstNode::new("IfThenElse", Some(value.span.into()), children)
            }

            super::ExprKind::Block(block) => AstNode::new(
                "BlockExpr",
                Some(value.span.into()),
                vec![AstNode::from(block)],
            ),

            UnOp(un_op, expr) => AstNode::new(
                format!("UnOp({})", un_op),
                Some(value.span.into()),
                vec![AstNode::from(expr.as_ref())],
            ),
        }
    }
}

// For now Parameters are leaves, not the singular parameter.
impl From<&Parameters> for AstNode {
    fn from(value: &Parameters) -> Self {
        AstNode::new(
            format!(
                "Parameters {}",
                value
                    .node
                    .0
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Some(value.span.into()),
            value.node.0.iter().map(AstNode::from).collect(),
        )
    }
}

impl From<&Parameter> for AstNode {
    fn from(value: &Parameter) -> Self {
        AstNode::new(
            format!("Parameter {:?}", value.node),
            Some(value.span.into()),
            vec![],
        )
    }
}

impl From<&Type> for AstNode {
    fn from(value: &Type) -> Self {
        AstNode::new(format!("Type {}", value), None, vec![])
    }
}

impl From<&Arguments> for AstNode {
    fn from(value: &Arguments) -> Self {
        AstNode::new(
            "Args",
            Some(value.span.into()),
            value.node.0.iter().map(AstNode::from).collect(),
        )
    }
}
