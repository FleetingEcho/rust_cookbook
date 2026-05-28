// Deref 解引用——让智能指针像引用一样工作
// 在 Rust 里，智能指针 之所以智能，是因为它们不仅仅是普通的指针，还实现了 Deref 和 Drop 这两个特征：

// Deref：让智能指针像引用一样工作，可以用 * 直接访问指向的数据
// Drop：让智能指针超出作用域时自动执行清理逻辑
// 本章重点讲 Deref，让我们一步步剖析它的原理和应用场景。

// 1. Deref 让智能指针像引用一样使用
// 来看一个简单的引用示例：

// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y); // ✅ 通过解引用 `*y` 获取 `x` 的值
// }
// 这里 y 是一个普通的引用（&x），用 *y 进行解引用，可以取出它指向的值 5。

// 但智能指针不是引用，为什么 Box<T> 也能用 * 解引用？

// fn main() {
//     let x = Box::new(1);
//     let sum = *x + 1; // ✅ 直接解引用 Box<T>
// }
// 这是因为 Box<T> 实现了 Deref 特征，让它能像引用一样使用。

// 2. 自己实现一个类似 Box<T> 的智能指针
// 要实现自己的智能指针，首先我们创建一个 MyBox<T> 结构体：

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// 这个 MyBox<T> 只是个元组结构体，封装了一个 T 类型的值。现在我们尝试直接对它进行解引用：

fn main() {
    let y = MyBox::new(5);
    assert_eq!(5, *y); // ❌ 报错，MyBox<T> 不能用 `*` 解引用
}


// Rust 不知道如何解引用 MyBox<T>，因为它只是个普通结构体，并没有 Deref 特征。

// 3. 实现 Deref 让 MyBox<T> 支持 * 解引用
// 要支持 *，我们需要 实现 Deref 特征：

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;  // 关联类型 Target 指定 Deref 解析出的类型

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// 这段代码做了两件事：

// 实现 Deref，告诉 Rust 如何从 MyBox<T> 取出 T
// deref 方法返回 &T，这样 * 操作符就可以通过 &T 解引用它
// 现在 MyBox<T> 就可以像 Box<T> 一样使用：

// fn main() {
//     let y = MyBox::new(5);
//     assert_eq!(5, *y); // ✅ 现在可以解引用了
// }
// 这里 *y 实际上执行了：

// *(y.deref())  // 先调用 `deref()`，返回 `&T`，再用 `*` 取值
// Rust 只会执行 一次 Deref 转换，不会无限嵌套 *(y.deref()).deref()。

// 4. 函数参数的 Deref 隐式转换
// Deref 还有个 隐式转换 机制，让智能指针可以直接当作引用传递给函数。

// 来看一个例子：

// fn display(s: &str) {
//     println!("{}", s);
// }

// fn main() {
//     let s = String::from("hello world");
//     display(&s);  // ✅ &String 自动转换为 &str
// }


// 这里 String 实现了 Deref<Target=str>，所以 &String 可以自动转换 为 &str。

// 如果我们用 MyBox<T> 包装 String，它仍然可以被 display 函数直接接受：

// fn main() {
//     let s = MyBox::new(String::from("hello world"));
//     display(&s); // ✅ 自动执行 `Deref`
// }
// 转换过程：

// MyBox<String> 先 Deref 为 &String
// String 再 Deref 为 &str
// 最终 &MyBox<String> 自动变成 &str
// 如果没有 Deref，我们必须手动解引用：

// display(&(*s)[..]); // 先 `*s` 得到 `String`，再切片转换成 `&str`
// 有了 Deref，我们可以写出更优雅的代码。

// 5. DerefMut：支持可变解引用
// DerefMut 允许我们对 可变智能指针 进行 * 解引用操作：

use std::ops::{Deref, DerefMut};

struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
// 这样 MyBox<T> 就支持 可变解引用 了：

fn main() {
    let mut s = MyBox::new(String::from("hello, "));
    s.push_str("world");  // ✅ 直接修改内部数据
    println!("{}", s);
}

// DerefMut 规则
// 当 T: DerefMut<Target=U>，可以把 &mut T 转换为 &mut U
// 当 T: Deref<Target=U>，可以把 &mut T 转换为 &U
// 不能从 &U 转换为 &mut U，否则会破坏 Rust 的所有权规则



// 6. 三种 Deref 转换
// 类型转换	触发条件

// &T -> &U	T: Deref<Target=U>
// &mut T -> &mut U	T: DerefMut<Target=U>
// &mut T -> &U	T: Deref<Target=U>
// ❌ &U -> &mut U	不允许（破坏借用规则）
// 来看个 DerefMut 示例：

// fn display(s: &mut String) {
//     s.push_str(" world");
//     println!("{}", s);
// }

// fn main() {
//     let mut s = MyBox::new(String::from("hello"));
//     display(&mut s);  // ✅ MyBox<T> 自动 DerefMut 成 &mut String
// }
// 转换过程：

// &mut MyBox<String> 先 DerefMut 成 &mut String
// &mut String 传递给 display 函数
// 如果 MyBox<T> 只实现了 Deref，但没有 DerefMut，那 &mut s 会报错。

// 7. 总结

// Deref 让智能指针像引用一样使用：

// Box<T>、Rc<T>、Arc<T> 等都实现了 Deref
// 可以 * 解引用获取数据
// 允许隐式转换，简化代码
// DerefMut 让可变智能指针支持 * 解引用：

// &mut T 可以转换为 &mut U
// &mut T 也能转换为 &U
// 不能 反向转换 &U -> &mut U
// Rust 在函数调用时支持自动 Deref 转换：

// Box<String> 可以自动变成 &str
// &Rc<T> 可以自动变成 &T
// 可以进行多次转换，直到找到匹配类型

// 📘 TypeScript 对比
// ====================
// Rust `Deref` — TS 中没有对应概念
//
// Deref 让智能指针像引用一样工作：
// ```rust
// let b = Box::new(5);
// let sum = *b + 1;  // ✨ 自动解引用
// ```
//
// TS 没有解引用操作符，普通变量已经是值/引用透明：
// ```ts
// let b = { value: 5 };
// let sum = b.value + 1;  // 直接访问属性
// ```
//
// Rust 的 Deref 还有**自动解引用**（Deref Coercion）：
// - `&Box<String>` 自动变成 `&String` 再变成 `&str`
// - 这就是为什么 `&String` 可以传给接受 `&str` 的参数
// - TS 里不需要这些——对象引用就是引用，没有"解引用层级"
//
// 详细对照 → rust_vs_typescript.rs §17 "Drop 与 Deref"