# 上手使用
cargo会在安装rust时一同安装

**使用cargo创建二进制项目**，二进制意味着该项目可以作为一个服务运行或被编译成可执行文件运行
```bash
cargo new hello_world
```
我们使用`cargo new`创建了一个项目，这个命令实际上等价于`cargo new hello_world --bin`， `bin`是`binary`的简写，代表着二进制程序，是默认参数，所以可以省略

创建成功后，默认的项目目录结构结构：
```
├── Cargo.toml
└── src
    └── main.rs
```
其中的`cargo.toml`是`cargo`的配置文件：
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```
使用`cargo build`编译项目：
```bash
$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```
编译结果：
```bash
$ ./target/debug/hello_world
Hello, world!
```
默认编译是`debug`模式，生产编译需要使用`cargo build --release`，开启`release`模式
如果想编译后直接运行，可以使用`cargo run`,`cargo run --release`



# 基础指南
## 为何会有cargo
Rust有两种类型的包：**库包和二进制包**，前者是我们常说的依赖包，用于被其他包引入，而后者是一个应用服务，可以编译成二进制可执行文件运行
包是通过Rust编译器`rustc`进行编译的；
```bash
rustc hello.rs
``` 
直接使用编译器虽然简单但是有几个问题：
- 必须指定文件名编译，当项目复杂后，这种方式也随之更加复杂
- 如果需要指定编译参数，将更加复杂
- 外部依赖的引入也是一个问题

cargo解决了这些问题，它主要做了以下四件事：
- 引入两个元数据文件，包含项目的方方面面信息: Cargo.toml 和 Cargo.lock
- 获取和构建项目的依赖，例如 Cargo.toml 中的依赖包版本描述，以及从 crates.io 下载包
- 调用 rustc (或其它编译器) 并使用的正确的参数来构建项目，例如 cargo build
- 引入一些惯例，让项目的使用更加简单

## 下载并构建 Package
如果看中GitHub上的某个项目，那么下载构建它都是非常简单的
```bash
$ git clone https://github.com/rust-lang/regex.git
$ cd regex

cargo build
```
该命令将下载相关的依赖库，等下载成功后，再对 package 和下载的依赖进行一同的编译构建。
这就是包管理工具的强大之处，cargo build 搞定一切，而背后隐藏的复杂配置、参数你都无需关心。


## 添加依赖
在`cargo.toml`中的`[dependencies]`中添加你需要的依赖和依赖的其他参数
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
time = "0.1.12"
regex = "0.1.41"
```
然后`cargo build`就会添加依赖
或者直接`cargo add time`


## 标准的 Package 目录结构
```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```
这也是 Cargo 推荐的目录结构，解释如下:
- Cargo.toml 和 Cargo.lock 保存在 package 根目录下
- 源代码放在 src 目录下
- 默认的 lib 包根是 src/lib.rs
- 默认的二进制包根是 src/main.rs
  - 其它二进制包根放在 src/bin/ 目录下
- 基准测试 benchmark 放在 benches 目录下
- 示例代码放在 examples 目录下
- 集成测试代码放在 tests 目录下