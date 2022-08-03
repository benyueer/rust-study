/**
 * 所有权
 * * 所有权规则
 * - rust中的所有值都有一个对应的变量作为他的所有者
 * - 在同一时间内，值有且只有一个所有者
 * - 当所有者离开自己的作用域时，他持有的值就会被释放掉
 * 
 * * 变量作用域
 *  
 * * 内存与分配
 * 
 */
mod quote_borrow;
mod slice;

use crate::quote_borrow::quote_borrow;
use crate::slice::slice;



fn main() {
    slice();
    return;
    
    quote_borrow();
    return;

    println!("Hello, world!");
    // not found in this scope
    // println!("{}", s);
    let s = String::from("hello");
    let s_1 = s;
    // 堆上的值被指定给另一个变量后不可访问
    // borrow of moved value: `s`
    // println!("{}, {}", s, s_1);


    // 将值传递给函数就像赋值语句一样
    let s_2 = change_val(s_1); 
    // borrow of moved value: `s_将值传递给1`就像赋值语句一样
    // println!("{}", s_1);


    // 但是存在栈上的值不会，而是在内存中复制了一份
    let num = 321;
    change_num(num);
    println!("{}", num);


}

fn change_val(mut s: String) -> String {
    s = String::from("dsd");
    s
}

fn change_num(mut num: i32) {
    num = 123;
}