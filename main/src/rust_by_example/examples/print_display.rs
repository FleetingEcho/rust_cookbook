// 通过 `use` 导入 `fmt` 模块使其可用。
use std::fmt;

// 定义一个包含两个数字的结构体。派生 `Debug` 特性,
// 以便与 `Display` 的结果进行对比。
#[derive(Debug)]
struct MinMax(i64, i64);

// 为 `MinMax` 实现 `Display` 特性。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 引用每个位置的数据点。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 定义一个结构体,其字段可命名以便比较。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 同样,为 `Point2D` 实现 `Display` 特性。
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义实现,只显示 `x` 和 `y`。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// 定义一个名为 `List` 的结构体,包含一个 `Vec`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组索引提取值,
        // 并创建一个指向 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 遍历 `vec` 中的 `v`,同时用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 除第一个元素外,为每个元素添加逗号。
            // 使用 ? 运算符在出错时返回。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // 闭合左括号并返回 fmt::Result 值
        write!(f, "]")
    }
}

pub fn test() {
    let minmax = MinMax(0, 14);

    println!("比较结构体：");
    println!("Display：{}", minmax);
    println!("Debug：{:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "大范围是 {big},小范围是 {small}",
        small = small_range,
        big = big_range
    );
    println!("大范围是 {big_range},小范围是 {small_range}");

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("比较点：");
    println!("Display：{}", point);
    println!("Debug：{:?}", point);

    // 错误。虽然实现了 `Debug` 和 `Display`,但 `{:b}` 需要
    // 实现 `fmt::Binary`。这行代码无法工作。
    // println!("Point2D 的二进制表示是什么：{:b}？", point);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
