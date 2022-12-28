/**
 * 在某种程度上，任何编程语言中的信道都类似于单所有权，因为一旦将一个值传送到信道中，将无法再使用这个值
 * 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。
 * 
 * 互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据
 * 为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据
 * 锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，我们描述互斥器为通过锁系统 保护（guarding）其数据
 * 互斥器以难以使用著称，因为你不得不记住：
 * 1. 在使用数据之前尝试获取锁
 * 2. 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁
 */

#[cfg(test)]
mod test {
    use std::{sync::{Mutex, Arc}, thread, rc::Rc};

    // 互斥器一次只允许一个线程访问数据
    #[test]
    fn mutual_exclusion_api() {
        let num = Mutex::new(5);
        {
            let mut num = num.lock().unwrap();
            *num = 6;
        }
        println!("num: {:?}", num);
    }


    // 在线程间共享 Mutex<T>
    #[test]
    fn mutex_between_threads() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter: {:?}", counter);
    }
}
