fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式 不能有分号， 否则就变成了语句

    // let a = 8;
    // let b: Vec<f64> = Vec::new();
    // let (a, c) = ("hi", false);
}

pub fn main_expression() {
    let y = {
        // 这个大括号内是一个表达式块
        let x = 3; // 定义变量 x
        x + 1 // 这里没有 `;`，表示它是一个**表达式**，返回 `x + 1`
    };

    println!("The value of y is: {}", y); // 4
    let z: () = {
        let x = 3;
        x + 1; // 这里加了分号，整个块的返回值变成 `()`
    };
    println!("{:?}", z); // 是的 输出 "()"，表示返回的是 `unit`
    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);
}

/*
  例如单元类型 ()，是一个零长度的元组。它没啥作用，但是可以用来表达一个函数没有返回值：

函数没有返回值，那么返回一个 ()
通过 ; 结尾的语句返回一个 ()

  */

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn another_function(x: i32, y: f32) {
    // 类型必须存在
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
/*
最后，表达式如果不返回任何值，会隐式地返回一个 () 。
fn main() {
    assert_eq!(ret_unit_type(), ())
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}

*/

/*
当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数：
fn dead_end() -> ! {
  panic!("你已经到了穷途末路，崩溃吧！");
}

📘 TypeScript 对比
====================
Rust 中很多"语句"其实是表达式（有返回值）：

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 代码块 | `{ let x=1; x+1 }` 有返回值 | `{ let x=1; return x+1 }` 需 return |
| if/else | `let x = if cond { a } else { b }` | 三元 `cond ? a : b` |
| match | 表达式，有返回值 | switch 语句，无返回值 |
| 函数返回值 | 最后一个表达式（不用 `;`） | 必须 `return` |
| 发散函数 | `fn() -> !` 永不返回 | `function(): never` |

⚠️ Rust 里去掉 `;` 就是返回值，加上 `;` 就是语句。
   这个规则一旦习惯会觉得非常自然！
*/
