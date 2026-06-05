// 运行命令：cargo run -p learning_notes --example rts_control_flow
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // if/else（语句，不是表达式）
// if (x > 0) { console.log("正"); } else { console.log("负"); }
// const label = x > 0 ? "正" : "负";  // 三元运算符才是表达式
//
// // for 循环
// for (let i = 0; i < 5; i++) { ... }
// for (const item of arr) { ... }
// for (const [i, item] of arr.entries()) { ... }
// for (const key in obj) { ... }
//
// // while
// while (condition) { ... }
// do { ... } while (condition);
//
// // 标签循环（TS 有但极少用）
// outer: for (...) {
//     for (...) { break outer; }
// }
//
// // switch（TS 常用）
// switch (x) {
//     case 1: ...; break;
//     default: ...;
// }
// ============================================================

fn main() {
    // ============================================================
    // 一、if / else
    // 关键差异：Rust 的 if 是表达式，可以直接赋值
    // TS 需要三元运算符才能做到，Rust 直接用 if
    // ============================================================
    let x = 7_i32;

    // 基本用法（与 TS 类似，但条件不需要括号）
    if x > 0 {
        println!("正数");
    } else if x < 0 {
        println!("负数");
    } else {
        println!("零");
    }

    // if 作为表达式（TS 需要三元运算符 x > 0 ? "正" : "负"）
    let label = if x > 0 { "正数" } else { "负数" };
    println!("label: {label}");

    // 在 let 中使用 if 表达式
    let abs_x = if x >= 0 { x } else { -x };
    println!("abs: {abs_x}");

    // ============================================================
    // 二、loop（无限循环 + 可以返回值）
    // TS 没有直接对应，最接近的是 while(true)
    // 关键差异：Rust 的 loop 可以通过 break 返回值
    // ============================================================

    // 基本 loop（等同于 TS 的 while(true)）
    let mut i = 0;
    loop {
        i += 1;
        if i >= 3 {
            break;
        }
    }
    println!("loop 后 i = {i}");

    // loop 返回值（TS 无此能力）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // break 携带返回值
        }
    };
    println!("loop 返回值: {result}"); // 10

    // ============================================================
    // 三、while
    // 与 TS 基本一致，但条件不需要括号
    // ============================================================
    let mut n = 3;
    while n > 0 {
        println!("while: {n}");
        n -= 1;
    }

    // while let（模式匹配循环，TS 没有对应）
    let mut stack = vec![1_i32, 2, 3];
    while let Some(top) = stack.pop() {
        // 弹出直到空
        println!("while let: {top}");
    }

    // ============================================================
    // 四、for 循环
    // TS: for (const item of arr) / for (let i=0; i<n; i++)
    // Rust: for item in iter（没有 C 风格的三段式 for）
    // ============================================================

    // 遍历数组（TS: for (const x of arr)）
    let arr = [10, 20, 30, 40, 50];
    for val in &arr {
        // &arr 借用，不消耗
        print!("{val} ");
    }
    println!();

    // 范围（TS: for (let i=0; i<5; i++)）
    for i in 0..5 {
        // 0,1,2,3,4（不含5）
        print!("{i} ");
    }
    println!();

    for i in 0..=5 {
        // 0,1,2,3,4,5（含5）
        print!("{i} ");
    }
    println!();

    // 带索引（TS: for (const [i, val] of arr.entries())）
    for (i, val) in arr.iter().enumerate() {
        println!("[{i}] = {val}");
    }

    // 反向迭代（TS: [...arr].reverse().forEach(...)）
    for val in arr.iter().rev() {
        print!("{val} ");
    }
    println!();

    // 步长（TS: for (let i=0; i<10; i+=2)）
    for i in (0..10).step_by(2) {
        print!("{i} ");
    }
    println!();

    // ============================================================
    // 五、标签循环（break/continue 指定层级）
    // TS 有但极少用；Rust 在嵌套循环中很实用
    // ============================================================
    'outer: for i in 0..4 {
        for j in 0..4 {
            if i == 2 && j == 2 {
                println!("跳出外层循环 at ({i},{j})");
                break 'outer; // TS: break outer;（语法不同）
            }
            print!("({i},{j}) ");
        }
    }
    println!();

    // continue 到外层循环
    'outer2: for i in 0..3 {
        for j in 0..3 {
            if j == 1 {
                continue 'outer2; // 跳过内层剩余，继续外层下一次迭代
            }
            print!("({i},{j}) ");
        }
    }
    println!();

    // ============================================================
    // 六、match（类似 switch，但更强大）
    // TS: switch，但 match 不需要 break，且编译器强制穷举
    // ============================================================
    let num = 3_i32;
    match num {
        1 => println!("一"),
        2 | 3 => println!("二或三"), // 多个值（TS: case 2: case 3:）
        4..=6 => println!("四到六"), // 范围（TS: 没有直接对应）
        _ => println!("其他"),       // 默认（TS: default:）
    }

    // match 作为表达式（TS: switch 不能直接作为表达式）
    let description = match num {
        1 => "一",
        2 | 3 => "二或三",
        _ => "其他",
    };
    println!("描述: {description}");

    // ============================================================
    // 七、条件表达式对比总结
    // ============================================================

    // TS 三元：x > 0 ? "positive" : "negative"
    // Rust if 表达式：
    let sign = if x > 0 { "positive" } else { "negative" };
    println!("sign: {sign}");

    // TS switch + 默认值：(() => { switch(x) {...} })()
    // Rust match 表达式：
    let day_type = match x % 7 {
        0 | 6 => "周末",
        _ => "工作日",
    };
    println!("day_type: {day_type}");

    // ============================================================
    // 八、范围类型
    // ============================================================
    let range = 1..=10; // 闭区间范围，可以用于 for、match、contains 等
    println!("5 在范围内: {}", range.contains(&5));

    // 范围可以用于 Vec 切片
    let v = vec![1, 2, 3, 4, 5];
    println!("v[1..3] = {:?}", &v[1..3]); // [2, 3]
}
