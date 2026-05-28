// Rust 结构体自引用总结
// 在 Rust 中，自引用结构体（Self-referential Struct）指的是结构体内部的字段引用自身的其他字段。Rust 严格的借用检查机制导致实现这种结构体非常困难，需要额外的技巧或特殊处理。



// 1. 为什么 Rust 不能直接支持自引用？

struct SelfRef<'a> {
    value: String,
    pointer_to_value: &'a str, // 这个引用指向 value
}

fn main() {
    let s = "hello".to_string();
    let v = SelfRef {
        value: s,
        pointer_to_value: &s, // ❌ 借用检查失败
    };
}
// 问题
// value 的所有权转移到结构体 SelfRef 中。
// 但 pointer_to_value 仍然引用着 s，违反 Rust 的所有权和借用规则，导致编译错误。


// 2. 解决方法
// ✅ 方法 1：使用 Option
// 先创建，再赋值：

#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

fn main() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);

    println!("{:?}", tricky);
}
// 局限：

// 无法在函数返回，会违反生命周期规则。
// 只能在方法内部修改，受限较大。



// ✅ 方法 2：使用 unsafe 裸指针

#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer_to_value: *const String, // 使用裸指针
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        SelfRef {
            value: txt.to_string(),
            pointer_to_value: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.pointer_to_value = &self.value;
    }

    fn pointer_to_value(&self) -> &String {
        assert!(!self.pointer_to_value.is_null(), "未初始化");
        unsafe { &*self.pointer_to_value }
    }
}

fn main() {
    let mut t = SelfRef::new("hello");
    t.init();
    println!("{}", t.pointer_to_value());
}
// 优点：

// 避免生命周期问题，可以安全访问 value。
// 清晰易懂，避免 Rust 严格的借用规则。
// 缺点：

// 需要 unsafe，使用不当可能导致未定义行为（UB）。



// ✅ 方法 3：使用 Pin 防止结构体被移动
// Pin 固定数据在内存中，防止它在初始化后被移动，确保指针不会悬空：


use std::pin::Pin;
use std::ptr::NonNull;
use std::marker::PhantomPinned;

struct Unmovable {
    data: String,
    slice: NonNull<String>, // 指针，确保不为空
    _pin: PhantomPinned,    // 标记为不可移动
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            slice: NonNull::dangling(), // 先设置为空指针
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res); // `Pin<Box<Self>>`
        let slice = NonNull::from(&boxed.data);

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

fn main() {
    let unmoved = Unmovable::new("hello".to_string());
    println!("slice 地址: {:?}", unmoved.slice);
}
// 优点：

// Pin<Box<Self>> 确保结构体不会被移动，避免悬空指针问题。
// NonNull<T> 代替裸指针，避免 null 访问。
// 缺点：

// Pin 使用较复杂，理解成本较高。


// ✅ 方法 4：使用 ouroboros 库
// ouroboros 是一个支持自引用的 Rust 库。


use ouroboros::self_referencing;

#[self_referencing]
struct SelfRef {
    value: String,
    #[borrows(value)]
    pointer_to_value: &'this str,
}

fn main(){
    let v = SelfRefBuilder {
        value: "aaa".to_string(),
        pointer_to_value_builder: |value: &String| value,
    }.build();

    println!("{}", v.borrow_pointer_to_value());
}
// 优点：

// 直接支持自引用，不需要 unsafe。
// 代码更简洁，可读性更好。
// 缺点：

// 受限于 ouroboros 规则，不能随意修改数据。





// ✅ 方法 5：使用 Rc + RefCell 或 Arc + Mutex
// 适用于：当多个所有者需要访问可变数据时。


use std::cell::RefCell;
use std::rc::Rc;

struct SelfRef {
    value: Rc<RefCell<String>>,
    pointer_to_value: Rc<RefCell<Option<String>>>,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        let value = Rc::new(RefCell::new(txt.to_string()));
        let pointer_to_value = Rc::new(RefCell::new(None));
        Self { value, pointer_to_value }
    }

    fn init(&self) {
        let value_ref = self.value.borrow();
        *self.pointer_to_value.borrow_mut() = Some(value_ref.clone());
    }

    fn pointer_to_value(&self) -> Option<String> {
        self.pointer_to_value.borrow().clone()
    }
}

fn main() {
    let s = SelfRef::new("hello");
    s.init();
    println!("{:?}", s.pointer_to_value());
}
// 优点：

// 适用于复杂数据结构。
// Rc<RefCell<T>> 允许多个可变引用，避免 Rust 严格的借用规则。
// 缺点：

// 运行时开销更高。


// RefCell 可能导致 运行时 panic（不安全的可变借用）。
// 总结
// 方法	适用场景	是否安全	复杂度
// Option	仅适用于局部变量	✅ 安全	⭐⭐
// unsafe 裸指针	允许可变自引用	❌ 不安全	⭐⭐
// Pin	防止移动，适用于稳定结构	✅ 安全	⭐⭐⭐
// ouroboros	需要更好封装的自引用	✅ 安全	⭐⭐⭐
// Rc + RefCell	允许多个可变引用	⚠️ 运行时检查	⭐⭐⭐


// 如何选择？
// 简单数据 → 使用 Option。
// 必须使用自引用 → unsafe 指针（需小心）或 Pin。
// 可读性更重要 → 使用 ouroboros。
// 涉及多个所有者 → Rc + RefCell（单线程）或 Arc + Mutex（多线程）。
