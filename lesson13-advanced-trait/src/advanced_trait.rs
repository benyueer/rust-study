/**
 * 关联类型在 trait 定义中指定占位符类型
 * 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型
 * trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型
 * 如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么
 *
 * 默认泛型类型参数和运算符重载
 *
 * 完全限定语法与消歧义：调用相同名称的方法
 *
 * 父 trait 用于在另一个 trait 中使用某 trait 的功能
 * 
 * newtype 模式用以在外部类型上实现外部 trait
 */

#[cfg(test)]
mod test {
    // 关联类型在 trait 定义中指定占位符类型
    // 一个带有关联类型的 trait 的例子是标准库提供的 Iterator trait。它有一个叫做 Item 的关联类型来替代遍历的值的类型
    pub trait Iterator {
        // Item 是一个占位类型，同时 next 方法定义表明它返回 Option<Self::Item> 类型的值
        // 这个 trait 的实现者会指定 Item 的具体类型，然而不管实现者指定何种类型, next 方法都会返回一个包含了此具体类型值的 Option
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // 默认泛型类型参数和运算符重载
    // 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型
    // 如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要
    // 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>
    // 这种情况的一个非常好的例子是用于运算符重载。运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作
    // Rust 并不允许创建自定义运算符或重载任意运算符
    // 不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载
    // 例子展示了如何在 Point 结构体上实现 Add trait 来重载 + 运算符，这样就可以将两个 Point 实例相加了
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    use std::{fmt, ops::Add};

    impl Add for Point {
        // Add trait 有一个叫做 Output 的关联类型，它用来决定 add 方法的返回值类型
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    #[test]
    fn add_test() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 4 },
            Point { x: 3, y: 4 }
        )
    }

    // 这是Add trait的定义
    // 比较陌生的部分是尖括号中的 Rhs=Self：这个语法叫做 默认类型参数（default type parameters）
    // Rhs 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数
    // 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型(例子中是 Point)
    // 当为 Point 实现 Add 时，使用了默认的 Rhs，因为我们希望将两个 Point 实例相加
    // trait Add<Rhs=Self> {
    //     type Output;

    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    // 让我们看看一个实现 Add trait 时希望自定义 Rhs 类型而不是使用默认类型的例子
    struct Millimeters(u32);
    struct Meters(u32);
    // 为了使 Millimeters 和 Meters 能够相加，我们指定 impl Add<Meters> 来设定 Rhs 类型参数的值而不是使用默认的 Self
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    // 默认参数类型主要用于如下两个方面
    // 1. 扩展类型而不破坏现有代码
    // 2. 在大部分用户都不需要的特定情况进行自定义

    // ----------------
    // 完全限定语法与消歧义：调用相同名称的方法
    // Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait
    // 甚至直接在类型上实现开始已经有的同名方法也是可能的
    // 当调用这些同名方法时，需要告诉 Rust 我们希望使用哪一个
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    #[test]
    fn same_fn() {
        let h = Human {};
        h.fly(); // 调用了直接实现在 Human 上的 fly 方法。
                 // 为了能够调用 Pilot trait 或 Wizard trait 的 fly 方法，我们需要使用更明显的语法以便能指定我们指的是哪个 fly 方法
        Pilot::fly(&h);
        Wizard::fly(&h);
        // 在方法名前指定 trait 名向 Rust 澄清了我们希望调用哪个 fly 实现。也可以选择写成 Human::fly(&person)
        Human::fly(&h);
    }

    // 因为 fly 方法获取一个 self 参数，如果有两个 类型 都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现
    // 然而，关联函数是 trait 的一部分，但没有 self 参数。当同一作用域的两个类型实现了同一 trait，Rust 就不能计算出我们期望的是哪一个类型，除非使用 完全限定语法（fully qualified syntax）
    // 例如：
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // 通常，完全限定语法定义为：
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // 对于关联函数，其没有一个 receiver，故只会有其他参数的列表
    // 可以选择在任何函数或方法调用处使用完全限定语法
    #[test]
    fn fully_qualified_syntax() {
        println!("{}", Dog::baby_name());
        // println!("{}", Animal::baby_name());
        println!("{}", <Dog as Animal>::baby_name());
    }

    // ---------------
    // 父 trait 用于在另一个 trait 中使用某 trait 的功能
    // 有时我们可能会需要某个 trait 使用另一个 trait 的功能
    // 在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的 父（超） trait（supertrait）
    // 例：
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 为 Point 实现 OutlinePrint 必须先实现 Display
    impl OutlinePrint for Point {}

    #[test]
    fn super_trait() {
        let poi = Point{x: 1, y: 2};
        println!("{}", poi.to_string());
        poi.outline_print();
        Point::outline_print(&poi);
    }


    // --------------
    // newtype 模式用以在外部类型上实现外部 trait
    // 在第十章的 “为类型实现 trait” 部分，我们提到了孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait
    // 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体（第五章 “用没有命名字段的元组结构体来创建不同的类型” 部分介绍了元组结构体）中创建一个新类型
    // 这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装
    // 接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait
    // Newtype 是一个源自 （U.C.0079，逃） Haskell 编程语言的概念
    // 使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了
    // 例如，如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外
    // 可以创建一个包含 Vec<T> 实例的 Wrapper 结构体，接着可以如例子 那样在 Wrapper 上实现 Display 并使用 Vec<T> 的值:
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    #[test]
    fn newtype_test() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("{}", w);
    }
}
