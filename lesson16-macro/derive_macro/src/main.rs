use derive_macro::HelloMacro;

#[derive(HelloMacro)]
struct Hello {
    #[hello(value = 100)]
    data: i32
}

fn main() {
    
}

