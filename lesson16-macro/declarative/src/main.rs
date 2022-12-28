mod macre_rules_r;

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let v = myvec!(1, 2, 3, 4);
    println!("{:?}", v);

    let a = add!(1);
    println!("---- {a}");

    let b = add!(1, 2, u8);
    println!("{b}");

    let added = add_as!(1, 2, 3, 4);
    println!("{added}");

    let added_add = add!(1, 2, 3, 4);
    println!("{added_add}");

    // println!("a{:?} b{:?}", ok_or_return!(some_work(1, 2)), ok_or_return!(some_work(1, 0)));
    println!("a{:?} b{:?}", ok_or_return_err!(some_work(1, 2)), ok_or_return_err!(some_work(1, 4)));

    make_public!{
        #[derive(Debug)]
        struct name {
            s: i32,
            t: String,
            d: i64,
        }
    };

    println!("{:?}", name{s: 1, t: "a".to_string(), d: 2});

    Ok(())
}


fn some_work(i: u64, j: u64) -> Result<(u64, u64), String> {
    if i+j > 2 {
        Ok((i, j))
    } else {
        Err("err".to_string())
    }
}


