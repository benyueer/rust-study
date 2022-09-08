/**
 * String的常用方法
 */

pub fn string() {
    // #![feature(string_remove_matches)]
    // let mut s = String::from("hello");
    // s.remove_matches("he");
    // println!("{}", s);
}

#[cfg(test)]
mod test {

    #[test]
    fn string_new() {
        /*
            * new
            创建新的 空 String
        */
        let s1 = String::new();
    }

    #[test]
    fn string_with_capacity() {
        /*
            * with_capacity(容量)
            创建具有特定容量的新 String
            通过 capacity 方法获取容量
        */
        let mut s = String::with_capacity(10);
        let cap = s.capacity();

        for _ in 0..10 {
            s.push('a');
        }

        assert_eq!(s.capacity(), cap);

        // 超过容量后会重新分配
        s.push('b');
        println!("s is {}", s);

        // 重新分配后扩容为20
        assert_eq!(s.capacity(), 20);
    }

    #[test]
    fn string_into_bytes() {
        /*
           * into_bytes() -> Vec<u8, Clobal>
           将 String 转为 字节数组
        */
        let s = String::from("hello");
        let bytes = s.into_bytes();

        bytes.into_iter().for_each(|b| {
            println!("byte: {}", b);
        })
        /*
        byte: 104
        byte: 101
        byte: 108
        byte: 108
        byte: 111
         */
    }

    #[test]
    fn string_as_str() {
        /*
            * as_str() -> &str
            获取整个 string 的切片
        */
        let mut s = String::from("hello");

        let str = s.as_str();
        // str.make_ascii_uppercase();  cannot borrow `*str` as mutable

        /*
            * as_mut_str() -> &str
            获取可变字符串切片
        */
        let mut_str = s.as_mut_str();
        mut_str.make_ascii_uppercase();
        assert_eq!("HELLO", mut_str);
    }

    #[test]
    fn string_push_str() {
        /*
            * push_str(&mut self, str: &str)
            将给定的 切片 添加到 string 末尾
        */
        let mut s = String::from("hello");
        s.push_str(" world");

        assert_eq!("hello world", s.as_str());
    }

    #[test]
    fn string_remove() {
        /*
        * remove(&mut self, idx: usize) -> char
        删除指定位置的字符并返回
        */
        let mut s = String::from("hello");
        let char = s.remove(2);
        assert_eq!(char, 'l');

        /*
        * remove_matches<P>(&'a mut self, pat: P)
        删除匹配部分
        */
        let mut s = String::from("hello world");
        // s.remove_matches("he");
        assert_eq!("llo world", s);
    }

    #[test]
    fn string_retain() {
        /*
            * retain<F>(&mut self, fn: F)
            F: (char) -> bool
            仅保留指定字符
        */
        let mut s = String::from("hello world");
        s.retain(|c| c != ' ');
        assert_eq!(s, "helloworld");
    }

    #[test]
    fn string_insert() {
        /*
            * insert(&mut self, ind: usize, ch: char)
            在指定位置插入字符
        */
        let mut s = String::from("hello");
        s.insert(3, 'd');
        assert_eq!(s, "heldlo");

        /*
            * insert_str(&mut self, ind: usize, str: &str)
            在指定位置插入字符穿切片
        */
        let mut s = String::from("hello");
        s.insert_str(5, " world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn string_chars() {
        /*
            * chars()
            在字符串切片上返回一个字符迭代器
        */

        let s = String::from("hello world");
        let mut chars = s.chars();
        assert_eq!(Some('h'), chars.next());

        /*
            * char_indices
            返回带下标的字符迭代器
        */
        let s = String::from("hello world");
        let mut chars = s.char_indices();
        assert_eq!(Some((0, 'h')), chars.next());
    }

    #[test]
    fn string_split_whitespace() {
        /*
            * split_whitespace
            根据空格切分string 返回迭代器
        */
        let mut iter = "hello world".split_whitespace();
        assert_eq!(Some("hello"), iter.next());

    }

    #[test]
    fn string_lines() {
        /*
            * lines
            按行返回迭代器
        */
        let s = "hello\nworld".to_string();
        let mut iter = s.lines();
        assert_eq!(Some("hello"), iter.next());
    }

    #[test]
    fn string_contains() {
        /*
            * contains
            判断是否包含
        */
        let s = String::from("hello world");
        assert!(s.contains("hello"));
        assert!(s.contains('w'));
        assert!(!s.contains("pat"));
    }

    #[test]
    fn string_starts_with() {
        /*
            * starts_with
            判断 string 是否由给定的切片开始
        */
        let s = String::from("hello");
        assert!(s.starts_with("he"));
        assert!(s.starts_with('h'));
        /*
            * ends_with
            判断结尾
        */
    }

    #[test]
    fn string_find() {
        /*
            * find
            返回第一个匹配的下标
        */
        let s = "hello world";
        assert_eq!(s.find("he"), Some(0));
        assert_eq!(s.find('w'), Some(6));
        assert_eq!(s.find('a'), None);

        /*
            * rfind
            从结尾开始匹配
        */
    }

    #[test]
    fn string_split() {
        /*
            * split<P>(pat: P)
            根据提供的模式匹配分隔字符串，返回迭代器
        */
        let s = "hello world";
        let mut v = s.split(' ');
        assert_eq!(Some("hello"), v.next());

        let mut iter = s.split(" ");
        assert_eq!(Some("hello"), iter.next());

        /*
            * split_inclusive
            将匹配部分作为终止字符
        */
        let mut iter = s.split_inclusive(" ");
        assert_eq!((Some("hello ")), iter.next());

        /*
            * rsplit
            分隔后反序
        */
    }

    #[test]
    fn string_replace() {
        /*
            * replace<P>(from: P, to: &str) -> string
            替换匹配部分
        */
        let s = "hello worle";
        assert_eq!("hdllo world", s.replace('e', "d"));

        /*
            * replacen
            替换前 n 个匹配项
        */

    }
}
