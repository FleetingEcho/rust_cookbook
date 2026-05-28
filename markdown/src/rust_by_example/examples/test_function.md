# 函数

## 基础函数

```rust
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
```

## 关联函数与方法

```rust
struct Point { x: f64, y: f64 }

impl Point {
    fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
    fn new(x: f64, y: f64) -> Point { Point { x, y } }
}

struct Rectangle { p1: Point, p2: Point }

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("正在销毁 Pair({}, {})", first, second);
    }
}
```

## 高阶函数

```rust
fn is_odd(n: u32) -> bool { n % 2 == 1 }

fn main() {
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper { break; }
        if is_odd(n_squared) { acc += n_squared; }
    }
    println!("命令式风格：{}", acc);

    // 函数式风格
    let sum_of_sqrs = (0..)
        .map(|n| n * n)
        .take_while(|n| *n < upper)
        .filter(|n| is_odd(*n))
        .fold(0, |acc, x| acc + x);
    println!("函数式风格：{}", sum_of_sqrs);
}
```
