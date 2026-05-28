# Rust vs TypeScript：模式匹配

> **运行命令**：`cargo run -p learning_notes --example rts_pattern_matching`

---

## TypeScript 参考版本

```ts
// TS 的解构（Destructuring）是模式匹配的子集
const [a, b] = [1, 2];
const { name, age } = user;
const { name: alias } = user;   // 重命名
const { x = 0, y = 0 } = point; // 默认值

// switch（无法穷举检查）
switch (shape.kind) {
    case "circle":    ...; break;
    case "rectangle": ...; break;
    // 忘写 default？TS 不报错
}

// 类型收窄（Type Narrowing）
if (typeof x === "string") { x.toUpperCase(); }
if ("radius" in shape) { shape.radius; }

// TS 没有：范围匹配、@ 绑定、守卫组合、ref 模式
```

---

## 一、match 基础

**TS**: `switch`，但 `match` 是表达式，且编译器强制穷举。

```rust
let x = 3_i32;

match x {
    1 => println!("一"),
    2 => println!("二"),
    3 => println!("三"),
    _ => println!("其他"),   // _ 是通配符，必须有（穷举）
}

// match 作为表达式（TS switch 不能直接作为表达式）
let desc = match x {
    1 | 2 => "小",           // 多个值（TS: case 1: case 2:）
    3..=6 => "中",           // 范围（TS: 没有直接对应！）
    7..=9 => "大",
    _     => "其他",
};
```

---

## 二、解构元组

**TS**: `const [a, b] = [1, 2]`

```rust
let pair = (1_i32, true);
match pair {
    (0, _)     => println!("第一个是0"),
    (x, true)  => println!("第一个是{x}，第二个是true"),
    (x, false) => println!("第一个是{x}，第二个是false"),
}
```

---

## 三、解构结构体

**TS**: `const { x, y } = point`

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

let p = Point { x: 3, y: -5 };
match p {
    Point { x: 0, y }  => println!("在Y轴上，y={y}"),
    Point { x, y: 0 }  => println!("在X轴上，x={x}"),
    Point { x, y }     => println!("其他位置: ({x},{y})"),
}

// 直接解构（不用 match）
let Point { x, y } = p;  // TS: const { x, y } = p

// 重命名字段
let Point { x: px, y: py } = p;  // TS: const { x: px, y: py } = p
```

---

## 四、解构枚举（最重要的用法）

**TS**: discriminated union + switch

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

let shapes = vec![
    Shape::Circle { radius: 5.0 },
    Shape::Rectangle { width: 4.0, height: 3.0 },
    Shape::Triangle(3.0, 4.0, 5.0),
];

for shape in &shapes {
    let area = match shape {
        Shape::Circle { radius }                   => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height }         => width * height,
        Shape::Triangle(a, b, c)                   => {
            // 海伦公式
            let s = (a + b + c) / 2.0;
            (s * (s-a) * (s-b) * (s-c)).sqrt()
        }
    };
    println!("{:?} → 面积: {area:.2}", shape);
}
```

---

## 五、守卫（Match Guard）

**TS**: `if` 条件在 `case` 内，Rust 更优雅。

```rust
let num = 7_i32;
match num {
    n if n < 0  => println!("{n} 是负数"),
    n if n == 0 => println!("是零"),
    n if n % 2 == 0 => println!("{n} 是正偶数"),
    n           => println!("{n} 是正奇数"),
}

// 守卫也可以用在解构中
let pair = (2_i32, -2_i32);
match pair {
    (x, y) if x == y     => println!("相等: {x}"),
    (x, y) if x + y == 0 => println!("互为相反数: {x}, {y}"),
    (x, _)               => println!("其他: {x}"),
}
```

---

## 六、@ 绑定（同时匹配和捕获值）

**TS 没有对应语法。**

```rust
let age = 15_u32;
match age {
    // @ 同时测试值是否在范围内，并把值绑定到变量
    n @ 0..=12  => println!("儿童，{n}岁"),
    n @ 13..=17 => println!("青少年，{n}岁"),
    n @ 18..=65 => println!("成年人，{n}岁"),
    n           => println!("老年人，{n}岁"),
}
```

---

## 七、.. 忽略剩余字段

**TS**: `const { name } = user`（直接解构需要的字段）

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u32,
    retries: u32,
}

let cfg = Config {
    host: String::from("localhost"), port: 8080,
    timeout: 30, retries: 3,
};

// 只关心部分字段，.. 忽略其余
let Config { host, port, .. } = cfg;  // TS: const { host, port } = cfg
println!("连接: {host}:{port}");
```

---

## 八、嵌套模式

```rust
#[derive(Debug)]
enum Message {
    Move { point: Point },
    Color(u8, u8, u8),
}

let msg = Message::Move { point: Point { x: 10, y: 20 } };
match msg {
    Message::Move { point: Point { x, y } } => {
        println!("移动到 ({x},{y})");
    }
    Message::Color(r, g, b) => println!("颜色 rgb({r},{g},{b})"),
}
```

---

## 九、if let / while let（简化的单分支 match）

**TS**: `if (x?.value !== undefined) { ... }`

```rust
let config: Option<i32> = Some(42);

// if let：只关心一个模式（TS: if (config !== null)）
if let Some(val) = config {
    println!("配置值: {val}");
}

// if let + else
if let Some(val) = config {
    println!("有值: {val}");
} else {
    println!("没有值");
}
```

---

## 十、let else（Rust 1.65+）

**TS**: 无对应，通常用 `if (!condition) return;`

```rust
fn process(input: Option<i32>) -> i32 {
    let Some(value) = input else {
        // TS: if (input === null) return 0;
        return 0;
    };
    value * 2  // 这里 value 一定有效
}
```

---

## 十一、matches! 宏（快速判断是否匹配某模式）

**TS**: `x.kind === "circle"` 或 `x instanceof Circle`

```rust
let val: Option<i32> = Some(42);
println!("是 Some: {}", matches!(val, Some(_)));
println!("是 Some(x) 且 x>10: {}", matches!(val, Some(x) if x > 10));

let shapes2 = vec![
    Shape::Circle { radius: 1.0 },
    Shape::Rectangle { width: 2.0, height: 3.0 },
    Shape::Circle { radius: 4.0 },
];
let circle_count = shapes2.iter().filter(|s| matches!(s, Shape::Circle { .. })).count();
println!("圆形数量: {circle_count}");
```
