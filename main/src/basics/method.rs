mod my {
    pub struct Rectangle {
        width: u32,
        pub height: u32,
    }

    impl Rectangle {
        // 结构体的构造函数 new 是 Rust 社区中 常见的命名约定，类似于其他语言的 constructor，方便开发者理解。
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        // 获取宽度（由于 width 是私有的，需要提供方法访问）
        pub fn width(&self) -> u32 {
            self.width
        }

        // 获取高度
        pub fn height(&self) -> u32 {
            self.height
        }

        // 计算矩形面积
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        // 判断当前矩形是否能容纳另一个矩形
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

// 定义 Message 枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 这里可以添加不同的逻辑处理
    }
}

fn main() {
    // 使用 Rectangle 结构体
    let rect1 = my::Rectangle::new(30, 50);
    let rect2 = my::Rectangle::new(10, 40);
    let rect3 = my::Rectangle::new(60, 45);

    println!("Rectangle 1 width: {}", rect1.width());
    println!("Rectangle 1 height: {}", rect1.height());
    println!("Rectangle 1 area: {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 使用 Message 枚举
    let m = Message::Write(String::from("hello"));
    m.call();
}



/// 定义 Rectangle 结构体
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// `new` 方法：创建一个新的 Rectangle 实例
    /// - 使用 `Self` 代替 `Rectangle`，提高代码可读性
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// `area` 方法：计算矩形的面积
    /// - `&self` 代表 **不可变借用**，不会修改结构体字段
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// `double_size` 方法：将矩形的宽度和高度 **扩大两倍**
    /// - `&mut self` 代表 **可变借用**，允许修改结构体字段
    pub fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    /// `into_square` 方法：将矩形转换为一个正方形
    /// - `self` 代表 **获取所有权**，意味着 `Rectangle` 实例将被消费
    /// - 适用于 **对象转换**，转换后原对象无法再被使用
    pub fn into_square(self) -> Square {
        Square {
            side: self.width.min(self.height), // 取宽高最小值作为正方形边长
        }
    }
}

/// 定义 Square 结构体（正方形）
struct Square {
    side: u32,
}

fn main() {
    // 创建一个矩形实例
    let mut rect = Rectangle::new(30, 50);

    // 计算矩形的面积
    println!("矩形面积: {}", rect.area());

    // 放大矩形尺寸
    rect.double_size();
    println!("放大后的矩形: {} x {}", rect.width, rect.height);

    // 将矩形转换为正方形（所有权转移）
    let square = rect.into_square();
    println!("转换后的正方形边长: {}", square.side);

    // 这里 rect 不能再被使用，因为它的所有权已经转移
}
