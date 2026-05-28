# Rust 数组与 Vector

## 1. 数组基础

我们称 `array` 为数组（固定大小），`Vector` 为动态数组。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // 类似 length 5
```

## 2. 数组初始化与 Copy

```rust
let array = [String::from("rust is good!"); 8]; // 报错！
```

因为 `String` 不是 `Copy` 类型。数组初始化 `[value; n]` 需要 `value` 实现 `Copy`，但 `String` 不是 `Copy` 类型，所以不能这样使用。`[value; n]` 表示创建 `n` 个 `value` 的副本，但这个 `value` 必须实现 `Copy` trait，否则 Rust 不知道如何复制它。`String` 存储在堆上，不能直接复制。

### 2.1 解决方案

```rust
fn main() {
    // 方案 1：用 vec![]
    let array = vec![String::from("rust is good!"); 8];
    println!("{:#?}", array);

    // 方案 2：用 map + clone
    let array2 = [String::from("rust is good!"); 8].map(|s| s.clone());
    println!("{:#?}", array2);
}
```

## 3. 数组类型推导与二维数组

```rust
let one             = [1, 2, 3];            // 编译器自动推导出类型
let two: [u8; 3]    = [1, 2, 3];            // 显式类型标注
let blank1          = [0; 3];
let blank2: [u8; 3] = [0; 3];

// arrays 是一个二维数组，其中每一个元素都是 [u8; 3] 类型的数组
let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
```

### 3.1 遍历数组（必须借用）

```rust
for a in &arrays {
    // 如果写 `arrays` 就变成了 a 试图接管 arrays 内元素的所有权
    // Rust 会尝试移动 arrays 的每个元素，但数组是栈上固定大小，不会自动克隆
    // & 表示借用 arrays，不会移动它的元素
    print!("{:?}: ", a);

    for n in a.iter() {
        print!("\t{} + 10 = {}", n, n + 10);
    }

    let mut sum = 0;
    for i in 0..a.len() {
        sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
}
```

牢记：在固定大小数组 `[T; N]` 上使用 `for`，必须 `&` 借用。

### 3.2 Vec 不需要借用

```rust
let arrays = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
];

for a in arrays {
    // Vec<T> 默认会借用，除非使用 into_iter()
    print!("{:?}: ", a);
}
```

## 4. 数组借用 vs 移动

```rust
let a = [1, 2, 3];

for n in &a { // &a 让 n 变成 &i32
    println!("{}", n);
}

println!("{:?}", a); // a 仍然可用
```

## 5. &a 的含义

```rust
let a = [1, 2, 3];
let ref_a = &a; // &a 获取 a 的地址
println!("{:p}", ref_a); // 打印 a 在内存中的地址
```

虽然 `&a` 表面上是获取 `a` 的地址，但 Rust 的借用机制比 C 语言的指针更安全：

- 防止悬垂指针（借用不能超过 `a` 的生命周期）
- 防止数据竞争（不可变借用和可变借用不能同时存在）
- 不会导致 `a` 失效（不像 `for n in a` 那样移动所有权）

可以把 `&a` 理解为一个"安全指针"，不仅仅是地址，还保证了数据安全。

## 6. 数组操作

```rust
let mut arr = [1, 2, 3, 4, 5];

println!("数组: {:?}", arr);
arr[0] = 10;
println!("修改后: {:?}", arr);
println!("第一个元素: {}", arr[0]);

// 遍历数组
for num in &arr {
    print!("{} ", num);
}
println!();

let filtered: Vec<_> = arr.iter().filter(|&x| x % 2 == 0).collect();
println!("偶数元素: {:?}", filtered);

let mapped: Vec<_> = arr.iter().map(|x| x * 10).collect();
println!("乘以 10: {:?}", mapped);

// 切片 & 长度
println!("数组切片: {:?}", &arr[1..4]);
println!("数组长度: {}", arr.len());
```

## 7. Vector 操作

```rust
let mut vec = vec![1, 2, 3, 4, 5];

println!("Vector: {:?}", vec);
vec.push(6);
println!("push(6): {:?}", vec);
vec.pop();
println!("pop(): {:?}", vec);
println!("第 2 个元素: {}", vec[1]);
println!("安全获取 get(10): {:?}", vec.get(10));

for num in &vec {
    print!("{} ", num);
}
println!();

let filtered: Vec<_> = vec.iter().filter(|&&x| x % 2 == 0).collect();
println!("偶数元素: {:?}", filtered);

let mapped: Vec<_> = vec.iter().map(|x| x * 10).collect();
println!("乘以 10: {:?}", mapped);

vec.insert(2, 99);
println!("insert(2, 99): {:?}", vec);
vec.remove(2);
println!("remove(2): {:?}", vec);

println!("Vector 切片: {:?}", &vec[1..4]);
println!("Vector 长度: {}", vec.len());
println!("Vector 容量: {}", vec.capacity());
```

---

## TypeScript 对比

| 特性 | Rust `[T; N]` | Rust `Vec<T>` | TypeScript |
|------|------|------|-----------|
| 大小 | 编译期固定 | 运行时动态 | 运行时动态 |
| 存储 | 栈上 | 堆上 | 堆上 |
| 遍历 | 必须 `&` 借用 | 可直接迭代 | `for...of` |
| 泛型 | `[T; N]` | `Vec<T>` | `Array<T>` |
| 切片 | `&arr[1..4]` | `&vec[1..4]` | `arr.slice(1, 4)` |

详细对照 → [rust_vs_typescript.rs](../rust_vs_typescript.rs)
