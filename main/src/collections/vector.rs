//动态数组 Vector
/*
let v: Vec<i32> = Vec::new(); // 必须指明类型

let mut v = Vec::new(); // 可以不指定
v.push(1);


let v = vec![1, 2, 3]; // 使用宏来创建数组

更新
let mut v = Vec::new();
v.push(1);
Vector 与其元素共存亡
跟结构体一样，Vector 类型在超出作用域范围后，会被自动删除


但是有引用之后，会不一样



读取元素
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {third}"),
    None => println!("去你的第三个元素，根本没有！"),
}




let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // ✅ 获取 `v` 的不可变引用

v.push(6); // ❌ 这里对 `v` 进行了可变借用

println!("The first element is: {first}"); // ❌ 这里仍然使用了 `first`

//修改后
let mut v = vec![1, 2, 3, 4, 5];

let first = v[0].clone(); // ✅ 获取值，而不是引用

v.push(6);

println!("The first element is: {first}");



*/



/*
存储不同类型的元素

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()), // IPv4 变体
        IpAddr::V6("::1".to_string()),       // IPv6 变体
    ];

    for ip in v {
        show_addr(ip);
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}


trait Ip {
    fn show(&self);
}

struct V4(String);
struct V6(String);

impl Ip for V4 {
    fn show(&self) {
        println!("IPv4: {}", self.0);
    }
}

impl Ip for V6 {
    fn show(&self) {
        println!("IPv6: {}", self.0);
    }
}

fn main() {
    let v: Vec<Box<dyn Ip>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.show();
    }
}
*/





fn vec_methods_demo() {
    // 创建一个可变的 Vec
    let mut v = vec![1, 2];
    println!("初始 Vec: {:?}", v);

    // 检查是否为空
    assert!(!v.is_empty()); // `is_empty()` 检查 `Vec` 是否为空

    // 在索引 2 处插入元素 3，注意索引不能超过 `Vec` 长度
    v.insert(2, 3);
    println!("插入 3 后: {:?}", v); // [1, 2, 3]

    // 移除索引 1 处的元素，返回被删除的元素
    assert_eq!(v.remove(1), 2);
    println!("移除索引 1 后: {:?}", v); // [1, 3]

    // `pop()` 移除并返回 `Vec` 尾部的元素
    assert_eq!(v.pop(), Some(3));
    println!("调用 pop() 后: {:?}", v); // [1]

    assert_eq!(v.pop(), Some(1));
    println!("再次调用 pop(): {:?}", v); // []

    assert_eq!(v.pop(), None);
    println!("`pop()` 返回 None，Vec 为空: {:?}", v); // []

    // 清空 `Vec`
    v.clear();
    println!("清空 Vec 后: {:?}", v); // []

    // 追加 `Vec`
    let mut v1 = [11, 22].to_vec(); // `.to_vec()` 把数组转换为 `Vec`
    v.append(&mut v1); // `append` 会清空 `v1`
    println!("追加 v1 后: {:?}, v1: {:?}", v, v1); // v: [11, 22], v1: []

    // `truncate(n)` 保留前 `n` 个元素
    v.truncate(1);
    println!("截断到 1 个元素: {:?}", v); // [11]

    // `retain(|x| ...)` 仅保留符合条件的元素
    v.retain(|x| *x > 10);
    println!("保留 >10 的元素: {:?}", v); // [11]

    // `drain(start..end)` 删除指定范围的元素，并返回被删除元素的迭代器
    let mut v2 = vec![11, 22, 33, 44, 55];
    let drained: Vec<_> = v2.drain(1..=3).collect();
    println!("`drain(1..=3)` 结果: v2 = {:?}, drained = {:?}", v2, drained); // v2: [11, 55], drained: [22, 33, 44]

    // `split_off(n)` 从指定索引 `n` 处分割成两个 `Vec`
    let v3 = drained.split_off(1);
    println!("`split_off(1)`: drained = {:?}, v3 = {:?}", drained, v3); // drained: [22], v3: [33, 44]
}

fn main() {
    vec_methods_demo();
}



//数组切片
fn main() {
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);
}



// Vector排序===============================

fn main() {
    let mut v = vec![5, 3, 8, 1, 2];
    v.sort(); // 按照从小到大的顺序排序
    println!("{:?}", v); // 输出: [1, 2, 3, 5, 8]
}



fn main() {
    let mut v = vec![5, 3, 8, 1, 2];

    v.sort_by(|a, b| b.cmp(a)); // `b.cmp(a)` 实现降序排序
    println!("{:?}", v); // 输出: [8, 5, 3, 2, 1]
}


fn main() {
    let mut words = vec!["apple", "banana", "kiwi", "cherry"];

    words.sort_by_key(|word| word.len()); // 按字符串长度排序
    println!("{:?}", words); // 输出: ["kiwi", "apple", "cherry", "banana"]
}

fn main() {
    let mut data = vec![(1, "apple"), (3, "banana"), (2, "kiwi")];

    data.sort_by_key(|item| item.0); // 按元组的第一个值排序
    println!("{:?}", data); // 输出: [(1, "apple"), (2, "kiwi"), (3, "banana")]
}

//倒叙
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.reverse();
    println!("{:?}", v); // 输出: [5, 4, 3, 2, 1]
}

//非稳定排序
fn main() {
    let mut v = vec![5, 3, 8, 1, 2];
    v.sort_unstable(); // 可能比 `sort()` 快
    println!("{:?}", v); // 输出: [1, 2, 3, 5, 8]
}




#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    people.sort_by(|a, b| a.age.cmp(&b.age)); // 按年龄升序

    people.sort_by(|a, b| b.name.cmp(&a.name)); // 按 name 降序
    println!("{:?}", people);
}


//多重排序

fn main() {
    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Charlie".to_string(), age: 25 },
    ];

    people.sort_by_key(|p| (p.age, p.name.clone())); // 按 age 排序，age 相同时按 name 排序
    println!("{:?}", people);
}

// 📘 TypeScript 对比
// ====================
// Rust `Vec<T>` ≈ TS `Array<T>`
//
// | 操作 | Rust | TypeScript |
// |------|------|-----------|
// | 创建 | `vec![1, 2, 3]` | `[1, 2, 3]` |
// | 追加 | `v.push(4)` | `v.push(4)` |
// | 索引 | `v[0]`（可能 panic！） | `v[0]`（返回 undefined） |
// | 安全读取 | `v.get(0)` 返回 `Option<&T>` | `v.at(0)`（ES2022） |
// | 遍历 | `for i in &v` 或 `v.iter()` | `for...of` 或 `.forEach()` |
// | 切片 | `&v[1..3]`（不拷贝） | `.slice(1,3)`（拷贝） |
// | 排序 | `v.sort()` / `v.sort_by()` | `.sort()` |
//
// ⚠️ Rust 的 Vec 索引越界会导致程序**直接崩溃**（panic），
//    这是有意设计——Rust 认为越界是程序 bug，不应静默失败。
//    TS 返回 undefined，可能让 bug 隐藏到更深的地方。
//
// 详细对照 → rust_vs_typescript.rs §13 "集合"

