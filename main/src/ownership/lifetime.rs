//悬垂指针和生命周期

/*
{
    let r;  // 这里 r 只是声明，但未初始化

    {
        let x = 5;  // x 在这个作用域内创建
        r = &x;     // r 试图存储对 x 的引用
    }  // x 作用域结束，x 被释放

    println!("r: {}", r);  // r 仍然引用 x，但 x 已被释放，导致悬垂指针
}
如何修正？
{
    let x = 5;
    let r = &x; // r 引用了 x，但 x 仍然有效
    println!("r: {}", r);
} // x 在这里才被释放

借用检查


*/


/*
函数中的生命周期

//编译失败
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}


x 和 y 是两个 &str（字符串切片），它们的生命周期可能不同。
函数返回值 &str 也是一个引用，但 Rust 无法推断它的生命周期应该与 x 还是 y 关联，或者是一个新的生命周期。
由于 Rust 的 借用检查 规则，必须显式声明生命周期，确保返回的引用在外部仍然有效。

因为 x 和 y 可能有不同的生命周期，Rust 必须 显式标注。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

生命周期参数
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用


一个生命周期标注，它自身并不具有什么意义，因为生命周期的作用就是告诉编译器多个引用之间的关系。
和泛型一样，使用生命周期参数，需要先声明 <'a>


如何修复？
 生命周期参数（lifetime parameters），告诉 Rust：返回的引用 必须 与 x 和 y 之中 至少存在时间最短的那个 一样长。


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

'a 是一个生命周期参数，它代表一个 未知但相同的生命周期。
x: &'a str, y: &'a str：表示 x 和 y 必须活得至少和 'a 一样长。
-> &'a str：表示 返回的引用也必须活得至少和 'a 一样长。


longest 不会返回一个比参数更短生命周期的引用（避免悬垂指针）。
longest 返回的引用 仍然有效，因为它和 x、y 共享生命周期 'a。




3. 如果 longest 直接返回一个新字符串，还需要生命周期吗？
不需要！因为返回值 不再是引用，而是一个新的 String，它有 自己的所有权：

fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}
这里 String 是 值类型，不会涉及生命周期问题。

*/


/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz"); // `string2` 在这个代码块内创建
        result = longest(string1.as_str(), string2.as_str()); // `result` 试图存储对 `string2` 或 `string1` 的引用
    } // `string2` 作用域结束，被释放

    println!("The longest string is {}", result); // ❌ `result` 可能存储了 `string2` 的引用，但 `string2` 已被释放！
}

如何修复？
方法 1：保证 string2 的生命周期足够长
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // ✅ `string2` 在 `main` 内部创建，作用域足够长
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}


方法 2：返回一个新的 String
fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()  // ✅ 直接返回新的 `String`，不会有生命周期问题
    } else {
        y.to_string()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // ✅ 现在 `result` 拥有一个新 `String`
    }
    println!("The longest string is {}", result); // ✅ 不再引用 `string2`
}

*/





/*
函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：

函数参数的生命周期
函数体中某个新建引用的生命周期

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() //编译错误， 因为返回了result的引用
}

//如何解决？
//最好的办法就是返回内部字符串的所有权，然后把字符串的所有权转移给调用者：

fn longest<'a>(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

fn main() {
   let s = longest("not", "important");
}


*/


/*
结构体中的生命周期

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");//novel 变量存储在 堆上（因为 String 是堆分配的）。
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");


    // split('.') 会返回 迭代器，next() 取第一个 子字符串切片 &str。
    // first_sentence 是 novel 的一部分，它只是一个 &str（指向 novel 内存的引用），并不拥有数据。


    let i = ImportantExcerpt {
        part: first_sentence,  // `part` 存储的是 `first_sentence` 的引用
    };
}


为什么不报错？

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago..."); // novel 生命周期开始
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,  // `part` 的生命周期 <= `novel`
    }; // `i` 作用域结束

} // `novel` 作用域结束，内存释放 ✅ 安全
first_sentence 依赖于 novel，它的生命周期 不能超过 novel。
i.part 引用了 first_sentence，所以 i 不能活得比 novel 更久。
Rust 自动推导生命周期，确保 i 及其字段 part 在 novel 释放前有效。


如何正确使用结构体中的生命周期？

使用 String 代替 &str
如果你 希望 ImportantExcerpt 具有独立的生命周期，可以 让它拥有字符串数据：
struct ImportantExcerpt {
    part: String, // ✅ `String` 有所有权，不需要生命周期参数
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'").to_string(); // ✅ 复制数据

    let i = ImportantExcerpt {
        part: first_sentence, // ✅ `ImportantExcerpt` 拥有 `String`，不再受 `novel` 影响
    };

    println!("Excerpt: {}", i.part);
}


*/


//生命周期消除

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
/*
s 是 &str 类型的 输入引用。
函数返回 &str（即从 s 的某部分截取的切片）。
返回值的生命周期一定不会超过 s 的生命周期，因为 &s[0..i] 或 &s[..] 都来自 s，所以 Rust 可以自动推导它们的生命周期。




*/


/*
Rust 的生命周期消除规则
规则 1：每个引用参数都有自己的生命周期
fn first_word<'a>(s: &'a str) -> &str { ... }
这里的 'a 代表 s 的生命周期，编译器会自动推断。


规则 2：如果只有一个输入生命周期，返回值继承该生命周期
fn first_word<'a>(s: &'a str) -> &'a str { ... }
s 的生命周期是 'a，返回值的 &str 也 必须活得和 s 一样长。
编译器会自动加上 'a，所以不用手写。

规则 3：如果有多个输入引用，但返回值只和某个参数有关，必须手动标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
这里 longest 可能返回 x 或 y，Rust 无法推断 返回值的生命周期，必须手动标注。




//自动推断举例？
//只有一个参数 s，返回值一定来自 s，所以可以安全推导生命周期。


fn first_word(s: &str) -> &str { // ✅ 只涉及一个参数，Rust 自动推断
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


总结
情况	是否适用生命周期消除	原因
一个引用参数（&str 或 &T）	✅ 适用	返回值的生命周期直接继承参数的生命周期
多个引用参数，返回值依赖其中之一	❌ 不适用	Rust 无法推测返回值属于哪个参数
多个引用参数，但返回 &self（方法）	✅ 适用	self 的生命周期 默认被继承
Rust 通过 生命周期消除，避免 不必要的手写生命周期，但在 多个引用参数的情况下，仍然需要 显式声明生命周期，以确保引用的安全性。

*/


/*
方法中的生命周期

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

关键点
'a: 'b 说明 'a 必须比 'b 活得久：

'a: 'b 代表 'a 的生命周期必须比 'b 长，确保 self.part（&'a str）的引用不会比 announcement（&'b str）短。
这样保证了 self.part 在 announcement 仍然有效时不会被释放，避免悬垂引用。
为什么 'a 必须比 'b 活得久？

&'a self 表示 ImportantExcerpt 结构体的 self 拥有 生命周期 'a。
announcement: &'b str 传入的是一个 短生命周期的字符串引用（可能只是一个临时字符串）。
由于 self.part 来自 self，所以 self 不能比 announcement 早被释放，否则 self.part 可能指向一个已经无效的值。
*/


/*
静态生命周期
静态生命周期（'static）
在 Rust 中，'static 是一个特殊的生命周期，意味着 引用的内容会在程序的整个生命周期内有效。通常用于：

字符串字面量（它们被存储在程序的静态数据区）
全局变量
某些长生命周期的资源，如线程中的数据

let s: &'static str = "我没啥优点，就是活得久，嘿嘿";


fn main() {
    let s: &'static str = "Hello, world!";
    println!("{}", s);
}


静态生命周期的变量
全局变量通常具有 'static 生命周期：

static HELLO: &str = "Hello, world!"; // 'static 生命周期

static 关键字意味着 整个程序运行期间，变量 HELLO 都是有效的。
区别： static 变量存储在 静态内存区，而非栈上。




如果你尝试错误地改成：

fn longest<'a>(x: &'a str, y: &'a str) -> &'static str {  // ❌ 错误！
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
错误原因：

这里的返回值声明为 &'static str，但 x 和 y 的生命周期 可能远小于 'static。
Rust 不能保证 x 或 y 会一直活着，所以编译器会报错。




总结
场景	是否 'static 生命周期	原因
字符串字面量 (&str)	✅ 默认 'static	编译时存入二进制，程序运行期间始终有效
全局 static 变量	✅ 默认 'static	全局变量存储在静态区，和程序同生命周期
线程中的数据	✅ 有时需要 'static	确保数据在线程运行期间不会被释放
普通字符串 String	❌ 不适用	String 存储在堆上，作用域结束会被释放
手动标注 'static	⚠️ 谨慎使用	可能会导致错误的生命周期推导，增加悬垂引用风险


最佳实践
让 Rust 自动推导生命周期，除非你真的需要 'static。
&'static str 适用于字符串字面量，不适用于 String。
在多线程或长期存活的数据时考虑 'static，但不要滥用它来绕过编译器的检查！ 🚀


*/


// Rust 生命周期+泛型+特征约束的组合使用！

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str // 避免悬垂引用，确保返回的 &str 在 x 和 y 失效前仍然有效。
where
    T: Display, // 特征约束， 必须实现Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let s1 = String::from("short");
    let s2 = String::from("very long string");

    let announcement = "Comparing lengths...";
    let result = longest_with_an_announcement(s1.as_str(), s2.as_str(), announcement);

    println!("The longest string is: {}", result);
}

// 📘 TypeScript 对比
// ====================
// ⚠️ **生命周期是 Rust 独有的概念，TS 完全没有！**
//
// TypeScript 使用 GC 管理内存，引用要么一直有效要么被回收，
// 不存在"引用比数据活得更久"的问题，也不需要生命周期标注。
//
// ```ts
// // TS 永远不需要生命周期
// function longest(s1: string, s2: string): string {
//   return s1.length > s2.length ? s1 : s2;
// }
// ```
//
// 理解技巧（从 TS 视角）：
// - 生命周期 `'a` 像一种"时间段标签"，告诉编译器引用的有效区间
// - `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` 读作：
//   "x 和 y 都至少存活 'a 这段时间，返回值也至少存活 'a"
// - 大多数情况下 Rust 编译器会自动推导（生命周期消除规则）
//
// 详细对照 → rust_vs_typescript.rs §8 "生命周期"

