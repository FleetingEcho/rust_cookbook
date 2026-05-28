// 运行命令：cargo run -p learning_notes --example rts_structs
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 接口（只描述形状，无方法实现）
// interface User {
//     name: string;
//     age: number;
//     email: string;
// }
//
// // 类（有构造函数和方法）
// class User {
//     name: string;
//     age: number;
//
//     constructor(name: string, age: number, public email: string) {
//         this.name = name;
//         this.age = age;
//     }
//
//     greet(): string {
//         return `Hi, I'm ${this.name}, age ${this.age}`;
//     }
//
//     birthday(): void { this.age++; }
//
//     static create(name: string, age: number): User {
//         return new User(name, age, `${name}@example.com`);
//     }
// }
//
// // 对象展开（更新部分字段）
// const updated = { ...user, email: "new@example.com" };
//
// // 可选字段
// interface Profile { bio?: string; avatar?: string; }
//
// // 只读字段
// interface Config { readonly host: string; readonly port: number; }
// ============================================================

// derive 宏自动实现常用 trait
// Debug：等价于 TS 的可序列化（支持 {:?} 打印）
// Clone：支持深拷贝（.clone()），TS 的对象展开 {...obj} 是浅拷贝
// PartialEq：支持 == 比较
#[derive(Debug, Clone, PartialEq)]
struct User {
    name:  String,
    age:   u32,
    email: String,
}

// impl 块：给结构体绑定方法，对应 TS 的 class 方法
impl User {
    // 关联函数（静态方法）：TS 的 static 方法 或 构造函数
    // 惯例命名为 new，但不是关键字
    fn new(name: &str, age: u32, email: &str) -> Self {
        User {
            name: name.to_string(),
            age,                        // 字段名与变量名相同时可省略，TS 也支持 { age }
            email: email.to_string(),
        }
    }

    // 另一种构造方式（工厂方法）
    fn from_name(name: &str) -> Self {
        User {
            name: name.to_string(),
            age: 0,
            email: format!("{}@example.com", name.to_lowercase()),
        }
    }

    // &self：不可变方法，只读 self，对应 TS 普通方法
    fn greet(&self) -> String {
        format!("你好！我是 {}，今年 {} 岁", self.name, self.age)
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // &mut self：可变方法，可以修改 self，TS 方法默认都能修改 this
    fn birthday(&mut self) {
        self.age += 1;
    }

    fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }

    // 消耗 self（转换为另一种类型后，原值失效）
    // TS 没有对应概念，因为 JS 有 GC
    fn into_summary(self) -> String {
        format!("{}（{}）", self.name, self.email)
        // self 在此被消耗，调用后不能再用原变量
    }
}

// ============================================================
// 嵌套结构体
// TS: interface Employee { name: string; address: Address; }
// ============================================================
#[derive(Debug, Clone)]
struct Address {
    city:    String,
    country: String,
}

#[derive(Debug)]
struct Employee {
    name:    String,
    address: Address,
}

// ============================================================
// 泛型结构体
// TS: interface Pair<T, U> { first: T; second: U; }
// ============================================================
#[derive(Debug)]
struct Pair<T, U> {
    first:  T,
    second: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn show(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

fn main() {
    // ============================================================
    // 一、创建实例
    // TS: const user: User = new User("Alice", 30, "...")
    // ============================================================
    let mut user = User::new("Alice", 30, "alice@example.com");
    println!("{:?}", user);            // Debug 输出，类似 console.log(user)
    println!("{:#?}", user);           // 多行美化输出

    // 字段访问
    // TS: user.name, user.age
    println!("姓名: {}", user.name);
    println!("年龄: {}", user.age);

    // 调用方法
    println!("{}", user.greet());
    println!("成年了吗: {}", user.is_adult());

    // 可变方法
    user.birthday();
    println!("生日后: {:?}", user);

    user.set_email("newalice@example.com");
    println!("新邮箱: {}", user.email);

    // ============================================================
    // 二、结构体更新语法（类似 TS 对象展开）
    // TS: const user2 = { ...user, name: "Bob", email: "bob@..." }
    // ============================================================
    let user2 = User {
        name:  String::from("Bob"),
        email: String::from("bob@example.com"),
        ..user.clone()   // 其余字段从 user 复制（需要 Clone）
    };
    println!("user2: {:?}", user2);

    // ============================================================
    // 三、结构体比较
    // TS: 对象比较是引用比较，需要手动比较字段
    // Rust: 派生 PartialEq 后可以用 ==
    // ============================================================
    let user3 = user.clone();
    println!("user == user3: {}", user == user3);  // true
    println!("user == user2: {}", user == user2);  // false

    // ============================================================
    // 四、消耗 self（转换操作）
    // ============================================================
    let summary = user.into_summary();  // user 被消耗
    println!("摘要: {summary}");
    // println!("{}", user.name); // ❌ user 已被消耗，编译错误

    // ============================================================
    // 五、嵌套结构体
    // TS: { name: "Charlie", address: { city: "Beijing", country: "China" } }
    // ============================================================
    let emp = Employee {
        name: String::from("Charlie"),
        address: Address {
            city:    String::from("北京"),
            country: String::from("中国"),
        },
    };
    println!("员工: {}, 城市: {}", emp.name, emp.address.city);

    // 解构嵌套结构体
    let Employee { name, address: Address { city, .. } } = emp;
    println!("解构: {name} 在 {city}");

    // ============================================================
    // 六、元组结构体（无字段名，用位置访问）
    // TS: type RGB = [r: number, g: number, b: number]（命名元组）
    // ============================================================
    struct Color(u8, u8, u8);
    let red = Color(255, 0, 0);
    println!("红色 RGB: ({}, {}, {})", red.0, red.1, red.2);

    // ============================================================
    // 七、单元结构体（零大小，常用于 trait 实现标记）
    // TS 没有直接对应
    // ============================================================
    struct AlwaysEqual;
    let _marker = AlwaysEqual;

    // ============================================================
    // 八、泛型结构体
    // TS: new Pair<number, string>(1, "hello")
    // ============================================================
    let int_str_pair = Pair::new(42, "hello");
    let float_pair   = Pair::new(1.1_f64, 2.2_f64);
    int_str_pair.show();
    float_pair.show();

    // ============================================================
    // 九、可选字段（用 Option<T> 模拟 TS 的 ?: ）
    // TS: interface Profile { bio?: string }
    // ============================================================
    #[derive(Debug)]
    struct Profile {
        username: String,
        bio:      Option<String>,   // 对应 TS 的 bio?: string
        age:      Option<u32>,      // 对应 TS 的 age?: number
    }

    let p1 = Profile {
        username: String::from("alice"),
        bio:      Some(String::from("Rust 爱好者")),
        age:      None,
    };
    println!("Profile: {:?}", p1);
    println!("bio: {}", p1.bio.as_deref().unwrap_or("未填写"));  // TS: p1.bio ?? "未填写"

    // ============================================================
    // 十、Vec 中的结构体（常见模式）
    // TS: User[]
    // ============================================================
    let users = vec![
        User::from_name("Alice"),
        User::from_name("Bob"),
        User::from_name("Charlie"),
    ];

    // 找到特定用户
    let found = users.iter().find(|u| u.name == "Bob");  // TS: users.find(u => u.name === "Bob")
    println!("找到: {:?}", found);

    // 过滤
    let adults: Vec<&User> = users.iter().filter(|u| u.is_adult()).collect();
    println!("成年人数: {}", adults.len());

    // 映射（提取字段）
    let names: Vec<&str> = users.iter().map(|u| u.name.as_str()).collect(); // TS: users.map(u => u.name)
    println!("所有名字: {:?}", names);
}
