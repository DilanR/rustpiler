use crate::ast::{
    Arguments, BinOp, Block, BlockKind, Expr, ExprKind, FnDeclaration, FnDeclarationKind, Literal,
    Mutable, Parameter, Parameters, Prog, Spanned, Statement, StatementKind, Type, UnOp,
};
use syn::{
    Error, Ident, Result, Token,
    parse::{Parse, ParseStream},
    token::{self},
};

/// A small helper function for parsing source strings.
pub fn parse<T: Parse>(src: &str) -> T {
    try_parse::<T>(src).unwrap()
}

/// A small helper function for parsing source strings.
pub fn try_parse<T: Parse>(src: &str) -> Result<T> {
    let ts: proc_macro2::TokenStream = src.parse()?;
    syn::parse2::<T>(ts)
}

/// Might be useful if you are struggling with parsing something and you want to see what the
/// tokens are that syn produces.
pub fn try_parse_debug<T: Parse + std::fmt::Debug>(src: &str) -> Result<T> {
    println!("parsing source string:\n{}", src);
    let ts: proc_macro2::TokenStream = src.parse()?;
    println!("tokens:\n{}", ts);
    let result = syn::parse2::<T>(ts);
    println!("parsed AST:\n{:?}", result);
    result
}

// Back-port your parser
// You may want to put the tests in a module.
// See e.g., the vm.rs

impl Parse for Literal {
    fn parse(input: ParseStream) -> Result<Self> {
        // Use the "built in" syn parser for literals
        if input.peek(syn::token::Paren) {
            let content;
            let _ = syn::parenthesized!(content in input);
            if content.is_empty() {
                return Ok(Literal::Unit);
            } else {
                let _e: Expr = content.parse()?;
                return Err(Error::new(
                    content.span(),
                    "expected `()` (unit), found expression",
                ));
            }
        }

        let lit: syn::Lit = input.parse()?;

        let lit = match lit {
            syn::Lit::Int(n) => Literal::Int(n.base10_parse().unwrap()),
            syn::Lit::Bool(b) => Literal::Bool(b.value),
            // for now only Int and Bool are covered
            syn::Lit::Str(s) => Literal::String(s.value()),

            _ => unimplemented!(),
        };
        Ok(lit)
    }
}

impl Parse for BinOp {
    fn parse(input: ParseStream) -> Result<Self> {
        // check if next token is `+`
        if input.peek(Token![+]) {
            // consume the token
            let _: Token![+] = input.parse()?;
            Ok(BinOp::Add)
        } else if input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            Ok(BinOp::Sub)
        } else if input.peek(Token![*]) {
            let _: Token![*] = input.parse()?;
            Ok(BinOp::Mul)
        } else if input.peek(Token![/]) {
            let _: Token![/] = input.parse()?;
            Ok(BinOp::Div)
        } else if input.peek(Token![&&]) {
            let _: Token![&&] = input.parse()?;
            Ok(BinOp::And)
        } else if input.peek(Token![||]) {
            let _: Token![||] = input.parse()?;
            Ok(BinOp::Or)
        } else if input.peek(Token![==]) {
            let _: Token![==] = input.parse()?;
            Ok(BinOp::Eq)
        } else if input.peek(Token![<]) {
            let _: Token![<] = input.parse()?;
            Ok(BinOp::Lt)
        } else if input.peek(Token![>]) {
            let _: Token![>] = input.parse()?;
            Ok(BinOp::Gt)
        } else {
            // to explicitly create an error at the current position
            input.step(|cursor| Err(cursor.error("expected operator")))
        }
    }
}

impl Parse for UnOp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![-]) {
            let _: Token![-] = input.parse()?;
            Ok(UnOp::Neg)
        } else if input.peek(Token![!]) {
            let _: Token![!] = input.parse()?;
            Ok(UnOp::Bang)
        } else {
            // to explicitly create an error at the current position
            input.step(|cursor| Err(cursor.error("expected unary operator")))
        }
    }
}

use proc_macro2::Span;

fn expr(start: Span, end: Span, kind: ExprKind) -> Expr {
    Spanned {
        node: kind,
        span: start.join(end).unwrap_or(start),
    }
}
impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the first part of the expression.
        let expr = {
            let left = parse_operand(input)?;
            // Now check if the rest is an Op Expr...
            if peek_op(input) {
                // In that case, we have to parse the rest of the expression.
                parse_binary_op_expr(input, left, 0)?
            } else {
                // Otherwise, the first part was the whole expression.
                left
            }
        };

        Ok(expr)
    }
}

// NOTE: About `peek_op` and `peek_prio`:
// We need to be able to look ahead at the next operator, but without parsing an
// Op and thereby consuming its tokens. I have not found a good way to either do
// something like `input.peek(Op)` or `input.unread(op)` when using syn.
// So forking and parsing is the most concise solution I could find. /chrfin

/// Check if the next token is some (binary) operator.
fn peek_op(input: ParseStream) -> bool {
    input.fork().parse::<BinOp>().is_ok()
}

/// Get the priority of the operator ahead. Assumes there is one!
fn peek_prio(input: ParseStream) -> u8 {
    input.fork().parse::<BinOp>().unwrap().priority()
}

/// Parse what could be an operand, i.e. the first part of a binary expression.
/// This could be a literal, an identifier, a unary op, or an expression in parentheses.
/// For example: `3 + ...`, `x + ...`, `!true && ...`, `(1+2) + ...`, or `[1,2,3][0] + ...`.
/// No point in spanning operands for now
fn parse_operand(input: ParseStream) -> Result<Expr> {
    let start = input.span();

    let result = if input.peek(syn::token::Paren) {
        let content;
        let _ = syn::parenthesized!(content in input);

        if content.is_empty() {
            ExprKind::Lit(Literal::Unit)
        } else {
            let e: Expr = content.parse()?;
            ExprKind::Par(Box::new(e))
        }
    } else if input.peek(Token![-]) || input.peek(Token![!]) {
        let op: UnOp = input.parse()?;
        let e = parse_operand(input)?;
        ExprKind::UnOp(op, Box::new(e))
    } else if input.peek(Ident) {
        return parse_ident_or_call(input);
    } else if input.peek(token::Brace) {
        ExprKind::Block(input.parse::<Block>()?)
    } else if input.peek(Token![if]) {
        let _: Token![if] = input.parse()?;
        let condition: Expr = input.parse()?;
        let then_block: Block = input.parse::<Block>()?;

        let opt_block = if input.peek(Token![else]) {
            let _: Token![else] = input.parse()?;
            if input.peek(Token![if]) {
                let else_block: Expr = input.parse()?;
                Some(else_block.into())
            } else {
                Some(input.parse::<Block>()?)
            }
        } else {
            None
        };

        ExprKind::IfThenElse(Box::new(condition), then_block, opt_block)
    } else if input.peek(syn::Lit) {
        let lit: Literal = input.parse()?;
        ExprKind::Lit(lit)
    } else {
        return Err(input.error("Invalid operand!"));
    };

    let end = input.span();
    Ok(expr(start, end, result))
}

/// Parse an expression consisting of binary operators, such as `1 + 2`, `1 + 2 + 3`,
/// `1 + 2 * 3 + 4`, or `(1 + 2) * (2 + 3)`.
/// To be more specific: given some beginning part of an expression (in `left`), parse the
/// remainder of the expression, starting with the next operator. For example, `left` might be
/// `1` or `1 + 2`, and the input might be `+ 2` or `+ 2 + 3`, etc.
/// The priority (or precedence) of operators is taken into account during parsing
/// so we get the correct AST.
fn parse_binary_op_expr(input: ParseStream, left: Expr, min_prio: u8) -> Result<Expr> {
    // peek next operator return left if precedence lower then min or no op
    if peek_op(input) {
        if peek_prio(input) < min_prio {
            return Ok(left);
        }
    } else {
        return Ok(left);
    };

    // safe to consume op
    let op = input.parse::<BinOp>()?; // NOTE: Unary operations will be handled in parse_operand.

    // next token needs to be operand or the statement is invalid
    let right = parse_operand(input)?;
    let op_prio = op.priority();
    // set next_min to ensure only higher precedence can interrupt
    // leftside accumilation
    let next_min = op_prio + 1;

    // recursively parse any operators that bind tighter on the right side
    let right = parse_binary_op_expr(input, right, next_min)?;

    // combine and continue parsing more operators at or above min_prio
    let new_left = expr(left.span, right.span, ExprKind::bin_op(op, left, right));
    parse_binary_op_expr(input, new_left, min_prio)
}

fn parse_ident_or_call(input: ParseStream) -> Result<Expr> {
    let start = input.span();

    let kind = match input.parse::<Ident>() {
        Ok(identifier) => {
            // macro call
            if input.peek(Token![!]) {
                let _: Token![!] = input.parse()?;
                let args: Arguments = input.parse()?;
                ExprKind::Call(format!("{}!", identifier), args)
            }
            // function call
            else if input.peek(syn::token::Paren) {
                let args: Arguments = input.parse()?;
                ExprKind::Call(identifier.to_string(), args)
            } else {
                ExprKind::Ident(identifier.to_string())
            }
        }
        Err(e) => return Err(e),
    };

    let end = input.span();

    Ok(expr(start, end, kind))
}

use quote::quote;

impl Parse for Type {
    fn parse(input: ParseStream) -> Result<Type> {
        // The syn::Type is very complex and overkill
        // Types in Rust involve generics, paths
        // etc., etc., etc. ...
        //
        // To make things simple, we just turn the syn::Type
        // to a token stream (`quote`) and turn that into a String
        // and turn that into an &str (`as_str`)

        let start = input.span();
        let syn_type: syn::Type = input.parse()?;
        let token_stream = quote!(#syn_type);
        let token_string = token_stream.to_string();

        let ty = match token_string.as_str() {
            "i32" => Type::I32,
            "bool" => Type::Bool,
            "String" => Type::String,
            "()" => Type::Unit,
            other => {
                return Err(Error::new(start, format!("Unsupported Type {}", other)));
            }
        };
        Ok(ty)
    }
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Arguments> {
        let content;
        syn::parenthesized!(content in input);

        let args: Punctuated<Expr, Token![,]> = content.parse_terminated(Expr::parse, Token![,])?;

        Ok(Arguments(args.into_iter().collect()))
    }
}

impl Parse for Parameter {
    fn parse(input: ParseStream) -> Result<Parameter> {
        let mut mutable_param = false;
        if input.peek(syn::token::Mut) {
            let _: syn::token::Mut = input.parse()?;
            mutable_param = true;
        }

        //Expecting Identifier
        let identifier: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let ty: Type = input.parse()?;

        Ok(Parameter {
            mutable: Mutable(mutable_param),
            id: identifier.to_string(),
            ty,
        })
    }
}

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Parameters {
    fn parse(input: ParseStream) -> Result<Parameters> {
        let content;
        let _ = syn::parenthesized!(content in input);
        if content.is_empty() {
            Ok(Parameters(vec![]))
        } else {
            let mut params: Vec<Parameter> = vec![];
            // Not empty should have atleast 1 param
            params.push(content.parse::<Parameter>()?);

            while !content.is_empty() {
                let _: syn::token::Comma = content.parse()?;
                if !content.is_empty() {
                    let param: Parameter = content.parse()?;
                    params.push(param);
                }
            }
            // params can have extra comma for last argument
            if content.peek(syn::token::Comma) {
                let _: syn::token::Comma = content.parse()?;
            }
            Ok(Parameters(params))
        }
    }
}

impl Parse for FnDeclaration {
    fn parse(input: ParseStream) -> Result<FnDeclaration> {
        let start = input.span();

        let _: syn::token::Fn = input.parse()?;
        let identifier: syn::Ident = input.parse()?;
        let parameters: Parameters = input.parse()?;
        let ty = if input.peek(syn::token::RArrow) {
            let _: syn::token::RArrow = input.parse()?;
            let rtype: Type = input.parse()?;
            Some(rtype)
        } else {
            None
        };

        let body: Block = input.parse()?;

        Ok(Spanned::new(
            FnDeclarationKind {
                id: identifier.to_string(),
                parameters,
                ty,
                body: body.clone(),
            },
            start.join(body.span).unwrap_or(start),
        ))
    }
}

//#[derive(Debug, Clone, PartialEq)]
//pub enum Statement {
//    Let(Mutable, String, Option<Type>, Option<Expr>),
//    Assign(Expr, Expr),
//    While(Expr, Block),
//    Expr(Expr),
//    Fn(FnDeclaration),
//}

impl Parse for Statement {
    fn parse(input: ParseStream) -> Result<Statement> {
        let start = input.span();

        let kind = {
            if input.peek(syn::token::Let) {
                let _: syn::token::Let = input.parse()?;
                let mutable = if input.peek(syn::token::Mut) {
                    let _: syn::token::Mut = input.parse()?;
                    Mutable(true)
                } else {
                    Mutable(false)
                };

                let identifier: Ident = input.parse()?;

                let ty: Option<Type> = if input.peek(syn::token::Colon) {
                    let _: syn::token::Colon = input.parse()?;
                    Some(input.parse::<Type>()?)
                } else {
                    None
                };

                let expr: Option<Expr> = if input.peek(syn::token::Eq) {
                    let _: syn::token::Eq = input.parse()?;
                    Some(input.parse::<Expr>()?)
                } else {
                    None
                };

                Ok(StatementKind::Let(
                    mutable,
                    identifier.to_string(),
                    ty,
                    expr,
                ))
            } else if input.peek(syn::token::While) {
                let _: syn::token::While = input.parse()?;
                let condition: Expr = input.parse()?;
                let block: Block = input.parse()?;
                Ok(StatementKind::While(condition, block))
            } else if input.peek(token::Fn) {
                Ok(StatementKind::Fn(input.parse::<FnDeclaration>()?))
            } else {
                // Expecting Assign or Expr
                let lhs: Expr = input.parse()?;
                if input.peek(syn::token::Eq) {
                    let _: syn::token::Eq = input.parse()?;
                    Ok(StatementKind::Assign(lhs, input.parse::<Expr>()?))
                } else {
                    Ok(StatementKind::Expr(lhs))
                }
            }
        };

        let end = input.span();
        match kind {
            Ok(k) => Ok(Spanned::new(k, start.join(end).unwrap_or(start))),
            Err(e) => Err(e),
        }
    }
}

use syn::punctuated::Punctuated;

// Here we take advantage of the parser function `parse_terminated`
//impl Parse for Block {
//    fn parse(input: ParseStream) -> Result<Block> {
//        let content;
//        let _ = syn::braced!(content in input);
//
//        let bl: Punctuated<Statement, Token![;]> =
//            content.parse_terminated(Statement::parse, Token![;])?;
//
//        // We need to retrieve the semi before we collect into a vector
//        // as into_iter consumes the value.
//        let semi = bl.trailing_punct();
//
//        Ok(Block {
//            // turn the Punctuated into a vector
//            statements: bl.into_iter().collect(),
//            semi,
//        })
//    }
//}

// Here we take advantage of the parser function `parse_terminated`
impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Block> {
        let start = input.span();

        let content;
        syn::braced!(content in input);

        let mut statements = Vec::new();
        let mut semi = false;

        while !content.is_empty() {
            let stmt: Statement = content.parse()?;

            let is_last = content.is_empty();

            let requires_semi = match &stmt.node {
                StatementKind::Let(..) => true,
                StatementKind::Assign(..) => !is_last,
                StatementKind::Expr(e) => match &e.node {
                    ExprKind::Block(_e) => false,
                    _ => !is_last,
                },
                StatementKind::While(..) | StatementKind::Fn(..) => false,
            };

            statements.push(stmt);

            if content.peek(Token![;]) {
                let _: Token![;] = content.parse()?;
                semi = true;
            } else if requires_semi {
                return Err(content.error("expected ;"));
            } else {
                semi = false;
            }
        }

        let end = input.span();

        Ok(Spanned::new(
            BlockKind { statements, semi },
            start.join(end).unwrap_or(start),
        ))
    }
}
impl Parse for Prog {
    fn parse(input: ParseStream) -> Result<Prog> {
        let mut fns: Vec<FnDeclaration> = vec![];
        fns.push(input.parse::<FnDeclaration>()?);
        while !input.is_empty() {
            fns.push(input.parse::<FnDeclaration>()?);
        }
        Ok(Prog(fns))
    }
}
