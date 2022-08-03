use std::{rc::Rc, cell::RefCell};

/**
 * RefCell<T> 和内部可变性模式
 * 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的
 * 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则
 *
 * * 通过 RefCell<T> 在运行时检查借用规则
 * RefCell<T> 代表其数据的唯一的所有权
 * 回忆一下第四章所学的借用规则：
 * 1. 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）。
 * 2. 引用必须总是有效的。
 * 对于引用和 Box<T>，借用规则的不可变性作用于编译时
 * 对于 RefCell<T>，这些不可变性作用于 运行时
 * 对于引用，如果违反这些规则，会得到一个编译错误
 * 而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出
 * RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候
 * 类似于 Rc<T>，RefCell<T> 只能用于单线程场景
 *
 * * 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
 * - Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
 * - Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
 * - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
 * **在不可变值内部改变值就是 内部可变性 模式**
 * 
 * 
 * * RefCell<T> 在运行时记录借用
 * 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法，这属于 RefCell<T> 安全 API 的一部分
 * borrow 方法返回 Ref<T> 类型的智能指针
 * borrow_mut 方法返回 RefMut<T> 类型的智能指针
 * 这两个类型都实现了 Deref，所以可以当作常规引用对待
 * 
 * RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
 * 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一。当 Ref<T> 值离开作用域时，不可变借用计数减一
 * ! RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用
 * 如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在运行时出现 panic
 * 
 * 
 * 
 * * 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
 * Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问
 * 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了
 */

pub fn ref_cell() {
    // 内部可变性：不可变值的可变借用
    // 借用规则的一个推论是当有一个不可变值时，不能可变地借用它
    let x = 10;
    // let y = &mut x;  这一行会报错，因为x是不可变的，引用也不可变


    // * 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil
    }

    use List::*;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("error, over");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("warning, 90%");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("warning, 75%");
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::{Messenger, LimitTracker};


    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut 方法来获取 RefCell 中值的可变引用
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 10);

        limit_tracker.set_value(8);

        // 调用 RefCell 的 borrow 以获取 vector 的不可变引用
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
