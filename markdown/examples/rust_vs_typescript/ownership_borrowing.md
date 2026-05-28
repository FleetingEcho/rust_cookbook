# Rust vs TypeScript：所有权与借用

> **运行命令**：`cargo run -p learning_notes --example rts_ownership_borrowing`

---

## TypeScript 参考版本

```ts
// TS/JS 没有所有权概念，GC 自动管理内存
// 赋值基本类型是值拷贝，对象是引用拷贝

let a = 5;
let b = a;   // 值拷贝，a 和 b 各自独立
a = 10;      // 不影响 b

const obj1 = { name: "Alice" };
const obj2 = obj1;    // 引用拷贝，两者指向同一对象
obj2.name = "Bob";
console.log(obj1.name);  // "Bob"！共享引用

// TS 没有借用的概念，函数参数都是引用传递（对象）
function greet(user: User): string { return user.name; }
// user 没有被"消耗"，调用后还能继续使用

// 深拷贝需要手动处理
const obj3 = { ...obj1 };  // 浅拷贝
const obj4 = JSON.parse(JSON.stringify(obj1)); // 深拷贝
```

---

## 一、所有权基础（Rust 独有，TS 无对应）

**核心规则**：每个值有且只有一个所有者，所有者离开作用域时值被丢弃。

```rust
// 基本类型（Copy 类型）：赋值是值拷贝，与 TS 相同
let a = 5_i32;
let b = a;      // a 的值被复制到 b
println!("a={a}, b={b}");  // a 仍然可用

// String（堆数据）：赋值是移动（Move），不是拷贝
let s1 = String::from("hello");
let s2 = s1;    // s1 的所有权移动到 s2，s1 不再有效
// println!("{s1}"); // ❌ 编译错误：s1 已被移动
println!("s2 = {s2}");  // ✅ 只能用 s2

// TS 对比：const s2 = s1 后，s1 和 s2 都指向同一对象，都可用
// Rust：移动后原变量失效，避免"双重释放"内存问题
```

---

## 二、Clone（显式深拷贝）

**TS**: `{...obj}`（浅拷贝）或 `JSON.parse(JSON.stringify(obj))`（深拷贝）

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 显式深拷贝，s1 和 s2 各自独立
println!("s1={s1}, s2={s2}");  // 两者都可用

// Clone 会复制堆上的数据，有性能开销
// 只在真正需要独立副本时才用 clone()
```

---

## 三、Copy trait（栈上的简单类型）

**TS** 的基本类型（`number`, `boolean`）都是值拷贝。

**Rust** 实现了 `Copy` 的类型赋值时自动复制，不移动。

```rust
// 实现了 Copy 的类型：i32, u32, f64, bool, char, &T, 元组（如果元素都 Copy）
let x: i32  = 5;
let y: i32  = x;   // 自动复制，x 仍可用
let flag    = true;
let flag2   = flag; // 自动复制
println!("x={x}, y={y}, flag={flag}, flag2={flag2}");

// 没有实现 Copy 的类型：String, Vec, HashMap, 自定义结构体（默认）
// 这些类型赋值时是移动，不是复制
```

---

## 四、函数中的所有权转移

**TS**：函数参数传递对象是引用传递，原对象不受影响。

**Rust**：传入非 Copy 类型会转移所有权，调用后原变量失效。

```rust
fn takes_ownership(s: String) {   // s 进来，获得所有权
    println!("获得所有权: {s}");
}   // s 在这里被丢弃

fn makes_copy(n: i32) {           // i32 是 Copy，复制进来
    println!("Copy 进来: {n}");
}

fn gives_ownership() -> String {  // 返回值转移所有权给调用者
    String::from("新字符串")
}

let s = String::from("hello");
takes_ownership(s);    // s 的所有权移入函数
// println!("{s}");    // ❌ s 已被移动

let n = 5;
makes_copy(n);         // i32 是 Copy，n 仍可用
println!("n 仍然可用: {n}");

let returned = gives_ownership();
println!("返回的所有权: {returned}");
```

---

## 五、借用（Borrowing）—— 不转移所有权地使用值

**TS** 没有此概念，函数参数本来就是"借用"。

**Rust** 需要显式用 `&` 表示借用。

```rust
fn calculate_length(s: &String) -> usize {  // &String 是不可变借用
    s.len()   // 只是借用，不获取所有权
}   // s 的借用结束，但原数据不被丢弃

let s1 = String::from("hello");
let len = calculate_length(&s1);  // 传入引用（借用），不转移所有权
println!("'{s1}' 的长度: {len}");  // s1 仍然有效！
```

---

## 六、可变借用（&mut）

**TS** 的对象方法通过 `this` 修改属性，不需要额外标记。

```rust
fn change(s: &mut String) {  // &mut 可变借用
    s.push_str(", world");   // TS: s += ", world"
}

let mut s = String::from("hello");
change(&mut s);   // 传入可变借用
println!("修改后: {s}");

// 可变借用规则：同一时间只能有一个 &mut
let mut data = String::from("hello");
let r1 = &mut data;
// let r2 = &mut data; // ❌ 不能同时有两个可变借用
r1.push_str(" world");
println!("{r1}");

// 不可变借用和可变借用不能同时存在
let mut s = String::from("hello");
let r1 = &s;      // 不可变借用
// let r2 = &mut s; // ❌ 已有不可变借用，不能再可变借用
println!("{r1}"); // r1 最后一次使用
// 此后 r1 的借用结束，可以再借用
let r2 = &mut s;  // ✅ r1 已不再使用
r2.push_str("!");
println!("{r2}");
```

---

## 七、借用规则总结（核心记忆点）

**TS 程序员需要牢记这个规则。**

```
规则 1：同一时间，可以有多个不可变借用（&T）
规则 2：同一时间，只能有一个可变借用（&mut T）
规则 3：不可变借用和可变借用不能同时存在
类比：读写锁（RwLock）—— 多读者 OR 一个写者
```

---

## 八、切片引用（&str 是 String 的借用切片）

```rust
let s = String::from("hello world");
let hello = &s[0..5];   // &str：对 s 的部分借用
let world = &s[6..11];
println!("{hello} {world}");
// s 仍然有效，hello 和 world 只是借用

// 函数参数使用 &str 比 &String 更灵活
fn first_word(s: &str) -> &str {   // 接受 &str 或 &String（自动解引用）
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

let sentence = String::from("hello rust");
let word = first_word(&sentence);
println!("第一个词: {word}");
```

---

## 九、所有权与 Vec

```rust
let v = vec![1, 2, 3];
// for x in v { ... }  // 这会消耗 v
// println!("{:?}", v); // ❌ v 已被移动

// 使用借用遍历不消耗所有权
let v = vec![1, 2, 3];
for x in &v {           // &v 借用
    print!("{x} ");
}
println!();
println!("v 仍然可用: {:?}", v); // ✅
```

---

## 十、#[derive(Clone)] 让自定义类型支持 clone()

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
    age: u32,
}

let u1 = User { name: String::from("Alice"), age: 30 };
let u2 = u1.clone();   // 深拷贝，TS: { ...u1 }（浅拷贝）
println!("u1={:?}, u2={:?}", u1, u2);
```

---

## 总结对照表

| TypeScript | Rust |
|---|---|
| GC 自动管理内存 | 所有权系统编译期验证 |
| 赋值对象=引用拷贝 | 赋值=移动（Move）或显式 `clone()` |
| 函数参数始终是引用 | 要么转移所有权，要么显式 `&` 借用 |
| 无可变/不可变区分 | `&`（多读）vs `&mut`（一写）严格区分 |
| 可以同时有多个引用 | 多 `&` 或一 `&mut`，不能共存 |
| 浅拷贝 `{...obj}` | 需 `clone()`（深拷贝，性能可预期） |
| 悬垂引用不可能（GC） | 编译器防止悬垂引用 |
| 无借用检查 | 借用检查器在编译期验证引用安全 |
