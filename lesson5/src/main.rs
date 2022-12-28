/**
 * struct
 * * 结构体声明
 * * 结构体创建
 * * 字段初始化简写语法
 * * 结构体更新语法
 * * 元组结构体
 * * 类单元结构体
 * * 方法
 * * 带有更多参数的方法
 * * 关联函数
 */

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area (&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let user_1 = User {
        active: true,
        username: String::from("Mick"),
        email: String::from("aeqwewq@dsd.cs"),
        sign_in_count: 232424
    };

    let mut user_2 = build_user(String::from("h1"), String::from("123124"));

    println!("{}", user_2.username);

    let user_3 = build_user_from_user(String::from("s"), user_2);

    println!("{}", user_3.username);

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;

    let rect = Rectangle{width: 12, height: 15};
    println!("area is {}", rect.area());
}

fn build_user (username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}


// 使用结构体更新语法
// 旧的结构体在使用更新语法后就交出了所有权，不能再被使用了
fn build_user_from_user (username: String, user: User) -> User {
    User {
        username,
        ..user
    }
}