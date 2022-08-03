/**
 * 迭代器模式允许你对一个序列的项进行某些处理
 * 负责遍历序列中的每一项和决定序列何时结束的逻辑
 */

pub fn iterator() {
    let v_1 = vec![1, 2, 3];

    let v_1_at_1 = &v_1[1];

    println!("{:?}", v_1);

    // iter 方法生成一个不可变引用的迭代器
    // into_iter 获取所有权并返回拥有所有权的迭代器
    // iter_mut 迭代可变引用

    // 消费迭代器的方法
    // next
    let v1_iter = v_1.iter();
    let total: i32 = v1_iter.sum();
    println!("total: {}", total);

    // 产生其他迭代器的方法
    // 迭代器适配器（iterator adaptors）
    let v2: Vec<i32> = v_1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    in_closuer();

    impl_iterator();
}

// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
/*

pub trait Iterator {
    type: Item;
    fn next(&mut self) -> Option<Self::Item> {

    }
}

*/

// 使用闭包获取环境
fn in_closuer() {
    #[derive(Debug)]
    struct Shoe {
        size: i32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|shoe| shoe.size == shoe_size)
            .collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: "qwe".to_string(),
        },
        Shoe {
            size: 32,
            style: "csd".to_string(),
        },
        Shoe {
            size: 10,
            style: "gs".to_string(),
        },
    ];

    let my_shoes = shoes_in_size(shoes, 10);
    println!("in size shoes: {:?}", my_shoes);
}

// 实现 Iterator trait 来创建自定义迭代器
fn impl_iterator() {
    #[derive(Debug)]
    struct Counter {
        count: i32,
    }

    impl Counter {
        fn new(count: i32) -> Counter {
            Counter { count }
        }
    }

    impl Iterator for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if (self.count < 5) {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new(0);
    println!("{:?}", counter);

    // for i in counter.into_iter() {
    //     print!("{}", i);
    // }
    let sum: i32 = counter.sum();
    println!("sum {}", sum);
}
