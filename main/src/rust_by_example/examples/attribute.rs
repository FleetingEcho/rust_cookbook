/*
属性
属性是应用于模块、crate 或条目的元数据。这些元数据可用于以下目的：

代码的条件编译
设置 crate 的名称、版本和类型（二进制或库）
禁用 代码检查（警告）
启用编译器特性（如宏、全局导入等）
链接外部库
将函数标记为单元测试
将函数标记为基准测试的一部分
类属性宏
属性的形式为 #[outer_attribute]（外部属性）或 #![inner_attribute]（内部属性），它们的区别在于应用的位置。

#[outer_attribute] 应用于紧随其后的条目。条目的例子包括：函数、模块声明、常量、结构体、枚举等。以下是一个示例，其中属性 #[derive(Debug)] 应用于结构体 Rectangle：

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#![inner_attribute] 应用于包含它的条目（通常是模块或 crate）。换句话说，这种属性被解释为应用于它所在的整个作用域。以下是一个示例，其中 #![allow(unused_variables)] 应用于整个 crate（如果放置在 main.rs 中）：

#![allow(unused_variables)]

fn main() {
    let x = 3; // 这通常会警告未使用的变量。
}
属性可以使用不同的语法接受参数：

#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]

属性可以有多个值，也可以跨多行分隔：


#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]

*/

//编译器提供了一个 dead_code lint，用于警告未使用的函数。可以使用属性来禁用这个 lint。



fn used_function() {}

// `#[allow(dead_code)]` 是一个用于禁用 `dead_code` lint 的属性
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 添加一个属性来抑制警告

fn main() {
    used_function();
}


/*
cfg
配置条件检查可以通过两种不同的操作符实现：

cfg 属性：在属性位置使用 #[cfg(...)]
cfg! 宏：在布尔表达式中使用 cfg!(...)
前者启用条件编译，后者在运行时条件性地求值为 true 或 false 字面量，允许在运行时进行检查。两者使用相同的参数语法。

cfg! 与 #[cfg] 不同，它不会移除任何代码，只会求值为 true 或 false。例如，当 cfg! 用于条件时，if/else 表达式中的所有代码块都需要是有效的，无论 cfg! 正在评估什么。
*/

// 这个函数只有在目标操作系统是 linux 时才会被编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("你正在运行 Linux！");
}

// 而这个函数只有在目标操作系统**不是** Linux 时才会被编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("你**不是**在运行 Linux！");
}

fn main() {
    are_you_on_linux();

    println!("你确定吗？");
    if cfg!(target_os = "linux") {
        println!("是的，这绝对是 Linux！");
    } else {
        println!("是的，这绝对**不是** Linux！");
    }
}


/*
自定义
一些条件（如 target_os）是由 rustc 隐式提供的，但自定义条件必须通过 --cfg 标志传递给 rustc。

尝试运行这段代码，看看没有自定义 cfg 标志会发生什么。

使用自定义 cfg 标志：

$ rustc --cfg some_condition custom.rs && ./custom
condition met!


*/