#[cfg(test)]
mod test {
    #[test]
    fn result_ok() {
        /*
            * ok
            转换为Option ok->some(ok) err->none
        */
        let r: Result<i32, &str> = Ok(12);
        let x: Result<i32, &str> = Err("(ds)");
        assert_eq!(r.ok(), Some(12));
        assert_eq!(x.ok(), None);

        /*
            * err
            转换为Option，ok->none err->some(err)
        */
        assert_eq!(x.err(), Some("(ds)"));
        assert_eq!(r.err(), None);

        assert_eq!(r.unwrap(), 12);
    }
}