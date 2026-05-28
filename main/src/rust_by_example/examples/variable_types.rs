#![allow(overflowing_literals)]
// 原生类型之间的类型转换
// 指定字面量的类型
// 使用类型推断
// 类型别名
pub fn main() {
    test4()
}

//Rust 不支持原始类型之间的隐式类型转换（强制转换）。但可以使用 as 关键字进行显式类型转换（转型）。
// 抑制所有由溢出转换引起的警告。

fn test1() {
    let decimal = 65.4321_f32;

    // 错误！不允许隐式转换
    // let integer: u8 = decimal;
    // 修复：^ 注释掉此行

    // 显式转换
    let integer = decimal as u8;
    let character = integer as char;

    // 错误！转换规则有限制。
    // 浮点数不能直接转换为字符。
    // let character = decimal as char;
    // 修复：^ 注释掉此行

    println!("类型转换：{} -> {} -> {}", decimal, integer, character);

    // 当将任何值转换为无符号类型 T 时,
    // 会反复加上或减去 T::MAX + 1,直到该值
    // 适合新类型

    // 1000 已经适合 u16
    println!("1000 转换为 u16 是：{}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 实际上,保留了最低有效位（LSB）的前 8 位,
    // 而朝最高有效位（MSB）方向的其余位被截断。
    println!("1000 转换为 u8 是：{}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 转换为 u8 是：{}", (-1i8) as u8);

    // 对于正数,这等同于取模运算
    println!("1000 对 256 取模是：{}", 1000 % 256);

    // 当转换为有符号类型时,（按位）结果等同于
    // 先转换为对应的无符号类型。如果该值的最高有效位
    // 为 1,则该值为负数。

    // 当然,如果已经适合的话就不需要转换。
    println!(" 128 转换为 i16 是：{}", 128 as i16);

    // 边界情况：128 在 8 位二进制补码表示中为 -128
    println!(" 128 转换为 i8 是：{}", 128 as i8);

    // 重复上面的例子
    // 1000 转换为 u8 -> 232
    println!("1000 转换为 u8 是：{}", 1000 as u8);
    // 而 232 在 8 位二进制补码表示中为 -24
    println!(" 232 转换为 i8 是：{}", 232 as i8);

    // 从 Rust 1.45 开始,`as` 关键字在浮点数转整数时执行*饱和转换*
    // 如果浮点值超出上界或低于下界,返回值
    // 将等于所越过的边界值。

    // 300.0 转换为 u8 是 255
    println!(" 300.0 转换为 u8 是：{}", 300.0_f32 as u8);
    // -100.0 转换为 u8 是 0
    println!("-100.0 转换为 u8 是：{}", -100.0_f32 as u8);
    // NaN 转换为 u8 是 0
    println!("   NaN 转换为 u8 是：{}", f32::NAN as u8);

    // 这种行为会产生少量运行时开销,可以通过不安全方法避免,
    // 但结果可能溢出并返回**不可靠的值**。请谨慎使用这些方法：
    unsafe {
        // 300.0 转换为 u8 是 44
        println!(
            " 300.0 转换为 u8 是：{}",
            300.0_f32.to_int_unchecked::<u8>()
        );
        // -100.0 转换为 u8 是 156
        println!(
            "-100.0 转换为 u8 是：{}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // NaN 转换为 u8 是 0
        println!("   NaN 转换为 u8 是：{}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn test2() {
    // 带后缀的字面值,其类型在初始化时确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面值,其类型取决于使用方式
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回变量的字节大小
    println!("`x` 的字节大小：{}", std::mem::size_of_val(&x));
    println!("`y` 的字节大小：{}", std::mem::size_of_val(&y));
    println!("`z` 的字节大小：{}", std::mem::size_of_val(&z));
    println!("`i` 的字节大小：{}", std::mem::size_of_val(&i));
    println!("`f` 的字节大小：{}", std::mem::size_of_val(&f));
}

fn test3() {
    // 通过类型注解,编译器得知 `elem` 的类型为 u8
    let elem = 5u8;

    // 创建一个空向量（可增长的数组）
    let mut vec = Vec::new();
    // 此时编译器还不知道 `vec` 的具体类型,
    // 只知道它是某种类型的向量（`Vec<_>`）。

    // 将 `elem` 插入向量中
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 `u8` 类型的向量（`Vec<u8>`）
    // TODO ^ 尝试注释掉 `vec.push(elem)` 这一行

    println!("{:?}", vec);
}

// `NanoSecond`、`Inch` 和 `U64` 都是 `u64` 的新名称。
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn test4() {
    // `NanoSecond` = `Inch` = `U64` = `u64`。
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // 注意,类型别名*不会*提供额外的类型安全性,因为别名*不是*新类型
    println!(
        "{} 纳秒 + {} 英寸 = {} 单位？",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
