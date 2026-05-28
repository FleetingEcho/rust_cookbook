pub fn string_practice() {
    let s: String = String::from("Hello, Rust!"); // 堆分配的字符串
    let s = "Hello".to_string(); // 和上面的一样

    let slice: &str = "Hello, world!"; // 字符串字面量，存储在程序的二进制中

    let name = "Rust";
    //拼接
    let s = format!("Hello, {}!", name);

    //追加字符
    let mut s = String::from("Hello,");
    s.push(' ');
    s.push_str(", Rust!"); //追加字符串
    println!("{}", s);

    {
        let s1 = String::from("Hello");
        let s2 = String::from(" Rust");
        let s3 = s1 + &s2; // 注意 s1 被移动消耗，不能再使用
        println!("{}", s3);
    }

    {
        let s1 = String::from("Hello");
        let s2 = String::from("Rust");
        let s3 = format!("{} {}", s1, s2); // s1, s2 仍然可用
        println!("{}", s3);
    }

    {
        let s = String::from("Hello, Rust!");
        println!("长度: {}", s.len());
    }

    //遍历字符串
    for c in "Hello Rust".chars() {
        println!("{}", c);
    }

    {
        let s = "Hello Rust";
        if let Some(ch) = s.chars().nth(2) {
            println!("{}", ch);
        }
        match s.chars().nth(2) {
            Some(ch) => println!("{}", ch),
            None => println!("No character at index 2."),
        };
    }

    //字符串切片（切片基于字节索引，要保证是合法 UTF-8 边界）
    {
        let mut s = String::from("hello world");
        {
            let slice = &s[0..5]; // 创建不可变借用
            println!("{}", slice); // 使用 slice
        } // `slice` 作用域结束，借用释放

        s.push_str("!!!"); // ✅ 现在可以修改 `s`
        println!("{}", s); // 输出 "hello world!!!"

        //& 创建引用，* 访问引用指向的值。
        // 即借用变量的所有权，不会转移所有权
    }

    {
        // 直接修改 s（避免借用冲突）
        let mut s = String::from("hello world");
        s = s.replace("world", "Rust"); // 直接修改
        println!("{}", s); // 输出 "hello Rust"
    }
}
