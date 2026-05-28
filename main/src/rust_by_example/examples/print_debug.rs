// 为 `Structure` 派生 `fmt::Debug` 实现。`Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)] // 自动创建打印
struct Structure(i32);

// 在 `Deep` 结构体中放入一个 `Structure`。使其也可打印。
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn test() {
    // 使用 `{:?}` 打印类似于使用 `{}`。
    println!("{:?} 个月在一年中。", 12);
    println!(
        "{1:?} {0:?} 是这个 {actor:?} 的名字。",
        "Slater",
        "Christian",
        actor = "演员"
    );

    // `Structure` 现在可以打印了！
    println!("现在 {:?} 将会打印！", Structure(3));

    // `derive` 的问题是无法控制输出的样式。
    // 如果我只想显示一个 `7` 怎么办？
    println!("现在 {:?} 将会打印！", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // 美化打印
    println!("{:?}", peter);
    println!("{:#?}", peter);
}
