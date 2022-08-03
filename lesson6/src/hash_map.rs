/**
 * hashMap
 */
use std::collections::HashMap;
pub fn hash_map() {
    // 创建
    // new
    let mut map_1: HashMap<String, i32> = HashMap::new();
    
    // 通过 vec 的 迭代器（iterator）和 collect 方法
    let key = vec!["hello".to_string(), "world".to_string()];
    let value = vec![1, 2];
    let mut map_2: HashMap<String, i32> = key.into_iter().zip(value.into_iter()).collect();


    // 添加值
    map_1.insert("key_1".to_string(), 2);

    // 访问值
    // get
    let key_1_value = map_1.get("key_1");
    
    // 遍历
    for (key, value) in &map_1 {
        print!("{}, {}", key, value);
    }


    // 更新
    // 覆盖一个值
    map_1.insert("key_2".to_string(), 2);
    map_1.insert("key_2".to_string(), 3);

    // 只在键没有对应值时插入
    map_1.entry("key_2".to_string()).or_insert(5);
    map_1.entry("key_3".to_string()).or_insert(5);

}