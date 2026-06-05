// 运行命令：cargo run -p learning_notes --example rts_smart_pointers
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // TS/JS 没有智能指针的概念，GC 自动管理内存
// // 所有对象都在堆上，通过引用访问
//
// // Box<T> 对应：普通对象引用（JS 所有对象都隐式在堆上）
// const obj = { value: 42 };  // 等价于 Box::new(42)
//
// // Rc<T> 对应：多个变量共享同一对象
// const shared = { count: 0 };
// const ref1 = shared;    // 两者指向同一对象
// const ref2 = shared;    // GC 在两者都消失后才释放
//
// // RefCell<T> 对应：运行时可变性（TS 对象默认可变）
// const obj = { value: 1 };
// obj.value = 2;  // TS 总是允许修改（即使是 const，对象属性可变）
//
// // Arc<T>：TS 没有线程概念（Web Worker 通过消息传递）
// // Mutex<T>：TS 没有多线程，不需要 Mutex
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // ============================================================
    // 一、Box<T>：堆分配
    // TS: 所有对象默认在堆上，不需要 Box
    // Rust: 默认栈分配；Box 显式将值放到堆上
    // ============================================================
    println!("=== Box<T> ===");

    // 基本用法：把值放到堆上
    let b = Box::new(5_i32); // TS: const b = 5（JS 数字在堆或栈，无需关心）
    println!("b = {b}"); // Box 自动解引用，直接用值

    // Box 的主要用途1：递归类型（编译器需要知道大小，直接递归无法确定大小）
    // TS: 链表节点直接用对象，GC 处理
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // Box 打断无限大小的递归
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("链表: {:?}", list);

    // Box 的主要用途2：trait 对象（动态分发）
    // TS: 接口类型变量
    trait Draw {
        fn draw(&self);
    }
    struct Button {
        label: String,
    }
    struct Image {
        src: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("按钮: {}", self.label);
        }
    }
    impl Draw for Image {
        fn draw(&self) {
            println!("图片: {}", self.src);
        }
    }

    // TS: Draw[] — 可以放不同类型
    let widgets: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            label: String::from("确定"),
        }),
        Box::new(Image {
            src: String::from("logo.png"),
        }),
    ];
    for w in &widgets {
        w.draw();
    }

    // ============================================================
    // 二、Rc<T>：引用计数（单线程共享所有权）
    // TS: 多个变量可以共享同一对象，GC 计数引用
    // Rust: 默认不允许多个所有者；Rc 允许单线程多所有者
    // ============================================================
    println!("\n=== Rc<T> ===");

    let a = Rc::new(String::from("共享字符串"));
    println!("引用计数: {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a); // 增加引用计数（不是深拷贝）
    println!("clone 后引用计数: {}", Rc::strong_count(&a)); // 2

    {
        let c = Rc::clone(&a);
        println!("再 clone 后: {}", Rc::strong_count(&a)); // 3
        println!("a={a}, b={b}, c={c}"); // 三者都指向同一数据
    } // c 离开作用域，引用计数 -1
    println!("c 离开后: {}", Rc::strong_count(&a)); // 2

    // Rc 是不可变共享，如果需要修改，需要配合 RefCell

    // ============================================================
    // 三、RefCell<T>：内部可变性（运行时借用检查）
    // TS: 对象属性随时可以修改，不需要 mut
    // Rust: 默认不可变；RefCell 允许在不可变引用下修改内部值
    // ============================================================
    println!("\n=== RefCell<T> ===");

    let data = RefCell::new(vec![1, 2, 3]);

    // borrow()：不可变借用（运行时检查，违规会 panic）
    {
        let r = data.borrow(); // 类似 &
        println!("读取: {:?}", *r);
    } // 不可变借用结束

    // borrow_mut()：可变借用
    {
        let mut w = data.borrow_mut(); // 类似 &mut
        w.push(4);
    } // 可变借用结束

    println!("修改后: {:?}", data.borrow()); // [1, 2, 3, 4]

    // ============================================================
    // 四、Rc<RefCell<T>>：共享可变数据（单线程经典组合）
    // TS: 多个变量共享同一对象并可以修改（默认行为）
    // ============================================================
    println!("\n=== Rc<RefCell<T>> ===");

    // 共享的可变计数器
    let shared_counter = Rc::new(RefCell::new(0_i32));

    let counter_a = Rc::clone(&shared_counter);
    let counter_b = Rc::clone(&shared_counter);

    // 多个地方修改同一数据（TS 默认就能做到）
    *counter_a.borrow_mut() += 10;
    *counter_b.borrow_mut() += 20;

    println!("共享计数器: {}", shared_counter.borrow()); // 30

    // ============================================================
    // 五、Arc<T>：原子引用计数（多线程安全的 Rc）
    // TS: 没有多线程（Web Worker 通过消息传递，不共享内存）
    // Rust: Arc 可以在线程间共享数据
    // ============================================================
    println!("\n=== Arc<T> ===");

    let shared = Arc::new(String::from("多线程共享"));
    let shared2 = Arc::clone(&shared);

    let handle = std::thread::spawn(move || {
        // move 把 shared2 的所有权移入线程
        println!("线程中: {shared2}");
    });

    println!("主线程: {shared}");
    handle.join().unwrap();

    // ============================================================
    // 六、Arc<Mutex<T>>：多线程共享可变数据（经典组合）
    // TS: 没有对应（Web Worker 不共享内存）
    // Rust: 多线程安全的可变共享
    // ============================================================
    println!("\n=== Arc<Mutex<T>> ===");

    let counter = Arc::new(Mutex::new(0_i32));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        let h = std::thread::spawn(move || {
            let mut num = c.lock().unwrap(); // 获取锁
            *num += 1;
        }); // 锁在这里自动释放（RAII）
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("5个线程各+1后: {}", counter.lock().unwrap()); // 5

    // ============================================================
    // 七、智能指针总结
    // ============================================================
    println!("\n=== 选择指南 ===");
    println!("Box<T>：单一所有者，堆分配，递归类型，trait 对象");
    println!("Rc<T>：单线程多所有者，不可变共享");
    println!("RefCell<T>：运行时借用检查，单线程内部可变性");
    println!("Rc<RefCell<T>>：单线程共享可变（TS 的默认行为）");
    println!("Arc<T>：多线程多所有者，不可变共享");
    println!("Arc<Mutex<T>>：多线程共享可变（最安全的多线程共享）");

    println!("\nTS 对比：GC 自动处理所有这些场景，代价是运行时开销");
    println!("Rust：手动选择合适的指针，换取零运行时 GC 开销");
}
