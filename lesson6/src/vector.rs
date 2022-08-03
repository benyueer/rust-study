pub fn vector () {
    // 新建vector
    let mut v_1: Vec<String> = Vec::new();
    let mut v_2: Vec<i32> = vec![1, 2, 3];

    // 更新vector
    v_1.push(String::from("hello"));
    v_1.push(String::from("world"));

    // 读取vector的元素
    // 下标
    let v_1_at_1 = &v_1[1];
    println!("v_1_at_1 is {}", v_1_at_1);
    println!("{}", v_1[1]);

    // get
    match v_1.get(0) {
        Some(v_1_at_0) => println!("{}", v_1_at_0),
        None => println!("none"),
    }
    match v_1.get(10) {
        Some(v_1_at_0) => println!("{}", v_1_at_0),
        None => println!("none"),
    }


    // 遍历
    for i in &mut v_1 {
        (*i).push_str(",");
        print!("{}", i);
    }

    for i in &mut v_2 {
        *i += 1;
        print!("{}", i);
    }
}