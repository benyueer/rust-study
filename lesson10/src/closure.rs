/**
 * 闭包
 * 可以保存在一个变量中或作为参数传递给其他函数的匿名函数
 * 可以在一个地方创建闭包，在不同上下文中执行闭包运算
 * 闭包允许捕获调用者作用域中的值
 */
use std::thread;
use std::time::Duration;

pub fn closure() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let v = vec![1, 2, 3];

    let equal_to_v = move |z: Vec<u32>| z == v;

    let z = vec![1, 2, 3];

    println!("{}", equal_to_v(z));

}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");

    thread::sleep(Duration::from_secs(2));

    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("calculating slowly ...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}



