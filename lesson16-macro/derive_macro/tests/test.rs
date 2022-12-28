use derive_macro::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[test]
fn test() {
    Sunfei::hello_macro();
}