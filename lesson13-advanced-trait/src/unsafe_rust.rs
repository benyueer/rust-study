/**
 * 不安全 Rust 之所以存在，是因为静态分析本质上是保守的
 * 另一个 Rust 存在不安全一面的原因是：底层计算机硬件固有的不安全性。如果 Rust 不允许进行不安全操作，那么有些任务则根本完成不了
 *
 *
 * 不安全的超能力
 * 可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块
 * 这里有五类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超能力。” 这些超能力是：
 * 1. 解引用裸指针
 * 2. 调用不安全的函数或方法
 * 3. 访问或修改可变静态变量
 * 4. 实现不安全 trait
 * 5. 访问 union 的字段
 *
 * unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查：如果在不安全代码中使用引用，它仍会被检查
 * unsafe 关键字只是提供了那五个不会被编译器检查内存安全的功能
 */

mod test {
    use std::slice;


    /**
     * 解引用裸指针
     * 裸指针是不可变或可变的，分别写作 *const T 和 *mut T
     * 这里的星号不是解引用运算符；它是类型名称的一部分
     * 在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值
     *
     *
     * 裸指针与引用和智能指针的区别在于:
     * 1. 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
     * 2. 不保证指向有效的内存
     * 3. 允许为空
     * 4. 不能实现任何自动清理功能
     */

    #[test]
    fn raw_pointers() {
        let mut num = 5;
        // 注意这里没有引入 unsafe 关键字。可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
        // 使用 as 将不可变和可变引用强转为对应的裸指针类型
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // 创建一个不能确定其有效性的裸指针
        // 创建一个指向任意内存地址的裸指针
        let address = 0x1234usize;
        let r = address as *const i32;

        // 对裸指针使用解引用运算符 *，这需要一个 unsafe 块
        unsafe {
            println!("r1 is :{}", *r1);
            println!("r2 is :{}", *r2);
            println!("r is :{}", *r);
        }

        // 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值
    }

    /**
     * 调用不安全函数或方法
     *
     */
    #[test]
    fn unsafe_fn() {
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }


        // 创建不安全代码的安全抽象
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            // as_mut_ptr 返回一个 *mut i32 类型的裸指针，储存在 ptr 变量中
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    // slice 是一个指向一些数据的指针，并带有该 slice 的长度。可以使用 len 方法获取 slice 的长度，使用 as_mut_ptr 方法访问 slice 的裸指针
                    // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice
                    // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的
                    // 裸指针上的 add 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针
                    // 因此必须将 slice::from_raw_parts_mut 和 add 放入 unsafe 块中以便能调用它们
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len-mid)
                )
            }
        }

        let mut v = vec![1, 2, 3 ,4 , 5, 6];
        let r = &mut v;
        let (a, b) = split_at_mut(r, 3);
        println!("{:?} -- {:?}", a, b);
    }

    // 使用 extern 函数调用外部代码
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn extern_c() {
        unsafe {
            println!("abs: {}", abs(-1));
        }
    }


    // 访问或修改可变静态变量
    // 常量 const
    // 静态变量 static
    // 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
    static mut COUNTER: i32 = 0;

    fn add_to_count(inc: i32) {
        unsafe {
            COUNTER += inc;
        }
    }
    #[test]
    fn change_static() {
        add_to_count(12);
        unsafe {
            println!("COUNTER is: {COUNTER}");
        }
    }
}
