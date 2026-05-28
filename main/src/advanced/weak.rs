// Weak 简单介绍
// Rust 的 Rc<T> 允许多个所有者共享数据，但如果 Rc<T> 之间形成循环引用，数据的引用计数永远不会归零，导致内存泄漏。


// Weak 解决循环引用
// Weak<T> 是 Rc<T> 的弱引用，它不增加引用计数，因此不会阻止数据释放。

// Weak vs Rc 对比

// 特性	Rc<T>	Weak<T>
// 引用计数	递增	不计入
// 是否拥有数据	是	否
// 是否阻止释放	是	否
// 访问方式	直接使用	upgrade() 返回 Option<Rc<T>>


use std::rc::{Rc, Weak};

fn main() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five); // 创建 Weak 引用

    println!("{:?}", weak_five.upgrade()); // Some(Rc(5))

    drop(five); // 释放 Rc

    println!("{:?}", weak_five.upgrade()); // None
}
// downgrade(&Rc<T>) 创建 Weak<T>，不会影响 Rc 计数。
// upgrade() 返回 Option<Rc<T>>，如果数据已释放，则返回 None。



// Weak 解决循环引用
// 场景：父子关系

// 父节点持有子节点的 Rc<T>（多个子节点）。
// 子节点使用 Weak<T> 关联父节点（避免循环引用）。

use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 弱引用父节点
    children: RefCell<Vec<Rc<Node>>>, // 强引用子节点
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // 让 leaf 指向 branch

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
}
// branch 持有 leaf 的 Rc<T>（强引用）。
// leaf 持有 branch 的 Weak<T>（弱引用）。
// 这样 Rc 不会形成循环引用，避免内存泄漏。



// 总结
// Weak<T> 不增加引用计数，不阻止数据释放。
// 适用于父子关系：父 -> Rc<T>，子 -> Weak<T>。
// 使用 upgrade() 转换为 Rc<T>，检查数据是否仍然存在。
// 解决 Rc<T> 形成的循环引用，防止内存泄漏。