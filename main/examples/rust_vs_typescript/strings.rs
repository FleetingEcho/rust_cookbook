// 运行命令：cargo run -p learning_notes --example rts_strings
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// let s: string = "hello";
//
// s.length                        // 字符数
// s.toUpperCase()
// s.toLowerCase()
// s.includes("ell")
// s.startsWith("he")
// s.endsWith("lo")
// s.indexOf("l")                  // 返回 number，找不到返回 -1
// s.slice(1, 3)
// s.replace("l", "r")
// s.replaceAll("l", "r")
// s.split("")
// s.trim() / trimStart() / trimEnd()
// s.repeat(3)
// `Hello ${name}!`                // 模板字符串
// String(42)
// (3.14).toFixed(2)
// parseInt("42")
// ============================================================

fn main() {
    // ============================================================
    // 一、两种字符串类型
    // TS 只有一种 string；Rust 有 &str 和 String，这是最大区别
    // ============================================================
    let s1: &str = "hello"; // &str：字符串切片，不可变，编译期已知，存在程序段
    let s2: String = String::from("world"); // String：堆分配，可增长，可变
    let s3: String = s1.to_string(); // &str → String（类似 TS 中的复制）
    let s4: &str = &s2; // String → &str（借用，不拷贝数据）
    println!("&str: {s1}, String: {s2}, 转换: {s3}, 借用: {s4}");

    // 函数参数推荐接收 &str，而不是 &String，更灵活
    fn print_str(s: &str) {
        println!("收到: {s}");
    }
    print_str("字面量"); // &str 直接传
    print_str(&s2); // String 自动解引用为 &str

    // ============================================================
    // 二、字符串拼接
    // TS: "a" + "b" 或模板字符串 `${a}${b}`
    // ============================================================
    let hello = String::from("你好");
    let world = String::from("，世界");
    // + 运算符消耗左侧所有权，右侧必须是 &str
    let combined = hello + &world;
    println!("+ 拼接: {combined}");
    // println!("{hello}"); // ❌ hello 已被消耗

    // format! 不消耗所有权，是最常用的拼接方式，对应 TS 模板字符串
    let name = "Rust";
    let greeting = format!("你好，{}！当前版本很棒。", name);
    println!("format!: {greeting}");

    // ============================================================
    // 三、长度
    // 关键差异：Rust 的 len() 是字节数，不是字符数！
    // TS 的 s.length 是 UTF-16 代码单元数
    // ============================================================
    let s = "hello 世界";
    println!("字节长度 len(): {}", s.len()); // 12（汉字各占3字节）
    println!("字符数 chars().count(): {}", s.chars().count()); // 8（真正的字符数）

    // ============================================================
    // 四、大小写
    // TS: toUpperCase() / toLowerCase()
    // ============================================================
    println!("大写: {}", "hello".to_uppercase()); // TS: "hello".toUpperCase()
    println!("小写: {}", "WORLD".to_lowercase()); // TS: "WORLD".toLowerCase()

    // ============================================================
    // 五、查找与检测
    // ============================================================
    let text = "hello rust programming";
    println!("contains: {}", text.contains("rust")); // TS: includes()
    println!("starts_with: {}", text.starts_with("hello")); // TS: startsWith()
    println!("ends_with: {}", text.ends_with("ing")); // TS: endsWith()

    // find 返回 Option<usize>，TS 的 indexOf 返回 -1 表示未找到
    match text.find("rust") {
        Some(idx) => println!("找到 'rust' 在位置: {idx}"),
        None => println!("未找到"),
    }
    // rfind：从右侧查找，TS: lastIndexOf()
    println!("rfind 'l': {:?}", text.rfind('l'));

    // ============================================================
    // 六、切片
    // TS: s.slice(1, 4)
    // 注意：Rust 切片按字节索引，中文字符必须在正确边界切割！
    // ============================================================
    let ascii = "hello world";
    let sliced = &ascii[0..5]; // TS: ascii.slice(0, 5)
    println!("切片: {sliced}");

    // 中文切片需要小心字节边界
    let chinese = "你好世界";
    // &chinese[0..2] 会 panic！汉字占 3 字节，需要 [0..3]
    let first_char = &chinese[0..3]; // "你"
    println!("汉字切片: {first_char}");

    // 安全的做法：用 chars() 迭代
    let first_two: String = chinese.chars().take(2).collect();
    println!("前两个字符: {first_two}");

    // ============================================================
    // 七、替换
    // TS: replace() 只替换第一个，replaceAll() 替换所有
    // ============================================================
    let sentence = "foo bar foo baz foo";
    println!("replace 第一个: {}", sentence.replacen("foo", "qux", 1)); // TS: replace()
    println!("replace 全部: {}", sentence.replace("foo", "qux")); // TS: replaceAll()

    // ============================================================
    // 八、分割
    // TS: s.split(",")
    // ============================================================
    let csv = "apple,banana,cherry";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split: {:?}", parts);

    // splitn：最多分割 n 份
    let limited: Vec<&str> = csv.splitn(2, ',').collect();
    println!("splitn(2): {:?}", limited); // ["apple", "banana,cherry"]

    // split_whitespace：按空白分割（忽略多余空格），TS 需要 split(/\s+/)
    let words: Vec<&str> = "  hello   rust  ".split_whitespace().collect();
    println!("split_whitespace: {:?}", words);

    // ============================================================
    // 九、去空格
    // TS: trim() / trimStart() / trimEnd()
    // ============================================================
    let padded = "  hello  ";
    println!("trim: '{}'", padded.trim());
    println!("trim_start: '{}'", padded.trim_start()); // TS: trimStart()
    println!("trim_end: '{}'", padded.trim_end()); // TS: trimEnd()

    // ============================================================
    // 十、重复
    // TS: "ab".repeat(3)
    // ============================================================
    println!("repeat: {}", "ab".repeat(3)); // "ababab"

    // ============================================================
    // 十一、字符迭代
    // TS: [...s] 或 Array.from(s)
    // ============================================================
    let emoji_str = "hi 🦀!";
    let chars: Vec<char> = emoji_str.chars().collect();
    println!("chars: {:?}", chars);

    // 按字节迭代（底层）
    for byte in "abc".bytes() {
        print!("{byte} "); // 97 98 99
    }
    println!();

    // ============================================================
    // 十二、数字 ↔ 字符串
    // TS: parseInt / parseFloat / String() / toFixed()
    // ============================================================
    let num: i32 = "42".parse().unwrap(); // TS: parseInt("42")
    let flt: f64 = "3.14".parse().unwrap(); // TS: parseFloat("3.14")
    println!("解析: {num}, {flt}");

    let to_str = 42.to_string(); // TS: String(42) 或 (42).toString()
    let fixed = format!("{:.2}", 3.14159); // TS: (3.14159).toFixed(2)
    let padded_n = format!("{:>8}", "hi"); // 右对齐填充，TS 需要 padStart()
    let zero_pad = format!("{:0>5}", 42); // "00042"，TS: String(42).padStart(5, "0")
    println!("to_string: {to_str}, toFixed: {fixed}");
    println!("padStart: '{padded_n}', zeroPad: {zero_pad}");

    // ============================================================
    // 十三、其他常用方法
    // ============================================================
    // is_empty：TS: s.length === 0 或 !s
    println!("is_empty: {}", "".is_empty());
    println!("非空: {}", "hi".is_empty());

    // chars().nth()：按索引取字符，TS: s[i] 或 s.charAt(i)
    let ch = "hello".chars().nth(1);
    println!("第2个字符: {:?}", ch); // Some('e')

    // lines()：按行分割，TS: s.split("\n")
    let multiline = "第一行\n第二行\n第三行";
    for line in multiline.lines() {
        println!("行: {line}");
    }

    // ============================================================
    // 总结对照表
    // ============================================================
    println!("\n=== Rust vs TS 字符串总结 ===");
    println!("┌────────────────────────────┬──────────────────────────────────────┐");
    println!("│ TypeScript                 │ Rust                                 │");
    println!("├────────────────────────────┼──────────────────────────────────────┤");
    println!("│ 只有一种 string 类型        │ &str（借用）和 String（自有）两种    │");
    println!("│ s.length = UTF-16 单元数    │ .len() = 字节数，chars().count() 字符│");
    println!("│ 字符串不可变               │ String 可增长，&str 不可变          │");
    println!("│ 拼接 \"a\" + \"b\"          │ format!(\"{{}}{{}}\" a, b) 或 a + &b  │");
    println!("│ s.slice(1, 3)              │ &s[1..3] 但按字节索引！危险！       │");
    println!("│ replace/replaceAll         │ .replace() / .replacen() 前 N 个    │");
    println!("│ trim/trimStart/trimEnd     │ .trim() / .trim_start() / .trim_end()│");
    println!("│ .repeat(3)                 │ \"ab\".repeat(3)                      │");
    println!("│ .padStart(10)              │ format!(\"{{:>10}}\" s)                │");
    println!("│ split(\",\")                 │ .split(',') 返回惰性迭代器          │");
    println!("│ s.includes(\"x\")            │ s.contains(\"x\")                   │");
    println!("│ 参数推荐用 string           │ 函数参数推荐 &str（更灵活）         │");
    println!("└────────────────────────────┴──────────────────────────────────────┘");
}
