use futures::executor::block_on;

async fn hello_world() {
    println!("hello world")
}

pub fn main() {
    let future = hello_world();
    block_on(future);
}

async fn learn_sing() -> String {
    "a sang".to_string()
}

async fn sing_sang(sang: String) {

}

async fn dance() {}

async fn learn_and_sing() {
    let sang = learn_sing().await;
    sing_sang(sang).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

pub fn do_sing_and_other() {
    block_on(async_main())
}
