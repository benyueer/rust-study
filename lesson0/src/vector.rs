#[cfg(test)]
mod test {

    #[test]
    fn vec_new() {
        /*
            * new
            构造一个空vec
        */
        let v: Vec<i32> = Vec::new();
    }

    #[test]
    fn vec_with_capacity() {
        /*
            * with_capacity()
            构造指定容量的 vec
        */
        let v: Vec<i32> = Vec::with_capacity(10);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 10);
    }

    #[test]
    fn vec_swap_remove() {
        /*
            * swap_remove(ind: usize)
            移除一个元素并返回
            用最后一个元素替代他
        */

        let mut v = vec![1, 2, 3];
        assert_eq!(v.swap_remove(0), 1);
        assert_eq!(v, [3, 2]);
    }

    #[test]
    fn vec_insert() {
        /*
            * insert(ind: usize, element: T)
            在指定位置插入元素，位置后的元素右移
        */
        let mut v = vec![1, 2, 3];
        v.insert(0, 2);
        assert_eq!(v, [2, 1, 2, 3]);
    }

    #[test]
    fn vec_remove() {
        /*
            * remove(ind: usize)
            删除并返回指定位置的元素
        */
        let mut v = vec![1, 2, 3];
        assert_eq!(v.remove(1), 2);
    }

    #[test]
    fn vec_retain() {
        /*
            * retain(f)
            仅保留符合要求的元素
        */
        let mut v = vec![1, 2, 3];
        v.retain(|i| i > &2);
        assert_eq!(v, vec![3]);

        /*
            * retain_mut
            传递可变引用
        */
        let mut v = vec![1, 2, 3];
        v.retain_mut(|i| {
            *i *= 2;
            *i > 2
        });
        assert_eq!(v, vec![4, 6]);
    }

    #[test]
    fn vec_append() {
        /*
         * append(vec: &mut Vec<>)
         */
        // 将传入的 vec 移入当前 vec
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        v1.append(&mut v2);

        assert_eq!(v1, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn vec_drain() {
        /*
            * drain(range: R)
            删除指定范围的元素，作为迭代器返回
        */
        let mut v = vec![1, 2, 3];
        let it: Vec<i32> = v.drain(1..).collect();
        assert_eq!(v, [1]);
        assert_eq!(it, vec![2, 3]);
    }

    #[test]
    fn vec_get() {
        /*
            * get(range) -> Option
            返回指定元素或子切片的引用
        */
        let v = &[1, 2, 3];
        assert_eq!(Some(&1), v.get(0));
        assert_eq!(Some(&[1, 2][..]), v.get(0..2));
        assert_eq!(None, v.get(10));

        /*
            * get_mut
            返回指定元素或子切片的可变引用
        */
        let v = &mut [1, 2, 3];
        if let Some(ind_1) = v.get_mut(1) {
            *ind_1 += 3;
        }
        assert_eq!(v, &mut [1, 5, 3]);
    }

    #[test]
    fn vec_swap() {
        /*
            * swap(a: usize, b: size)
            交换切片中的两个元素
        */
        let v = &mut [1, 2, 3];
        v.swap(1, 2);
        assert_eq!(v, &mut [1, 3, 2]);
    }

    #[test]
    fn vec_reverse() {
        /*
            * reverse
            就地反转
        */
        let mut v = [1, 2, 3];
        v.reverse();
        assert_eq!(v, [3, 2, 1]);
    }

    #[test]
    fn vec_iter() {
        /*
            * iter() -> Iter
            返回迭代器
        */
        let v = [1, 2, 3];
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));

        /*
            * iter_mut()
            返回可变引用的迭代器
        */
        let mut v = [1, 2, 3];
        let mut iter = v.iter_mut();

        for i in iter {
            *i *= 2;
        }
        assert_eq!(v, [2, 4, 6]);

        /*
            * chunks(len: i32)
            返回指定步长的迭代器
        */
        let v = [1, 2, 3];
        let mut iter = v.chunks(2);
        assert_eq!(iter.next().unwrap(), &[1, 2]);
        assert_eq!(iter.next().unwrap(), &[3]);

        /*
            * chunk_mut
            返回可变引用的迭代器
        */
        let mut v = [1, 2, 3];
        let iter = v.chunks_mut(2);
        for s in iter {
            for item in s.iter_mut() {
                *item *= 2;
            }
        }
        assert_eq!(v, [2, 4, 6]);
    }

    #[test]
    fn vec_contains() {
        /*
            * contains(x: &T)
            判断是否包含给定元素
        */
        let v = [1, 2, 3];
        assert_eq!(true, v.contains(&1));

        /*
            * starts_with(needles: &[T])
            判断开始
        */
        /*
            * ends_with()
            判断结束
        */

        /*
            * binary_search(x: T) -> Result
            返回指定元素下标，多个匹配则可以返回任何一个匹配项
        */
        let v = [1, 2, 3, 3];
        assert_eq!(v.binary_search(&1), Ok(0));
        let s = v.binary_search(&3);
        println!("{:?}", s);
    }

    #[test]
    fn vec_sort() {
        /*
            * sort_unstable()
            排序，但不保证相同元素的顺序
        */
        let mut v = [2, 1, 2, 3, 4];
        v.sort_unstable();
        assert_eq!(v, [1, 2, 2, 3, 4]);

        /*
            * sort_unstable_by(f: F)
            不稳定排序，提供一个比较函数
        */
        let mut v = [3f64, 1.2, 3.0, 2.4];
        v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(v, [1.2, 2.4, 3.0, 3.0]);

        /*
            * sort()
            稳定排序
        */
        /*
            * sort_by()
            使用比较函数稳定排序
        */
    }

    #[test]
    fn vec_fill() {
        /*
            * fill(v: T)
            通过clone值填充vec
        */
        let mut v = [0; 10];
        v.fill(1);
        assert_eq!(v, [1; 10]);

        /*
            * fill_with(f: F)
            使用多次调用闭包返回值填充vec
        */
        let mut v = [0; 10];
        let mut a = 0;
        v.fill_with(|| {a += 1; return a;});
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn vec_concat() {
        /*
            * concat()
            将cev 扁平并转换为单个值
        */
        let v = ["hello", "world"];
        assert_eq!(v.concat(), "helloworld");
        assert_eq!([[1, 2], [3, 4]].concat(), [1,2, 3,4]);

        /*
            * join(sep)
            将切片扁平化并转换为单个值，在每两个值之间插入给定值
        */
        assert_eq!(["hello", "world"].join(" "), "hello world");
        assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3 ,4]);
        assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
    }
}
