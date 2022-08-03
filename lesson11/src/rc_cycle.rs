use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::mpsc::Receiver, borrow::{BorrowMut, Borrow},
};

/**
 * * 引用循环与内存&泄漏
 *
 * * 避免引用循环：将 Rc<T> 变为 Weak<T>
 * 通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）
 * 调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针
 * 调用 Rc::downgrade 会将 weak_count 加 1
 * Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。其区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理
 *
 * 强引用代表如何共享 Rc<T> 实例的所有权
 * 
 * Weak<T> 实例的 upgrade 方法返回 Option<Rc<T>>
 * 如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None
 * 
 */

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::*;

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
// 我们希望能够 Node 拥有其子节点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，为此 Vec<T> 的项的类型被定义为 Rc<Node>。我们还希望能修改其他节点的子节点，所以 children 中 Vec<Rc<Node>> 被放进了 RefCell<T>
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn rc_cycle() {
    {
        // // 制造引用循环
        // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        // println!("a count {}", Rc::strong_count(&a));
        // println!("a next {:?}", a.tail());

        // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        // println!("a count {}", Rc::strong_count(&a));
        // println!("b count {}", Rc::strong_count(&b));
        // println!("b next {:?}", b.tail());

        // if let Some(link) = a.tail() {
        //     println!("{:?}", link);
        //     *link.borrow_mut() = Rc::clone(&b);
        // }

        // println!("b count {}", Rc::strong_count(&b));
        // println!("a count {}", Rc::strong_count(&a));

        // // println!("a next {:?}", a.tail());
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade()); // None

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade()); // Some
}
