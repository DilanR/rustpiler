use serde::Serialize;

use crate::{
    ast::{
        Arguments, Block, Expr, ExprKind::*, FnDeclaration, Parameter, Parameters, Prog, Spanned,
        Statement, StatementKind::*, Type, TypeExpr,
    },
    error::ErrRange,
};
#[derive(Debug, Clone, Copy, Serialize)]
pub enum AstKind {
    Prog,
    Function,
    Parameters,
    Parameter,
    Block,

    Let,
    Assign,
    While,

    Ident,
    Literal,
    Type,

    BinOp,
    UnOp,

    Call,
    Arguments,

    IfThenElse,
    BlockExpr,
    Par,

    Mutable,
}

// Feature for frontend, making Ast svgs on hover of code.
// If span is None assume all subtrees with children None belongs to 'local' Ast
#[derive(Debug, Serialize)]
pub struct AstNode {
    kind: AstKind,
    label: String,
    span: Option<ErrRange>,
    children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(
        kind: AstKind,
        label: impl Into<String>,
        span: Option<ErrRange>,
        children: Vec<AstNode>,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            span,
            children,
        }
    }
}

impl From<&Prog> for AstNode {
    fn from(prog: &Prog) -> Self {
        AstNode::new(
            AstKind::Prog,
            "Prog".to_string(),
            None,
            prog.0.iter().map(AstNode::from).collect(),
        )
    }
}

impl From<&FnDeclaration> for AstNode {
    fn from(value: &FnDeclaration) -> Self {
        let rtype = &value
            .node
            .ty
            .clone()
            .unwrap_or(Spanned::dummy(TypeExpr::Unit));
        AstNode::new(
            AstKind::Function,
            format!("fn {}", value.node.id),
            Some(value.span.into()),
            vec![
                (&value.node.parameters).into(),
                rtype.into(),
                (&value.node.body).into(),
            ],
        )
    }
}

impl From<&Block> for AstNode {
    fn from(value: &Block) -> Self {
        AstNode::new(
            AstKind::Block,
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
                let mut children: Vec<AstNode> = Vec::new();

                children.push(AstNode::new(
                    AstKind::Mutable,
                    format!("Mutable({})", mutable.0),
                    None,
                    vec![],
                ));

                children.push(AstNode::new(
                    AstKind::Ident,
                    format!("Ident({})", name),
                    None,
                    vec![],
                ));

                if let Some(ty) = ty {
                    children.push(AstNode::from(ty));
                }

                if let Some(expr) = expr {
                    children.push(AstNode::from(expr));
                }

                AstNode::new(AstKind::Let, "Let", Some(value.span.into()), children)
            }

            Assign(lhs, rhs) => AstNode::new(
                AstKind::Assign,
                "Assign",
                Some(value.span.into()),
                vec![AstNode::from(lhs), AstNode::from(rhs)],
            ),

            While(cond, body) => AstNode::new(
                AstKind::While,
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
            Ident(ident) => AstNode::new(
                AstKind::Ident,
                format!("Ident({})", ident),
                Some(value.span.into()),
                vec![],
            ),
            Lit(literal) => AstNode::new(
                AstKind::Literal,
                format!("Literal({})", literal),
                Some(value.span.into()),
                vec![],
            ),
            BinOp(bin_op, spanned, spanned1) => AstNode::new(
                AstKind::BinOp,
                format!("BinOp({})", bin_op),
                Some(value.span.into()),
                vec![spanned.as_ref().into(), spanned1.as_ref().into()],
            ),
            Par(spanned) => AstNode::new(
                AstKind::Par,
                "Par()",
                Some(value.span.into()),
                vec![spanned.as_ref().into()],
            ),
            Call(ident, arguments) => AstNode::new(
                AstKind::Call,
                format!("Call({})", ident),
                Some(value.span.into()),
                vec![arguments.into()],
            ),
            IfThenElse(cond, then_block, else_block) => {
                let mut children = vec![AstNode::from(cond.as_ref()), AstNode::from(then_block)];

                if let Some(else_block) = else_block {
                    children.push(AstNode::from(else_block));
                }

                AstNode::new(
                    AstKind::IfThenElse,
                    "IfThenElse",
                    Some(value.span.into()),
                    children,
                )
            }

            super::ExprKind::Block(block) => AstNode::new(
                AstKind::BlockExpr,
                "BlockExpr",
                Some(value.span.into()),
                vec![AstNode::from(block)],
            ),

            UnOp(un_op, expr) => AstNode::new(
                AstKind::UnOp,
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
            AstKind::Parameters,
            "Parameters",
            Some(value.span.into()),
            value.node.0.iter().map(AstNode::from).collect(),
        )
    }
}

impl From<&Parameter> for AstNode {
    fn from(value: &Parameter) -> Self {
        AstNode::new(
            AstKind::Parameter,
            value.to_string(),
            Some(value.span.into()),
            vec![],
        )
    }
}

impl From<&Type> for AstNode {
    fn from(value: &Type) -> Self {
        AstNode::new(
            AstKind::Type,
            format!("Type {}", value.node),
            Some(value.span.into()),
            vec![],
        )
    }
}

impl From<&Arguments> for AstNode {
    fn from(value: &Arguments) -> Self {
        AstNode::new(
            AstKind::Arguments,
            "Args",
            Some(value.span.into()),
            value.node.0.iter().map(AstNode::from).collect(),
        )
    }
}
