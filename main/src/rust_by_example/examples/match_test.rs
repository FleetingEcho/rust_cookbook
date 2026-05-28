// 第一个匹配的分支会被求值,并且必须覆盖所有可能的值。
pub fn test() {
    let number = 13;
    // TODO ^ 尝试为 `number` 赋不同的值

    println!("告诉我关于 {} 的信息", number);
    match number {
        // 匹配单个值
        1 => println!("一！"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("这是个质数"),
        // TODO ^ 尝试将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("一个青少年"),
        // 处理其余情况
        _ => println!("没什么特别的"),
        // TODO ^ 尝试注释掉这个匹配所有情况的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 的分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // TODO ^ 尝试注释掉其中一个分支
    };

    println!("{} -> {}", boolean, binary);

    test4();
    test5();
    test_guard();
    test_binding();
}

// 解构元组
fn test1() {
    let triple = (0, -2, 3);
    // TODO ^ 尝试为 `triple` 赋不同的值

    println!("告诉我关于 {:?} 的信息", triple);
    // match 可用于解构元组
    match triple {
        // 解构第二和第三个元素
        (0, y, z) => println!("第一个是 `0`,`y` 是 {:?},`z` 是 {:?}", y, z),
        (1, ..) => println!("第一个是 `1`,其余的不重要"),
        (.., 2) => println!("最后一个是 `2`,其余的不重要"),
        (3, .., 4) => println!("第一个是 `3`,最后一个是 `4`,其余的不重要"),
        // `..` 可用于忽略元组中的其余部分
        _ => println!("它们是什么并不重要"),
        // `_` 表示不将值绑定到变量
    }
}
// 解构数组和切片

fn test2() {
    // 尝试改变数组中的值,或将其变成切片！
    let array = [1, -2, 6];

    match array {
        // 将第二个和第三个元素分别绑定到相应的变量
        [0, second, third] => println!("array[0] = 0,array[1] = {},array[2] = {}", second, third),

        // 单个值可以用 _ 忽略
        [1, _, third] => println!("array[0] = 1,array[2] = {},array[1] 被忽略了", third),

        // 你也可以绑定一部分值并忽略其余的
        [-1, second, ..] => println!("array[0] = -1,array[1] = {},其他所有的都被忽略了", second),
        // 下面的代码无法编译
        // [-1, second] => ...

        // 或者将它们存储在另一个数组/切片中（类型取决于
        // 正在匹配的值的类型）
        [3, second, tail @ ..] => {
            println!("array[0] = 3,array[1] = {},其他元素是 {:?}", second, tail)
        }

        // 结合这些模式,我们可以,例如,绑定第一个和
        // 最后一个值,并将其余的存储在一个单独的数组中
        [first, middle @ .., last] => println!(
            "array[0] = {},中间部分 = {:?},array[2] = {}",
            first, middle, last
        ),
    }
}
// 解构枚举

// 使用 `allow` 来抑制警告，因为只使用了一个变体。
#[allow(dead_code)]
enum Color {
    // 这 3 个仅通过名称指定。
    Red,
    Blue,
    Green,
    // 这些同样将 `u32` 元组与不同的名称（颜色模型）关联。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn test3() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ 尝试为 `color` 使用不同的变体

    println!("这是什么颜色？");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red => println!("颜色是红色！"),
        Color::Blue => println!("颜色是蓝色！"),
        Color::Green => println!("颜色是绿色！"),
        Color::RGB(r, g, b) => println!("红：{}，绿：{}，蓝：{}！", r, g, b),
        Color::HSV(h, s, v) => println!("色相：{}，饱和度：{}，明度：{}！", h, s, v),
        Color::HSL(h, s, l) => println!("色相：{}，饱和度：{}，亮度：{}！", h, s, l),
        Color::CMY(c, m, y) => println!("青：{}，品红：{}，黄：{}！", c, m, y),
        Color::CMYK(c, m, y, k) => println!("青：{}，品红：{}，黄：{}，黑（K）：{}！", c, m, y, k),
        // 不需要其他分支，因为所有变体都已检查
    }
}
// 解构指针
// 解引用使用 *
// 解构使用 &、ref 和 ref mut
fn test4() {
    // 分配一个 `i32` 类型的引用。`&` 表示
    // 正在分配一个引用。
    let reference = &4;

    match reference {
        // 如果 `reference` 与 `&val` 进行模式匹配，结果
        // 就像这样的比较：
        // `&i32`
        // `&val`
        // ^ 我们可以看到，如果去掉匹配的 `&`，那么 `i32`
        // 应该被赋值给 `val`。
        &val => println!("通过解构获得的值：{:?}", val),
    }

    // 为了避免 `&`，你可以在匹配前解引用。
    match *reference {
        val => println!("通过解引用获得的值：{:?}", val),
    }

    // 如果你一开始没有引用怎么办？`reference` 是一个 `&`
    // 因为右侧已经是一个引用。这不是
    // 一个引用，因为右侧不是引用。
    let _not_a_reference = 3;

    // Rust 提供 `ref` 正是为了这个目的。它修改了
    // 赋值，为元素创建一个引用；
    // 这个引用被赋值。
    let ref _is_a_reference = 3;

    // 相应地，通过定义两个没有引用的值，
    // 可以通过 `ref` 和 `ref mut` 获取引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字创建引用。
    match value {
        ref r => println!("获得了一个值的引用：{:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 获得了一个引用。在我们能够
            // 对其进行任何添加操作之前，必须先解引用。
            *m += 10;
            println!("我们加了 10。`mut_value`：{:?}", m);
        }
    }
}

// 解构结构体

fn test5() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 尝试更改结构体中的值，看看会发生什么
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("x 的第一个元素是 1，b = {}，y = {}", b, y),

        // 你可以解构结构体并重命名变量，
        // 顺序并不重要
        Foo { y: 2, x: i } => println!("y 为 2，i = {:?}", i),

        // 你也可以忽略某些变量：
        Foo { y, .. } => println!("y = {}，我们不关心 x 的值", y),
        // 这会导致错误：模式中未提及字段 `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // 解构结构体不一定需要 match 块：
    let Foo { x: x0, y: y0 } = faa;
    println!("外部：x0 = {x0:?}，y0 = {y0}");

    // 解构也适用于嵌套结构体：
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("嵌套：nested_x = {nested_x:?}，nested_y = {nested_y:?}");
}

//================ Guard========================

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn test_guard() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO：尝试为 `temperature` 赋予不同的值

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}°C 高于 30°C", t),
        // `if condition` 部分 ^ 就是守卫
        Temperature::Celsius(t) => println!("{}°C 不高于 30°C", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}°F 高于 86°F", t),
        Temperature::Fahrenheit(t) => println!("{}°F 不高于 86°F", t),
    }

    //编译器在检查 match 表达式是否涵盖了所有模式时，不会考虑守卫条件。
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("零"),
        i if i > 0 => println!("大于零"),
        _ => unreachable!("不应该发生。"),
        // TODO ^ 取消注释以修复编译错误
    }
}

// 绑定

// 一个返回 `u32` 的 `age` 函数。
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn test_binding() {
    println!("告诉我你是什么类型的人");

    match age() {
        0 => println!("我还没有过第一个生日"),
        // 可以直接匹配 1 ..= 12，但那样无法知道具体年龄
        // 相反，我们将 1 ..= 12 的序列绑定到 `n`
        // 现在就可以报告具体年龄了
        n @ 1..=12 => println!("我是 {:?} 岁的儿童", n),
        n @ 13..=19 => println!("我是 {:?} 岁的青少年", n),
        // 没有绑定。直接返回结果。
        n => println!("我是 {:?} 岁的人", n),
    }

    match some_number() {
        // 获得 `Some` 变体，检查其值（绑定到 `n`）是否等于 42
        Some(n @ 42) => println!("答案是：{}！", n),
        // 匹配任何其他数字
        Some(n) => println!("不感兴趣... {}", n),
        // 匹配其他任何情况（`None` 变体）
        _ => (),
    }
}
