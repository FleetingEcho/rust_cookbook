// Rust 中的五种 Unsafe 兵器
// 在 Rust 语言中，unsafe 代码块提供了一些突破 Rust 规则的能力，这些能力虽然强大，但也可能带来内存安全问题。因此，unsafe 代码的使用需要 极度谨慎。

// 1. 裸指针（Raw Pointers）
// Rust 提供了 三种指针类型：

// 引用（&T, &mut T）：遵循 Rust 的借用规则，安全但受限。
// 智能指针（Box<T>, Rc<T>, Arc<T> 等）：提供所有权管理，防止内存泄漏。

// 裸指针（*const T, *mut T）：类似 C 指针，可以绕过 Rust 安全检查，但也容易引发错误。
// 创建裸指针
// 裸指针可以通过 引用 或 内存地址 创建：


fn main() {
    let mut num = 5;

    // 通过引用创建裸指针
    let r1 = &num as *const i32;  // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针

    // 通过内存地址创建裸指针（危险！）
    let addr = 0x012345usize;
    let r3 = addr as *const i32; // 这个指针可能是无效的

    // 访问裸指针需要 unsafe 代码块
    unsafe {
        println!("r1: {}", *r1);
        // println!("r3: {}", *r3); // 可能会导致未定义行为 (UB)
    }
}
// 注意：

// 不能直接解引用裸指针，必须在 unsafe 代码块中。
// 不保证指针指向合法内存，可能发生 段错误（Segmentation Fault）。


// 2. 调用 unsafe 函数
// 在 Rust 中，unsafe fn 允许执行一些 编译器无法检查安全性 的操作，例如：

// 直接操作裸指针
// 调用 FFI（外部 C 代码）
// 访问可变的全局变量

unsafe fn dangerous_function() {
    println!("This is an unsafe function!");
}

fn main() {
    // 必须在 unsafe 代码块中调用
    unsafe {
        dangerous_function();
    }
}
// 规则：

// unsafe fn 本身不会自动要求 unsafe 代码块，但调用时必须使用 unsafe。
// 除非必要，不要定义 unsafe fn，而是用安全的抽象封装 unsafe 代码。



// 3. 用 unsafe 访问和修改静态变量
// 在 Rust 中，全局变量（static）通常是不可变的。如果需要定义一个可变的全局变量，则必须使用 unsafe：


static mut COUNTER: i32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}

fn main() {
    increment();
    increment();
}
// 注意：

// 访问 可变静态变量 必须用 unsafe 代码块，因为它可能导致数据竞争。
// 如果多个线程访问静态变量，建议使用 互斥锁（Mutex） 或 原子类型（Atomic）。



// 4. unsafe 实现特征
// Rust 允许实现 不安全的特征（unsafe trait），用于一些低级操作，如并发控制。

// 示例：实现 Send

unsafe trait MyUnsafeTrait {
    fn dangerous_method(&self);
}

unsafe impl MyUnsafeTrait for i32 {
    fn dangerous_method(&self) {
        println!("Unsafe method called: {}", self);
    }
}

fn main() {
    let x = 10;
    unsafe {
        x.dangerous_method();
    }
}
// 注意：

// unsafe trait 适用于 编译器无法验证 其安全性的情况，例如 Send、Sync。
// 一般不推荐手动实现 Send 或 Sync，除非完全理解它们的含义。



// 5. FFI（外部函数调用）
// FFI (Foreign Function Interface) 允许 Rust 调用其他语言（如 C）的函数。

// 调用 C 标准库

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -5: {}", abs(-5));
    }
}
// 规则：

// extern "C" 用于定义 C 语言风格 的函数接口。
// Rust 不能检查外部函数的安全性，因此调用时必须使用 unsafe。
// 让 C 调用 Rust 函数

// #[no_mangle] // 避免 Rust 进行名称修饰 (Name Mangling)
// pub extern "C" fn rust_function() {
//     println!("Hello from Rust!");
// }
// 应用场景：

// 让 Rust 代码可以被 C 语言动态库 调用（比如 lib.so、dll）。

// 需要搭配 cargo build --release --crate-type=cdylib 生成共享库。

// 安全封装 unsafe 代码
// 为了减少 unsafe 代码的使用范围，可以用 安全抽象 包装 unsafe 代码。例如：

// split_at_mut 示例
// split_at_mut 允许 安全地 将可变切片分成两部分：


use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    println!("{:?} {:?}", a, b);
}
// 安全性分析：

// 通过 assert!(mid <= len) 确保指针不会越界。
// 让 unsafe 代码封装在安全函数中，使得 API 仍然是 安全的。
// Rust 的 unsafe 生态
// 1. thiserror
// 用于定义 自定义错误类型：


// use thiserror::Error;

// #[derive(Error, Debug)]
// enum MyError {
//     #[error("I/O Error: {0}")]
//     Io(#[from] std::io::Error),
// }


// 2. anyhow
// 用于简化 错误处理：


use anyhow::Result;
fn read_file() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")?;
    println!("{}", content);
    Ok(())
}
// 3. rust-bindgen
// 用于 自动生成 Rust FFI 代码：

// bindgen wrapper.h -o bindings.rs
// 4. miri
// 用于检测 未定义行为（UB）：

// cargo miri test
// 总结
// 五种 unsafe 兵器
// 兵器	作用	典型用途
// 裸指针	绕过 Rust 借用规则	需要 C 风格的内存管理
// 调用 unsafe 函数	允许调用不安全代码	FFI, 裸指针操作
// 访问/修改静态变量	修改 static mut 变量	需要全局状态
// 实现 unsafe 特征	允许手动实现 Send/Sync	低级并发优化
// FFI（外部函数接口）	调用 C/C++ 代码	复用 C 库
// Rust 强调安全性，因此 unsafe 代码应尽量减少，并使用安全抽象封装。只有在性能优化、底层操作、FFI 交互等场景中，才应谨慎使用 unsafe。

// 📘 TypeScript 对比
// ====================
// Rust `unsafe` ≈ TS 的 `as any` / `as unknown as T`
//
// 都是"绕过类型检查的逃生舱"，但级别不同：
//
// | 特性 | Rust unsafe | TS any |
// |------|------------|--------|
// | 绕过什么？ | 借用检查 + 内存安全 | 类型检查 |
// | 风险 | 内存泄露/段错误 | 运行时类型错误 |
// | 用途 | FFI、裸指针、性能优化 | 动态数据、迁移期 |
// | 封装方式 | 安全函数包裹 unsafe | 类型守卫 / 类型断言 |
//
// ⚠️ TS 的 `any` 只跳过类型检查，运行时仍然是安全的（GC 语言）。
//    Rust 的 unsafe 可以导致内存崩溃——要极度谨慎！
//
// 详细对照 → rust_vs_typescript.rs §22 "unsafe"