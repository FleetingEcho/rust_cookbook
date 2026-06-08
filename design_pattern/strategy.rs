// ============================================================
// Strategy Pattern — 将算法封装起来，使其可以互换
// 对比 TS: 07_strategy.ts
// 运行: cargo run --bin strategy
// ============================================================

// === 方式 1: trait object（运行时可换策略）===

trait Sorter {
    fn sort(&self, data: &mut Vec<i32>);
    fn name(&self) -> &str;
}

struct Bubble;
struct Insertion;

impl Sorter for Bubble {
    fn name(&self) -> &str { "BubbleSort" }
    fn sort(&self, data: &mut Vec<i32>) {
        let n = data.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if data[j] > data[j + 1] { data.swap(j, j + 1); }
            }
        }
    }
}

impl Sorter for Insertion {
    fn name(&self) -> &str { "InsertionSort" }
    fn sort(&self, data: &mut Vec<i32>) {
        for i in 1..data.len() {
            let key = data[i];
            let mut j = i;
            while j > 0 && data[j - 1] > key {
                data[j] = data[j - 1];
                j -= 1;
            }
            data[j] = key;
        }
    }
}

struct SortContext {
    strategy: Box<dyn Sorter>,
}

impl SortContext {
    fn new(strategy: Box<dyn Sorter>) -> Self { Self { strategy } }
    fn set(&mut self, strategy: Box<dyn Sorter>) { self.strategy = strategy; }
    fn run(&self, data: &mut Vec<i32>) {
        println!("策略: {}", self.strategy.name());
        self.strategy.sort(data);
    }
}

fn main() {
    println!("=== Strategy Pattern ===\n");

    let raw = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];

    // Trait object 方式
    println!("--- Trait Object ---");
    let mut ctx = SortContext::new(Box::new(Bubble));
    let mut data = raw.clone();
    ctx.run(&mut data);
    println!("结果: {:?}\n", data);

    ctx.set(Box::new(Insertion));
    let mut data = raw.clone();
    ctx.run(&mut data);
    println!("结果: {:?}\n", data);

    // 函数式方式（工作中最常用）
    println!("--- 函数式（闭包作策略）---");
    let sort_with = |data: &mut Vec<i32>, f: &dyn Fn(&mut Vec<i32>)| f(data);

    let mut data = raw.clone();
    sort_with(&mut data, &|v| v.sort());
    println!("升序: {:?}", data);

    let mut data = raw.clone();
    sort_with(&mut data, &|v| v.sort_by(|a, b| b.cmp(a)));
    println!("降序: {:?}", data);
}

// Rust 关键差异：
// - 泛型版 Sorter<S: Sorter> 是编译期单态化，零虚函数开销
// - Box<dyn Sorter> 是运行时动态派发，等价于 TS 接口
// - 闭包策略最简洁，也最惯用
