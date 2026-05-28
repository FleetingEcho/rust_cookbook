# Rust vs TypeScript: 数组

**运行命令：** `cargo run -p learning_notes --example rts_arrays`

## TypeScript 版本

```ts
const fixed: readonly number[] = [1, 2, 3, 4, 5];

const arr: number[] = [1, 2, 3, 4, 5];
arr.push(6);
arr.pop();
arr.unshift(0);
arr.shift();
arr.splice(2, 0, 99);
arr.splice(2, 1);
arr.includes(3);
arr.indexOf(3);
arr.findIndex(x => x > 3);
arr.slice(1, 3);
arr.reverse();
arr.sort((a, b) => a - b);
arr.map(x => x * 2);
arr.filter(x => x % 2 === 0);
arr.reduce((acc, x) => acc + x, 0);
arr.find(x => x > 3);
arr.some(x => x > 3);
arr.every(x => x > 0);
arr.flat();
arr.concat([6, 7]);
arr.join(", ");
arr.length = 0;
```

## 一、固定数组 [T; N]

TS 对应：`readonly T[]`（长度固定，类型固定）。存在栈上，大小编译期确定。

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
println!("固定数组: {:?}", arr);
println!("长度: {}", arr.len());
println!("第一个: {}", arr[0]);
println!("最后一个: {}", arr[arr.len() - 1]);
println!("切片: {:?}", &arr[1..4]);

let zeros = [0_i32; 5];
let ones  = [1_u8; 3];
println!("零数组: {:?}", zeros);
println!("一数组: {:?}", ones);

for x in &arr {
    print!("{x} ");
}
println!();

for (i, x) in arr.iter().enumerate() {
    print!("[{i}]={x} ");
}
println!();
```

## 二、动态数组 Vec<T>

TS 对应：`number[]` 或 `Array<number>`。存在堆上，长度可变。

```rust
let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
println!("Vec 初始: {:?}", v);

v.push(6);
println!("push(6): {:?}", v);

v.insert(0, 0);
println!("insert(0,0): {:?}", v);

v.insert(3, 99);
println!("insert(3,99): {:?}", v);

v.pop();
println!("pop(): {:?}", v);

v.remove(0);
println!("remove(0): {:?}", v);

let mut sv = vec![10, 20, 30, 40, 50];
println!("swap_remove 前: {:?}", sv);
sv.swap_remove(1);
println!("swap_remove(1): {:?}", sv);

v.retain(|&x| x != 99);
println!("retain(!= 99): {:?}", v);

v.clear();
println!("clear(): {:?}", v);

let mut dv = vec![1, 2, 3, 4, 5, 6];
let drained: Vec<_> = dv.drain(1..4).collect();
println!("drain(1..4): {:?}, 剩余: {:?}", drained, dv);
```

## 三、切片操作

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4];
println!("切片: {:?}", slice);

let mut v = vec![1, 2, 3, 4, 5];
let slice = &v[1..4];
println!("Vec 切片: {:?}", slice);
```

## 四、排序

```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
v.sort();
println!("升序: {:?}", v);
v.sort_unstable();
println!("不稳定升序: {:?}", v);
v.reverse();
println!("降序: {:?}", v);
v.sort_by(|a, b| b.cmp(a));
println!("手动降序: {:?}", v);
```

## 五、查找

```rust
let v = vec![1, 2, 3, 4, 5];
println!("contains(3): {}", v.contains(&3));
println!("contains(99): {}", v.contains(&99));
println!("binary_search(3): {:?}", v.binary_search(&3));
println!("binary_search(99): {:?}", v.binary_search(&99));
```

## 六、映射、过滤、归约

```rust
let v = vec![1, 2, 3, 4, 5, 6];
let doubled: Vec<_> = v.iter().map(|&x| x * 2).collect();
println!("map(x*2): {:?}", doubled);

let evens: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
println!("filter(even): {:?}", evens);

let sum: i32 = v.iter().sum();
println!("sum: {}", sum);

let product: i32 = v.iter().product();
println!("product: {}", product);

let max = v.iter().max().unwrap();
let min = v.iter().min().unwrap();
println!("max: {max}, min: {min}");

let found = v.iter().find(|&&x| x > 3);
println!("find(>3): {:?}", found);

println!("any(>3): {}", v.iter().any(|&x| x > 3));
println!("all(>0): {}", v.iter().all(|&x| x > 0));

let first_over = v.iter().position(|&x| x > 3);
println!("find_index(>3): {:?}", first_over);
```

## 七、flat 和 join

```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
let flat: Vec<_> = nested.into_iter().flatten().collect();
println!("flatten: {:?}", flat);

let words = vec!["hello", "world", "rust"];
let joined = words.join(", ");
println!("join: {joined}");
```

## 八、concat 和迭代

```rust
let v1 = vec![1, 2, 3];
let v2 = vec![4, 5, 6];
let concatenated: Vec<_> = v1.iter().chain(v2.iter()).cloned().collect();
println!("concat: {:?}", concatenated);

let mut v = vec![1, 2, 3];
for x in &mut v {
    *x *= 2;
}
println!("iter_mut: {:?}", v);
```

## 九、Vec 与 &Vec

```rust
fn print_vec(v: &Vec<i32>) {
    println!("{:?}", v);
}

fn print_slice(v: &[i32]) {
    println!("{:?}", v);
}

let v = vec![1, 2, 3];
print_vec(&v);
print_slice(&v);
```

## 十、切片操作 vs Vec

```rust
let mut v = vec![1, 2, 3, 4, 5];
let slice = &mut v[1..4];
slice.sort();
println!("切片排序后: {:?}", v);

let v = vec![1, 2, 3, 4, 5];
let first_half = &v[..2];
let second_half = &v[2..];
println!("前半: {:?}, 后半: {:?}", first_half, second_half);
```

## 总结对照表

| TypeScript | Rust |
|------------|------|
| `function add(a,b) {...}` | `fn add(a: i32, b: i32) -> i32` |
| `return x + y` | `x + y` (无分号，表达式返回) |
| (参数不强制写类型) | 每个参数必须有类型注解 |
| `void` | `()` 单元类型/省略返回 |
| `never (throw)` | `-> !` (发散函数) |
| `type Fn = (i32)=>i32` | `fn(i32) -> i32` (函数指针) |
| 默认参数 / 可选参数 | `Option<T>` / Builder 模式 |
| `...rest` 参数 | `&[T]` 切片参数 |
| 函数重载 | trait / 枚举替代 |
| `static method` | 关联函数（impl 内的 fn） |
| `this` (默认可变) | `&self` / `&mut self` / `self` |
| 嵌套函数 | 支持内嵌 `fn`，但不能捕获变量 |

详细对照 → `rust_vs_typescript.rs §1 "数组"`
