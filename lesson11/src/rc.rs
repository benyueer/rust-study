use std::rc::Rc;

/**
 * Rc<T> 引用计数（reference counting）
 * Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它
 * 如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效
 * 
 * 克隆 Rc<T> 会增加引用计数
 */

pub fn rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("after create a {}", Rc::strong_count(&a)); // 1
    // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间
    let b = Cons(3, Rc::clone(&a));
    println!("after create b {}", Rc::strong_count(&a)); // 2
    let c = Cons(4, Rc::clone(&a));
    println!("after create c {}", Rc::strong_count(&a)); // 3
    {
        let d = Cons(4, Rc::clone(&a));
        println!("after create d {}", Rc::strong_count(&a)); // 4
    }
    println!("after d delete {}", Rc::strong_count(&a)); // 3
}

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::rc::List::{Cons, Nil};