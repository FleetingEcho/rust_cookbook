use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    // 纬度
    lat: f32,
    // 经度
    lon: f32,
}

impl Display for City {
    // `f` 是一个缓冲区,此方法必须将格式化的字符串写入其中。
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` 类似于 `format!`,但它会将格式化后的字符串
        // 写入一个缓冲区（第一个参数）。
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn test_print1() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // 一旦你为 fmt::Display 添加了实现,就把这里改成使用 {}。
        println!("{:?}", color);
    }
}

pub fn test() {
    test_print1();
    // 通常,`{}` 会被自动替换为任何参数。
    // 这些参数会被转换为字符串。
    println!("{} 天", 31);

    // 可以使用位置参数。在 `{}` 中指定一个整数
    // 来决定替换哪个额外的参数。参数编号
    // 从格式字符串后立即开始,从 0 开始。
    println!("{0},这是 {1}。{1},这是 {0}", "Alice", "Bob");

    // 还可以使用命名参数。
    println!(
        "{subject} {verb} {object}",
        object = "那只懒惰的狗",
        subject = "那只敏捷的棕色狐狸",
        verb = "跳过"
    );

    // 在 `:` 后指定格式字符,
    // 可以调用不同的格式化方式。
    println!("十进制：               {}", 69420); // 69420
    println!("二进制：               {:b}", 69420); // 10000111100101100
    println!("八进制：               {:o}", 69420); // 207454
    println!("十六进制：             {:x}", 69420); // 10f2c

    // 可以指定宽度来右对齐文本。这将输出
    // "    1"。（四个空格和一个 "1",总宽度为 5。）
    println!("{number:>5}", number = 1);

    // 可以用额外的零来填充数字,
    println!("{number:0>5}", number = 1); // 00001
                                          // 通过翻转符号来左对齐。这将输出 "10000"。
    println!("{number:0<5}", number = 1); // 10000

    // 在格式说明符后添加 `$` 可以使用命名参数。
    println!("{number:0>width$}", number = 1, width = 5); // 00001

    // Rust 甚至会检查使用的参数数量是否正确。
    println!("我的名字是 {0}, {1},{0}", "Bond", "James");
    // FIXME ^ 添加缺失的参数："James"

    // 只有实现了 fmt::Display 的类型才能用 `{}` 格式化。
    // 用户定义的类型默认不实现 fmt::Display。

    #[allow(dead_code)] // 禁用 `dead_code`,它会警告未使用的模块
    struct Structure(i32);

    // 这无法编译,因为 `Structure` 没有实现 fmt::Display。
    // println!("这个结构体 `{}` 无法打印...", Structure(3));
    // TODO ^ 尝试取消注释这一行

    // 在 Rust 1.58 及以上版本,你可以直接从周围的变量捕获参数。
    // 就像上面一样,这将输出 "    1",4 个空格和一个 "1"。
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}"); // 1
}
