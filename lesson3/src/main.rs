mod mod_type;
mod school;
use crate::mod_type::mod_type::type_fn;

/**
 * rust变量
 */


// use mod_type;

fn main() {
    /*
     * 使用let声明变量，但变量默认是不能重新赋值的，使用mut关键字可以使变量可变，但类型不可变
     */ 
    let x = 1;
    println!("x value is: {}", x);
    let x_1 = &x;


    // Err: cannot assign twice to immutable variable `x`
    // x = 2;
    println!("x value is: {}, {}", x, x_1);

    let mut y = 1;
    println!("y value is: {}", y);
    // 可以被修改
    y = 2;
    println!("y value is: {}", y);
    // 但是不能改变类型
    // y = 'y';

    /*
    * 常量，使用const声明，不能使用mut修饰，必须指定类型，不能使用函数返回值，只能绑定一个常量表达式
    */
    const PI: f64 = 3.14;
    // 报错
    // const PI_1: f64 = getPI();
    let pi_1: f64 = get_pi();
    println!("{}, {}", PI, pi_1);


    /*
    * 变量隐藏，新的变量声明可以覆盖掉旧的同名变量，那么在当前作用域就只能访问到新的变量
    * 隐藏与使用mut重新赋值不同，重新声明会创建出新的变量，这时可以改变其类型
    */

    let name = "mical";
    {
        let name = "Anmy";
        println!("name: {}", name); // Anmy
    }
    println!("name: {}", name); // mical
    let name = "mary";
    println!("name: {}", name); // mary

    type_fn();

    println!("stu count: {}", school::student::get_stu_count());
    println!("{}", school::student::GRAND)

}

fn get_pi() -> f64 {
    return 3.14;
}
