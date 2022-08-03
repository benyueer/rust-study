/**
 * 智能指针
 * 指针 是一个包含内存地址的变量的通用概念，这个地址可以 引用 或 指向 其他数据
 * Rust中最常见的指针如 引用
 *
 * 智能指针是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能
 *
 * 在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针 拥有 他们指向的数据
 *
 * 已介绍的智能指针：String 和 Vec<T>
 *
 * 智能指针通常用结构体实现
 * 实现了 Deref 和 Drop trait
 * Deref trait 允许智能指针结构体实例表现的像引用一样，这样就可以编写既用于引用、又用于智能指针的代码
 * Drop trait 允许我们自定义当智能指针离开作用域时运行的代码
 */

mod ptr_box;
mod deref;
mod drop;
mod rc;
mod ref_cell;
mod rc_cycle;


fn main() {
    // ptr_box::ptr_box();
    // deref::deref();
    // drop::drop_trait();
    // rc::rc();
    // ref_cell::ref_cell();
    rc_cycle::rc_cycle();
}
