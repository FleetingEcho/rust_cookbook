// Cell 与 RefCell 简要总结
// Rust 通过严格的所有权和借用规则保证安全性，但有时会限制灵活性。因此，Cell 和 RefCell 提供了内部可变性，让不可变引用的数据也能修改。

// Cell
// 特点：

// 适用于 T: Copy 类型的数据。
// 通过 get() 读取值，set() 修改值。
// 违反借用规则但不会 panic，可用来存储简单的数值类型。
// 示例：

use std::cell::Cell;

fn main() {
    let c = Cell::new(10);
    let x = c.get();
    c.set(20);
    let y = c.get();
    println!("{}, {}", x, y); // 输出: 10, 20
}
// 限制：不能存储非 Copy 类型的数据，如 String。

// RefCell
// 特点：

// 适用于存储引用类型，可以借用可变引用 (borrow_mut())，但违规则会在运行时 panic。
// 解决 Rc<T> 共享数据时的可变性问题。
// 适用于编译器误报或引用管理复杂的场景。
// 示例：

use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("Hello"));
    {
        let mut s_mut = s.borrow_mut();
        s_mut.push_str(", world!");
    }
    println!("{}", s.borrow()); // 输出: Hello, world!
}
// 注意：

// borrow_mut() 和 borrow() 不能同时存在，否则运行时 panic：
// let s1 = s.borrow(); // 不可变借用
// let s2 = s.borrow_mut(); // ❌ 运行时 panic

// Cell vs RefCell
// 特性	Cell<T>	RefCell<T>
// 适用数据类型	T: Copy	引用类型
// 可变性	通过 set() 直接修改	borrow_mut() 运行时检查
// 运行时开销	无	有 (借用计数)
// 违规则后果	不会 panic	运行时 panic
// 选择：

// Cell<T> → 适用于 Copy 类型数据，无额外开销。
// RefCell<T> → 适用于引用类型，适合复杂借用管理。
// Rc + RefCell 组合
// 用于多个所有者共享可变数据：

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

    let s1 = s.clone();
    let s2 = s.clone();
    // let mut s2 = s.borrow_mut();
    s2.borrow_mut().push_str(", oh yeah!");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

fn main() {
    let data = Rc::new(RefCell::new(String::from("共享数据")));

    let data1 = Rc::clone(&data);
    data1.borrow_mut().push_str("，可变修改");

    println!("{}", data.borrow()); // 输出: 共享数据，可变修改
}
// 原理：

// Rc<T> → 多个所有者
// RefCell<T> → 内部可变性
// 缺点：borrow_mut() 仍需小心，违规则 panic。
// 性能
// 方案	内存消耗	CPU 开销	适用场景
// Cell<T>	低	无	适用于 Copy 类型数据
// RefCell<T>	低	有	适用于引用类型，少量借用
// Rc<T> + RefCell<T>	较高	有	共享可变数据，但非线程安全

// 建议：

// 能用 Cell<T> 就不要用 RefCell<T>。
// 需要多个所有者，且数据可变时，使用 Rc<T> + RefCell<T>。
// 性能敏感代码需进行 benchmark 评估。

// 总结
// Cell<T> 适用于 Copy 类型，修改值不会 panic。
// RefCell<T> 适用于引用类型，借用规则检查在运行时。
// Rc<T> + RefCell<T> 适用于多个所有者共享可变数据，但要小心 panic。

// 在 Rust 中，内部可变性是一个强大但需要谨慎使用的特性，合理选择 Cell 或 RefCell，才能既保证安全性，又提高代码的灵活性！

// 📘 TypeScript 对比
// ====================
// Cell/RefCell ≈ TS 中不需要的概念（JS 默认允许任意修改）
//
// 在 TS 中，你可以随时修改任何对象的属性：
// ```ts
// const obj = { x: 1 };
// obj.x = 2;  // 可以
// ```
// 在 Rust 中，如果只有不可变引用 `&T`，就无法修改。
// Cell/RefCell 提供了"内部可变性"——通过不可变引用修改内部值。
//
// | 类型 | 适用 | 检查时机 | panic？ |
// |------|------|---------|--------|
// | `Cell<T>` | Copy 类型 | 编译期 | ❌ |
// | `RefCell<T>` | 非 Copy | 运行时 | ✅ 违反规则时 panic |
//
// 详细对照 → rust_vs_typescript.rs §16 "智能指针"
