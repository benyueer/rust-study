# RUST标准库体系概述
RUST标准库体系分为三个模块：语言核心库--core; alloc库；用户态 std库。

## core 库
RUST语言核心库，适用于操作系统内核及用户态，包括RUST的基础类型，基本Trait, 类型行为函数，其他函数等内容。core库是硬件架构和操作系统无关的可移植库。主要内容：

### 编译器内置intrinsics函数
包括内存操作函数，算数函数，位操作函数，原子变量操作函数等， 这些函数通常与CPU硬件架构紧密相关，且一般需要汇编来提供最佳性能。 intrinsic函数实际上也是对CPU指令的屏蔽层。

### 基本Trait
[【完整】Rust 标准库 Trait 指南](https://rustmagazine.github.io/rust_magazine_2021/chapter_7/rusts-standard-library-traits.html)介绍了标准库 Trait

core 库中的 Trait：
1. 运算符（opt）Trait
   主要是各种用于表达式的RUST符号重载，包括算数计算符号，逻辑运算符号，位操作符号，解引用(*)符号, \[index\]数组下标符号
   `../start..end/start../start..=end/..end/..=end` `Range`符号
   `?`号，`||{..}`闭包符号等，RUST原则是所有的运算符号都要能重载, 所以所有运算操作都定义了重载 Trait。
2. 编译器内部实现的派生宏 Trait
   如果类型结构中的每一个变量都实现了该Trait, 则此结构的该Trait可通过派生宏实现.
   `Clone, Copy`: Copy浅复制，Clone提供深复制.
   `Debug`: 类型的格式化输出.
   `Default`: 类型的default值
   `Eq, Ord，PartialEQ, PartialOrd`: 实现后可以对类型的变量做大,小,相等比较.
   `Sync, Send`: 实现此Trait的类型变量的引用可以安全在线程间共享.
   `Hash`: 实现结构的整体Hash值，这个Trait Hash是因为复杂才被加入，意义没有前面的大
3. Iterator
   迭代器，之后介绍
4. 类型转换Trait
   `AsRef， AsMut, From，Into，TryFrom，TryInto, FloatToInt, FromStr`
5. 字符串Trait
6. 异步编程Trait
7. 内存相关Trait
  
### 基本数据类型
包括整数类型，浮点类型，布尔类型，字符类型，单元类型，内容主要是实现运算符Trait, 类型转换Trait, 派生宏Trait等，字符类型包括对unicode，ascii的不同编码的处理。整数类型有大小端变换的处理。

### 数组、切片以及Range
主要为类型结构对Iterator Trait, 运算符Trait, 类型转换Trait, 派生宏Trait的实现。

### `Option/Result/Marker`等关键的语言级别`Enum`类型
RUST安全特性的重点

### RUST内存相关类型及内容
`alloc, mem, ptr`等模块，RUST的内存操作，后继章节重点详述。

### RUST字符串相关库
字符串`str，string，fmt, panic, debug, log`等

### RUST时间库
`Duration`等

### `alloc`库
`alloc`库主要实现需要进行动态堆内存申请的智能指针类型，集合类型及他们的行为，函数，Trait等内容，仅建立在core库模块之上。std会对alloc模块库的内容做重新的封装。
`alloc`库适用于操作系统内核及用户态程序。 包括： 
1. 基本内存申请；Allocator Trait; Allocator的实现结构Global
2. 基础智能指针：Box, Rc, 
3. 动态数组内存类型: RawVec, Vec 
4. 字符串类型：&str, String 
5. 并发编程指针类型: Arc 
6. 指针内访问类型: Cell, RefCell 还有些其他类型，一般仅在标准库内部使用，后文在需要的时候再介绍及分析。

### `std`库
`std`是在操作系统支撑下运行的只适用于用户态程序的库，`core`库实现的内容基本在`std`库也有对应的实现。
其他内容主要是将操作系统系统调用封装为适合rust特征的结构和Trait,包括： 
1. 进程，线程库 
2. 网络库 
3. 文件操作库 
4. 环境变量及参数 
5. 互斥与同步库，读写锁 
6. 定时器 
7. 输入输出的数据结构， 
8. 系统事件，对epoll,kevent等的封装 
可以将std库看做基本常用的容器类型及操作系统封装库。

# RUST泛型小议
RUST是一门生存在泛型的基础之上的语言。其他语言不使用泛型也不影响编程，泛型只是一个语法中的强大工具。与之相对，RUST离开泛型就无法编写程序，泛型与语法共生。

## 直接针对泛型的方法和trait实现
其他语言的泛型，是作为类型结构体成员，或是函数的输入/返回参数出现在代码中，是配角。RUST的泛型则可以作为主角，可以直接对泛型实现方法和trait。如：
```rs
//T:?Sized是所有的类型， 不带约束的T实际是 T:Sized
//即类型内存空间固定，所以 T:?Sized才是全部的类型
impl<T: ?Sized> Borrow<T> for T {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> BorrowMut<T> for T {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
```
以上代码对所有的类型都实现了Borrow的trait。

直接针对泛型做方法和trait的实现是强大的工具，它的作用：
1. 针对泛型的代码会更内聚，方法总比函数具备更明显的模块性
2. 逻辑更清晰及系统化更好
3. 具备更好的可扩展性
4. 更好的支持函数式编程

## 泛型的层次关系
RUST的泛型从一般到特殊会形成一种层次结构，有些类似于面对对象的基类和子类关系：
最基层：`T:?Sized` `?Sized`的约束表明了所有的类型
一级子层：默认内存空间固定类型`T`；裸指针类型`* const T/* mut T`; 切片类型`[T]`; 数组类型`[T;N]`; 引用类型`&T/&mut T`; trait约束类型`T:trait`; 泛型元组`(T, U...)`; 泛型复合类型`struct <T>`; `enum <T>`; `union<T>` 及具体类型 `u8/u16/i8/bool/f32/&str/String...`
二级子层： 对一级子层的T赋以具体类型 如：`* const u8`; `[i32]`，或者将一级子层中的`T`再次做一级子层的具化，例如：`* const [T]`; `[*const T]`; `&(*const T)`; `* const T where T:trait`; `struct <T:trait>`

可以一直递归下去。 显然，针对基层类型实现的方法和trait可以应用到层级高的泛型类型中。 例如：
```rs
impl<T> Option<T> {...}
impl<T, U> Option<(T, U)> {...}
impl<T: Copy> Option<&T> {...}
impl<T: Default> Option<T> {...}
```
以上是标准库对Option 的不同泛型的方法实现定义。遵循了从一般到特殊的规则。

类似的实现再试举如下几例：
```rs
impl <T:?Sized> *const T {...}
impl <T:?Sized> *const [T] {...}
impl <T:?Sized> *mut T{ ...}
impl <T:?Sized> *mut [T] {...}
impl <T> [T] { ...}
impl <T, const N:usize> [T;N]{...}
impl AsRef<[u8]> for str {...}
impl AsRef<str> for str {...}
```

当在代码中需要实现一个新的trait时，都要考虑其是否具备满足所有的类型或某类特殊类型的集体需求，如果是，就可以考虑基于泛型实现。当然，要按照泛型层级从一般到特殊来编写代码。
基于泛型来实现trait或方法，是一种微妙的提升代码良好设计的语言特点.

# RUST内存安全杂述
经过对标准库源代码的学习，很容易能够发现，rust编译器提供的安全特性实际很少：
明确的安全特性：
1. 变量必须初始化之后才能使用；
2. 引用必须是内存对齐的，引用指向的变量必须已经初始化；
3. 模块成员默认私有
4. 严格的类型及类型无效值限制
5. 基础类型都满足`Copy/Send/Sync auto trait`
6. if及match的分支语法

明确的不安全特性：
1. 裸指针解引用；
2. 线程间转移变量必须支持`Send`, 共享变量必须支持`Sync`
3. 所有的`FFI`调用,`unsafe intrinsic`函数调用
4. 对类型产生无效类型值
5. 嵌入式汇编使用
6. 含有以上成分的代码单元

为安全提供的工具:
1. 所有权，生命周期，自动drop；
2. 自动解引用

编译器提供的安全特性实际上只是实现内存安全的基础设施

