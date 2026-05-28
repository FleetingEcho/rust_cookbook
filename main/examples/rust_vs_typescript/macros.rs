// 运行命令：cargo run -p learning_notes --example rts_macros
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 打印
// console.log("hello", name);
// console.error("错误:", e);
// console.warn("警告");
//
// // 字符串格式化
// `Hello, ${name}! You are ${age} years old.`
// `Result: ${value.toFixed(2)}`
//
// // 断言（测试中）
// expect(result).toBe(42);
// expect(arr).toEqual([1, 2, 3]);
//
// // 抛出错误
// throw new Error("未实现");
// throw new Error("不可能到达这里");
//
// // 调试打印
// console.log("value:", value);  // 需要手动写变量名
// ============================================================

fn main() {
    let name = "Alice";
    let age = 30_i32;
    let score = 95.678_f64;

    // ============================================================
    // 一、打印宏
    // TS: console.log / console.error / console.warn
    // ============================================================

    // println!：打印并换行（TS: console.log）
    println!("Hello, {name}!");          // 直接内嵌变量名（Rust 1.58+）
    println!("age = {}", age);           // 位置参数
    println!("score = {:.2}", score);    // 格式化：保留2位小数（TS: score.toFixed(2)）
    println!("debug: {:?}", vec![1,2,3]); // Debug 格式（TS: JSON.stringify）
    println!("pretty: {:#?}", vec![1,2,3]); // 美化 Debug 格式

    // print!：不换行（TS: process.stdout.write）
    print!("A");
    print!("B");
    print!("C");
    println!();  // 只打印换行

    // eprintln!：打印到 stderr（TS: console.error）
    eprintln!("这是错误输出: {name}");

    // eprint!：不换行打印到 stderr
    eprint!("警告 ");
    eprintln!("信息");

    // ============================================================
    // 二、format!：字符串格式化（不打印，返回 String）
    // TS: 模板字符串 `Hello, ${name}!`
    // ============================================================
    let greeting = format!("你好，{}！", name);   // TS: `你好，${name}！`
    let age_str  = format!("{}岁", age);
    let fixed    = format!("{:.2}", score);        // TS: score.toFixed(2)
    let padded   = format!("{:>10}", "hi");        // 右对齐，宽度10（TS: "hi".padStart(10)）
    let zero_pad = format!("{:0>5}", 42);          // "00042"（TS: String(42).padStart(5,"0")）
    let hex      = format!("{:x}", 255);           // "ff"（TS: (255).toString(16)）
    let binary   = format!("{:b}", 10);            // "1010"（TS: (10).toString(2)）
    let sci      = format!("{:e}", 1234567.89);    // 科学计数法
    println!("{greeting}, {age_str}");
    println!("fixed={fixed}, padded='{padded}', zero_pad={zero_pad}");
    println!("hex={hex}, binary={binary}, sci={sci}");

    // 命名参数
    let msg = format!("{name}今年{age}岁，得了{score:.1}分");
    println!("{msg}");

    // ============================================================
    // 三、vec!：创建 Vec（语法糖）
    // TS: [1, 2, 3] 或 Array.from(...)
    // ============================================================
    let v1 = vec![1, 2, 3, 4, 5];           // TS: [1, 2, 3, 4, 5]
    let v2 = vec![0_i32; 5];                 // [0, 0, 0, 0, 0]（TS: new Array(5).fill(0)）
    let v3: Vec<String> = vec!["a", "b", "c"].into_iter().map(String::from).collect();
    println!("vec!: {:?}", v1);
    println!("重复: {:?}", v2);
    println!("字符串: {:?}", v3);

    // ============================================================
    // 四、断言宏（测试和调试用）
    // TS: expect(x).toBe(y) / jest / vitest
    // ============================================================

    // assert!：条件为 false 时 panic
    assert!(1 + 1 == 2);                      // TS: expect(1+1).toBe(2)
    assert!(age > 0, "年龄必须为正，实际: {age}");  // 带自定义错误信息

    // assert_eq!：相等断言（打印两者的值）
    assert_eq!(2 + 2, 4);                    // TS: expect(2+2).toEqual(4)
    assert_eq!(vec![1,2,3], vec![1,2,3]);

    // assert_ne!：不相等断言
    assert_ne!(1, 2);                        // TS: expect(1).not.toBe(2)

    println!("所有断言通过");

    // ============================================================
    // 五、dbg!：调试打印（自动打印文件名、行号、变量名和值）
    // TS: console.log("value:", value)（需要手动写名字）
    // ============================================================
    let x = 5_i32;
    let y = dbg!(x * 2) + 1;  // 打印 [src/...:行号] x * 2 = 10，并返回值
    println!("y = {y}");       // 11

    // dbg! 在表达式中使用（不中断代码流）
    let v = vec![1, 2, 3];
    let sum: i32 = dbg!(v.iter().sum());
    println!("sum = {sum}");

    // ============================================================
    // 六、todo! / unimplemented! / unreachable! / panic!
    // TS: throw new Error("TODO") / throw new Error("不可能")
    // ============================================================

    // todo!()：标记未完成的代码（编译通过，运行时 panic）
    // TS: throw new Error("TODO: 实现这个功能")
    fn not_yet_done() -> i32 {
        todo!("这个函数还没实现")
    }

    // unimplemented!()：标记故意不实现的代码
    fn deliberately_skipped() -> i32 {
        unimplemented!("此平台不支持此功能")
    }

    // unreachable!()：标记理论上不可能到达的代码
    // TS: throw new Error("不可能到达这里")
    fn categorize(n: i32) -> &'static str {
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => "正数",
            std::cmp::Ordering::Less    => "负数",
            std::cmp::Ordering::Equal   => "零",
            // 不需要 unreachable! 因为 Ordering 只有三个变体（穷举）
        }
    }
    println!("categorize(5): {}", categorize(5));

    // panic!：立即终止并报错（TS: throw new Error(...)）
    // panic!("致命错误: {}", message);  // 不运行，只是演示

    // ============================================================
    // 七、matches!：模式匹配检查（前面 pattern_matching 也有）
    // TS: x instanceof X 或 x.kind === "..."
    // ============================================================
    let opt: Option<i32> = Some(42);
    println!("是 Some: {}", matches!(opt, Some(_)));
    println!("是 Some(x>10): {}", matches!(opt, Some(x) if x > 10));

    // ============================================================
    // 八、write! / writeln!：写入到任意实现了 Write 的对象
    // TS: 没有直接对应（类似 Node.js 的 stream.write）
    // ============================================================
    use std::fmt::Write as FmtWrite;
    let mut output = String::new();
    write!(output, "Hello, {}!", name).unwrap();   // 写入 String
    writeln!(output, " 你今年 {} 岁。", age).unwrap();
    println!("write! 结果: {output}");

    // ============================================================
    // 九、自定义宏（macro_rules!）入门
    // TS: 没有宏，只有函数和泛型
    // Rust: 宏在编译前展开，可以接受可变数量的参数、生成代码
    // ============================================================

    // 简单宏：打印带标签的值
    macro_rules! log_value {
        ($label:expr, $value:expr) => {
            println!("[{}] = {:?}", $label, $value);
        };
    }
    log_value!("name", name);
    log_value!("vec", vec![1,2,3]);

    // 可变参数宏（类似 TS 的 ...args）
    macro_rules! my_vec {
        // 空
        () => { Vec::new() };
        // 一个或多个，逗号分隔
        ($($x:expr),+ $(,)?) => {
            {
                let mut v = Vec::new();
                $(v.push($x);)+
                v
            }
        };
    }
    let mv: Vec<i32> = my_vec![10, 20, 30];
    println!("my_vec!: {:?}", mv);

    // 宏的关键优势（TS 函数无法做到）：
    println!("\n=== 宏 vs TS 函数 ===");
    println!("1. 可变参数（无需数组包装）：println!({{a}}, {{b}}, {{c}})");
    println!("2. 接受表达式、类型、代码块作为参数");
    println!("3. 编译期展开，零运行时开销");
    println!("4. 可以生成代码（#[derive(Debug)] 就是过程宏）");
}
