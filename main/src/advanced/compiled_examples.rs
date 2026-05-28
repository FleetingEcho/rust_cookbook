use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Rc<T> 用于单线程共享所有权；RefCell<T> 把借用检查从编译期推迟到运行期。
// 学习重点：Rc 解决“多个所有者”，RefCell 解决“内部可变性”。
pub fn rc_refcell_counter() -> i32 {
    let counter = Rc::new(RefCell::new(0));
    let a = Rc::clone(&counter);
    let b = Rc::clone(&counter);

    *a.borrow_mut() += 1;
    *b.borrow_mut() += 2;

    let value = *counter.borrow();
    value
}

// Arc<T> 是线程安全的引用计数指针；Mutex<T> 保证同一时间只有一个线程修改数据。
// 学习重点：跨线程共享数据通常需要 Arc<Mutex<T>>。
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

// Drop trait 可以自定义离开作用域时的清理逻辑。
pub struct DropMessage {
    pub name: String,
}

impl Drop for DropMessage {
    fn drop(&mut self) {
        println!("释放资源: {}", self.name);
    }
}

// Deref 让自定义类型可以像引用一样被使用。
pub struct MyBox<T>(pub T);

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
