#![allow(unreachable_code, unused_labels)]

pub fn test() {
    let n = 5;

    if n < 0 {
        print!("{} 是负数", n);
    } else if n > 0 {
        print!("{} 是正数", n);
    } else {
        print!("{} 是零", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(",是一个小数字,扩大十倍");

        // 这个表达式返回 `i32` 类型。
        10 * n
    } else {
        println!(",是一个大数字,将数字减半");

        // 这个表达式也必须返回 `i32` 类型。
        n / 2
        // TODO ^ 尝试用分号结束这个表达式。
    };
    //   ^ 别忘了在这里加分号！所有 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);

    // test1();
    // test2();
    test8();
}

fn test1() {
    let mut count = 0u32;

    println!("让我们数到无穷大！");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过本次迭代的剩余部分
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("好了,够了");

            // 退出这个循环
            break;
        }
    }
}

// fn test2() {
//     'outer: loop {
//         println!("进入外层循环");

//         'inner: loop {
//             println!("进入内层循环");

//             // 这只会中断内层循环
//             //break;

//             // 这会中断外层循环
//             break 'outer;
//         }

//         println!("这一点永远不会到达");
//     }

//     println!("退出外层循环");
// }

fn test3() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn test4() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时继续循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器递增
        n += 1;
    }
}

fn test5() {
    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn test6() {
    let names = vec!["Bob", "Frank", "Ferris"];

    //iter - 在每次迭代中借用集合的每个元素。因此,集合保持不变,并且在循环之后可以重复使用。
    for name in names.iter() {
        match name {
            &"Ferris" => println!("我们中间有一个 Rustacean！"),
            // TODO ^ 尝试删除 & 并只匹配 "Ferris"
            _ => println!("你好 {}", name),
        }
    }

    println!("names: {:?}", names);

    //into_iter - 这会消耗集合,使得在每次迭代中提供确切的数据。一旦集合被消耗,它就不再可用于重复使用,因为它已经在循环中被"移动"了。
    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("我们中间有一个 Rustacean！"),
                _ => println!("你好 {}", name),
            }
        }
    }

    // println!("names: {:?}", names);
    // 修复：^ 注释掉此行

    //原地修改数据
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "我们中间有一个 Rustacean！",
                _ => "你好",
            }
        }

        println!("names: {:?}", names);
    }
}

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("无法分割计数项对：'{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("无法解析整数：'{count_str}'");
    };
    (count, item)
}

//其实等价于
fn old_get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("无法分割计数项对：'{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("无法解析整数：'{count_str}'");
    };
    (count, item)
}

fn test7() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    assert_eq!(old_get_count_item("3 chairs"), (3, "chairs"));
}

fn test8() {
    // 创建 `Option<i32>` 类型的 `optional`
    let mut optional = Some(0);

    // 这段代码的含义是：当 `let` 将 `optional` 解构为 `Some(i)` 时,
    // 执行代码块 `{}`,否则 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("大于 9,退出！");
            optional = None;
        } else {
            println!("`i` 是 `{:?}`。再试一次。", i);
            optional = Some(i + 1);
        }
        // ^ 减少了代码缩进右移,无需显式处理失败情况
    }
    // ^ `if let` 可以有额外的 `else`/`else if` 子句,`while let` 则没有。
}

//if let
fn test9() {
    // 以下都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构的含义是：如果 `let` 能将 `number` 解构为
    // `Some(i)`,则执行代码块（`{}`）。
    if let Some(i) = number {
        println!("匹配到 {:?}！", i);
    }

    // 如果需要指定匹配失败的情况,可以使用 else：
    if let Some(i) = letter {
        println!("匹配到 {:?}！", i);
    } else {
        // 解构失败。转到失败处理的情况。
        println!("没有匹配到数字。那就用一个字母吧！");
    }

    // 提供一个修改后的失败条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("匹配到 {:?}！", i);
    // 解构失败。评估 `else if` 条件,看是否应该执行替代的失败分支：
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况：
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }
}

// 我们的示例枚举
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn test10() {
    // 创建示例变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配 Foo::Bar
    if let Foo::Bar = a {
        println!("a 是 foobar");
    }

    // 变量 b 不匹配 Foo::Bar
    // 所以这里不会打印任何内容
    if let Foo::Bar = b {
        println!("b 是 foobar");
    }

    // 变量 c 匹配 Foo::Qux,它包含一个值
    // 类似于前面例子中的 Some()
    if let Foo::Qux(value) = c {
        println!("c 是 {}", value);
    }

    // `if let` 也可以进行绑定
    if let Foo::Qux(value @ 100) = c {
        println!("c 是一百");
    }
}
