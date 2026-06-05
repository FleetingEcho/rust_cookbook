/*
所有整数类型，比如 u32
布尔类型，bool，它的值是 true 和 false
所有浮点数类型，比如 f64
字符类型，char
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的
*/

fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn main2() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

fn example_references() {
    let x = 5;
    let y = &x; //引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
}

fn example_mut_ref() {
    let mut s = String::from("hello"); // 可变

    change(&mut s); // 同一作用域，特定数据只能有一个可变引用：
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
会报错
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：

两个或更多的指针同时访问同一数据
至少有一个指针被用来写入数据
没有同步数据访问的机制

*/

// 可变引用与不可变引用不能同时存在！！！！
// ⛔ 下面代码无法编译，用于演示 Rust borrow 规则：
/*
fn example_borrow_conflict(){
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  let r3 = &mut s; // 大问题

  println!("{}, {}, and {}", r1, r2, r3);
}
*/

/*
注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }

*/

/*

悬垂引用(Dangling References)

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！


解决办法

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// 📘 TypeScript 对比
// ====================
// ⚠️ **这是 Rust 最独特的概念，TS 完全没有！**
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 内存管理 | 编译期所有权 + 借用检查 | 运行时 GC（V8） |
// | 赋值语义 | 默认 **move**（移动所有权） | 默认引用拷贝（同一对象） |
// | 拷贝 | 深拷贝需 `.clone()` | 展开运算符或引用 |
// | 可变借用 | 同一时间 **1 个** `&mut` 或 **N 个** `&` | 随时可修改 |
// | 悬垂引用 | 编译期阻止 | GC 语言极少发生 |
//
// ```ts
// // TS 没有所有权——对象赋值只是复制引用
// let a = { name: "hello" };
// let b = a;          // b 和 a 指向同一对象！
// a.name = "world";
// console.log(b.name); // "world" — 有副作用
// ```
//
// Rust 的所有权是编译期检查，不牺牲运行时性能。
// 借用规则看起来严格，但能彻底消除数据竞争和悬垂指针。
//
// 详细对照 → rust_vs_typescript.rs §7 "所有权与借用"
*/
