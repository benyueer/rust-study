
use function_like_macro::make_answer;
make_answer!();

#[test]
fn test() {
    println!("123{}", answer());
}

#[test]
fn test1() {
    println!("dadasda")
}
