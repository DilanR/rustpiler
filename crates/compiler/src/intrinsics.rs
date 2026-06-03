use crate::ast::{
    Block, BlockKind, FnDeclarationKind, Mutable, Parameter, ParameterKind, Parameters,
    ParametersKind, Spanned, Type,
};
use proc_macro2::Span;
use regex::Regex;
// Implementation of intrinsics for the vm
use crate::ast::Literal;
pub type Intrinsic = fn(Vec<Literal>, &mut String) -> Literal;
pub fn vm_println() -> (FnDeclarationKind, Intrinsic) {
    (
        FnDeclarationKind {
            id: "println!".to_string(),
            parameters: Parameters::new(
                ParametersKind(vec![
                    Spanned::dummy(ParameterKind {
                        mutable: Mutable(false),
                        id: "str".to_string(),
                        ty: Type::String,
                    }),
                    Spanned::dummy(ParameterKind {
                        mutable: Mutable(false),
                        id: "i".to_string(),
                        ty: Type::I32,
                    }),
                ]),
                Span::call_site(),
            ),
            ty: None,
            body: Block {
                node: BlockKind {
                    statements: vec![],
                    semi: false,
                },
                span: Span::call_site(),
            },
        },
        |lit_vec, stdout| {
            match &lit_vec[0] {
                Literal::String(s) => {
                    // this regex will find either '{}' or '{:?}'
                    let re = Regex::new(r"\{(:\?)?\}").unwrap();

                    // we split at these points
                    let split = re.split(s);
                    // and collect into vector
                    let vec: Vec<&str> = split.collect();

                    let mut output = String::new();

                    // first print the leading part
                    print!("{}", vec[0]);
                    output.push_str(vec[0]);
                    // then print each matching pair
                    // the value followed by the trailing part
                    for (text, lit) in vec[1..].iter().zip(lit_vec[1..].iter()) {
                        print!("{}{}", lit, text);
                        output.push_str(&lit.to_string());
                        output.push_str(text);
                    }

                    output.push('\n');
                    stdout.push_str(&output);

                    println!();
                }
                _ => panic!("ICE - no formatting string in println!"),
            }
            Literal::Unit
        },
    )
}

#[test]
fn regex_test() {
    // this regex will find either '{}' or '{:?}'
    let re = Regex::new(r"\{(:\?)?\}").unwrap();

    // we split at these points
    let split = re.split("a {} b {:?} c");

    // and collect into vector
    let vec: Vec<&str> = split.collect();
    println!("{:?}", vec);
}
