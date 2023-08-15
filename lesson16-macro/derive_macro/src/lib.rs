extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 结构体名称
    let name = &ast.ident;

    // 检查结构体字段
    let fields = if let Data::Struct(data) = &ast.data {
        &data.fields
    } else {
        panic!("HelloMacro can only be derived for structs.");
    };

    // 检查结构体字段数量
    let field_count = match fields {
        Fields::Named(fields) => fields.named.len(),
        Fields::Unnamed(fields) => fields.unnamed.len(),
        Fields::Unit => 0,
    };

    println!("aaa ");

    // 解析属性值
    let value = parse_attribute_value(&ast.attrs);

    // 生成代码
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro(&self) {
                println!("Hello, {}!", stringify!(#name));
                println!("Field count: {}", #field_count);

                // 访问带有 #[hello(value = 100)] 属性的字段
                let field_value = #value;
                println!("Field data: {}", field_value);
            }
        }
    };

    // 返回生成的代码作为 TokenStream
    proc_macro::TokenStream::from(expanded)
}

// 解析属性值
fn parse_attribute_value(attrs: &[syn::Attribute]) -> Lit {
    for attr in attrs {
        if attr.path.is_ident("hello") {
            if let Some(meta) = attr.parse_meta().ok() {
                if let Meta::List(list) = meta {
                    for nested_meta in list.nested {
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested_meta {
                            if name_value.path.is_ident("value") {
                                if let Lit::Int(value) = name_value.lit {
                                    return syn::Lit::Int(value);
                                } else {
                                    panic!("Attribute value must be an integer literal.");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Cannot find attribute hello(value = X)");
}

#[proc_macro_derive(HelloMacro, attributes(hello))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(&ast)
}
