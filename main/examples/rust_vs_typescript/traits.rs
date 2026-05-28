// 运行命令：cargo run -p learning_notes --example rts_traits
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // interface：描述行为（无默认实现）
// interface Drawable {
//     draw(): void;
// }
//
// // abstract class：可以有默认实现
// abstract class Shape {
//     abstract area(): number;
//     describe(): string {           // 默认实现
//         return `面积是 ${this.area().toFixed(2)}`;
//     }
// }
//
// class Circle extends Shape implements Drawable {
//     constructor(private radius: number) { super(); }
//     area() { return Math.PI * this.radius ** 2; }
//     draw() { console.log(`画圆，半径 ${this.radius}`); }
// }
//
// // 泛型约束
// function printArea<T extends Shape>(shape: T): void {
//     console.log(shape.describe());
// }
//
// // 多约束
// function showAndMeasure<T extends Drawable & { area(): number }>(s: T) { ... }
//
// // 运行时多态（接口类型）
// function drawAll(shapes: Drawable[]): void {
//     shapes.forEach(s => s.draw());
// }
// ============================================================

// ============================================================
// 一、定义 trait
// TS 对应：interface 或 abstract class
// 关键区别：Rust trait 可以有默认方法实现（TS interface 不支持，需用 abstract class）
// ============================================================
trait Drawable {
    fn draw(&self);

    // 默认方法：TS interface 不支持，需要 abstract class
    fn label(&self) -> String {
        String::from("可绘制图形")  // 子类可以覆盖
    }
}

trait Area {
    fn area(&self) -> f64;

    // 默认方法可以调用同 trait 内的其他方法
    fn describe(&self) -> String {
        format!("面积: {:.2}", self.area())  // TS: abstract class 的模板方法模式
    }
}

// ============================================================
// 二、实现 trait
// TS 对应：class Circle implements Drawable
// ============================================================
struct Circle {
    radius: f64,
}

struct Rectangle {
    width:  f64,
    height: f64,
}

struct Triangle {
    base:   f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("画圆，半径: {}", self.radius);
    }
    fn label(&self) -> String {
        format!("圆形(r={})", self.radius)  // 覆盖默认实现
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("画矩形 {}×{}", self.width, self.height);
    }
    // 不覆盖 label()，使用默认实现
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// ============================================================
// 三、泛型约束
// TS 对应：function f<T extends Area>(shape: T)
// ============================================================

// 单约束
// TS: function printArea<T extends Area>(shape: T)
fn print_area<T: Area>(shape: &T) {
    println!("{}", shape.describe());
}

// 多约束（where 语法更清晰，TS: T extends Drawable & Area）
// TS: function showShape<T extends Drawable & { area(): number }>(s: T)
fn show_shape<T>(shape: &T)
where
    T: Area + Drawable,
{
    shape.draw();
    println!("{}", shape.describe());
    println!("标签: {}", shape.label());
}

// impl Trait 语法（函数参数的简写，效果等同于泛型）
// TS: function printAny(shape: Area)
fn print_any_area(shape: &impl Area) {
    println!("impl Trait: {:.2}", shape.area());
}

// ============================================================
// 四、trait 对象（动态分发）
// TS 对应：接口类型的参数 (shapes: Drawable[])
// dyn Trait 在运行时决定调用哪个实现，有一点性能开销
// 泛型是编译期单态化，无运行时开销
// ============================================================

// TS: function drawAll(shapes: Drawable[]): void
fn draw_all(shapes: &[Box<dyn Drawable>]) {
    for s in shapes {
        s.draw();
    }
}

// 返回 trait 对象（不知道具体类型时使用）
// TS: function makeShape(isCircle: boolean): Shape
fn make_shape(is_circle: bool) -> Box<dyn Area> {
    if is_circle {
        Box::new(Circle { radius: 3.0 })
    } else {
        Box::new(Rectangle { width: 4.0, height: 5.0 })
    }
}

// ============================================================
// 五、为标准类型实现自定义 trait
// TS 无法扩展内置类型（无原型修改的安全等价物）
// Rust 可以为任何类型实现任何 trait（孤儿规则：trait 或类型至少有一个在本 crate）
// ============================================================
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("字符串摘要: {}...", &self[..self.len().min(10)])
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("数字: {self}")
    }
}

fn main() {
    let c = Circle    { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };
    let t = Triangle  { base: 3.0, height: 8.0 };

    // --- 直接调用 trait 方法 ---
    c.draw();
    r.draw();
    println!("{}", c.label());
    println!("{}", r.label());  // 使用默认实现

    // --- 泛型函数 ---
    print_area(&c);
    print_area(&r);
    print_area(&t);

    // --- 多约束泛型 ---
    show_shape(&c);
    show_shape(&r);

    // --- impl Trait ---
    print_any_area(&c);
    print_any_area(&t);

    // --- dyn Trait（运行时多态）---
    // TS: const shapes: Drawable[] = [new Circle(3), new Rectangle(4,5)]
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle    { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
    ];
    draw_all(&shapes);

    // --- 返回 trait 对象 ---
    let shape1 = make_shape(true);
    let shape2 = make_shape(false);
    println!("动态圆: {:.2}", shape1.area());
    println!("动态矩形: {:.2}", shape2.area());

    // --- 为内置类型实现 trait ---
    let s = String::from("hello, rust programming!");
    let n: i32 = 42;
    println!("{}", s.summarize());
    println!("{}", n.summarize());

    // ============================================================
    // 六、标准库常用 trait
    // ============================================================

    // Display：自定义打印格式，对应 TS 的 toString()
    struct Point { x: f64, y: f64 }
    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({:.1}, {:.1})", self.x, self.y)
        }
    }
    let p = Point { x: 1.5, y: 2.5 };
    println!("Display: {p}");              // 自动调用 Display::fmt

    // From / Into：类型转换 trait，对应 TS 的隐式/显式转换
    // TS: Number(x) 或 String(x)
    let s: String = String::from("hello");  // 使用 From
    let _: String = "world".into();         // 使用 Into（自动推断）

    // Clone：深拷贝，TS 的 {...obj} 是浅拷贝
    // 通过 #[derive(Clone)] 自动实现
    #[derive(Clone, Debug)]
    struct Config { value: String }
    let cfg1 = Config { value: String::from("test") };
    let cfg2 = cfg1.clone();  // TS: { ...cfg1 }（浅拷贝），Rust clone 是深拷贝
    println!("Clone: {:?}", cfg2);

    // Default：默认值 trait，对应 TS 的默认参数或 ?? 操作符
    // TS: value ?? defaultValue
    #[derive(Debug, Default)]
    struct Settings { timeout: u32, retries: u32 }
    let settings = Settings::default();  // timeout: 0, retries: 0
    println!("Default: {:?}", settings);

    // Iterator trait：实现后可以使用所有迭代器方法
    struct Counter { count: u32, max: u32 }
    impl Counter {
        fn new(max: u32) -> Self { Counter { count: 0, max } }
    }
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // 实现了 Iterator 后，自动获得所有迭代器方法
    // TS: 需要实现 [Symbol.iterator]() 生成器
    let sum: u32 = Counter::new(5).sum();
    let doubled: Vec<u32> = Counter::new(3).map(|x| x * 2).collect();
    println!("Counter sum: {sum}");
    println!("Counter doubled: {:?}", doubled);
}
