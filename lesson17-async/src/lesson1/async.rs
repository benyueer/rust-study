use futures::executor::block_on;
use futures::Future;


fn bar() -> impl Future<Output = u8> {
    async {
        5
    }
}


pub fn main() {
    println!("{}", block_on(bar()));
}