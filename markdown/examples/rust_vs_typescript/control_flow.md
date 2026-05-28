# Rust vs TypeScript: 流程控制

**运行命令：** `cargo run -p learning_notes --example rts_control_flow`

## TypeScript 版本

```ts
if (x > 0) { console.log("正"); } else { console.log("负"); }
const label = x > 0 ? "正" : "负";

for (let i = 0; i < 5; i++) { ... }
for (const item of arr) { ... }
for (const [i, item] of arr.entries()) { ... }
for (const key in obj) { ... }

while (condition) { ... }
do { ... } while (condition);

switch (x) {
    case 1: ...; break;
    default: ...;
}
```

## 一、if / else

关键差异：Rust 的 `if` 是**表达式**，可以直接赋值。TS 需要三元运算符。

```rust
let x = 7_i32;

if x > 0 {
    println!("正数");
} else if x < 0 {
    println!("负数");
} else {
    println!("零");
}

let label = if x > 0 { "正数" } else { "负数" };
let abs_x = if x >= 0 { x } else { -x };
```

## 二、loop（无限循环 + 返回值）

```rust
let mut i = 0;
loop {
    i += 1;
    if i >= 3 { break; }
}

let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 5 {
        break counter * 2;
    }
};
println!("loop 返回值: {result}");
```

## 三、while

```rust
let mut n = 3;
while n > 0 {
    println!("while: {n}");
    n -= 1;
}

let mut stack = vec![1_i32, 2, 3];
while let Some(top) = stack.pop() {
    println!("while let: {top}");
}
```

## 四、for 循环

```rust
let arr = [10, 20, 30, 40, 50];
for val in &arr { print!("{val} "); }
println!();

for i in 0..5 { print!("{i} "); }    // 不含5
println!();
for i in 0..=5 { print!("{i} "); }   // 含5
println!();

for (i, val) in arr.iter().enumerate() {
    println!("[{i}] = {val}");
}

for val in arr.iter().rev() { print!("{val} "); }
println!();

for i in (0..10).step_by(2) { print!("{i} "); }
println!();
```

## 五、标签循环

```rust
'outer: for i in 0..4 {
    for j in 0..4 {
        if i == 2 && j == 2 {
            println!("跳出外层循环 at ({i},{j})");
            break 'outer;
        }
        print!("({i},{j}) ");
    }
}
```

## 六、match

```rust
let num = 3_i32;
match num {
    1 => println!("一"),
    2 | 3 => println!("二或三"),
    4..=6 => println!("四到六"),
    _ => println!("其他"),
}

let description = match num {
    1 => "一",
    2 | 3 => "二或三",
    _ => "其他",
};
```

## 总结对照表

| TypeScript | Rust |
|------------|------|
| `if/else` 语句 | `if/else` 表达式 |
| 三元 `? :` | `if` 表达式直接赋值 |
| `for (i=0; i<n; i++)` | `for i in 0..n` |
| `for (x of arr)` | `for x in &arr` |
| `while(condition)` | `while condition` |
| `do...while` | 无（用 `loop` + `break`） |
| `while(true)` | `loop` |
| `break` 不能返回值 | `break value` 可返回值 |
| `switch` | `match`（强制穷举） |
