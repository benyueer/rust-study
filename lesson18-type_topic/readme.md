# 类型专题
## 指针
什么是指针？计算机引用无法立即直接访问的数据的一种方式
数据在物理内存（RAM）中分散存储，地址空间是检索系统，指针就是被编码为内存地址，使用usize类型的整数表示，一个地址就会指向地址空间中的某个地方
- 内存地址：就是指代内存中单个字节的一个数，是汇编语言提供的抽象
- 指针：就是指向某种类型的一个地址，是高级语言提供的抽象
- 引用：就是指针，如果是动态大小的类型，就是指针和就有额外保证的一个整数
  - 引用是Rust提供的抽象

## Rust 里的引用
- 引用始终引用的是有效数据
- 引用与 usize 的倍数对其（CPU 读取没对齐的内存时性能下降，Rust 的内存系统会填充一些字节保证内存对齐）
- 引用可以为动态大小的类型提供上述保障

```rs
static B: [u8; 10] = [123,31,4,5,876,3453,756,867,867,35];
static C: [u8; 11] = [123,31,4,5,876,3453,756,867,867,35, 0];

fn main() {
  let a = 43;
  let b = &B;
  let c = &C;

  println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
```

## 原始指针
Raw Pointer
没有 Rust 标准保障的内存地址，是 unsafe 的

语法：
```rs
// 不可变 Raw Pointer
*const T

// 可变 Raw Pointer
*mut T

// 指 String 类型的原始指针
*const String
```
`*const T  *mut T`之间的差异很小，可以互相转换
Rust 的引用（`&mut T  &T`）会被编译成原始指针，这意味着无需冒险进入 unsafe 就可获得原始指针的性能

```rs
// 把引用转为原始指针
fn main() {
  let a: i64 = 42;
  let a_ptr = &a as *const i64;
  let a_addr: uszie = unsafe {std::mem::transmute(a_ptr)};

  println!("a: {} ({:p}) ... 0x{:x}", a, a_ptr, a_addr+7);
}
```

解引用：通过指针从 RAM 内存提取数据的过程

在底层，引用（`&T  &mut T`）被实现为原始指针，但引用带有额外的保障，应该始终作为首选使用
访问 Raw Pointer 的值总是 unsafe 的
Raw Pointer 不拥有值得所有权，在访问时编译器不会检查数据的合法性
允许多个 Raw Pointer 指向同一数据，Rust无法保证共享数据的合法性

Rust 智能指针
|名称|简介|强项|弱项|
|-|-|-|-|
|`Raw Pointer`|`*const T *mut T`，自由基，闪电般快，极其unsafe|速度、与外界交互|unsafe|
|`Box<T>`|可把任何东西都放在 Box 里，可接受几乎任何类型的长期存储，新的安全编程时代的主力军|将值集中存储在 Heap|大小增加|
|`Rc<T>`|他知道谁借了什么，何时借了什么|对值的共享访问|大小增加、运行时成本、线程不安全|
|`Arc<T>`|可以跨线程共享值，保证这些值不会相互干扰|对值的共享访问、线程安全|大小增加、运行时成本|
|`Cell<T>`|具有改变不可变值的能力|内部可变性|大小增加、性能|
|`RefCell<T>`|对不可变引用执行改变，但有代价|内部可变性、可与仅接受不可变引用的 Rc、Arc 嵌套使用|大小增加、运行时成本、缺乏编译时保障|
|`Cow<T>`|封闭并提供对借用数据的不可变访问，并在需要修改或所有权时延迟克隆数据|当只是只读访问时避免写入|大小可能会增大|
|`String`|可处理可变长度的文本，展示了如何构建安全的抽象|动态按需增长、在运行时保证正确编码|过度分配内存大小|
|`Vec<T>`|程序最常用的存储系统，他在创建和销毁值时保持数据有序|动态按需增长|过度分配内存大小|
|`RawVec<T>`|是`Vec<T>`和其他动态大小类型的基石|动态按需增长、与内存分配器一起配合寻找空间|不直接适用于您的代码|
|`Unique<T>`|作为值的唯一所有者，可保证拥有完全控制权|需要独占值的类型（如String）的基础|不直接适用于您的代码|
|`Shared<T>`|分享所有权|共享所有权、可以将内存与T的宽度对齐即使是空的时候|不直接适用于您的代码|


## 内存
值：类型 + 类型值域中的一个元素
变量：值会存储到一个地方，可以是 Stack 或 Heap，最常见的就是存在变量，他是 Stack 上一个被命名的值的槽

深入变量：
- 高级类型：生命周期、借用等角度
- 低级类型：不安全代码、原始指针角度

变量的高级模型：
- 变量就是给值一个名称当把值赋给一个变量后，这个值就由变量命名了
- 变量只有在他持有的值合法的时候才存在
- 如果变量值未初始化，或者已经被移动了，那么这个变量就无法被使用了
- 使用该模型，整个程序会有许多依赖线组成，这些线叫做 flow
- 每个 flow 都在追踪一个值的特定实例的生命周期
- 当有分支存在时，flow 可以分叉或合并，每个分叉都在追踪该值的不同的生命周期
- 在程序中的任何给定点，编译器可以检查所有的 flow 是否可以互相兼容、并行存在
  - 例如：一个值不可以有两个具有可变访问的并行 flow ，也不可以一个 flow 借用了一个值，但却没有 flow 拥有该值
    
变量的低级模型：
- 变量会给那些可能存储合法值的内存地点进行命名
- 可以把变量想象为值的槽：当你赋值的时候槽就装满了，而他原有的值就被丢弃或替换了
- 当访问他时，编译器会检查槽是不是空的，如果是空的，就说明变量未初始化，或者他的值被移动了
- 指向变量的指针，其实是指向变量的幕后内存，通过解引用可以获得他的值


内存区域
- 有许多内存区域，并不是都在 DRAM 上
- 三个比较重要的内存区域：stack、heap、static


Stack 内存
- 有疑问时，首选 Stack，
  - 想把数据放在 Stack，编译器必须知道类型的大小
  - 换句话说：有疑问时，使用实现了 Sized 的类型
- Stack 是一段内存，程序把它作为一个暂存空间，用于函数调用
- 为什么叫 Stack？因为在 Stack 上的条目是 LIFO
- 为什么快
  - 因为函数里所有变量在内存里都是紧挨着的

Stack Frame
- 每个函数每次调用时，都会在 Stack 顶部分配一块连续的内存块，叫做 Stack Frame（栈帧）
- 在接近 Stack 底部的地方是 main 函数的 frame
- 函数的 frame 包含了函数的所有变量以及函数所带的参数
- 当函数返回时，他的 frame 就被回收了
  - 构成函数本地变量值的那些字节不会立即擦除，但访问他们也是不安全的，因为他们可能被后续的函数调用所重写（如果后续函数调用的 frame 与回收的这个有重合的话）
  - 即使没有被重写，他们也可能含有无法合法使用的值，例如函数返回后被移动的值

Stack Pointer
- 随着程序的执行，CPU 里有一个游标会随之更新，他会反映出当前 frame 的当前地址，这个游标叫 stack pointer（stack 指针）


Heap 内存
- Heap 意味着混乱
- Heap 是一个内存池，并没有绑定到当前程序的调用栈
- Heap 是为了在编译时不知道大小的类型准备的
  - 有一些类型大小可变：String、Vec<T>  [T]
- Heap 允许你显式的分配连续的内存块，当这么做时，你会得到一个指针，他指向内存块开始的地方
- Heap 中的值会一直有效，直到你对他显式的释放
- 如果你想让值比函数的 frame 活得更长，这很有效

Heap 与线程安全
- 如果想把值送到另一个线程，当前线程可能无法与另一个线程共享 stack frame，你就可以把它存放在 Heap 上
- 因为函数返回时 heap 上的值不会消失，所以你可以在一个地方为值分配内存，把指向他的指针传给另一个线程，就可以让另一个线程继续安全的操作该值

Heap 交互机制
- Heap 上的变量必须通过指针访问
- Rust 里 与 Heap 交互的首要机制就是 Box 类型


Stack 静态内存
- Stack 内存实际是一个统称，他指的是程序编译后的文件中几个密切相关的区域
  - 当程序执行时，这些区域会自动加载到程序的内存里
- Stack 内存的值在程序执行期间会一直存活
- 程序的 Stack 内存是包含程序二进制代码的（通常映射为只读的）
  - 随着程序的执行，他会在本文段的二进制代码中挨个指令进行遍历，而当函数被调用时就进行跳跃
- Stack 内存会持有使用 static 声明的变量的内存，也包括某些常量值，例如字符串

stack 生命周期
- static 是特殊的生命周期
  - 他的名字就是来自于 static 内存区，他将引用标记为只要 static 内存还存在（程序关闭前），那么引用就合法

const 与 static 的区别：
- const 关键字会把变量声明为常量
- 常量可以在编译时完全计算出来
- 在编译期间，任何引用常量的代码都会被替换为常量的计算结果的值
- 常量没有内存或关联其他存储


动态内存分配
- 任何时刻，运行中的程序都需要一定数量的内存
- 当程序需要更多内存时，就需要从 OS 请求，这就是*动态内存分配(dynamic allocation)*
- 步骤
  1. 通过系统调用从 OS 请求内存
     - unix 类：alloc()
     - windows：HeapAlloc()
  2. 使用分配的内存
  3. 将不需要的内存释放给 OS
     - unix: free()
     - windows: HeapFree()


虚拟内存
- 程序的内存视图，程序可访问的所有数据都是由操作系统在其地址空间中提供
- 在直觉上，程序的内存就是一系列字节，从开始位置 0 到 结束位置 n
- 程序里访问数据需要虚拟地址（程序只能访问虚拟地址）
- 虚拟地址会被翻译成物理地址
  - 涉及程序、OS、CPU、RAM，有时还涉及硬盘或其他设备
  - CPU 负责执行翻译，OS 负责存储指令
  - CPU 包含一个内存管理单元（MMU），负责这项工作 
  - 这些指令也存在内存中一个预定义地址中
  

## 生命周期
- Rust 里每个引用都有生命周期，他就是引用保持合法的作用域（scope），大多数时候是隐式和推断出来的
- 某个变量取得引用时生命周期开始，当变量移动或离开作用域时生命周期结束
- 生命周期：对于某个引用来说，他必须保持合法的一个代码区域的名称
  - 生命周期通常与作用域重合，但也不一定

借用检查器
- 当具有某个生命周期 'a 的引用被使用，借用检查器就会检查 'a 是否还存活
  - 追踪路径直到 'a 开始（获得引用）的地方
  - 从这开始，检查沿着路径是否存在冲突
  - 保证引用指向一个安全可访问的值

泛型生命周期
- 有时候需要在自己的类型里存储引用：
  - 这些引用都有生命周期，以便借用检查器检查合法性
  - 例如：在该类型方法中返回引用，且存活比 self 长
- Rust 允许你基于一个或多个生命周期将类型的定义泛型化
- 如果类型实现了 Drop，那么丢弃类型时，就被记作是使用了类型所泛型的生命周期或类型
  - 类型实例要被 drop 了，在 drop 之前，借用检查器会检查看是否仍然合法去使用你类型的泛型生命周期，因为你 drop 里的代码可能会用到这些引用
- 如果类型没有实现 Drop，那么类型丢弃的时候就不会当作使用了生命周期，可以忽略类型内的引用
- 类型可以泛型多个生命周期但通常会不必要的让类型签名更复杂
  - 还有类型包含多个引用时，才应该使用多个生命周期参数
  - 并且他方法返回的引用只应绑定到其中一个引用的生命周期


生命周期 Variance
- variance
  - 哪些类型是其他类型的“子类”
  - 什么时候“子类”可以替换“超类”（反之亦然）
- 通常来说：
  - 如果 A 是 B 的子类，那么A 至少 和 B 一样有用
  - Rust 的例子：
    - 如果函数接收 &'a str 的参数，那么就可以传入 &'static str 
    - 'static 是 'a 的子类，因为 'static 至少可以和 'a 活的一样长

三种 Variance
- 所有的类型都有 Variance
  - 定义了哪些类似类型可以用在该类型的位置上
- 三种 Variance
  - covariant：某类型只能用“子类型”代替
    - 例如：&'static T 代替 &'a T
  - invariant：必须提供指定的类型
    - 例如：&mut T，对于 T 来说就是 invariant 的
  - contrvariant：函数对参数的要求越低，参数可发挥的作用越大


## 内存中的类型
- 每个 Rust 值都有类型
  - 一个基本职责：告诉你如何解释内存中的 bits
- 当自定义类型时：编译器决定该类型的各部分在内存表示中的位置

对齐（Alignment）
- 对齐：决定了类型的字节可以被存在哪
- 实际上，计算机硬件对给定的类型可以存放的位置是有约束的
  - 例如：指针指向的是字节（bytes）而不是比特（bits）
    - 如果将某类型 T 的值放在计算机内存中索引为4的位（bit）上，那就无法引用他的地址，你只能创建一个指针指向 byte 0 或 byte 1
- 所有的值，无论是什么类型，都必须开始于 byte 的边界
  - 必须至少是 字节对齐 （byte-aligned）
  - 存放的地址必须是 8 bits 的倍数


## Trait(Bound) 的编译与分派

静态分派（static dispatch）
- 编译泛型代码或者调用 dyn Trait 上的方法时发生了什么
- 当编写关于泛型 T 的类型或者函数时：
  - 编译器会针对每个T（的类型），都将函数或类型复制一份
  - 当你构建 Vec<i32> 或 HashMap<String, bool>的时候：
    - 编译器会复制他的泛型类型以及所有的实现块
      - 例如 Vec<i32>，就是对 Vec 做一个完整的复制，所有遇到的 T 都换成 i32
    - 并把每个实例的泛型参数用具体类型替换
    - 编译器不会做完整的复制粘贴，他只会复制你用的代码

例子：
```rs
// 为 String 实现一个方法，方法参数有一个泛型
// 针对不同的 Pattern，该方法都会复制一遍
// 因为我们需要知道 is_contained_in 方法的地址，以便进行调用，CPU 需要知道在哪跳转和继续执行 
// 对于任何给定的 Pattern，编译器知道哪个地址是 Pattern 类型实现 Trait 方法的地址
// 不存在给任何一个类型通用的地址
impl String {
  pub fn contains(&self, p: impl Pattern) -> bool {
    p.is_contailed_in(self)
  }
}
```


单态化（monomorphization）
- 从一个泛型类型到多个非泛型类型的过程叫单态化
- 当编译器开始优化代码时，就好像根本没有泛型
  - 每个实例都是单独优化的具有了所有的已知类型
  - 所以 is_contained_in 方法调用的执行效率就像 Trait 不存在一样
  - 编译器对涉及的类型完全掌握，甚至可以将它 inline 实现

单态化的代价：
- 所有的实例都需要单独编译，编译时间增加（如果不能优化编译）
- 每个单态化的函数都会有一段自己的机器码，让程序更大
- 指令在泛型方法的不同实例间无法共享，CPU 的指令缓存效率降低，因为他需要持有相同指令的多个不同副本


动态分派（dynamic dispatch）
- 使代码可以调用泛型类型上的 Trait 方法，而无需知道具体的类型
- 调用者之需要提供两个信息：
  - Pattern 的地址
  - is_contained_in 的地址
```rs
// 
impl String {
  pub fn contains(&self, p: &dyn Pattern) -> bool {
    p.is_contained_in(&*self)
  }
}
```

vtable：
- 实际上，调用者会提供指向一块内存的指针，它叫做虚方法表（virtual method table）或叫 vtable
  - 他持有上例该类型所有 trait 方法实现的地址
    - 其中一个地址就是 is_contained_in 
- 当代码想调用提供类型的一个 trait 方法时，就会从 viable 查询方法的实现地址，并调用
  - 这允许我们使用相同的函数体，而不关心调用者想要使用的类型
- 每个 viable 还包含具体类型的布局和对齐信息（总是需要这些）


对象安全（Object-Safe）
- 类型实现了一个 trait 和他的 viable 的组合就形成了一个 trait object（trait 对象）
- 大部分 trait 可以转换成 trait 对象，但不是所有：
  - 例如 Clone trait就不行（他的 clone 方法返回 Self），Extend trait 也不行
  -  这些例子就不是对象安全的（object-self）
- 对象安全的要求：
  - trait 的所有方法都不能是泛型的，也不可以使用 Self
  - trait 不可拥有静态方法（无法知道在哪个实例上调用的方法）

Self:Sized
- Sel:Sized 意味着 Self 无法用于 trait object，因为他是 !Sized
- 将 Self:Sized 用在某个 Trait，就是要求他永远不使用动态分派
- 也可以将 Self:Sized 用在某个方法上，这时将 trait 通过 trait object 访问的时候，该方法就不可用了
- 当检查 trait 是否对象安全的时候，使用了 where Self:Sized 的方法就会被免除

动态分派优缺点：
- 优点
  - 编译时间少
  - 提升 CPU 指令缓存效率
- 缺点
  - 编译器无法对特定类型优化
    - 只能通过 viable 调用函数
  - 直接调用方法的开销增加
    - trait object 上的每次方法调用都需要检查 viable

如何选择（一般而言）
- 静态分派
  - 在 library 里使用静态分派
    - 无法知道用户的需求
    - 如果使用动态分派，用户也只能如此
    - 如果使用静态分派用户可自行选择
- 动态分派
  - 在 binary 中使用动态分派
    - binary 是最终代码
    - 动态分派使代码更整洁（省去了泛型参数）
    - 编译更快
    - 以边际性能为代价


## 泛型 Trait
Trait 的泛型方式
- 两种：
  - 泛型类型参数：trait Foo<T>
  - 关联类型：trait Foo { type Bar; }
- 区别
  - 使用关联类型：对于指定类型的 trait 只能有一个实现
  - 使用泛型类型参数：可以有多个实现
- 建议（简单来说）
  - 可以的话尽量使用关联类型

类型参数 Trait
- 必须指定所有的泛型类型参数，并重复写这些参数的 Bound
  - 维护较难
    - 如果添加泛型类型参数到某个 Trait，该 Trait 的所有用户必须都进行更新代码
- 真对给定的类型，一个 Trait 可以存在多重实现
  - 缺点：对于你想要用的是 Trait 的哪个实例编译器决定起来更困难了
    - 不得不调用这样的 FromIterator::<u32>::from_iter 可消除歧异的函数
  - 也是优点
    - impl PartialEq<BookFormat> for Book  实现多个类型
    - 可同时实现 FromIterator<T> 和 FromIterator<&T> where T:Clone

关联类型 Trait
- 使用关联类型
  - 编译器之需要知道实现 Trait 的类型
  - Bound 可完全位于 Trait 本身，不必重复使用
  - 未来再添加关联类型也不影响用户使用
  - 具体的类型会决定 Trait 内关联类型的类型，无需使用消除歧义的函数

## 孤儿规则与连贯性/一致性
连贯性/一致性 属性
- 定义：对于给定的类型和方法，只会有一个正确的选择，用于该方法对该类型的实现
- 孤儿规则（orphan rule）
  - 只要 trait 或类型在你本地的 crate，那就可以对该类型实现该 trait 
    - 可以为你的类型实现 Debug，可以为 bool 实现 MyTrait
    - 但不能为 bool 实现 Debug
  - 也有例外
    - Blanket Implementation
      - impl <T> MyTrait for T where T:
        - 例如：impl <T: Display> ToString for T {}
      - 不局限于一个特定的类型，而是应用于更广泛的类型
      - 只有定义 trait 的 crate 允许使用 Blanket Implementation
      - 添加 Blanket Implementation 到现有 trait 属于破坏性变化
    - 基础类型：有些类型太基础了，需要任何人在他们上实现 trait
      - 这些类型被标记了 #[fundamental]，目前包括 &、&mut、Box
    - Covered Implementation
      - 有时需要为外部类型实现外部 trait
        - 例如：impl From<MyType> for Vec<i32>
      - 孤儿规则制定了一个狭窄的豁免：
        - 允许在非常特定的情况下为外来类型实现外来 trait




