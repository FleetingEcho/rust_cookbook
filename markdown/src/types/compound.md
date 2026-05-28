# Rust 复合类型

## 1. 类型别名

使用 `type` 关键字为现有类型创建别名：

```rust
type File = String;

fn open(f: &mut File) -> bool { true }
fn close(f: &mut File) -> bool { true }

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
}
```

## 2. String 与 &str

### 2.1 String 切片

```rust
let s = String::from("hello");

let slice = &s[0..2]; // 切片
let slice = &s[..2];  // 同上

let len = s.len();
let slice = &s[4..len];
let slice = &s[4..];  // 同上
let slice = &s[..];   // 整个字符串
```

### 2.2 借用规则

以下代码会报错：

```rust
let mut s = String::from("hello world");
let word = first_word(&s); // 不可变借用
s.clear();                 // 可变借用，冲突！
println!("the first word is: {}", word);
```

`s.clear()` 需要清空改变 `String`，因此需要一个可变借用。而 `println!` 又使用了不可变借用，两者冲突。

```rust
fn first_word(s: &String) -> &str {
    &s[..1]
}
```

### 2.3 String 与 &str 的转换

```rust
let s = String::from("hello,world!");
say_hello(&s);        // &String 自动解引用为 &str
say_hello(&s[..]);    // 切片
say_hello(s.as_str()); // 显式转换

fn say_hello(s: &str) {
    println!("{}", s);
}
```

### 2.4 字符串索引的注意事项

Rust 不允许去索引字符串，因为索引操作期望性能是 O(1)，然而对于 `String` 类型来说，需要从零开始遍历来定位合法的字符。通过索引区间访问字符串时，一不注意就会导致程序崩溃：

```rust
let hello = "中国人";
let s = &hello[0..2]; // 报错：索引的字节落在了字符的内部
```

## 3. String 修改操作

```rust
let mut s = String::from("Hello ");

s.push_str("rust"); // 追加字符串
println!("追加字符串 push_str() -> {}", s);

s.push('!'); // 追加字符
println!("追加字符 push() -> {}", s);

s.insert(5, ',');
println!("插入字符 insert() -> {}", s); // Hello, rust!

s.insert_str(6, " I like");
println!("插入字符串 insert_str() -> {}", s); // Hello, I like rust!
```

## 4. 字符串替换

```rust
let string_replace = String::from("I like rust. Learning rust is my favorite!");
let new_string_replace = string_replace.replace("rust", "RUST");
// 替换所有匹配到的

let string_replace = "I like rust. Learning rust is my favorite!";
let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
// new_string_replacen = "I like RUST. Learning rust is my favorite!"
// 只替换一次

let mut string_replace_range = String::from("I like rust!");
string_replace_range.replace_range(7..8, "R");
// 直接操作原来的字符串，不会返回新的字符串，需要使用 mut
```

## 5. 字符操作

### 5.1 pop

```rust
let mut string_pop = String::from("rust pop 中文!");
let p1 = string_pop.pop(); // Some('!')
let p2 = string_pop.pop(); // Some('文')
// string_pop = "rust pop 中"
```

### 5.2 remove

```rust
let mut string_remove = String::from("测试remove方法");
println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));

string_remove.remove(0); // 删除第一个汉字
// string_remove.remove(1); // 错误：不在字符边界上
// string_remove.remove(3); // 直接删除第二个汉字
dbg!(string_remove);
```

因为中文占 3 个字节，所以删除时必须按字符边界操作。

### 5.3 truncate 与 clear

```rust
let mut string_truncate = String::from("测试truncate");
string_truncate.truncate(3); // 一个汉字 3 字节
dbg!(string_truncate); // string_truncate = "测"

let mut string_clear = String::from("string clear");
string_clear.clear();
dbg!(string_clear);
```

## 6. 字符串连接

### 6.1 使用 + 运算符

```rust
let string_append = String::from("hello ");
let string_rust = String::from("rust");

let result = string_append + &string_rust;
// &string_rust 自动解引用为 &str，+ 即 add() 方法的第二个参数是引用

let mut result = result + "!";
result += "!!!";
println!("连接字符串 + -> {}", result); // hello rust!!!!
```

### 6.2 所有权转移

```rust
let s1 = String::from("hello,");
let s2 = String::from("world!");
let s3 = s1 + &s2;
assert_eq!(s3, "hello,world!");
// s1 的所有权被转移走了，后面不能再使用 s1
```

### 6.3 使用 format!

```rust
let s1 = "hello";
let s2 = String::from("rust");
let s = format!("{} {}!", s1, s2);
println!("{}", s); // hello rust!
```

## 7. 遍历字符串

```rust
for c in "中国人".chars() {
    println!("{}", c);
}

for b in "中国人".bytes() {
    println!("{}", b);
}
// 输出字节: 228 184 173 229 155 189 228 186 186
```

---

## TypeScript 对比

Rust 的复合类型对应 TS 的 class + object。

**Rust：**

```rust
struct User { name: String, age: u8 }
impl User { fn greet(&self) { ... } }
```

**TypeScript：**

```ts
class User {
  constructor(public name: string, public age: number) {}
  greet() { ... }
}
```

| 维度 | Rust | TypeScript |
|------|------|-----------|
| 结构体 | `struct` 纯数据 | `class` 数据+方法 |
| 方法定义 | 单独 `impl` 块 | class 内部定义 |
| 更新语法 | `User { name: "a", ..old }` | `{ ...old, name: "a" }` |
| 字段可见性 | 默认私有（`pub` 公开） | `public` / `private` |
| new 方法 | 惯例 `fn new() -> Self` | `constructor()` |

详细对照 → [rust_vs_typescript.rs §4](../rust_vs_typescript.rs) "复合类型"
