/**
 * 一个日益流行的确保安全并发的方式是 消息传递（message passing），这里线程或 actor 通过发送包含数据的消息来相互沟通
 * 这个思想来源于 Go 编程语言文档中 的口号：“不要通过共享内存来通讯；而是通过通讯来共享内存。
 *
 * Rust 中一个实现消息传递并发的主要工具是 信道（channel）
 */

#[cfg(test)]
mod test {
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn channel_test() {
        // 这里使用 mpsc::channel 函数创建一个新的信道
        // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // 信道的接收端有两个有用的方法：recv 和 try_recv
        // recv 方法会阻塞主线程执行直到从信道中接收一个值
        // 一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它
        // 当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了
        // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息
        let received = rx.recv().unwrap();
        println!("rec: {received}");
    }

    // 发送多个值并观察接收者的等待
    #[test]
    fn channel_mult() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = [1, 2, 4, 5];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1000));
            }
        });

        // 不再显式调用 recv 函数：而是将 rx 当作一个迭代器
        for rec in rx {
            println!("val: {rec}");
        }
    }

    // 通过克隆发送者来创建多个生产者
    #[test]
    fn mult_sender() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
