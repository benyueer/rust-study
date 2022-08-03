/**
 * trait  特征
 * trait类似于接口，但有所不同
 * trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型
 * 也就是说trait约束了泛型应当具有的特性
 *
 */

pub trait Summary {
    fn summarize(&self) -> String;
}

trait Display {
    fn display(&self) -> String {
        format!("this is display")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub context: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}", self.author, self.context)
    }
}

impl Display for NewsArticle {}

pub struct Tweet {
    pub username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("user is {}", self.username)
    }
}

pub fn trait_fn() {
    let news_1 = NewsArticle {
        headline: "head".to_string(),
        author: "mic".to_string(),
        context: "this is context".to_string(),
        location: "hz".to_string(),
    };

    let tweet_1 = Tweet {
        username: "he".to_string(),
    };

    notify(&news_1);
    notify_b(&news_1);
    notify_c(&news_1);
    notify_e(&news_1, &tweet_1);
    create_new("ge".to_string());
}

// trait作为参数
fn notify(item: &impl Summary) {
    println!("is {}", item.summarize());
}

// Trait Bound 语法
fn notify_b<T: Summary>(item: &T) {
    println!("is {}", item.summarize());
}

// 通过 + 指定多个 trait bound
fn notify_c(item: &(impl Summary + Display)) {
    println!("summery & display");
}

fn notify_d<T: Summary + Display>(item: &T) {
    println!("d");
}

// 通过 where 简化 trait bound
fn notify_e<T, U>(item_1: &T, item_2: &U)
where
    T: Display,
    U: Summary,
{
    println!("where")
}


// 返回实现了 trait 的类型
fn create_new(author: String) -> impl Summary {
    NewsArticle {
        author,
        headline: "head_line".to_string(),
        context: "this is other".to_string(),
        location: "unknow".to_string(),
    }
}