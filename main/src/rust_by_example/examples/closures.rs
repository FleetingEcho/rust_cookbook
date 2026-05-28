/*
闭包的语法和功能使其非常适合即时使用。调用闭包与调用函数完全相同。不过，闭包的输入和返回类型可以被推断，而输入变量名必须指定。

闭包的其他特点包括：

使用 || 而不是 () 来包围输入变量。
单行表达式可省略函数体定界符（{}），其他情况则必须使用
能够捕获外部环境的变量
*/

pub fn test() {
    let outer_var = 42;

    // 常规函数无法引用外部环境的变量
    // fn function(i: i32) -> i32 { i + outer_var }
    // TODO：取消上面这行的注释，查看编译器错误
    // 编译器会建议我们定义一个闭包来替代

    // 闭包是匿名的，这里我们将它们绑定到引用
    // 注解与函数注解相同，但是可选的
    // 包裹函数体的 `{}` 也是可选的
    // 这些无名函数被赋值给适当命名的变量
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // 调用闭包
    println!("closure_annotated：{}", closure_annotated(1));
    println!("closure_inferred：{}", closure_inferred(1));
    // 闭包类型一旦被推断，就不能再用其他类型重新推断。
    //println!("不能用其他类型重用 closure_inferred：{}", closure_inferred(42i64));
    // TODO：取消上面这行的注释，观察编译器错误。

    // 一个无参数并返回 `i32` 的闭包。
    // 返回类型是推断的。
    let one = || 1;
    println!("返回 1 的闭包：{}", one());

    test1();
    test2();
}

/*
捕获
闭包本质上很灵活，无需注解就能根据功能需求自动适应。这使得捕获可以灵活地适应不同场景，有时移动，有时借用。闭包可以通过以下方式捕获变量：

通过引用：&T
通过可变引用：&mut T
通过值：T
闭包优先通过引用捕获变量，仅在必要时才使用更底部的的捕获方式。
*/

fn test1() {
    use std::mem;

    let color = String::from("green");

    // 打印 `color` 的闭包，立即借用（`&`）`color` 并
    // 将借用和闭包存储在 `print` 变量中。借用状态
    // 将持续到 `print` 最后一次使用。
    //
    // `println!` 只需要不可变引用参数，所以
    // 不会施加更多限制。
    let print = || println!("`color`: {}", color); //只持有不可变引用

    // 使用借用调用闭包。
    print();

    // `color` 可以再次被不可变借用，因为闭包只持有
    // `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // `print` 最后一次使用后，允许移动或重新借用
    let _color_moved = color;

    let mut count = 0;
    // 增加 `count` 的闭包可以接受 `&mut count` 或 `count`，
    // 但 `&mut count` 限制更少，所以选择它。立即
    // 借用 `count`。
    //
    // `inc` 需要 `mut` 因为内部存储了 `&mut`。因此，
    // 调用闭包会修改 `count`，这需要 `mut`。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包。
    inc();

    // 闭包仍然可变借用 `count`，因为它稍后会被调用。
    // 尝试重新借用会导致错误。
    // let _reborrow = &count;
    // ^ TODO：尝试取消注释这行。
    inc();

    // 闭包不再需要借用 `&mut count`。因此，
    // 可以在没有错误的情况下重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型。
    let movable = Box::new(3);

    // `mem::drop` 需要 `T`，所以这里必须通过值获取。可复制类型
    // 会被复制到闭包中，原始值保持不变。
    // 不可复制类型必须移动，所以 `movable` 立即移动到闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了变量，所以只能调用一次。
    consume();
    // consume();
    // ^ TODO：尝试取消注释这行。
}

/*
作为输入参数
Rust 通常能自动选择如何捕获变量，无需类型标注。但在编写函数时，这种模糊性是不允许的。当将闭包作为输入参数时，必须使用特定的 trait 来注解闭包的完整类型。这些 trait 由闭包对捕获值的处理方式决定。按限制程度从高到低排列如下：

Fn：闭包通过引用使用捕获的值（&T）
FnMut：闭包通过可变引用使用捕获的值（&mut T）
FnOnce：闭包通过值使用捕获的值（T）
编译器会以尽可能最少限制的方式逐个捕获变量。

例如，考虑一个注解为 FnOnce 的参数。这表示闭包可能通过 &T、&mut T 或 T 进行捕获，但编译器最终会根据捕获变量在闭包中的使用方式来决定。

这是因为如果可以移动，那么任何类型的借用也应该是可能的。注意反过来并不成立。如果参数被注解为 Fn，那么通过 &mut T 或 T 捕获变量是不允许的。但 &T 是允许的。

*/

// 这个函数接受一个闭包作为参数并调用它
// <F> 表示 F 是一个"泛型类型参数"
fn apply<F>(f: F)
where
    // 这个闭包不接受输入也不返回任何值
    F: FnOnce(),
{
    // ^ TODO：试着将其改为 `Fn` 或 `FnMut`

    f();
}

// 这个函数接受一个闭包并返回 `i32`
fn apply_to_3<F>(f: F) -> i32
where
    // 这个闭包接受一个 `i32` 并返回一个 `i32`
    F: Fn(i32) -> i32,
{
    f(3)
}

fn test2() {
    use std::mem;

    let greeting = "hello";
    // 一个非复制类型
    // `to_owned` 从借用的数据创建拥有所有权的数据
    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量：通过引用捕获 `greeting`，
    // 通过值捕获 `farewell`
    let diary = || {
        // `greeting` 是通过引用捕获的：需要 `Fn`
        println!("我说{}。", greeting);

        // 修改强制 `farewell` 通过可变引用捕获
        // 现在需要 `FnMut`
        farewell.push_str("!!!");
        println!("然后我喊{}。", farewell);
        println!("现在我可以睡觉了。呼呼");

        // 手动调用 drop 强制 `farewell` 通过值捕获
        // 现在需要 `FnOnce`
        mem::drop(farewell);
    };

    // 调用应用闭包的函数
    apply(diary);

    // `double` 满足 `apply_to_3` 的 trait 约束
    let double = |x| 2 * x;

    println!("3 的两倍是：{}", apply_to_3(double));
}

/*
类型匿名
当定义一个闭包时，编译器会隐式创建一个新的匿名结构来存储内部捕获的变量，同时通过 Fn、FnMut 或 FnOnce 这些 trait 之一为这个未知类型实现功能。这个类型被赋给变量并存储，直到被调用。

由于这个新类型是未知类型，在函数中使用时就需要泛型。然而，一个无界的类型参数 <T> 仍然会是模糊的，不被允许。因此，通过 Fn、FnMut 或 FnOnce 这些 trait 之一（它实现的）来约束就足以指定其类型。
*/

// `F` 必须实现 `Fn` 用于一个不接受输入且不返回任何内容的闭包
// - 这正是 `print` 所需要的
// fn apply<F>(f: F) where
//     F: Fn() {
//     f();
// }

fn test3() {
    let x = 7;

    // 将 `x` 捕获到一个匿名类型中并为其实现 `Fn`
    // 将其存储在 `print` 中
    let print = || println!("{}", x);

    apply(print);
}

/*
输入函数
既然闭包可以作为参数使用，你可能会想知道函数是否也可以这样。确实可以！如果你声明一个函数，它接受一个闭包作为参数，那么任何满足该闭包 trait 约束的函数都可以作为参数传递。
*/
// 定义一个函数，它接受一个由 `Fn` 约束的泛型参数 `F`，并调用它
fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义一个满足 `Fn` 约束的包装函数
fn function() {
    println!("我是函数！");
}

fn test4() {
    // 定义一个满足 `Fn` 约束的闭包
    let closure = || println!("我是闭包！");

    call_me(closure);
    call_me(function);
}

/*
作为输出参数
既然闭包可以作为输入参数，那么将闭包作为输出参数返回也应该是可行的。然而，匿名闭包类型本质上是未知的，因此我们必须使用 impl Trait 来返回它们。

可用于返回闭包的有效 trait 包括：

Fn
FnMut
FnOnce
此外，必须使用 move 关键字，它表示所有捕获都是按值进行的。这是必要的，因为任何通过引用捕获的变量都会在函数退出时被丢弃，从而在闭包中留下无效的引用。
*/

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("这是一个：{}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("这是一个：{}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("这是一个：{}", text)
}

fn test5() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn test6() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 使用 `iter()` 会产生 `&i32`。
    let mut iter = vec1.iter();
    // 对 vec 使用 `into_iter()` 会产生 `i32`。
    let mut into_iter = vec2.into_iter();

    // 对 vec 使用 `iter()` 会产生 `&i32`，而我们想要引用其中的一个
    // 元素，所以我们必须将 `&&i32` 解构为 `i32`
    println!("在 vec1 中查找 2：{:?}", iter.find(|&&x| x == 2));
    // 对 vec 使用 `into_iter()` 会产生 `i32`，而我们想要引用其中的
    // 一个元素，所以我们必须将 `&i32` 解构为 `i32`
    println!("在 vec2 中查找 2：{:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组使用 `iter()` 会产生 `&&i32`
    println!("在 array1 中查找 2：{:?}", array1.iter().find(|&&x| x == 2));
    // 对数组使用 `into_iter()` 会产生 `&i32`
    println!(
        "在 array2 中查找 2：{:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}
