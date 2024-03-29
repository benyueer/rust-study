/**
 * Rust 的并发模型中一个有趣的方面是：语言本身对并发知之 甚少
 * 我们之前讨论的几乎所有内容，都属于标准库，而不是语言本身的内容
 * 由于不需要语言提供并发相关的基础设施，并发方案不受标准库或语言所限：我们可以编写自己的或使用别人编写的并发功能
 * 
 * 然而有两个并发概念是内嵌于语言中的：std::marker 中的 Sync 和 Send trait
 * 
 * 通过 Send 允许在线程间转移所有权
 * 实现了 Send 的类型值的所有权可以在线程间传送
 * 几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>：这是不能 Send 的，因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数
 * 任何完全由 Send 的类型组成的类型也会自动被标记为 Send
 * 几乎所有基本类型都是 Send 的，除了第十九章将会讨论的裸指针（raw pointer）
 * 
 * Sync 允许多线程访问
 * 一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用
 * 换一种方式来说，对于任意类型 T，如果 &T（T 的不可变引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程
 * 基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的
 * 
 * 
 * 手动实现 Send 和 Sync 是不安全的
 * 通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的
 */

mod test {

}