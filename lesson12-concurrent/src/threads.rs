#[cfg(test)]
mod Test {
    use std::{thread, time::Duration};

    // 使用 spawn 创建新线程
    #[test]
    fn spawn_test() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("{} of spawn thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // 使用 join 等待所有线程结束
        handle.join().unwrap();
    }


    // 线程与 move 闭包
    #[test]
    fn move_spawn() {
        let v = vec![1, 2];
        let handle = thread::spawn(move || {
            println!("spawn v: {:?}", v);
        });

        handle.join().unwrap();
    }
}