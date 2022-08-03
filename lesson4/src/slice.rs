/**
 * slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
 * 
 * * 字符串slice
 */

pub fn slice() {
    let mut s = String::from("hello world");

    let s_w = first_word(&s);

    // s.clear(); 此时不能修改 s ，因为他已经有了一个可变引用，这个引用直到 println结束才失效，根据引用的规则，当引用有效时，就不能有其他的可变引用
    // s = String::from("change"); 同样不允许
    
    println!("{}", s_w);

    s = String::from("change"); // 引用结束后可以修改

    println!("{}", s);

    /*
     * 字符串字面值是不可变的, 如下 s_1的类型是 &str， 代表一个不可变引用
     */
    let s_1 = "hello";

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c== b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
