/**
 * * 引用与借用
 * 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用确保指向某个特定类型的有效值
 * 创建一个引用的行为叫做借用
 * 
 * * 可变引用
 * 变量必须声明为可变
 * ! 同一作用域同一时间内对一个变量只能创建一个可变引用
 * ! 创建可变引用后就不能创建普通引用，
 * 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
 * 引用必须总是有效的。
 */
pub fn quote_borrow() {
    let mut s1 = String::from("hello");

    /*
    *   &s1语法创建一个指向s1的引用，但所有权不变，因此当&s1使用完后，他指向的值也不会丢弃
    */
    let len = calculate_length(&s1);
    println!("{}", len);

    // let s2 = String::from("hello");

    append_hello(&mut s1);

    println!("{}", s1);

    
}

fn calculate_length (s: &String) -> i32 {
    // 变量 s 的作用域与函数参数一样
    s.len().try_into().unwrap()
    // 当 s 停止使用时并不会丢弃，因为他没有值的所有权
}



fn append_hello(s: &mut String) {
    s.push_str(", world")
}