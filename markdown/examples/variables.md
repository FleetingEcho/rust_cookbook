# Rust vs TypeScript: 变量

**运行命令：** `cargo run -p learning_notes --example rts_variables`

## TypeScript 版本
```
ts
const x = 5;           // 不可变绑定
let y = 5;             // 可变绑定
y = 10;

// 类型注解
const name: string = "Alice";
let count: number = 0;

// 没有 shadowing 概念，重新声明会报错或覆盖
const n = 1;
const n = n + 1;       // ❌ TS: 重复声明错误（严格模式）

// 常量（编译期）
const MAX_SIZE: number = 100;

// 解构赋值
const [a, b] = [1, 2];
const { name: userName, age } = user;

// 类型推断
const inferred = 42;   // 推断为 number
```

## 一、不可变绑定（默认）

TS: const x = 5
Rust 默认不可变，这与 TS 的 const 类似

```rust
let x = 5;
println!("x = {x}");
// x = 6; // ❌ 编译错误：不可变绑定不能修改

// 显式类型注解
// TS: const name: string = "Alice"
let name: &str = "Alice";
let count: i32 = 0;
println!("name={name}, count={count}");
```

## 二、可变绑定

TS: let y = 5; y = 10;
Rust 必须显式声明 mut，TS 的 let 默认可变

```rust
let mut y = 5;
println!("y = {y}");
y = 10;  // ✅ mut 变量可以重新赋值
println!("y = {y}");

let mut counter = 0;
counter += 1;
counter += 1;
println!("counter = {counter}");
```

## 三、变量遮蔽（Shadowing）

TS 没有此概念（严格模式下重复 const 声明是错误）
Rust 的 shadowing 允许重新声明同名变量，甚至改变类型

```rust
let s = 5;
let s = s + 1;      // 遮蔽：创建新变量，不是修改原变量
let s = s * 2;
println!("遮蔽后 s = {s}");  // 12

// 遮蔽还可以改变类型（这是 mut 做不到的）
let spaces = "   ";         // &str 类型
let spaces = spaces.len();  // 遮蔽为 usize 类型
println!("spaces = {spaces}");  // 3

// mut 不能改变类型：
// let mut spaces = "   ";
// spaces = spaces.len(); // ❌ 类型不匹配
```

## 四、常量

TS: const MAX_SIZE: number = 100（都是常量，但含义不同）
Rust 的 const 是真正的编译期常量，必须有类型注解，全大写命名

```rust
const MAX_POINTS: u32 = 100_000;  // 编译期确定，可用于数组大小等
const PI: f64 = 3.14159265358979;
println!("MAX_POINTS={MAX_POINTS}, PI={PI}");

// const 可以在任何作用域声明，包括全局
// TS 的 const 也可以，但 Rust 的 const 是字面量嵌入，不占内存地址
```

## 五、静态变量

TS 没有直接对应，类似模块级 const 但有内存地址

```rust
static GREETING: &str = "你好，世界";  // 程序运行期间一直存在，有固定内存地址
static mut CALL_COUNT: u32 = 0;        // 可变静态变量（需要 unsafe）
println!("GREETING = {GREETING}");

// 可变静态变量需要 unsafe（多线程下不安全）
unsafe {
    CALL_COUNT += 1;
    println!("调用次数: {CALL_COUNT}");
}
```

## 六、类型推断

与 TS 类似，但 Rust 的推断更强（可以跨语句推断）

```rust
let inferred = 42;           // 推断为 i32（TS 推断为 number）
let inferred_f = 3.14;       // 推断为 f64
let inferred_s = "hello";    // 推断为 &str（TS 推断为 string）

// Rust 可以根据后续使用推断类型（TS 不支持）
let mut collected = Vec::new();   // 现在还不知道类型
collected.push(1_i32);            // 根据这里推断出 Vec<i32>
println!("推断类型: {inferred}, {inferred_f}, {inferred_s}, {:?}", collected);
```

## 七、解构绑定

TS: const [a, b] = [1, 2]; const { name, age } = user;
元组解构

```rust
let (a, b, c) = (1, 2, 3);  // TS: const [a, b, c] = [1, 2, 3]
println!("元组解构: a={a}, b={b}, c={c}");

// 结构体解构
struct Point { x: i32, y: i32 }
let p = Point { x: 10, y: 20 };
let Point { x, y } = p;     // TS: const { x, y } = p
println!("结构体解构: x={x}, y={y}");

// 忽略某些字段
let (first, _, third) = (1, 2, 3);  // TS: const [first, , third] = [1, 2, 3]
println!("忽略中间: {first}, {third}");
```

## 八、变量作用域

TS 用 {} 创建块作用域，let/const 遵循块作用域
Rust 完全相同，但块可以作为表达式返回值

```rust
let result = {
    let a = 3;
    let b = 4;
    a * a + b * b  // 块的最后一个表达式就是返回值（不需要 return）
    // TS 需要 IIFE: (() => { const a=3,b=4; return a*a+b*b; })()
};
println!("块表达式结果: {result}");  // 25

// 内层作用域的变量不会泄漏到外层
{
    let inner = "只在这个块里存在";
    println!("{inner}");
}
// println!("{inner}"); // ❌ inner 已超出作用域
```

## 九、整数字面量的多种写法（TS 只有十六进制前缀 0x）

```rust
    let decimal     = 1_000_000;   // 十进制，下划线分隔（TS 也支持）
    let hex         = 0xFF;        // 十六进制（TS 也支持）
    let octal       = 0o77;        // 八进制（TS: 0o77 也支持）
    let binary      = 0b1111_0000; // 二进制（TS: 0b... 也支持）
    let byte        = b'A';        // 字节字面量（u8），TS 没有
    println!("字面量: {decimal}, {hex}, {octal}, {binary}, {byte}");
}
```
