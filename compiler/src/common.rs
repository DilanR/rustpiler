use crate::error::Error;

pub trait Eval<T: Clone> {
    fn eval(&self) -> Result<T, Error>
    where
        T: Clone;
}

pub fn parse<T1>(s: &str) -> T1
where
    T1: syn::parse::Parse + std::fmt::Display,
{
    let ts: proc_macro2::TokenStream = s.parse().unwrap();
    let r: T1 = syn::parse2(ts).unwrap();
    println!("{}", r);
    r
}

// emit instructions using Eval trait
use mips::instr::Instr;
pub fn codegen_instrs<T1>(s: &str) -> Result<Vec<Instr>, Error>
where
    T1: syn::parse::Parse + std::fmt::Display + Eval<Vec<Instr>>,
{
    let ast = parse::<T1>(s);
    ast.eval()
}
