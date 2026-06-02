# Rust vs TypeScript: 闭包与迭代器

**运行命令：** `cargo run -p learning_notes --example rts_closures_iter`

## TypeScript 版本

```ts
const double = (x: number) => x * 2;
const add = (a: number, b: number) => a + b;
const greet = (name: string) => `Hello, ${name}!`;

const multiplier = 3;
const multiply = (x: number) => x * multiplier;

function applyTwice(f: (x: number) => number, x: number): number {
    return f(f(x));
}

function makeAdder(n: number) {
    return (x: number) => x + n;
}
const add5 = makeAdder(5);

const nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
nums.filter(x => x % 2 === 0).map(x => x * x).reduce((a, b) => a + b, 0);
nums.find(x => x > 5);
nums.findIndex(x => x > 5);
nums.some(x => x > 8);
nums.every(x => x > 0);
nums.flatMap(x => [x, x * 2]);
nums.slice(0, 3);
nums.slice(3);
nums.entries();
[...a, ...b];
a.map((v, i) => [v, b[i]]);
```

## 一、闭包基础

```rust
let double  = |x: i32| x * 2;
let add     = |a: i32, b: i32| a + b;
let is_even = |x: i32| x % 2 == 0;
println!("double(5): {}", double(5));
println!("add(3,4): {}", add(3, 4));
println!("is_even(4): {}", is_even(4));

let complex = |x: i32| {
    let doubled = x * 2;
    doubled + 1
};
println!("complex(5): {}", complex(5));
```

## 二、捕获外部变量

```rust
let multiplier = 3_i32;
let multiply = |x| x * multiplier;
println!("multiply(5): {}", multiply(5));
println!("multiplier 还能用: {multiplier}");

let greeting = String::from("你好");
let greet = move |name: &str| format!("{}, {}！", greeting, name);
println!("{}", greet("Alice"));
// println!("{}", greeting); // ❌ greeting 所有权已移入闭包
```

## 三、Fn / FnMut / FnOnce

```rust
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}
println!("Fn apply_twice: {}", apply_twice(|x| x + 3, 7));

let mut count = 0;
let mut increment = || { count += 1; count };
println!("FnMut: {}", increment()); // 1, 2, 3

let name = String::from("Rust");
let consume = move || {
    println!("FnOnce 消耗: {name}");
};
consume();
// consume(); // ❌ 不能再调用
```

## 四、工厂函数

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

let add5   = make_adder(5);
let triple = make_multiplier(3);
println!("add5(10): {}", add5(10));
println!("triple(7): {}", triple(7));
```

## 五、迭代器链

```rust
let numbers = vec![1_i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: i32 = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();
println!("偶数平方和: {result}");

let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
let sum = numbers.iter().fold(0_i32, |acc, &x| acc + x);

println!("sum: {}", numbers.iter().sum::<i32>());
println!("product: {}", numbers.iter().product::<i32>());
println!("min: {:?}", numbers.iter().min());
println!("max: {:?}", numbers.iter().max());
println!("count: {}", numbers.iter().count());

let first_even = numbers.iter().find(|&&x| x % 2 == 0);
let pos = numbers.iter().position(|&x| x > 5);
println!("any >8: {}", numbers.iter().any(|&x| x > 8));
println!("all >0: {}", numbers.iter().all(|&x| x > 0));

let first3: Vec<_> = numbers.iter().take(3).collect();
let rest: Vec<_>   = numbers.iter().skip(3).collect();

for (i, val) in numbers.iter().enumerate().take(3) {
    println!("  [{i}] = {val}");
}

let sentences = vec!["hello world", "rust is great"];
let words: Vec<&str> = sentences.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
println!("flat_map: {:?}", words);

let letters = vec!['a', 'b', 'c'];
let digits  = vec![1_i32, 2, 3];
let zipped: Vec<_> = letters.iter().zip(digits.iter()).collect();
println!("zip: {:?}", zipped);
```

## 总结对照表

| TypeScript | Rust |
|------------|------|
| `(x) => x * 2` | `\|x\| x * 2` |
| 自动捕获 | `move` 关键字 |
| 可重复调用 | `Fn` / `FnMut` / `FnOnce` |
| `.filter().map().reduce()` | `.filter().map().sum()` |
| `.find()` | `.find()` |
| `.findIndex()` | `.position()` |
| `.some()` | `.any()` |
| `.every()` | `.all()` |
| `.slice(0, 3)` | `.take(3)` |
| `.slice(3)` | `.skip(3)` |
| `.flatMap()` | `.flat_map()` |
| 手动 zip | `.zip()` |
