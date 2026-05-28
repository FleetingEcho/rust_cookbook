fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

/*
使用方法	等价使用方式	所有权
for item in collection	for item in IntoIterator::into_iter(collection)	转移所有权
for item in &collection	for item in collection.iter()	不可变借用
for item in &mut collection	for item in collection.iter_mut()	可变借用
*/


fn main() {
    let mut n = 0;

    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }

    println!("我出来了！");
}

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}


fn main() {
    let mut counter = 0;

    let result = loop { //loop 是一个表达式，因此可以返回一个值
        counter += 1;

        if counter == 10 {
            break counter * 2;//break 可以单独使用，也可以带一个返回值，有些类似 return
        }
    };

    println!("The result is {}", result);
}

// 📘 TypeScript 对比
// ====================
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | if 是表达式 | ✅ 有返回值 `let x = if cond { a } else { b }` | ❌ if 是语句，用三元 `cond ? a : b` |
// | 循环 | `loop` / `while` / `for` | `while` / `for` / `do...while` |
// | 无限循环 | `loop { }` 内置关键字 | `while (true) { }` |
// | break 带值 | ✅ `break value;` | ❌ 不支持 |
//
// ⚠️ Rust 没有 `do...while`，但有 `loop`（无限循环）和
//    `for`（遍历迭代器）。TS 的 `for...of` 对应 Rust 的 `for item in iter`。
//
// 详细对照 → rust_vs_typescript.rs

