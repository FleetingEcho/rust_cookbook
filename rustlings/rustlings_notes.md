```rust
// for 循环的范围表达式 0..num 会消耗（consume） num 的值，而不是借用它。
// 0..num 这里 num 确实被"消耗"进了 Range，但因为 u16 实现了 Copy，所以 Rust 自动复制了一份给 Range 用，原来的 num 绑定完好无损。
// u16 支持 Copy，0..num 时自动隐式复制一份给 Range 
fn call_me(num: u16) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
```

**不支持 `Copy` 的类型**
记忆技巧：

栈上固定大小、轻量的 → 一般是 Copy
涉及堆分配、资源管理 → 一定不是 Copy

核心规律：**凡是需要管理堆内存或独占资源的类型**，都不支持。

| 类型 | 原因 |
|------|------|
| `String` | 堆上数据，复制需要深拷贝 |
| `Vec<T>` | 同上 |
| `Box<T>` | 独占堆内存所有权 |
| `HashMap` 等集合 | 堆内存 |
| `File`, `TcpStream` | 系统资源，不能随意复制 |
| 包含上述类型的结构体 | 成员不是Copy，整体也不是 |

自定义结构体默认不支持，需要手动派生：
```rust
rust#[derive(Copy, Clone)]  // 加上这个才支持隐式复制
struct Point {
    x: f32,
    y: f32,  // 所有字段都是 Copy，才能派生
}

struct User {
    name: String,  // String 不是 Copy
    age: u8,
}
```
// ❌ 无法 derive(Copy)，因为 name 是 String

```rs
let c: char = 'A';          // 单引号 → char
let s: &str = "A";          // 双引号 → 字符串切片

// 编译错误！类型不匹配
let err: char = "A";        // error[E0308]: expected `char`, found `&str`
let err2: &str = 'A';       // error[E0308]: expected `&str`, found `char`

// 类型显式说明才能看出问题
let x = 'A';       // x: char
let y = "A";       // y: &str
```



```rs
let a = [1, 2, 3, 4, 5];

&a[1..4]    // [2, 3, 4]       → 从索引 1 到 3（不含 4）
&a[..3]     // [1, 2, 3]       → 从头到索引 2（不含 3）
&a[2..]     // [3, 4, 5]       → 从索引 2 到结尾
&a[..]      // [1, 2, 3, 4, 5] → 整个数组（等价于 &a）
&a[0..=3]   // [1, 2, 3, 4]    → 含右端（Rust 1.26+）
&a[1..=3]   // [2, 3, 4]       → 含右端
```


```rs
// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: &[i32]) -> Vec<i32> {
    let mut new_vec = vec.to_vec();   // 内部克隆，调用方看不见
    new_vec.push(88);
    new_vec
}
```


场景                                 → 推荐
──────────────────────────────────────────────────────────
关心 1 个变体，其他忽略                → if let
关心 ≥2 个变体                        → match
需要 else/else if 链                   → match（更结构化）
从 Option/Result 快速取值             → if let
需要 @ 绑定、| 多模式、if guard       → match


```rs

  fn process2(&mut self, message: Message) {
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(p) => self.move_position(p),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        if let Message::Resize { width, height } = message {
            self.resize(width, height);
        } else if let Message::Move(p) = message {
            self.move_position(p);
        } else if let Message::Echo(s) = message {
            self.echo(s);
        } else if let Message::ChangeColor(r, g, b) = message {
            self.change_color(r, g, b);
        } else if message == Message::Quit {
            self.quit();
        }
    }
    ```

```rs
// ✅ match：处理 2-3 个变体时清晰
match x {
    Some(val) if val > 0 => positive(val),
    Some(val) => zero_or_neg(val),
    None => default(),
}

// ❌ if let 链：写起来啰嗦
if let Some(val) = x {
    if val > 0 { positive(val) }
    else { zero_or_neg(val) }
} else { default() }


// ✅ if let：1 行搞定
if let Ok(value) = risky_operation() {
    process(value);
}

// ❌ match：要写 3 行以上
match risky_operation() {
    Ok(value) => process(value),
    Err(_) => {},  // 为了穷尽写一个空分支
}
```

```rs
所有权被转移了

     let entries = vec![
        (String::from("a"), 1_i32),
        (String::from("b"), 2),
        (String::from("c"), 3),
    ];
    let map_from_vec: HashMap<String, i32> = entries.into_iter().collect();
    println!("{:?}__ {:?}",map_from_vec,entries);
```


```rs
fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
          basket.entry(fruit).or_insert(1);
    }
}

 let team_1 = scores.entry(team_1_name).or_default();
	如果队伍不存在则插入 TeamScores::default()（0, 0），返回 &mut TeamScores


// basket.entry(Fruit::Banana).or_insert(1);
第 1 步：basket.entry(Fruit::Banana)

在 HashMap 里查找 Banana 这个键
没找到 → 返回 Entry::Vacant(...)（含一个空的槽位）
第 2 步：.or_insert(1)

在 Vacant 上调用 → 在空槽位插入 1
返回 &mut 1（指向新插入值的可变引用）



// if 写法——查了两次 HashMap
for fruit in fruit_kinds {
    if !basket.contains_key(&fruit) {    // 第 1 次查找
        basket.insert(fruit, 1);         // 第 2 次查找
    }
}

// entry 写法——只查一次
for fruit in fruit_kinds {
    basket.entry(fruit).or_insert(1);    // 唯一的一次查找
}
```

```rs
// ❌ 编译错误——类型不匹配
let mut count: u32 = basket.get(&Fruit::Apple).unwrap();
//  ^^^ 期望 u32，得到 &u32
// error[E0308]: mismatched types

// ✅ 必须加 *
let mut count: u32 = *basket.get(&Fruit::Apple).unwrap();


// 如果 get 返回值（而非引用）：
fn get(&self, key: &K) -> Option<V>
// 那就意味着每次 get 都要把值拷贝/移动出来，
// HashMap 里的数据就被掏空了！

// 所以返回引用是合理的设计：
fn get(&self, key: &K) -> Option<&V>
// 只借不拿
```


```rs
split_iterator.next().unwrap().parse().unwrap();
next()      → Option<&str>     → 错误：缺少字段
parse()     → Result<u8, E>    → 错误：字段不是数字

都有可能panic
解决的话

let team_1_score: u8 = if let Ok(val) = split_iterator.next().unwrap().parse() {
    val
} else {
    0  // 或者 panic!("invalid score format")
};

let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap_or(0);

let team_1_score: u8 = match split_iterator.next().unwrap().parse() {
    Ok(val) => val,
    Err(_) => 0,
};
```


```rs
match中不能有表达式
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    match hour_of_day {
        0..=21 => Some(5),
        22|23 => Some(0),
        _ => None,
    }
}
```


struct规则总结
场景	语法	示例
定义结构体	struct Name { }	struct Point { x: i32 }
创建实例	Name { }	Point { x: 10, y: 20 }
解构/模式匹配	Name { }	let Point { x, y } = p
类型注解	Name	let p: Point = ...

```rs
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 1, y: 2 };  // 创建：写 Point

// 解构时也要写 Point
let Point { x: x1, y: y1 } = p;  // 解构：写 Point
```


```rs
// Ok() 的完整类型是 Result<T, E>
Ok("hello")  
// Rust 知道 T = &str，但 E 可以是任何类型（未指定）

// Err() 的完整类型是 Result<T, E>  
Err("error")
// Rust 知道 E = &str，但 T 可以是任何类型（未指定）


let right = Ok("Hi! My name is Beyoncé");
// Rust 推断：T = &str，E = ?（未指定，可以是任何类型）

let left = result.as_deref();  // Result<&str, &String>
//                     ^^^^              ^^^^
//                     T=&str            E=&String

// assert_eq! 比较时，Rust 看到 left 的 E = &String
// 于是推断 right 的 E = &String
// 最终 right 类型：Result<&str, &String> ✅ 匹配！


let right = Err("Empty names aren't allowed");
// Rust 推断：E = &str，T = ?（未指定，可以是任何类型）

let left = result.as_ref().map_err(|e| e.as_str());  
// Result<&String, &str>
//   ^^^^^^^      ^^^^
//   T=&String    E=&str

// assert_eq! 比较时，Rust 看到 left 的 T = &String
// 于是推断 right 的 T = &String
// 最终 right 类型：Result<&String, &str> ✅ 匹配！
```

```rs
use std::path::PathBuf;

fn main() {
    // 例子1: String -> &str
    let ok_string: Result<String, &str> = Ok("Rust".to_string());
    println!("as_ref: {:?}", ok_string.as_ref());     // Ok(&"Rust")
    println!("as_deref: {:?}", ok_string.as_deref()); // Ok("Rust")
    
    // 例子2: PathBuf -> &Path
    let ok_path: Result<PathBuf, &str> = Ok(PathBuf::from("/tmp"));
    println!("as_ref: {:?}", ok_path.as_ref());       // Ok(&PathBuf)
    println!("as_deref: {:?}", ok_path.as_deref());   // Ok(&Path)
    
    // 例子3: 错误处理中的使用
    let err_result: Result<String, String> = Err("error".to_string());
    
    // as_ref 得到 Err(&String)
    let err_ref = err_result.as_ref();
    println!("{:?}", err_ref); // Err(&"error")
    
    // map_err 配合 as_ref 使用
    let mapped = err_result
        .as_ref()
        .map_err(|e| e.as_str());
    println!("{:?}", mapped); // Err("error")
}
```

```rs
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        let result = generate_nametag_text("Beyoncé".to_string());
        let left = result.as_deref();
        let right = Ok("Hi! My name is Beyoncé");// ok 是 Result<&str, &String>
        
        assert_eq!(left, right);
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        let result = generate_nametag_text(String::new());
        
        let left = result.as_ref().map_err(|e| e.as_str());
        let middle=result.as_deref();
        let right = Err("Empty names aren't allowed");// Err却是  Result<&String, &str>
        println!("{:?}",middle);
        
        assert_eq!(left, right);
    }
```


```rs

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.

    // let qty = item_quantity.parse::<i32>();
    // match qty{
    //     Ok(val)=> Ok(val * cost_per_item + processing_fee),
    //     Err(e) => Err(e),
    // }
    // let qty = item_quantity.parse::<i32>()?;
    // Ok(qty * cost_per_item + processing_fee)

    let qty = item_quantity.parse::<i32>();
    qty.map(|val| val * cost_per_item + processing_fee)
}


```



```rs
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
        
        // match value {
        //     0=>Err(CreationError::Zero),
        //     1..=100=> Ok(Self(value as u64)),
        //     _=>Err(CreationError::Negative),
        // }
        // match value {
        //     value if value > 0 => Ok(Self(value as u64)),
        //     0 => Err(CreationError::Zero),
        //     _ => Err(CreationError::Negative),  // 所有负数
        // }

        
        // 方法	类型	失败时	使用场景
        // as	强制转换	静默溢出/截断	确定安全时
        // into()	自动转换	编译错误	总是成功的转换
        // try_into()	尝试转换	返回 Result	可能失败的转换

        if value > 0 {
            match value.try_into() {
                Ok(v) => Ok(Self(v)),
                Err(_) => Err(CreationError::Negative), // 理论上不会发生
            }
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}

```


```rs
// Box<dyn Error> 以及解析错误

fn main()-> Result<(),Box<dyn Error> > {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

//     match main() {
        // Ok(()) => println!("Success!"),
        // Err(e) => println!("Error occurred: {}", e),  // 直接打印错误
    // }
```


```rs
// map_err 是 Rust 中 Result 类型的一个方法，用于转换错误类型，保持成功值不变。

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // TODO: change this to return an appropriate error instead of panicking
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }

```


```rs
fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers:Vec<i16> = Vec::new();

    // let n1: u8 = 42;
    // numbers.push(n1.into());  // 将 u8 转换为 Vec 的元素类型
    // let n2: i8 = -1;
    // numbers.push(n2.into());  // 将 i8 转换为 Vec 的元素类型
    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
```


```rs
// ❌ 错误：没有声明 T
impl Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
// 编译器不知道 T 是什么

// ✅ 正确：声明了 T
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```



```rs
// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
assert!(compare_license_types(SomeSoftware, OtherSoftware));

fn compare_license_types2<T: Licensed, U: Licensed>(software1: T, software2: U) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
assert!(compare_license_types2(SomeSoftware, OtherSoftware));

fn compare_license_types3(software1: Box<dyn Licensed>, software2: Box<dyn Licensed>) -> bool {
    software1.licensing_info() == software2.licensing_info()
}
assert!(compare_license_types3(Box::new(SomeSoftware), Box::new(OtherSoftware)));
```


```rs
//  交集类型：必须是既是 A 又是 B 的类型
fn some_func(item:  impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn some_func2<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}
```


```rs
struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// 只为需要打印的方法添加约束
impl<T> ReportCard<T> {
    fn print(&self) -> String 
    where
        T: std::fmt::Display,
    {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}
```

```rs
// Rust 无法知道返回的引用来自 x 还是 y，也就无法保证它在使用时的安全性。
生命周期参数：
'a = 生命周期参数 #1

'b = 生命周期参数 #2

'c = 生命周期参数 #3

fn main() {
    let string1 = String::from("短字符串");
    let result;
    {
        let string2 = String::from("这是一个非常长的字符串");
        result = longest(&string1, &string2);  // ✅ 编译通过，如果没加'a的话
        println!("较长的是：{}", result);  // 输出："这是一个非常长的字符串"
    }
    // println!("{}", result);  // ❌ 如果取消注释会报错！
                               // 因为 string2 已经被释放
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

```rs
特殊的生命周期：'static
Rust 中有一个特殊的生命周期 'static：

没有单引号会冲突：static 是关键字

表示整个程序运行期间都有效的引用

例如：字符串字面量

rust
let s: &'static str = "Hello";  // 字符串字面量的生命周期是 'static
let t: &str = "World";          // 简写，实际也是 &'static str
```


```rs
where用法，约束泛型参数
什么时候用 where？

✅ 约束很多（超过 2 个）
✅ 约束很复杂（有泛型嵌套）
✅ 需要生命周期约束
✅ 需要约束关联类型
✅ 想让函数签名更易读

fn print_and_clone<T>(x: T) -> T 
where 
    T: Clone + Debug,  // T 必须实现 Clone 和 Debug
{
    println!("{:?}", x);
}


fn example<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str 
where 
    'a: 'c,      // 'a 必须比 'c 活得久（或相等）
    'b: 'c,      // 'b 必须比 'c 活得久
{
    if x.len() > y.len() { x } else { y }
}

// 'static 是特殊的生命周期
fn with_static<T>(x: T) -> &'static str 
where 
    T: Debug,
{
    "always lives forever"
}


// 要求迭代器的元素类型必须实现 Display
fn print_all<I>(iter: I) 
where 
    I: Iterator,
    I::Item: std::fmt::Display,  // 关联类型约束
{
    for item in iter {
        println!("{}", item);
    }
}


// 方式1：直接写在尖括号里（简单情况）
fn old_style<T: Clone + Debug, U: PartialEq + Display>(a: T, b: U) {}

// 方式2：使用 where（复杂情况）
fn new_style<T, U>(a: T, b: U) 
where 
    T: Clone + Debug,
    U: PartialEq + Display,
{}

```


```rs
常见的断言宏对比
宏	用途	失败时的输出
assert!(expr)	判断表达式是否为 true	assertion failed
assert_eq!(left, right)	判断左右是否相等	显示左右两边的值
assert_ne!(left, right)	判断左右是否不相等	显示左右两边的值

```


```rs
// 当前代码
fn capitalize_words_vector1(words: &[&str]) -> Vec<String> {
    let new_words = words.to_vec();  // ❌ 不必要的转换
    new_words.iter().map(|x| capitalize_first(x)).map(|x| x.to_string()).collect()
    //                                    ↑ 已经返回 String  ↑ 又转一次 String？
}

fn capitalize_words_string1(words: &[&str]) -> String {
    let arr: Vec<String> = words.to_vec().iter().map(|x| capitalize_first(x).to_string()).collect();
    //                     ↑ 不必要        ↑ iter() ↑ capitalize_first 已经返回 String，不需要 .to_string()
    arr.join("")
}


fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
    //     ↑ 遍历切片      ↑ 解引用    ↑ 直接收集
}

fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| capitalize_first(word)).collect()
    //     ↑ 直接 collect() 到 String，不需要 join
}

```

```rs
全能的collect 
只要迭代器产出的类型匹配，collect() 就能转换成任何实现了 FromIterator 的类型。

// 口诀：迭代器 + collect() = 你想要的数据结构
iterator.collect() → Vec / String / HashSet / Result / ...

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    // collect() 可以将 Iterator<Item = Result<T, E>> 转换为 Result<Vec<T>, E>
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    // collect() 直接收集成 Vec<Result>
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}
```


```rs
product() 是 Rust 迭代器的一个方法，用于计算迭代器中所有元素的乘积。

let numbers = [2, 3, 4];
let result: i32 = numbers.iter().product();
assert_eq!(result, 2 * 3 * 4);  // 24

```

```rs
// ✅ Box 是一个指针（8字节），指向堆上的数据
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


rev() - 反转迭代器  //// 5 4 3 2 1
fold() - 累积计算
作用：遍历迭代器，把每个元素"折叠"成一个最终值

// 反转字符串
let s = "hello";
let reversed = s.chars().fold(String::new(), |acc, c| format!("{}{}", c, acc));
assert_eq!(reversed, "olleh");

```

Fold 对比Reduce

```rs

fold()：更灵活，更安全（处理空集合），但需要多写一个初始值

reduce()：更简洁，但可能返回 None，需要处理 Option

// fold: 需要初始值
let sum1 = numbers.iter().fold(0, |acc, x| acc + x);
assert_eq!(sum1, 10);

// reduce: 不需要初始值，第一个元素作为初始 acc
let sum2 = numbers.iter().reduce(|acc, x| acc + x);
assert_eq!(sum2, Some(10));

```


1. Cow 的两种状态
状态	含义	数据位置
Cow::Borrowed(&T)	借用数据	原始数据（不可变）
Cow::Owned(T)	拥有数据	自己的数据（可变）

```rs
// ✅ 正确：传入引用 → Borrowed
let vec = vec![0, 1, 2];
let input = Cow::from(&vec);  // 传入引用
// 结果：input = Cow::Borrowed(&vec)

// ✅ 正确：传入所有权 → Owned  
let vec = vec![0, 1, 2];
let input = Cow::from(vec);   // 传入所有权（没有 &）
// 结果：input = Cow::Owned(vec)
```

Join
```rs
// 假设没有 join，只有 is_finished()
if handle.is_finished() {
    // 问题1: 线程可能还没完成，结果拿不到
    // 问题2: 怎么获取返回值？JoinHandle 没有提供其他方法
    // 问题3: 需要循环等待，浪费 CPU
}

for handle in handles {
    let result = handle.join().unwrap();  // unwrap 获取线程的返回值
    results.push(result);
}
需求	解决方案
等待线程完成	join()
获取线程返回值	join() 返回 Result<T>
检查是否完成但不等待	is_finished() (不常用)
```


```rs
fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            let mut data = status_shared.lock().unwrap();  // 获取锁
            data.jobs_done += 1;  // 可以修改
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_data = status.lock().unwrap();
    println!("Jobs done: {}", final_data.jobs_done);
}

```


```rs
mpsc::Sender<u32> 是什么？
MPSC = Multiple Producer, Single Consumer（多生产者，单消费者）

想象一个餐厅：

tx（Sender） = 服务员点餐的机器（多个服务员都可以点餐）

rx（Receiver） = 厨房的打印机（一个地方接收所有订单）

send() = 服务员发送订单

recv() = 厨房接收订单

rust


```


```rs
mod macros {
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}

fn main() {
    macros::my_macro!();
}

```


记住：涡轮鱼是给编译器看的，帮助它理解你想用哪种类型！

涡轮鱼 语法
涡轮鱼 ::<> 的核心作用是在无法通过上下文推断类型时，显式指定泛型参数。主要用途：

✅ 类型转换 - parse::<i32>()
✅ 集合创建 - Vec::<String>::new()
✅ 迭代器收集 - collect::<Vec<_>>()
✅ 默认值 - Default::<i32>::default()
✅ 调用泛型方法 - 当有多个实现时区分
✅ 指定复杂类型的参数 - HashMap::<String, u32>::new()

```rs
fn main() {
    let s = "42";
    
    // 这些都可以工作：
    let a = s.parse::<u8>();      // 涡轮鱼
    let b: u8 = s.parse();         // 类型注解
    let c = s.parse::<i32>();      // 解析成 i32
    let d = s.parse::<f64>();      // 解析成 f64
    
    // ❌ 这不行：无法推断
    let e = s.parse();  // 编译错误
}

// 创建特定类型的集合
let vec1 = Vec::new();  // ❌ 无法推断类型
let vec2 = Vec::<i32>::new();  // ✅ 涡轮鱼

// 带容量的集合
let vec3 = Vec::<String>::with_capacity(10);

// 常见集合
let hashmap = std::collections::HashMap::<String, u32>::new();


let numbers = vec![1, 2, 3, 4, 5];

// collect() 时需要指定目标类型
let set1 = numbers.iter().collect();  // ❌ 无法推断
let set2 = numbers.iter().collect::<std::collections::HashSet<_>>();  // ✅
let vec2 = numbers.iter().map(|x| x * 2).collect::<Vec<_>>();

// 部分指定（用 _ 让编译器推断剩余部分）
let vec3 = numbers.iter().collect::<Vec<_>>();
```



```rs
        fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // 数组长度固定是3，不需要检查长度
        let red = arr[0];
        let green = arr[1];
        let blue = arr[2];
        
        if (0..=255).contains(&red) && 
           (0..=255).contains(&green) && 
           (0..=255).contains(&blue) {
            Ok(Color {
                red: red as u8,
                green: green as u8,
                blue: blue as u8,
            })
        } else {
            Err(IntoColorError::IntConversion)
        }
    }
```

AsRef 和 AsMut 的作用：提供一种"廉价"的引用转换能力。
谁实现了 AsRef<str>？
常见类型包括：

String - 可以转换成 &str

&str - 可以转换成自己

&String - 也可以转换

Box<str>、Cow<str> 等

实际例子

```rs
// 只能接受 &str
fn print_len(s: &str) {
    println!("长度: {}", s.len());
}

fn main() {
    let text1 = "hello";        // &str 类型
    let text2 = String::from("hello"); // String 类型
    
    print_len(text1);  // ✅ 可以
    // print_len(text2); // ❌ 错误！String 不能直接当 &str 用
    print_len(&text2); // 需要手动取引用
}


// 可以接受任何能转换成 &str 的类型
fn print_len<T: AsRef<str>>(s: T) {
    println!("长度: {}", s.as_ref().len());
}

fn main() {
    let text1 = "hello";
    let text2 = String::from("hello");
    
    print_len(text1);  // ✅ 可以，&str 实现了 AsRef<str>
    print_len(text2);  // ✅ 也可以！String 也实现了 AsRef<str>
    print_len(&text2); // ✅ 引用也可以
}

```

AsMut
特性	AsRef<T>	AsMut<T>
用途	只读访问	可修改访问
方法	as_ref(&self) -> &T	as_mut(&mut self) -> &mut T
权限	不能修改原数据	可以修改原数据
常见场景	读取字符串、数组长度	修改数值、更新数据

```rs
// 修改值的函数
fn double<T: AsMut<u32>>(value: &mut T) {
    let num = value.as_mut();  // 获得 &mut u32
    *num *= 2;                  // 修改实际的值
}

fn main() {
    let mut x = 5u32;
    double(&mut x);
    println!("{}", x);  // 输出: 10
    
    let mut y = Box::new(3u32);
    double(&mut y);  // Box<u32> 也实现了 AsMut<u32>
    println!("{}", y);  // 输出: 6
}
```



```rs
let mut x = 5;      // x 是 u32 类型
x = x * x;          // ✅ 可以直接操作
x *= x;             // ✅ 也可以
println!("{}", x);  // 25

// ========================
let mut x = 5;
let ref_x = &mut x;  // ref_x 是 &mut u32 类型

// 错误示范
// ref_x = ref_x * ref_x;  // ❌ 不能对引用做乘法
// ref_x *= ref_x;         // ❌ 也不能

// 正确做法：先用 * 解引用
*ref_x = *ref_x * *ref_x;  // ✅ 先拿到值，计算，再放回去
*ref_x *= *ref_x;           // ✅ 简写形式

```