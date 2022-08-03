/**
 * * 使用Box <T>指向堆上的数据
 * 
 * 最简单直接的智能指针是Box，box允许你将一个值放在堆上而不是栈上，留在栈上的是指向堆数据的指针
 * 
 * box没有性能损失，也没有很多额外功能，多用于如下场景：
 * - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
 * - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
 * - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
 */
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

pub fn ptr_box() {


    // 使用 Box<T> 给递归类型一个已知的大小
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}