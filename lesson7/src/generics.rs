use core::num;

/**
 * * 泛型
 * 
 */
pub fn generics() {
    let (a, b) = (1, 2);
    let res = add::<i32>(a, b);
    println!("{}", res);

    let p_1 = Point{x: 1, y: 2};
}

// 函数中使用泛型
fn add<T>(num_1: T, num_2: T) -> T {
    // num_1 + num_2
    num_1
}

// 结构体定义中的泛型
struct Point<T> {
    x: T,
    y: T,
}

// 方法定义中的泛型
impl<T> Point<T> {

}