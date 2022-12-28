# Rust 中的过程宏
过程宏（Procedural macros）是一种更为高级的宏。过程宏能够扩展 Rust 的现有语法。它接收任意输入并产生有效的 Rust 代码。
过程宏接收一个TokenStream作为参数并返回另一个TokenStream。过程宏对输入的TokenStream进行操作并产生一个输出。有三种类型的过程宏：
1. 属性式宏（Attribute-like macros）    用在结构体、字段、函数等地方，为其指定属性等功能。如标准库中的#[inline]、#[derive(...)]等都是属性宏。
2. 派生宏（Derive macros）  用于结构体（struct）、枚举（enum）、联合（union）类型，可为其实现函数或特征（Trait）。
3. 函数式宏（Function-like macros） 用法与普通的规则宏类似，但功能更加强大，可实现任意语法树层面的转换功能。
接下来我们将会对它们进行详细讨论。

## 属性式宏
属性式宏能够让你创建一个自定义的属性，该属性将其自身关联一个项（item），并允许对该项进行操作。它也可以接收参数。
```rust
#[some_attribute_macro(some_argument)]
fn perform_task(){
// some code
}
```
在上面的代码中，`some_attribute_macros`是一个属性宏，它对函数`perform_task`进行操作。
为了编写一个属性式宏，我们先用`cargo new macro-demo --lib`来创建一个项目。创建完成后，修改`Cargo.toml`来通知 `cargo`，该项目将会创建过程宏。
```toml
# Cargo.toml
[lib]
proc-macro = true
```
过程宏是公开的函数，接收`TokenStream`作为参数并返回另一个`TokenStream`。要想写一个过程宏，我们需要先实现能够解析`TokenStream`的解析器。`Rust` 社区已经有了很好的 `crate——syn`，用于解析`TokenStream`。
`syn`提供了一个现成的 `Rust` 语法解析器能够用于解析`TokenStream`。你可以通过组合`syn`提供的底层解析器来解析你自己的语法、
把`syn`和`quote`添加到`Cargo.toml`。
```toml
# Cargo.toml
[dependencies]
syn = {version="1.0.57",features=["full","fold"]}
quote = "1.0.8"
```

现在我们可以使用`proc_macro`在`lib.rs`中写一个属性式宏，`proc_macro`是编译器提供的用于写过程宏的一个 `crate`。对于一个过程宏 `crate`，除了过程宏外，不能导出其他任何东西，`crate` 中定义的过程宏不能在 `crate` 自身中使用。
```rust
// lib.rs
extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};

// using proc_macro_attribute to declare an attribute like procedural macro
#[proc_macro_attribute]
// _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    // returing a simple TokenStream for Struct
    TokenStream::from(quote!{struct H{}})
}
```
为了测试我们添加的宏，我们需要创建一个测试。创建一个名为`tests`的文件夹然后在该文件夹添加文件`attribute_macro.rs`。在这个文件中，我们可以测试我们的属性式宏。
```rust
// tests/attribute_macro.rs

use macro_demo::*;

// macro converts struct S to struct H
#[my_custom_attribute]
struct S{}

#[test]
fn test_macro(){
// due to macro we have struct H in scope
    let demo=H{};
}
```
使用命令`cargo test`来运行上面的测试。

现在，我们理解了过程宏的基本使用，让我们用`syn`来对`TokenStream`进行一些高级操作和解析。
为了理解`syn`是如何用来解析和操作的，让我们来看`syn Github` 仓库上的一个示例。这个示例创建了一个 Rust 宏，这个宏可以追踪变量值的变化。

首先，我们需要去验证，我们的宏是如何操作与其所关联的代码的
```rust
#[trace_vars(a)]
fn do_something(){
  let a=9;
  a=6;
  a=0;
}
```
`trace_vars`宏获取它所要追踪的变量名，然后每当输入变量（也就是`a`）的值发生变化时注入一条打印语句。这样它就可以追踪输入变量的值了。

首先，解析属性式宏所关联的代码。`syn`提供了一个适用于 `Rust` 函数语法的内置解析器。`ItemFn`将会解析函数，并且如果语法无效，它会抛出一个错误。
```rust
#[proc_macro_attribute]
pub fn trace_vars(_metadata: TokenStream, input: TokenStream) -> TokenStream {
// parsing rust function to easy to use struct
    let input_fn = parse_macro_input!(input as ItemFn);
    TokenStream::from(quote!{fn dummy(){}})
}
```

现在我们已经解析了`input`，让我们开始转移到`metadata`。对于`metadata`，没有适用的内置解析器，所以我们必须自己使用`syn`的`parse`模块写一个解析器。

要想`syn`能够工作，我们需要实现`syn`提供的`Parse trait`。`Punctuated`用于创建一个由,分割`Indent的vector`。
```rust
struct Args{
    vars:HashSet<Ident>
}

impl Parse for Args{
    fn parse(input: ParseStream) -> Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}
```

一旦我们实现`Parse trait`，我们就可以使用`parse_macro_input`宏来解析`metadata`。
```rust
#[proc_macro_attribute]
pub fn trace_vars(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    // using newly created struct Args
    let args= parse_macro_input!(metadata as Args);
    TokenStream::from(quote!{fn dummy(){}})
}
```
现在，我们准备修改`input_fn`以便于在当变量值变化时添加`println!`。为了完成这项修改，我们需要过滤出有复制语句的代码，并在那行代码之后插入一个 `print` 语句。
```rust
impl Args {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
 // variable shouldn't start wiht ::
                if e.path.leading_colon.is_some() {
                    false
// should be a single variable like `x=8` not n::x=0
                } else if e.path.segments.len() != 1 {
                    false
                } else {
// get the first part
                    let first = e.path.segments.first().unwrap();
// check if the variable name is in the Args.vars hashset
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

// used for checking if to print let i=0 etc or not
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
// check if variable name is present in set
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

// manipulate tree to insert print statement
    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
 // recurive call on right of the assigment statement
        let right = fold::fold_expr(self, right);
// returning manipulated sub-tree
        parse_quote!({
            #left #op #right;
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        })
    }

// manipulating let statement
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
// get the variable name of assigned variable
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
// new sub tree
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}
```
在上面的示例中，`quote`宏用于模板化和生成 Rust 代码。#用于注入变量的值。

现在，我们将会在`input_fn`上进行 `DFS`，并插入 `print` 语句。`syn`提供了一个`Fold trait` 可以用来对任意`Item`实现 `DFS`。我们只需要修改与我们想要操作的 `token` 类型所对应的 `trait` 方法。
```rust
impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
// for changing assignment like a=5
            Expr::Assign(e) => {
// check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
// continue with default travesal using default methods
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
// for changing assigment and operation like a+=1
            Expr::AssignOp(e) => {
// check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
// continue with default behaviour
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
// continue with default behaviour for rest of expressions
            _ => fold::fold_expr(self, e),
        }
    }

// for let statements like let d=9
    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}
```
现在我们可以使用`fold_item_fn`在我们解析的代码中注入 `print` 语句。
```rust
#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
// parse the input
    let input = parse_macro_input!(input as ItemFn);
// parse the arguments
    let mut args = parse_macro_input!(args as Args);
// create the ouput
    let output = args.fold_item_fn(input);
// return the TokenStream
    TokenStream::from(quote!(#output))
}
```
这个代码示例来自于syn 示例仓库，该仓库也是关于过程宏的一个非常好的学习资源。


## 派生宏

## 类函数宏


[过程宏](https://zhuanlan.zhihu.com/p/356427780)
[派生宏辅助属性](https://www.linuxzen.com/notes/notes/20210616141500-rust_%E5%B1%9E%E6%80%A7%E5%AE%8F%E8%A7%A3%E6%9E%90/)