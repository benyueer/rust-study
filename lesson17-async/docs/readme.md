# 为什么使用Async

## 其他的并发模型
- OS 线程
  - 无需改变编程模型，线程间同步困难，性能开销大
  - 线程池可以降低一些成本，但难以支撑大量IO绑定的工作
- Event-driven 编程
  - 与回调函数一起用，可能高效
  - 非线性的控制流，数据流和错误传播难以追踪
- Coroutines
  - 类似线程，无需改变编程模型
  - 类似async，支持大量任务
  - 抽象掉了底层细节
- Actor 模型
  - 将所有并发计算划分为actor，消息通信易出错
  - 可以有效的实现Actor模型，但许多实际问题没解决

## Rust 中的 async
- Future 是惰性的
  - 只有 poll 时才能取得进展；被丢弃的 future 就无法取得进展
- Async 是零成本的
  - 使用 async 可以无需堆内存分配和动态调度，对性能大好，且允许在受限环境使用 async
- 不提供内置运行时
  - 运行时由社区提供
- 蛋线程、多线程均支持
  - 但优缺点不同

## Rust 中的 async 和 线程
- OS 线程
  - 适用于少量任务，有内存和CPU开销，且线程的创建和切换消耗很大
  - 线程池可以降低一些成本
  - 允许重用同步代码，代码无需大改，无需特定编程模型
  - 有些系统支持修改线程优先级
- Async
  - 显著降低内存和CPU消耗
  - 同等条件下，支持比线程多几个数量级的任务
  - 可执行文件大 

# async/await 入门
async
- async 把一段代码转化成一个实现`Ruture trait`的状态机
- 虽然在同步方法中调用阻塞函数会阻塞整个线程，但阻塞的 Future 将放弃对线程的控制，从而允许其他 Future 来执行

async fn
- 异步函数语法
  ```rs
  async fn do_something() {
    /** */
  }

  // async fn 返回的是一个 Future，需要一个执行者来运行
  ```
- `futures::executor::block_on`
  - block_on 阻塞当前线程，直到提供的 Future 运行完成
  - 其他执行者提供更复杂的行为，例如将多个 Future 安排到同一线程上

await
- 在 async fn 中，可以使用`.await`等待另一个实现 Future trait 的完成
- 与 block_on 不同 .await 不会阻塞当前线程，而是异步的等待 Future 的完成（如果该 Future 目前无法取得进展，就允许其他任务执行）



# 幕后原理 Future trait

## Future trait
Future trait 是 Rust Async 编程的核心
Future 是一种异步运算，它可以产生一个值
实现了 Future trait 的类型表示 *目前可能还不可用的值*
下面是一个简化版 Future：
```rs
trait SampleFuture {
  type Output;
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
  Ready(T),
  Pending,
}
```

- Future 代表着一种你可以*检验其是否完成*的操作
- Future 可以通过调用 poll 函数取得进展：
  - poll 函数会驱动 Future 尽可能接近完成
  - 如果 Future 完成：就会返回 `poll::Ready(result)` ，其中 result 就是最终结果
  - 如果 Future 还未完成：就返回`poll::Pending`，并当 Future 准备好取得更多进展时调用一个 waker 的 wake 函数
- 针对 Future，你唯一能做的就是一直用 poll 来敲它，直到一个值掉出来


wake 函数
- 当 wake 函数被调用时
  - 执行器将驱动 Future 再次调用 poll 函数，以便 Future 取得更多进展
- 没有 wake 函数，执行器就不知道特定的 Future 何时能取得进展（不断的 poll）
- 通过 wake 函数，执行器就确切的知道哪些 Future 已经准备好进行 poll 的调用

例子：
```rs
pub struct SocketRead<'a> {
  socket: &'a Socket,
}

impl SimpleFuture for SockerRead<'_> {
  type Output = Vec<u8>;

  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if self.socket.has_data_to_read() {
      Poll::Ready(self.socket.read_buf())
    } else {
      self.socket.set_readable_allback(wake);
      Poll::Pending
    }
  }
}
```

例子2:
组合多个异步操作，无需中间分配
可以通过无分配的状态机来实现多个 Future 同时运行或串行
```rs
pub struct Join<FutureA, FutureB> {
  a: Option<FutureA>,
  b: Option<FutureB>,
}


impl<FutureA, FutureB> SampleFuture for Join<FutureA, FutureB>
where
  FutureA: SampleFuture<Output = ()>,
  FutureB: SampleFuture<Output = ()>,
{
  type Output = ();

  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if let Some(a) = &mut self.a {
      if let Poll::Ready(()) = a.poll(wake) {
        self.a.take();
      }
    }

    if let Some(b) = &mut self.b {
      if let Poll::Ready(()) = b.poll(wake) {
        self.b.take();
      }
    }  

    if self.a.is_none() && self.b.is_none() {
      Poll::Ready(())
    } else {
      Poll::Pending
    }
  }
}
```

例子3:
多个 Future 串行
```rs
pub struct AndThenFut<FutureA, FutureB> {
  first: Option<FutureA>,
  second: FutureB,
}

impl<FutureA, FutureB> SampleFuture for AndThenFut<FutureA, FutureB>
where
  FutureA: SimpleFuture<Output = ()>,
  FutureB: SimpleFuture<Output = ()>,
{
  type Output = ();

  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if let Some(first) = &mut self.first {
      match first.poll(wake) {
        Poll::Ready(()) => self.first.take(),
        Poll::Pending => return Poll::Pending,
      };
    }

    self.second.poll(wake)
  }
}
```


真正的 Future trait：
```rs
trait Future {
  type Output;

  fn poll(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>
  ) -> Poll<Self::Output>;
}
```


## 使用 Wake 唤醒任务

### Waker 类型的作用
- Future 在第一次 poll 的时候通常无法完成任务，所以 Future 需要保证在准备好取得更多进展后可以再次被 poll
- 每次 Future 被 poll，他都是作为一个任务的一部分
- 任务（Task）就是被提交给执行者的顶层 Future
- Waker 提供了 wake 方法，它可以被用来告诉执行者：相关的任务应该被唤醒
- 当 wake 被调用，执行者知道 Waker 所关联的任务已经准备好取得更多进展，Future 应当再次被 poll
- Waker 实现了 Clone，可以复制和存储

例子，使用 Waker 实现定时器（见 TimerFuture）


## Future 执行者（Executor）
- Future 是惰性的：除非驱动他们来完成，否则就什么都不做
- 一种驱动方式是在 async 函数里使用 `.await`，但这只是把问题推到了上一层面
- 谁来执行顶层 async 函数返回的 Future
  - 需要一个 Future 执行者
- Future 执行者会获取一系列顶层的 Future，通过在 Future 可能有进展的时候调用 poll，来将这些 Future 运行至完成
- 通常，执行者会 poll 一个 Future 一次以开始
- 当 Future 通过调用 wake 表示他们已经准备好取得进展时，他们就会被放回到一个队列里，然后 poll 再次被调用，重复此操作直到 Future 完成

例子，构造简单执行者



# async & .await
async .await 是 Rust 的特殊语法，在发生阻塞时，他让放弃当前线程的控制权成为可能，这就允许在等待操作完成的时候，允许其他代码取得进展

## 使用 async 的两种方式
```rs
async fn foo() -> u8 {5}

fn bar() -> impl Future<Output = u8>{
  async {
    let x: u8 = foo().await;
    x+5
  }
}
```
- async fn 和 async blocks 


## async 的生命周期
- 于传统函数不同，如果 async fn 的参数是引用或是其他非 `'static` 的，那么他返回的 Future 将绑定到参数的生命周期上
- 这意味着 async fn 返回的 Future 在 .await 的同时，fn 的非 'static 参数必须保持有效

```rs
async fn foo(x: &u8) -> u8 {*x}

fn foo_expanded<'a'>(x: &'a u8) -> impl Future<Output = u8> + 'a {
  async move {
    *x
  }
}
```

## 存储 或 传递 Future 
- 通常，async fn 会在调用后立即 await ，这就不是问题
- 一种变通解决办法
  - 思路：把使用引用作为参数的 async fn 转为一个 'satic future
  - 做法：在 async 块里，将参数和 async fn 的调用绑定到一起（延长参数的生命周期来匹配 future）
```rs
fn bad() -> impl Future<Output = u8> {
  let x = 8;
  borrow_x(&x)
}

fn good() -> impl Future<Output = u8> {
  async {
    let x = 6;
    borrow_x(&x).await
  }
}
```

## async move
- async 块和闭包都支持 move
- async move 会获得其引用变的所有权：
  - 允许其比当前作用域活得长
  - 但同时也放弃了与其他代码共享这些变量的能力


# Pin
Pin 会保证指针指向的值不被移动

Unpin
- 大多数类型如果被移动，不会造成问题，他们实现了 Unpin
- 指向 Unpin 类型的指针，可自由的放入或从 Pin 取出
  - 例如 u8 是 Unpin 的，`Pin<&mut u8>`和普通的 `&mut u8`一样
- 如果类型拥有`!Unpin`标记，那么在 Pin 之后他们就无法移动了

- 如果`T:Unpin`（默认情况），那么`Pin<'a, T>`与`&'a mut T`完全等价
  - Unpin 意味着该类型如果被 Pin 了，那么他也是可以移动的，所以 Pin 对这种类型没有作用
- 如果`T:!Unpin`，那么把`&mut T`变成`Pin<T>`，需要 unsafe 操作
- 大多数标准库类型都实现了 Unpin，Rust 里大多数正常类型也是。由 async/await 产生的 Future 是一个例外
- 可以使用特性标记为类型添加一个 `!Unpin`绑定，或者通过添加`std::marker::PhantomPinned`到类型上
- 可以把数据 Pin 到 Stack 或 Heap 上
- 把`!Unpin`对象 Pin 到 Stack 上需要 unsafe 操作
- 把`!Unpin`对象 Pin 到 Heap 上不需要 unsafe 操作
  - 快捷操作：使用`Box::pin`
- 针对已经 Pin 的数据，如果他是`T:!Unpin`的，则需要他保证在被 Pin 后，内存一直有效且不会调整其用途，直到 drop 被调用
  - 这是 Pin 合约的重要部分


# 调用 Future 的方法
- join!：等待所有 future 完成
- select!：等待多个 future 中的一个完成
- Spawning：创建一个顶级任务，他会运行一个 future 直至完成
- FuturesUnordered：一组 future，他们会产生每个子 future 的结果

## join 宏
来自`futures::join`
等待所有 future 完成
`try_join`
- 对于返回 Result 的 future，更考虑使用 `try_join`
  - 如果子 future 中某一个返回了错误，try_join 会立即完成

## select 宏
在任意一个子 future 完成时响应
