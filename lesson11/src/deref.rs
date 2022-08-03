use std::ops::Deref;

/**
 * 通过 Deref trait 将智能指针当作常规引用处理
 * 
 * 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或通配符相区别）
 * 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。
 * 
 * 
 * 函数和方法的隐式 Deref 强制转换
 * Deref 强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。Deref 强制转换只能作用于实现了 Deref trait 的类型。Deref 强制转换将这样一个类型的引用转换为另一个类型的引用。
 * 例如，Deref 强制转换 可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以返回 &str
 * 
 * 
 */
pub fn deref() {

    // 通过解引用运算符追踪指针的值
    // 常规引用是一个指针类型，一种理解指针的方式是将其看成指向储存在其他某处值的箭头
    let x = 5;
    let y = &x;
    println!("{}, {}", *y, y);

    // 像引用一样使用 Box<T>
    // 将 y 设置为一个指向 x 值拷贝的 box 实例
    let y = Box::new(x);
    println!("{}, {}", *y, y);



    // 没有 Deref trait 的话，编译器只会解引用 & 引用类型。deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力。
    let y = MyBox::new(x);
    // 实际上执行了 *(y.deref())
    // Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作
    // 每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换
    println!("{}, {}", *y, *(y.deref()));
    assert_eq!(5, *y);





}
// 自定义智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
} 

// 通过实现 Deref trait 将某类型像引用一样处理
impl<T> Deref for MyBox<T> {
    type Target = T;

    // deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}