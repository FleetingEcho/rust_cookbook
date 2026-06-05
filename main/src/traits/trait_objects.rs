//动态分配

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    // 这里的参数 x: Box<dyn Draw> 是一个特征对象，允许在 运行时 处理不同的类型
    //由于 Box<T> 具有 Deref 特性，所以 Box<dyn Draw> 可以自动解引用调用 draw() 方法。
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    //这里的 x: &dyn Draw 是一个 Trait Object（特征对象），表示 x 只要实现了 Draw 特征就可以作为参数传递。
    x.draw();
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    draw1(Box::new(x));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
}
/*
而 dyn Trait 使得代码可以在运行时处理不同的类型，即 动态分派（dynamic dispatch）：

dyn Trait 表示 特征对象，允许在运行时决定调用哪个 draw() 方法。
Box<dyn Draw> 可以 存储不同的类型，但它们都实现了 Draw。
&dyn Draw 允许在 编译时未知确切类型 的情况下调用 draw() 方法。

*/

/*
Self 与 self
在 Rust 中，有两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名：

也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。
trait Draw {
    fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;
impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone()
    }
}

fn main() {
    let button = Button;
    let newb = button.draw();
}

*/

/*
特征对象的限制：对象安全（Object Safety）
在 Rust 中，并不是所有的特征（Trait）都能作为 特征对象（Trait Object），只有**对象安全（Object Safe）**的特征才能用于 dyn Trait。

对象安全的两个条件
1.方法的返回类型不能是 Self
因为 dyn Trait 代表一个不确定的具体类型，如果方法返回 Self，编译器无法知道 Self 具体是什么类型。
2.方法不能有泛型参数
由于特征对象在 运行时动态分派，而泛型是 编译时静态分派，它们是不兼容的。

不满足对象安全的情况
fn create() -> Self; ❌ 不允许返回 Self
fn execute<T>(&self, value: T); ❌ 不允许泛型参数
如何修复
移除 Self 返回值
避免泛型参数
对象安全的特征可以作为 dyn Trait 使用
例如 fn draw(&self) -> String;
例如 fn print_message(&self, msg: &str);



trait NotObjectSafe {
    fn create() -> Self; // ❌ 违反规则 1
    fn execute<T>(&self, value: T); // ❌ 违反规则 2
}
//所以 报错
fn run(obj: &dyn NotObjectSafe) { // ❌ 编译错误
    println!("Running...");
}


解决办法
trait ObjectSafe {
    fn execute(&self); // ✅ 移除了泛型
}
fn run(obj: &dyn ObjectSafe) { //OK
    obj.execute();// OK
}


trait ObjectSafe {
    fn draw(&self) -> String; // ✅ 返回值不是 Self
    fn print_message(&self, msg: &str); // ✅ 没有泛型参数
}

// 📘 TypeScript 对比
// ====================
// Rust `dyn Trait` ≈ TS interface 的运行时多态
//
// ```rust
// fn draw(x: &dyn Draw) { x.draw(); }     // Rust：显式动态分发
// ```
// ```ts
// function draw(x: Draw) { x.draw(); }    // TS：默认就是动态
// ```
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 默认多态 | 编译期泛型（单态化，零开销） | 运行时动态 |
// | 运行时多态 | 显式 `dyn Trait`（虚表开销） | 默认行为 |
// | 对象安全 | ✅ 有规则限制 | 无限制 |
// | 性能 | 静态分发零开销 | 全部动态 |
//
// ⚠️ TS 代码天生就是动态分发（vtable 在 JS 引擎里）。
//    Rust 需要你明确选择静态(`impl Trait`)还是动态(`dyn Trait`)。
//    Rust 默认走静态——性能更好，但需要理解两种模式的区别。
//
// 详细对照 → rust_vs_typescript.rs §11 "Trait 对象"
 */
