## Rust 宏是什么？
Rust 对宏（macro）有着非常好的支持。宏能够使得你能够通过写代码的方式来生成代码，这通常被称为元编程（metaprogramming）。
宏提供了类似函数的功能，但是没有运行时开销。但是，因为宏会在编译期进行展开（expand），所以它会有一些编译期的开销。
Rust 宏非常不同于 C 里面的宏。Rust 宏会被应用于词法树（token tree），而 C 语言里的宏则是文本替换。

## Rust 宏的类型
Rust 有两种类型的宏：
声明式宏（Declarative macros）使得你能够写出类似 match 表达式的东西，来操作你所提供的 Rust 代码。它使用你提供的代码来生成用于替换宏调用的代码。
过程宏（Procedural macros）允许你操作给定 Rust 代码的抽象语法树（abstract syntax tree, AST）。过程宏是从一个（或者两个）TokenStream到另一个TokenStream的函数，用输出的结果来替换宏调用。
