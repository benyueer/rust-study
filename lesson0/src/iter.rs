#[cfg(test)]
mod test {
    use std::iter::FilterMap;

    #[test]
    fn iter_count() {
        /*
            * count
            消费迭代器，返回迭代次数
        */
        let v = [1, 2, 3];
        let mut iter = v.iter();
        assert_eq!(3, iter.count());
    }

    #[test]
    fn iter_nth() {
        /*
            * nth(ind: usize)
            返回第ind个元素
        */
        let v = [1, 2, 3];
        let mut iter = v.iter();

        assert_eq!(Some(&2), iter.nth(1));
        // assert_eq!(Some(&2), iter.nth(1)); 不能重复消费
        assert_eq!(None, iter.nth(1));
        // assert_eq!(Some(&1), iter.nth(0)); 不能向前消费
    }

    #[test]
    fn iter_step_by() {
        /*
            * step_by
            按照给定的步长迭代
        */
        let mut iter = [1, 2, 3, 4, 5].iter().step_by(2);
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&3), iter.next());
    }

    #[test]
    fn iter_chain() {
        /*
            * chain(iter)
            将给定迭代器链接到指定迭代器后，返回新迭代器
        */
        let v1 = [1, 2, 3];
        let v2 = [4, 5, 6];
        let mut iter = v1.iter().chain(v2.iter());
        assert_eq!(Some(&4), iter.nth(3));
    }

    #[test]
    fn iter_zip() {
        /*
            * zip(iter)
            将两个迭代器压缩成一个成对的迭代器，返回新迭代器
        */
        let v1 = [1, 2, 3];
        let v2 = [4, 5, 6];
        let mut iter = v1.iter().zip(v2.iter());
        assert_eq!(Some((&1, &4)), iter.next());
    }

    #[test]
    fn iter_map() {
        /*
            * map()
            接收一个闭包，创建一个迭代器，这个迭代器在每个元素上调用该闭包
        */
        let v = [1, 2, 3];
        let mut iter = v.into_iter().map(|mut _i| _i * 2);
        assert_eq!(Some(2), iter.next());
    }

    #[test]
    fn iter_for_each() {
        /*
            * for_each()
            对迭代器的每个元素调用闭包
        */

        let mut iter = (0..6).for_each(|f| println!("{}", f));
    }

    #[test]
    fn iter_filter() {
        /*
            * filter
            执行闭包来确定新迭代器是否具有这一项
        */
        let mut iter = (0..4).filter(|f| f % 2 == 0);
        assert_eq!(Some(2), iter.nth(1));
    }

    #[test]
    fn iter_filter_map() {
        /*
            * filter_map
            创建一个即过滤又映射的迭代
        */

        let mut v = ["1", "two", "NaN", "four", "5"];
        // let mut iter = v.iter().filter_map(|f| f.parse().ok());
    }

    #[test]
    fn iter_enumerate() {
        /*
            * enumerate
            创建包括索引的迭代器
        */
        let mut iter = [1, 2, 3].iter().enumerate();
        assert_eq!(Some((0, &1)), iter.nth(0));
    }

    #[test]
    fn iter_take() {
        /*
            * take(n: usize)
            创建前n项的迭代器，如果旧迭代器长度小于n，会提前退出
        */
        let mut iter = [1, 2, 3].iter().take(2);
        assert_eq!(Some(&1), iter.next());
    }

    #[test]
    fn iter_collect() {
        /*
            * collect
            将迭代器转为集合
        */
        let v = [1, 2, 3];
        let v1: Vec<i32> = v.iter().map(|&a| a*2).collect();
        assert_eq!(v1, [2, 4, 6]);
    }
}
