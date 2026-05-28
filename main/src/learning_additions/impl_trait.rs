// impl Trait 和 dyn Trait 都能表示"某个实现了 Trait 的类型"，
// 但机制完全不同：
//
//   impl Trait  — 编译期确定具体类型，单态化（monomorphization），零运行时开销。
//                 限制：同一函数只能返回一种具体类型。
//
//   dyn Trait   — 运行时通过虚表（vtable）分发，有指针和 vtable 的开销。
//                 优势：可以在运行时决定返回哪种具体类型（异构集合）。

// ── 参数位置：impl Trait ──────────────────────────────────────────────────────

// 参数位置的 impl Trait 和泛型写法等价：
//   fn foo(x: impl Display)  ≡  fn foo<T: Display>(x: T)
// 区别：impl Trait 写法不能在调用时手动指定类型参数。

use std::fmt::Display;

pub fn print_twice(value: impl Display) {
    println!("{value}");
    println!("{value}");
}

// ── 返回位置：impl Trait（RPIT）───────────────────────────────────────────────

// 返回 impl Trait 时，编译器知道具体类型，但调用方看不到。
// 常见用途：返回闭包或迭代器，避免写出复杂的具体类型名。

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // 编译器知道返回的是某个闭包类型，调用方只知道它实现了 Fn(i32)->i32。
    move |x| x + n
}

pub fn evens_up_to(limit: u32) -> impl Iterator<Item = u32> {
    // 返回类型实际是 Filter<...>，写出来很长，用 impl Iterator 更简洁。
    (0..=limit).filter(|n| n % 2 == 0)
}

// ── 返回位置：dyn Trait ───────────────────────────────────────────────────────

// 当需要根据运行时条件返回不同的具体类型时，必须用 Box<dyn Trait>。
// （所有具体类型必须大小相同，Box 保证了这一点。）

pub trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 根据参数在运行时决定返回哪种形状 — 必须用 dyn Trait
pub fn make_shape(kind: &str) -> Box<dyn Shape> {
    match kind {
        "circle" => Box::new(Circle { radius: 3.0 }),
        _         => Box::new(Square { side: 4.0 }),
    }
}

// ── 异构集合：只有 dyn Trait 能做到 ──────────────────────────────────────────

pub fn heterogeneous_shapes() {
    // 这个 Vec 里可以存不同的具体类型，因为每个元素都是 Box<dyn Shape>。
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Circle { radius: 5.0 }),
    ];

    for shape in &shapes {
        println!("面积: {:.2}", shape.area());
    }
}

// ── 速查对比 ──────────────────────────────────────────────────────────────────
//
//  场景                              选择
//  ─────────────────────────────     ──────────────
//  参数只需满足某个 trait           impl Trait（或泛型）
//  返回迭代器 / 闭包，类型固定      impl Trait（RPIT）
//  运行时选不同类型，单个返回值     Box<dyn Trait>
//  异构集合（Vec 里放多种类型）     Vec<Box<dyn Trait>>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_works() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
    }

    #[test]
    fn evens_correct() {
        let evens: Vec<_> = evens_up_to(6).collect();
        assert_eq!(evens, [0, 2, 4, 6]);
    }

    #[test]
    fn dyn_shape_area() {
        let s = make_shape("square");
        assert_eq!(s.area(), 16.0);
    }
}
