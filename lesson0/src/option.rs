#[cfg(test)]
mod test {
    #[test]
    fn option_is_some() {
        /*
            * is_some
            判断是否为Some
        */
        let x: Option<i32> = Some(12);
        assert!(x.is_some());

        /*
            * is_some_and
            padnuan
        */

        /*
         * is_none
         */
    }

    #[test]
    fn option_expect() {
        /*
            * expect()
            返回包含值，为None则退出并显示给定值
        */
        let a: Option<String> = Some("hello".to_string());
        let x: Option<&str> = None;
        assert_eq!(a.expect(""), "hello");
        // assert_eq!(x.expect("hello"), "hello");
    }

    #[test]
    fn option_unwrap() {
        /*
            * unwrap
            返回包含值
        */
        let a: Option<i32> = Some(12);
        let x: Option<i32> = None;
        assert_eq!(a.unwrap(), 12);
        // assert_eq!(x.unwrap(), 12); 退出

        /*
            * unwrap_or()
            返回包含值或提供的默认值
        */
        assert_eq!(x.unwrap_or(13), 13);

        /*
            * unwrap_or_else(f)
            返回包含值或闭包结果
        */
        let c = 15;
        assert_eq!(x.unwrap_or_else(|| c), 15);

        /*
            * unwrap_or_default()
            返回包含值或该类型默认值
        */
        let s1 = "123";
        let s2 = "123fafsad";
        assert_eq!(s1.parse::<i32>().ok().unwrap_or_default(), 123);
        assert_eq!(s2.parse::<i32>().ok().unwrap_or_default(), 0);
    }

    #[test]
    fn option_as_ref() {
        /*
         * as_ref
         */

        /*
         * take(&mut self) -> Option<T>
         */
    }

}
