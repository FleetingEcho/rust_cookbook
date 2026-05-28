// 运行命令：cargo run -p learning_notes --example rts_functions
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 函数声明
// function add(x: number, y: number): number { return x + y; }
//
// // 箭头函数
// const double = (x: number): number => x * 2;
//
// // 默认参数（Rust 不支持）
// function greet(name: string, greeting: string = "Hello"): string {
//     return `${greeting}, ${name}!`;
// }
//
// // 可选参数（Rust 用 Option）
// function findUser(id: number, includeDeleted?: boolean): User { ... }
//
// // rest 参数（Rust 用 &[T]）
// function sumAll(...numbers: number[]): number { ... }
//
// // 函数类型
// type MathOp = (a: number, b: number) => number;
// const add: MathOp = (a, b) => a + b;
//
// // never 类型
// function throwError(msg: string): never { throw new Error(msg); }
//
// // 重载（TS 特有，但 Rust 有更严格的替代）
// function len(s: string): number;
// function len(arr: number[]): number;
// function len(x: any): number { return x.length; }
// ============================================================

use std::fmt::Display;

fn main() {
    // ============================================================
    // 一、基本函数
    // TS: function add(x: number, y: number): number { return x + y; }
    // 关键差异：Rust 用表达式（最后一个表达式不写 return 自动返回）
    // ============================================================

    // 带 return 的风格（与 TS 最接近）
    fn add_return(x: i32, y: i32) -> i32 {
        return x + y;
    }

    // Rust 惯用风格：最后一个表达式就是返回值（无分号，无 return）
    // TS 箭头函数也有类似写法: const add = (x: number, y: number) => x + y;
    fn add(x: i32, y: i32) -> i32 {
        x + y   // 不加分号，这就是返回值！(TS: return x + y)
    }

    // 表达式体函数（单表达式时甚至可以省略 {}）
    fn add_short(x: i32, y: i32) -> i32 { x + y }

    println!("add(3, 5) = {}", add(3, 5));
    println!("add_return(3, 5) = {}", add_return(3, 5));
    println!("add_short(3, 5) = {}", add_short(3, 5));

    // -------------------- 对比说明 --------------------
    // TS 写法                           Rust 写法
    // function f(x: number): number {    fn f(x: i32) -> i32 {
    //     return x * 2;                      x * 2    // 无return无分号！
    // }                                  }
    // const f = (x: number) => x * 2;    fn f(x: i32) -> i32 { x * 2 }
    // ------------------- 最简写法 --------------------

    // ============================================================
    // 二、无返回值函数（void / ()）
    // TS: function log(msg: string): void { console.log(msg); }
    // Rust: 返回 ()，称为「单元类型」，可以省略 -> ()
    // ============================================================

    // 显式返回单元类型 (TS: void)
    fn log_message(msg: &str) -> () {
        println!("{msg}");
    }

    // 省略返回类型，等价于 -> () (TS: 省略也返回 void)
    fn log_message_short(msg: &str) {
        println!("{msg}");
    }

    log_message("显式 void 函数");
    log_message_short("省略返回类型");

    // ============================================================
    // 三、参数模式：Rust 的参数总有类型注解
    // TS: function f(x, y) 可以不写类型（但建议写）
    // Rust: 每个参数必须标注类型
    // ============================================================
    fn print_coord(x: i32, y: i32) {
        println!("坐标: ({x}, {y})");
    }
    print_coord(10, 20);

    // ============================================================
    // 四、TS 默认参数 vs Rust 惯用模式
    // TS: function greet(name: string, greeting = "Hello"): string
    // Rust 不支持默认参数，但有常用替代方案
    // ============================================================

    // 方案A：Option 参数
    fn greet(name: &str, greeting: Option<&str>) -> String {
        let g = greeting.unwrap_or("Hello");  // TS: greeting ?? "Hello"
        format!("{g}, {name}!")
    }
    println!("greet: {}", greet("Alice", None));
    println!("greet: {}", greet("Alice", Some("你好")));

    // 方案B：为常见场景提供便捷包装函数
    fn greet_default(name: &str) -> String {
        greet(name, None)  // 内部调用完整版本
    }
    println!("greet_default: {}", greet_default("Bob"));

    // 方案C：Builder 模式（适用于参数很多的场景）
    // TS: function createUser({ name, age, email }: UserParams)
    // 详见 structs.rs 中的结构体更新语法

    // ============================================================
    // 五、TS rest 参数 vs Rust 切片参数
    // TS: function sumAll(...numbers: number[]): number
    // Rust 用 &[T] 切片
    // ============================================================

    fn sum_all(numbers: &[i32]) -> i32 {
        // TS: numbers.reduce((a, b) => a + b, 0)
        numbers.iter().sum()
    }

    fn max_all(numbers: &[i32]) -> i32 {
        // TS: Math.max(...numbers)
        *numbers.iter().max().unwrap_or(&0)
    }

    println!("sum_all: {}", sum_all(&[1, 2, 3, 4, 5]));   // 15
    println!("max_all: {}", max_all(&[3, 7, 2, 9, 1]));   // 9

    // 如果需要真正的 rest 参数语法，用宏或可变参数：
    macro_rules! sum_all_macro {
        ($($n:expr),*) => {
            {
                let mut sum = 0_i32;
                $(sum += $n;)*
                sum
            }
        };
    }
    println!("sum_all_macro: {}", sum_all_macro!(1, 2, 3, 4, 5)); // 15

    // ============================================================
    // 六、发散函数（Diverging Function）-> !
    // TS: function throwError(msg: string): never { throw new Error(msg); }
    // Rust: -> ! 表示永不返回（编译器的类型系统特性）
    // ============================================================

    // 发散函数（永远不返回）
    fn exit_process() -> ! {
        std::process::exit(0);
    }

    // -> ! 最常见的实际用途：panic 辅助函数
    fn unreachable_msg(msg: &str) -> ! {
        panic!("不应该到达这里: {msg}");
    }

    // TS 的 never 和 Rust 的 ! 都在类型收窄/穷尽性检查中发挥作用
    fn process_optional(x: Option<i32>) -> i32 {
        match x {
            Some(n) => n,
            None    => unreachable_msg("x 应该总有值"),
            // ! 类型可以兼容任何类型，所以这里编译通过
        }
    }
    println!("process_optional: {}", process_optional(Some(42)));

    // ============================================================
    // 七、函数指针类型（fn 类型）
    // TS: type MathOp = (a: number, b: number) => number;
    // Rust: fn(i32, i32) -> i32  注意是小写 fn
    // ============================================================

    fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))  // TS: f(f(x))
    }

    fn square(x: i32) -> i32 { x * x }
    fn double(x: i32) -> i32 { x * 2 }

    // 函数名自动转为函数指针
    println!("do_twice(square, 3): {}", do_twice(square, 3));   // 81
    println!("do_twice(double, 3): {}", do_twice(double, 3));   // 12

    // 也可以传入闭包（如果闭包不捕获变量）
    println!("do_twice(|x| x+1, 5): {}", do_twice(|x| x + 1, 5)); // 7

    // fn 类型 vs Fn trait（详见 closures_iter.rs）
    // fn 是函数指针，不能捕获变量
    // Fn/FnMut/FnOnce 是闭包 trait，可以捕获变量

    // ============================================================
    // 八、方法：&self / &mut self / self
    // TS: class 方法用 this，默认可变
    // Rust 需要显式声明 self 的所有权方式
    // ============================================================

    struct Counter {
        value: i32,
    }

    impl Counter {
        fn new(value: i32) -> Self { // 关联函数，类似 TS 的 static 方法/constructor
            Counter { value }
        }

        // &self：不可变借用，只读 (TS: 普通方法，能读 this)
        fn get(&self) -> i32 {
            self.value
        }

        // &mut self：可变借用，可修改 (TS: 普通方法，能改 this)
        fn increment(&mut self) {
            self.value += 1;  // TS: this.value++
        }

        // self：消耗所有权 (TS 没有对应，但可以用 return this 链式调用)
        fn into_display(self) -> String {
            format!("Counter: {}", self.value)
            // self 在此被消耗，调用后不能再使用
        }
    }

    let mut c = Counter::new(10);     // TS: const c = new Counter(10)
    println!("get: {}", c.get());     // 10
    c.increment();                    // TS: c.increment()
    println!("after increment: {}", c.get()); // 11

    let display = c.into_display();   // c 的所有权被消耗
    println!("into_display: {display}");
    // println!("{}", c.get()); // ❌ c 已被消耗

    // 重新创建演示
    let mut c = Counter::new(5);
    println!("方法链调用: {}", c.get());

    // ============================================================
    // 九、泛型函数
    // TS: function identity<T>(x: T): T { return x; }
    // ============================================================

    fn identity<T>(x: T) -> T {
        x  // TS: return x
    }

    fn first<T>(list: &[T]) -> Option<&T> {
        list.first()  // TS: arr.length > 0 ? arr[0] : undefined
    }

    fn swap<T>(a: &mut T, b: &mut T) {
        std::mem::swap(a, b);  // TS: [a, b] = [b, a]（解构交换）
    }

    println!("identity(42): {}", identity(42));
    println!("identity('hi'): {}", identity("hi"));
    println!("first(&[1,2,3]): {:?}", first(&[1, 2, 3]));
    println!("first(&[] as &[i32]): {:?}", first(&[] as &[i32]));

    let mut x = 1_i32;
    let mut y = 2_i32;
    swap(&mut x, &mut y);
    println!("swap: x={x}, y={y}"); // x=2, y=1

    // ============================================================
    // 十、impl Trait（返回位置的高级特性）
    // TS 没有直接对应（TS 用抽象类或接口）
    // ============================================================

    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        // 返回一个闭包，但调用者不需要知道闭包的具体类型
        move |x| x + n
    }

    let add5 = make_adder(5);
    println!("add5(10): {}", add5(10)); // 15

    // impl Trait 也可以用于参数位置（语法糖）
    fn print_displayable(value: impl Display) {
        println!("值: {value}");
    }
    print_displayable(42);
    print_displayable("hello");

    // ============================================================
    // 十一、高阶函数（函数作为参数和返回值）
    // TS: 高阶函数非常常见，Rust 也一样
    // ============================================================

    // 接收函数
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x)) // TS: f(f(x))
    }
    println!("apply_twice(|n| n*3, 2): {}", apply_twice(|n| n * 3, 2)); // 18

    // 返回函数（工厂函数）
    fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x * n
    }
    let triple = make_multiplier(3);
    println!("triple(7): {}", triple(7)); // 21

    // ============================================================
    // 十二、TS 函数重载 vs Rust 替代方案
    // TS: function len(s: string): number;
    //      function len(arr: number[]): number;
    // Rust 不支持同名不同参的重载，但可以用 trait 或枚举
    // ============================================================

    // 方案A：trait（静态分发，零开销）
    trait Length {
        fn length(&self) -> usize;
    }

    impl Length for String {
        fn length(&self) -> usize { self.len() }
    }

    impl<T> Length for Vec<T> {
        fn length(&self) -> usize { self.len() }
    }

    fn print_len<T: Length>(item: &T) {
        println!("长度: {}", item.length());
    }

    print_len(&String::from("hello"));   // TS: len("hello")
    print_len(&vec![1, 2, 3, 4]);        // TS: len([1,2,3,4])

    // 方案B：枚举（运行时分发，类似 TS union type）
    enum Input<'a> {
        Text(&'a str),
        Numbers(&'a [i32]),
    }

    fn len_input(input: &Input) -> usize {
        match input {
            Input::Text(s)    => s.len(),
            Input::Numbers(v) => v.len(),
        }
    }

    println!("len_input(Text): {}", len_input(&Input::Text("hello")));
    println!("len_input(Numbers): {}", len_input(&Input::Numbers(&[1, 2, 3])));

    // ============================================================
    // 十三、内嵌函数（TS 不支持函数内的函数）
    // TS: 没有嵌套函数，但可以用 const inner = () => ...
    // Rust: 函数内可以定义函数
    // ============================================================
    fn outer(x: i32) -> i32 {
        fn inner(y: i32) -> i32 {
            y * 2
        }
        inner(x) + 1
    }
    println!("outer(5): {}", outer(5)); // 11

    // 注意：内嵌函数不能捕获外部变量（要用闭包）
    let factor = 3_i32;
    // fn cant_capture(y: i32) -> i32 { y * factor } // ❌ 编译错误
    let can_capture = |y: i32| y * factor;             // ✅ 闭包才能捕获
    println!("闭包捕获: {}", can_capture(5));

    // ============================================================
    // 十四、条件编译函数（TS 没有对应，预处理指令）
    // ============================================================

    #[cfg(target_os = "linux")]
    fn platform_specific() {
        println!("运行在 Linux");
    }

    #[cfg(not(target_os = "linux"))]
    fn platform_specific() {
        println!("运行在非 Linux 系统");
    }

    platform_specific();

    // ============================================================
    // 总结对照表
    // ============================================================
    println!("\n=== Rust vs TS 函数总结 ===");
    println!("┌──────────────────────────┬──────────────────────────────────┐");
    println!("│ TypeScript               │ Rust                             │");
    println!("├──────────────────────────┼──────────────────────────────────┤");
    println!("│ function add(a,b) {{...}} │ fn add(a: i32, b: i32) -> i32   │");
    println!("│ return x + y            │ x + y  (无分号，表达式返回)      │");
    println!("│ (参数不强制写类型)       │ 每个参数必须有类型注解           │");
    println!("│ void                     │ () 单元类型/省略返回             │");
    println!("│ never (throw)            │ -> ! (发散函数)                  │");
    println!("│ type Fn = (i32)=>i32     │ fn(i32) -> i32 (函数指针)       │");
    println!("│ 默认参数 / 可选参数      │ Option<T> / Builder 模式         │");
    println!("│ ...rest 参数             │ &[T] 切片参数                   │");
    println!("│ 函数重载                  │ trait / 枚举替代               │");
    println!("│ static method            │ 关联函数（impl 内的 fn）        │");
    println!("│ this (默认可变)          │ &self / &mut self / self        │");
    println!("│ 嵌套函数                 │ 支持内嵌 fn，但不能捕获变量     │");
    println!("└──────────────────────────┴──────────────────────────────────┘");
}
