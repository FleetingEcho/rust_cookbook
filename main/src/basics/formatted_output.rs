/*
println!("Hello");                 // => "Hello"
println!("Hello, {}!", "world");   // => "Hello, world!"
println!("The number is {}", 1);   // => "The number is 1"
println!("{:?}", (3, 4));          // => "(3, 4)"
println!("{value}", value=4);      // => "4"
println!("{} {}", 1, 2);           // => "1 2"
println!("{:04}", 42);             // => "0042" with leading zeros



print! 将格式化文本输出到标准输出，不带换行符
println! 同上，但是在行的末尾添加换行符
format! 将格式化文本输出到 String 字符串// 类似模板字符串
eprint!，eprintln!  打印错误

fn main() {
    let s = "hello";
    println!("{}, world", s);
    let s1 = format!("{}, world", s);
    print!("{}", s1);
    print!("{}\n", "!");
}
*/


/*
Debug 特征

与 {} 类似，{:?} 也是占位符：

{} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
{:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景
其实两者的选择很简单，当你在写代码需要调试时，使用 {:?}，剩下的场景，选择 {}。

但是对于结构体，需要派生Debug特征后，才能进行输出，总之很简单。

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "sunface".to_string(), age: 18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}

*/


/*
Display 特征


为自定义类型实现 Display 特征
struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
fn main() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}


为外部类型实现 Display 特征
struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}
fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}

*/

// Newtype 模式解决方案
// Newtype 模式 通过 定义一个新的封装类型（即 新结构体），来包装原始类型，并为这个新类型实现 Display。
/*

Rust 不允许为外部类型实现外部特征（孤儿规则）。


Newtype 模式的优势
绕过孤儿规则：因为 Array 是我们自己定义的类型，所以可以为它实现 Display。
提供额外功能：可以在 Array 内部实现其他方法，增强 Vec<i32> 的功能，而不影响标准库的 Vec<i32>。
类型安全：避免直接暴露 Vec<i32>，可以在 Array 里添加类型约束或额外的行为。

*/
// 定义一个新的结构体 `Array`，内部存储 `Vec<i32>`，本质上是一个 `Vec<i32>` 的封装
struct Array(Vec<i32>);

use std::fmt;
// 为 `Array` 实现 `Display`
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 访问 `self.0`，即 `Vec<i32>`，然后使用 `{:?}` 进行格式化输出
        write!(f, "数组是：{:?}", self.0)
    }
}


use std::ops::Deref;
impl Deref for Array {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let arr = Array(vec![1, 2, 3]); // 创建 `Array` 实例
    println!("{}", arr); // 使用 `Display` 格式化输出
    println!("{}", arr.len()); // 访问 Vec<i32> 的方法
}


/*
位置参数
fn main() {
    println!("{}{}", 1, 2); // =>"12"
    println!("{1}{0}", 1, 2); // =>"21"
    // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{1}{}{0}{}", 1, 2); // => 2112
}


具名参数
fn main() {
    println!("{argument}", argument = "test"); // => "test"
    println!("{name} {}", 1, name = 2); // => "2 1" 带名称的参数必须放在不带名称参数的后面，
    println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    带名称的参数必须放在不带名称参数的后面，
}

格式化输出，意味着对输出格式会有更多的要求，例如只输出浮点数的小数点后两位：


fn main() {
    let v = 3.1415926;
    // Display => 3.14
    println!("{:.2}", v);
    // Debug => 3.14
    println!("{:.2?}", v);
}

*/

// 字符串填充
// 字符串格式化默认使用空格进行填充，并且进行左对齐。

fn main() {
    //-----------------------------------
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    //-----------------------------------

    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);
}


// 数字填充:符号和 0
// 数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐。

fn main() {
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
}

/*
数字填充:符号和 0
数字格式化默认也是使用空格进行填充，但与字符串左对齐不同的是，数字是右对齐。

fn main() {
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);
}
对齐
fn main() {
    // 以下全部都会补齐5个字符的长度
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");

    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");
}
精度
精度可以用于控制浮点数的精度或者字符串的长度


fn main() {
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");
}
*/


/*
进制
可以使用 # 号来控制数字的进制输出：

#b, 二进制
#o, 八进制
#x, 小写十六进制
#X, 大写十六进制
x, 不带前缀的小写十六进制
fn main() {
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);

    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);

    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);
}
指数
fn main() {
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9
}
*/

// 指针地址
fn main(){
  let v= vec![1, 2, 3];
  println!("{:p}", v.as_ptr()) // => 0x600002324050
}

/*
转义
有时需要输出 {和}，但这两个字符是特殊字符，需要进行转义：

fn main() {
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}"
    println!(" Hello \"{{World}}\" ");

    // 下面代码会报错，因为占位符{}只有一个右括号}，左括号被转义成字符串的内容
    // println!(" {{ Hello } ");
    // 也不可使用 '\' 来转义 "{}"
    // println!(" \{ Hello \} ")
}
*/



/*
自动捕获变量

fn get_person() -> String {
    String::from("sunface")
}

fn main() {
    let person = get_person();
    println!("Hello, {person}!");
}



fn get_format() -> (usize, usize) {
    (10, 2) // 宽度 = 10，精度 = 2
}

fn get_scores() -> Vec<(&'static str, f64)> {
    vec![("Alice", 92.543), ("Bob", 88.99)]
}

fn main() {
    let (width, precision) = get_format(); // 获取格式化参数
    for (name, score) in get_scores() {
        println!("{name}: {score:width$.precision$}");
    }
}
width$ 和 precision$ 从作用域中的变量 width 和 precision 获取数值，而不是手动写 width = width, precision = precision。
{score:width$.precision$} 等价于：
println!("{name}: {score:10.2}");

其中：

width$ 会从作用域中查找 width 变量，并替换到 {} 里的 宽度 部分。


*/








