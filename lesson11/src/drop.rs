/**
 * Drop trait 允许我们在值要离开作用域时执行一些代码
 * 可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源
 * 例如，当 Box<T> 被丢弃时会释放 box 指向的堆空间
 */

pub fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };
    let d = CustomSmartPointer {
        data: String::from("world"),
    };

    // 变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃
    // dropping data world
    // dropping data hello

    // 通过 std::mem::drop 提早丢弃值
    println!("CustomerSmartPointer c d created");
    drop(c);
    println!("the end of main");
    /*
    CustomerSmartPointer c d created
    dropping data hello
    the end of main
    dropping data world
    */
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping data {}", self.data);
    }
}
