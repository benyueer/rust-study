/**
 * Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。
 *
 * 生命周期避免了悬垂引用
 */
#[cfg(test)]
mod Test {
    /**
     * 函数中的泛型生命周期
     * 编写一个返回两个字符串 slice 中较长者的函数。这个函数获取两个字符串 slice 并返回一个字符串 slice
     * 但是如下代码回报错：
     * this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y
     * consider introducing a named lifetime parameter: `<'a>`, `'a `, `'a `, `'a `
     *
     * 提示文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y
     * */
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // 现在函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期 'a 存在的一样长的字符串 slice。函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice
    // 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
    fn logest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() < y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    pub fn lifetime() {
        // 接下来调用longest函数
        let string1 = String::from("qwe");
        {
            let string2 = String::from("asd");
            let longest_str = logest(&string1, &string2);
            println!("longest str is: {}", longest_str);
        }
        // 此时对于s1，s2和longest的使用在内部作用域都是有效的
        // 但是在作用域外呢
        let result;
        {
            let string2 = String::from("asd");
            // 会报错
            // borrowed value does not live long enough
            result = logest(&string1, &string2);
        }
        // println!("longest str is: {}", result);
    }

    /**
     * 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配
     * 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域
     * 
     * cannot return reference to temporary value
     */
    fn longest_1<'a>(x: &'a str, y: &'a str) -> &'a str {
        // String::from("qqq").as_str()
        x
    }


    /**
     * 结构体定义中的生命周期注解
     * 一个存放引用的结构体，所以其定义需要生命周期注解
     * 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
     */
    struct ImportantExcerpt<'a> {
        part: &'a str
    }

    #[test]
    fn struct_lifetime() {
        let novel = String::from("qweqwe.csda");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt{
            part: first_sentence
        };
    }



    /**
     * 生命周期省略（Lifetime Elision）
     * 现在我们已经知道了每一个引用都有一个生命周期，而且我们需要为那些使用了引用的函数或结构体指定生命周期
     * 
     * 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）
     * 
     * 编译器采用三条规则来判断引用何时不需要明确的注解
     * 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块
     * 1. 每一个是引用的参数都有它自己的生命周期参数
     * 2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
     * 3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)，那么所有输出生命周期参数被赋予 self 的生命周期
     */
    fn elision() {

    }


    /**
     * 方法定义中的生命周期注解
     */
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    // 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}
