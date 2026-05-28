enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main_action() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

//match 的匹配必须穷尽所有情况 否则会报错！

/*
或者用通配符
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}

除了_通配符，用一个变量来承载其他情况也是可以的。

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}

    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    简写后是 if let 匹配
    if let Some(3) = v {
    println!("three");
}


1. Some 是什么？
Some 是 Rust 标准库 Option<T> 枚举（enum）中的一个变体，用于表示 有值 的情况。

2. Some 的作用
Rust 没有 null，而是使用 Option<T> 来安全地处理可能为空的值。Some 主要用于：

表示可能为空的值
避免 null 引发的错误
进行安全的模式匹配



3. Some 的基本用法
(1) 定义一个 Option<T> 变量
fn main() {
    let x: Option<i32> = Some(10); // 有值
    let y: Option<i32> = None; // 无值

    println!("{:?}", x); // 输出：Some(10)
    println!("{:?}", y); // 输出：None
}

    fn main() {
    let x = Some(100);
    println!("{}", x.unwrap()); // 输出 100
}
fn main() {
    let x = Some(5);
    let y: Option<i32> = None;

    println!("{}", x.unwrap_or(0)); // 输出 5
    println!("{}", y.unwrap_or(0)); // 输出 0（因为 y 是 None）
}
*/


/*

struct User {
    id: i32,
    email: Option<String>, // 可能为空
}

fn main() {
    let user = User { id: 1, email: None };

    if let Some(email) = user.email {
        println!("Email: {}", email);
    } else {
        println!("这个用户没有提供邮箱");
    }
}


总结：为什么用 Some(T) 而不是直接 T？
原因	如果用 Some(T)	如果直接用 T
表示可能无值	Option<T> 强制你考虑 None	你可能会忘记 null 情况
避免 null	Rust 没有 null，None 更安全	其他语言可能用 null，容易出错
编译器强制检查	Rust 强制你处理 None	可能导致 null pointer exception
API 设计清晰	Option<T> 让调用者知道可能无值	直接 T 让人误以为总是有值
链式操作方便	.map() 和 ? 语法更优雅	需要额外的 if 逻辑
如果你 确定值永远不会缺失，可以直接用 T，但如果值可能为空，Option<T> + Some(T) 是更安全、更清晰的做法！

*/


pub fn match_if() {
    enum MyEnum {
        Foo,
        Bar,
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    let res: Vec<&MyEnum> = v.iter().filter(|x| {
        println!("Type of x: {:?}", std::any::type_name::<&MyEnum>());
        //   Type of x: &MyEnum
        matches!(x, MyEnum::Foo)
    }).collect();
    println!("Filtered Foo count: {}", res.len());

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'), "foo is not an alphabet");

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2), "bar does not match the condition");
}


// 变量遮蔽
/*

🌟 为什么这里是变量遮蔽？
在 if let Some(age) = age 这一行：

新的 age 变量 被创建，它与外部 age 同名。
由于 if let 引入了新的作用域，新 age 只在 if let 内部可用。
if let 结束后，原来的 age 仍然可用。
这就叫 变量遮蔽（shadowing）：新的 age 覆盖了旧的 age，但在作用域结束后，旧的 age 仍然有效。

fn main() {
    let x = 10;
    let x = "hello"; // 遮蔽之前的 x
    println!("{}", x); // 输出 "hello"
}


🌟 什么时候使用变量遮蔽？
1. 防止错误修改原始变量

let age = Some(30);
if let Some(age) = age {
    println!("{}", age); // 这里 age 是 i32，不会影响外部变量
}
println!("{:?}", age); // 这里 age 仍然是 Option<i32>

2. 改变变量类型

let num = "42";
let num: i32 = num.parse().unwrap(); // 变量遮蔽
println!("{}", num);
num 原本是 &str，但被遮蔽后变成 i32。

📌 结论
✅ 变量遮蔽 允许在相同作用域或子作用域中，创建一个新变量来临时覆盖旧变量。
✅ 适用于 转换类型，或 在特定范围内使用不同值。
✅ if let Some(age) = age 创建了新的 age，遮蔽了外部的 age，但不会影响外部变量。
*/



// 结构 Option
fn test(){

    fn plus_one1(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    //或者直接使用
    fn plus_one(x: Option<i32>) -> Option<i32> {
        x.map(|i| i + 1)
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

/*
let x = Some(5);
let y = x + 1; // ❌ 编译错误，`Option<i32>` 不能直接加法

if let Some(i) = x {
    let y = i + 1;
}


📌 总结
✅ Option<T> 代表可选值，避免 null。
✅ match 处理 Option<T>，确保 None 不会引起错误。
✅ 无法对 Some(T) 直接进行运算，必须先解构。
✅ map() 是更简洁的方式，适用于 Option<T> 变换。


📘 TypeScript 对比
====================
| 特性 | Rust `match` | TypeScript `switch` |
|------|------|-----------|
| 穷尽检查 | ✅ 必须覆盖所有可能 | ❌ 不强制 |
| 返回值 | 是表达式（有值） | 语句（无值） |
| 模式匹配 | 支持解构/守卫/范围 | 不支持 |
| `if let` | 单分支简洁匹配 | 无对应 |
| `_` 通配 | `_ => {}` 兜底分支 | `default:` |

⚠️ Rust 的 match 是 C/TS switch 的"超级升级版"：
- 可以解构枚举、元组、结构体
- 可以用 `if` 守卫（`n if n > 0 => ...`）
- 必须穷尽——编译器强制你处理所有情况
*/