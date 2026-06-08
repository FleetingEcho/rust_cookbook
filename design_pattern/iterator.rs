// ============================================================
// Iterator Pattern — 统一遍历接口，Rust Iterator trait 极其强大
// 对比 TS: 10_iterator.ts
// 运行: cargo run --bin iterator
// ============================================================

// 自定义迭代器：步进范围
struct StepRange { current: i32, end: i32, step: i32 }

impl StepRange {
    fn new(start: i32, end: i32, step: i32) -> Self {
        Self { current: start, end, step }
    }
}

impl Iterator for StepRange {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.current >= self.end { return None; }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}

// 自定义迭代器：斐波那契（无限）
struct Fibonacci { a: u64, b: u64 }

impl Fibonacci {
    fn new() -> Self { Self { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

fn main() {
    println!("=== Iterator Pattern ===\n");

    println!("--- 自定义步进范围 ---");
    let v: Vec<i32> = StepRange::new(0, 20, 3).collect();
    println!("0..20 step 3: {:?}", v);

    println!("\n--- 链式操作（惰性求值）---");
    let result: Vec<i32> = StepRange::new(0, 20, 1)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .take(5)
        .collect();
    println!("前5个偶数的平方: {:?}", result);

    println!("\n--- 斐波那契（无限 + take 截断）---");
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("前10项: {:?}", fibs);

    println!("\n--- 标准库 Iterator 方法 ---");
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("sum:        {}", nums.iter().sum::<i32>());
    println!("product:    {}", nums.iter().product::<i32>());
    println!("evens:      {:?}", nums.iter().filter(|&&x| x % 2 == 0).collect::<Vec<_>>());
    println!("any > 5:    {}", nums.iter().any(|&x| x > 5));
    println!("all > 0:    {}", nums.iter().all(|&x| x > 0));
    println!("max:        {:?}", nums.iter().max());

    println!("\n--- zip ---");
    let names  = vec!["Alice", "Bob", "Carol"];
    let scores = vec![95, 87, 92];
    let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("{:?}", paired);

    println!("\n--- flat_map ---");
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("{:?}", words);
}

// Rust 关键差异：
// - Iterator 是惰性的，链式操作不产生中间集合
// - collect() 是"消费"触发点，只在这一步执行整个链
// - TS 的 .filter().map() 每步都产生新数组
