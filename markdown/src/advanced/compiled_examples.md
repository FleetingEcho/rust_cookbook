# Advanced Compiled Examples

## 概述

本文件包含可稳定编译的高级主题示例，方便 `cargo check/test` 覆盖。

> 注意：目录下其他笔记文件是"片段合集"，同一个文件里可能有多个 `main` 函数或重复类型名，所以它们适合作为阅读材料，不适合直接全部接入模块树编译。

## 1. Rc + RefCell 计数器

`Rc<T>` 用于单线程共享所有权；`RefCell<T>` 把借用检查从编译期推迟到运行期。

**学习重点：** Rc 解决"多个所有者"，RefCell 解决"内部可变性"。

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub fn rc_refcell_counter() -> i32 {
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 2;

    let value = *counter.borrow();
    value
}
```

## 2. Arc + Mutex 计数器

`Arc<T>` 是线程安全的引用计数指针；`Mutex<T>` 保证同一时间只有一个线程修改数据。

**学习重点：** 跨线程共享数据通常需要 `Arc<Mutex<T>>`。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_mutex_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().expect("锁被污染，说明持锁线程发生过 panic");
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("线程执行失败");
    }

    let value = *counter.lock().expect("锁被污染");
    value
}
```

## 3. Drop Trait — 自定义清理逻辑

```rust
pub struct DropMessage {
    pub name: String,
}

impl Drop for DropMessage {
    fn drop(&mut self) {
        println!("释放资源: {}", self.name);
    }
}
```

## 4. Deref — 自定义解引用

```rust
pub struct MyBox<T>(pub T);

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

## 5. 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_refcell_counts_inside_one_thread() {
        assert_eq!(rc_refcell_counter(), 3);
    }

    #[test]
    fn arc_mutex_counts_across_threads() {
        assert_eq!(arc_mutex_counter(), 4);
    }

    #[test]
    fn my_box_deref_works() {
        let value = MyBox(String::from("rust"));
        assert_eq!(value.len(), 4);
    }
}
```
