// 运行命令：cargo run -p learning_notes --example rts_primitives
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// let age: number = 30;          // TS 只有一种 number 类型
// let price: number = 9.99;
// let isActive: boolean = true;
// let grade: string = "A";       // TS 的 string 可以存多个字符
//
// // 类型转换
// let x: number = parseInt("42");
// let y: number = parseFloat("3.14");
// let s: string = String(42);
//
// // 数字运算
// console.log(10 / 3);              // 3.3333...（浮点）
// console.log(Math.floor(10 / 3));  // 3（整除）
// console.log(10 % 3);              // 1（取余）
// console.log(2 ** 8);              // 256（幂运算）
//
// // 数字方法
// Math.abs(-3.7)
// Math.ceil(-3.7)
// Math.floor(-3.7)
// Math.round(-3.7)
// Math.sqrt(16)
// Number.MAX_SAFE_INTEGER
// ============================================================

fn main() {
    // ============================================================
    // 一、整数类型
    // TS 只有 number，Rust 区分有符号/无符号、位宽
    // ============================================================
    let a: i8  = -128;             // 有符号 8 位：-128 ~ 127
    let b: i32 = -100_000;         // 有符号 32 位：最常用，下划线增加可读性（TS 也支持）
    let c: i64 = 9_000_000_000;    // 有符号 64 位
    let d: u8  = 255;              // 无符号 8 位：0 ~ 255
    let e: u32 = 100_000;          // 无符号 32 位：0 ~ 2^32-1
    let f: usize = 42;             // 平台相关（64位系统=u64），用于索引和长度
    println!("i8:{a}, i32:{b}, i64:{c}, u8:{d}, u32:{e}, usize:{f}");

    // ============================================================
    // 二、浮点类型
    // TS 的 number 默认是 64 位浮点，Rust 需要显式选择
    // ============================================================
    let f32_val: f32 = 3.14;               // 单精度，约 7 位有效数字
    let f64_val: f64 = 3.141592653589793;  // 双精度，对应 TS 的 number
    println!("f32:{f32_val}, f64:{f64_val}");

    // ============================================================
    // 三、布尔类型
    // 与 TS 完全一致，true/false
    // ============================================================
    let is_active: bool = true;
    let is_done: bool = false;
    println!("bool: {is_active}, {is_done}");

    // ============================================================
    // 四、字符类型
    // TS 没有单独的 char 类型；Rust 的 char 是 Unicode 标量值（4字节）
    // TS 用 string[0] 取单字符，Rust 用 char 字面量
    // ============================================================
    let letter: char = 'A';
    let emoji:  char = '🦀';
    let chinese: char = '锈';
    println!("char: {letter}, {emoji}, {chinese}");
    println!("char 是字母吗: {}", letter.is_alphabetic()); // TS 没有直接等价
    println!("char 转大写: {}", 'a'.to_uppercase().next().unwrap());

    // ============================================================
    // 五、整数运算
    // 关键差异：Rust 整数除法截断，TS 返回浮点
    // ============================================================
    let x: i32 = 10;
    let y: i32 = 3;
    println!("整除: {x}/{y} = {}", x / y);   // Rust: 3；TS: 3.333...
    println!("取余: {x}%{y} = {}", x % y);   // 两者一致: 1
    println!("幂: 2^10 = {}", i32::pow(2, 10)); // TS: 2 ** 10

    // 溢出：debug 模式会 panic，release 模式会环绕
    // let overflow: u8 = 255_u8 + 1; // ❌ debug 下会 panic
    let wrapped = 255_u8.wrapping_add(1); // ✅ 显式环绕：0
    println!("u8 环绕: {wrapped}");

    // ============================================================
    // 六、浮点运算
    // TS: Math.abs / Math.ceil / Math.floor / Math.round / Math.sqrt
    // ============================================================
    let n: f64 = -3.7;
    println!("abs:   {}", n.abs());    // TS: Math.abs(n)
    println!("ceil:  {}", n.ceil());   // TS: Math.ceil(n)
    println!("floor: {}", n.floor());  // TS: Math.floor(n)
    println!("round: {}", n.round());  // TS: Math.round(n)
    println!("sqrt(16): {}", 16_f64.sqrt()); // TS: Math.sqrt(16)
    println!("min(3,5): {}", f64::min(3.0, 5.0)); // TS: Math.min(3, 5)
    println!("max(3,5): {}", f64::max(3.0, 5.0)); // TS: Math.max(3, 5)

    // ============================================================
    // 七、类型转换（casting）
    // TS 用 Number(), parseInt() 等；Rust 用 as 关键字
    // ============================================================
    let int_val: i32 = 42;
    let as_f64: f64 = int_val as f64;   // i32 → f64
    let truncated: i32 = 3.99_f64 as i32; // f64 → i32，截断（不是四舍五入！）
    let as_u8: u8 = 300_i32 as u8;      // 超出范围时会截断位，300 % 256 = 44
    println!("i32 as f64: {as_f64}");
    println!("f64 as i32 (截断): {truncated}"); // 3，不是 4
    println!("300_i32 as u8 (截断): {as_u8}");  // 44

    // ============================================================
    // 八、字符串 ↔ 数字
    // TS: parseInt("42") / parseFloat("3.14") / String(42) / (3.14).toFixed(2)
    // ============================================================
    let parsed_int: i32  = "42".parse().unwrap();    // TS: parseInt("42")
    let parsed_f64: f64  = "3.14".parse().unwrap();  // TS: parseFloat("3.14")
    println!("解析: {parsed_int}, {parsed_f64}");

    let to_str = 42.to_string();                     // TS: String(42)
    let formatted = format!("{:.2}", 3.14159);       // TS: (3.14159).toFixed(2)
    println!("转字符串: {to_str}, 格式化: {formatted}");

    // ============================================================
    // 九、类型范围常量
    // TS: Number.MAX_SAFE_INTEGER (9007199254740991)
    // Rust 每种类型有自己的 MIN/MAX
    // ============================================================
    println!("i32 范围: {} ~ {}", i32::MIN, i32::MAX);
    println!("u8  范围: {} ~ {}", u8::MIN,  u8::MAX);
    println!("f64 无穷大: {}", f64::INFINITY);
    println!("f64 NaN 检查: {}", f64::NAN.is_nan()); // TS: isNaN(NaN)

    // ============================================================
    // 总结对照表
    // ============================================================
    println!("\n=== Rust vs TS 原始类型总结 ===");
    println!("┌────────────────────────┬──────────────────────────────────────┐");
    println!("│ TypeScript             │ Rust                                 │");
    println!("├────────────────────────┼──────────────────────────────────────┤");
    println!("│ number（64位浮点）      │ i8/i16/i32/i64/u8/u16/u32/u64/f32/f64│");
    println!("│ boolean                │ bool                                 │");
    println!("│ string                 │ String（堆）或 &str（借用）          │");
    println!("│ 无 char 类型            │ char（4字节 Unicode 标量）          │");
    println!("│ string[0] 取字符        │ .chars().nth(i) 安全取字符          │");
    println!("│ parseInt/parseFloat     │ .parse::<i32>() / .parse::<f64>()   │");
    println!("│ Math.floor/ceil/round   │ .floor()/.ceil()/.round()           │");
    println!("│ Number.MAX_SAFE_INTEGER │ i32::MIN, u8::MIN 等                │");
    println!("│ 整数除法返回浮点         │ 整数除法截断（用 f64 得浮点结果）    │");
    println!("└────────────────────────┴──────────────────────────────────────┘");
}
