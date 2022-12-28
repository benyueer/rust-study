use attribute_macros::trace_var;
use quote::{quote, ToTokens};
use syn::{parse_quote, Stmt};
use core::fmt::Debug;

#[trace_var(n, p)]
fn factorial(mut n: u64) -> u64 {
    let mut p = 1;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}


#[test]
fn test_macro() {
    // due to macro we have struct H in scope
    println!("{}", factorial(8));
}

#[test]
fn parse() {
    let name = quote!(v);
    let ty = quote!(u8);

    let stmt: Stmt = parse_quote! {
        let #name: #ty = Default::default();
    };

    println!("{:#?}", stmt.to_token_stream());
}