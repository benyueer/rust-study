#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn map_new() {
        /*
            * new
            新建
        */
        let m: HashMap<String, i32> = HashMap::new();

        /*
            * from()
            通过元组数组生成
        */
        let m: HashMap<&str, i32> = HashMap::from([("item1", 1), ("item2", 2), ("item3", 3)]);

        /*
            * from_iter
        */
        let v1 = ['c', 'e'];
        let v2 = [3, 5];
        let iter = v1.into_iter().zip(v2.into_iter());
        let m: HashMap<char, i32> = HashMap::from_iter(iter);
        for (k, v) in m {
            println!("{k} {v}");
        }
    }

    #[test]
    fn map_iter() {
        /*
            * keys() -> Iter
            返回键的迭代器, 不保证顺序
        */
        let mut m: HashMap<&str, i32> = HashMap::from([("item1", 1), ("item2", 2), ("item3", 3)]);

        for k in m.keys() {
            println!("{k}");
        }

        /*
            * into_keys
            返回key的迭代器，map无法使用
        */
        let mut m: HashMap<&str, i32> = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

        let mut keys: Vec<&str> = m.into_keys().collect();
        // m.get("a"); // borrow of moved value: `m`
        keys.sort_unstable();
        assert_eq!(keys, ["a", "b", "c"]);

        /*
            * values
            返回值的引用迭代器
        */

        let m: HashMap<char, i32> = HashMap::from([('a', 1), ('b', 2), ('c', 3)]);
        for v in m.values() {
            println!("{v}");
        }

        /*
            * values_mut
            返回一个可修改的值的迭代器
        */
        let mut m: HashMap<char, i32> = HashMap::from([('a', 1), ('v', 3), ('g', 6)]);
        for v in m.values_mut() {
            *v += 2;
        }

        let mut values: Vec<&i32> = m.values().collect();
        values.sort_unstable();

        assert_eq!(values, [&3, &5, &8]);

        /*
            * into_values
            创建值的迭代器，原map被消费无法使用
        */
        let mut m: HashMap<char, i32> = HashMap::from([('c', 2), ('g', 7), ('f', 3)]);

        let mut values: Vec<i32> = m.into_values().collect();
        values.sort();
        assert_eq!(values, [2, 3, 7]);

        /*
            * iter
            返回迭代器，元素为key value元组
        */
        let mut m: HashMap<char, i32> = HashMap::from([('c', 3), ('h', 7), ('j', 3)]);

        let iter = m.iter();
        for i in iter {
            println!("{:?}", i);
        }

        /*
            * iter_mut
            返回值可变引用的 键值对元组迭代器
        */
        let mut m: HashMap<char, i32> = HashMap::from([('q', 4), ('g', 5), ('u', 7), ('t', 3)]);

        for (k, v) in m.iter_mut() {
            *v += 2;
        }

        for (key, val) in &m {
            println!("{key} {val}");
        }

        /*
            * into_iter
            返回迭代器，消费该map
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        m.insert('k', 12);
        for (k, v) in m.into_iter() {
            println!("{k} {v}");
        }
    }

    #[test]
    fn map_entry() {
        /*
            * entry
            获取给定 key 的 value 的可变引用
        */
        let mut m: HashMap<char, i32> = HashMap::new();

        for c in "hello world".chars() {
            let counter = m.entry(c).or_insert(0);
            *counter += 1;
        }

        for (k, v) in &m {
            println!("{k} {v}");
        }
    }

    #[test]
    fn map_get() {
        /*
            * get(& key) -> Option
            根据指定的 key 返回value 的引用
        */
        let mut m: HashMap<char, i32>  = HashMap::new();
        m.insert('c', 2);
        assert_eq!(m.get(&'c'), Some(&2));
        assert_eq!(m.get(&'v'), None);


        /*
            * get_key_value(& key) -> Option
            根据提供的 key 返回 键值对引用元组
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        m.insert('k', 12);
        assert_eq!(m.get_key_value(&'k'), Some((&'k', &12)));

        /*
            * get_mut(& key) -> Option
            根据提供的 key 返回 value 的可变引用
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        m.insert('k', 23);
        if let Some(v) = m.get_mut(&'k') {
            *v += 2;
        }
        assert_eq!(m[&'k'], 25);


        /*
            * insert(k, v) -> Option
            插入键值对，如果map不存在该 key 返回 None， 否则返回旧的 value
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        assert_eq!(m.insert('k', 2), None);
        assert_eq!(m.insert('k', 45), Some(2));


        /*
            * remove(& k) -> Option
            移除键值对，如果存在返回值的引用，否则返回None
        */
        let mut m: HashMap<char, i32> = HashMap::from([
            ('k', 3)
        ]);
        assert_eq!(m.remove(&'f'), None);
        assert_eq!(m.remove(&'k'), Some(3));


        /*
            * remove_entry(& key) -> Option
            移除指定 key 的键值对， 存在返回元组
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        m.insert('k', 23);
        assert_eq!(m.remove_entry(&'f'), None);
        assert_eq!(m.remove_entry(&'k'), Some(('k', 23)));
    }

    #[test]
    fn map_contains_key() {
        /*
            * contains_key(& key) -> bool
            返回是否包含 key
        */
        let mut m: HashMap<char, i32> = HashMap::new();
        m.insert('k', 3);
        assert!(m.contains_key(&'k'));
    }
}
