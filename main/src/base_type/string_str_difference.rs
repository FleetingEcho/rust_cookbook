// 1. String
// 特点：
// 是 堆分配的可变字符串。
// 存储 UTF-8 编码的文本。
// 由 Vec<u8> 实现，可以动态扩展。
// 用途：
// 适用于需要修改字符串内容的情况，如 push、push_str 或 clear。
// 示例：

// let mut s = String::from("hello");
// s.push_str(" world"); // 可以修改
// println!("{}", s);

// 2. str
// 特点：
// 是 字符串切片的底层类型，本身不存储字符串数据。
// 只能通过引用（&str）使用，无法直接声明 let s: str。
// 用途：
// 仅作为字符串数据的一部分存在（如 &str）。
// 主要用于 &str 形式，适合处理不可变借用的字符串数据。

// 3. &str
// 特点：
// 是 字符串切片，指向某个 String 或字面量字符串的部分或全部。
// 存储在堆上（如果是 String 的切片）或静态数据段（如果是字符串字面量）。
// 长度固定，不可变。
// 用途：
// 高效、轻量级的字符串引用，适用于大多数场景（如函数参数）。
// 示例：

// let s: &str = "hello world"; // 静态字符串字面量
// let s2: &str = &String::from("hello"); // String 的切片

// 4. &String
// 特点：
// 是对 String 的不可变引用，但本质上仍然是 String 类型。
// 不能直接传递给需要 &str 的函数，需要使用 .as_str() 或 &*s 转换。
// 用途：
// 通常不建议使用 &String 作为函数参数，而是使用 &str，因为 &str 更灵活。
// 示例：

// fn print_str(s: &str) {
//     println!("{}", s);
// }

// let s = String::from("hello");
// let s_ref: &String = &s;
// print_str(&s);       // 直接传 `&s`（自动解引用）
// print_str(s_ref);    // 自动解引用
// print_str(s_ref.as_str()); // 显式转换

// 5. Box<str>
// 特点：
// 是 str 的 Box 分配版本（即堆上的 str）。
// 不能像 String 那样修改，但可以转 String。
// 适用于当字符串数据不会变更，并希望减少堆分配的开销时。
// 用途：
// 适合存储字符串但不需要修改的场景。
// 可用于优化结构体，使其存储更紧凑（减少 String 的 Vec<u8> 额外空间）。
// 示例：

// let boxed_str: Box<str> = "hello world".into(); // 字符串字面量转换
// println!("{}", boxed_str);

// let string = String::from("hello");
// let boxed_str: Box<str> = string.into_boxed_str();
// println!("{}", boxed_str);

// 6. Box<&str>（极少使用）
// 特点：
// 是 &str 的 Box 版本，即 &str 的指针被存储在堆上，而 str 数据仍然可能在堆上或静态数据段上。
// 几乎没有实际用途，因为 &str 已经是一个引用，存入 Box 只是让它的指针本身堆分配，基本没有收益。
// 用途：
// 几乎不使用，一般 Box<str> 更合理。
// 示例：

// let s: &str = "hello";
// let boxed_ref: Box<&str> = Box::new(s); // 仅存储 `&str` 的指针
// println!("{}", boxed_ref);

// 📝 总结对比
// 类型	  是否可变	是否堆分配	是否是引用	主要用途
// String	✅ 可变	✅ 是	❌ 否	可变长字符串
// str	❌ 不可变	❌ 否	❌ 否	作为 &str 使用
// &str	❌ 不可变	❓ 可能是（堆/静态）	✅ 是	高效字符串引用
// &String	❌ 不可变	✅ 是	✅ 是	过渡用途，不推荐
// Box<str>	❌ 不可变	✅ 是	❌ 否	存储 str 但减少 String 开销
// Box<&str>	❌ 不可变	✅ 是	✅ 是	几乎无用途

// 🚀 什么时候用哪种字符串？
// 场景	推荐使用
// 需要可变字符串	String
// 只需要引用字符串	&str
// 需要堆分配但不可变的字符串	Box<str>
// 需要函数参数接受 String 或 &str	&str
// 需要长久持有但不修改字符串	Box<str>

// 最佳实践
// 尽量使用 &str 作为参数，除非必须可变或拥有所有权。
// 避免 &String 作为参数，直接用 &str 更灵活。
// 如果字符串不会变更且需要长期存储，可用 Box<str> 以减少 String 额外开销。
// Box<&str> 基本没有实际用途，避免使用。

// 📘 TypeScript 对比
// ====================
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 默认编码 | UTF-8 | UTF-16 |
// | 可变字符串 | `String`（堆分配，可变） | 字符串不可变 |
// | 不可变引用 | `&str`（借用） | `string` 全部不可变 |
// | 切片 | `&s[0..5]`（字节索引！可能 panic） | `.slice(0, 5)`（安全） |
// | 拼接 | `s.push_str("!")` 或 `format!` | `s + "!"` 或模板字面量 |
//
// ⚠️ TS 开发者的最大误区：
//    - TS 只有一种 `string`，Rust 有 `String` / `&str` / `Box<str>` 等
//    - Rust 的 `&str` 是**引用**，不是"另一种字符串"
//    - Rust 字符串不能通过索引访问！`s[0]` 是语法错误
//    - 详细对照 → rust_vs_typescript.rs §3 "字符串"
