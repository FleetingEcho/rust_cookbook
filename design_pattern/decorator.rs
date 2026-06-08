// ============================================================
// Decorator Pattern — 动态给对象添加职责，比继承更灵活
// 对比 TS: 05_decorator.ts
// 运行: cargo run --bin decorator
// ============================================================

trait DataSource {
    fn write(&mut self, data: &str);
    fn read(&self) -> String;
}

struct MemoryStore {
    data: String,
}

impl MemoryStore {
    fn new() -> Self { Self { data: String::new() } }
}

impl DataSource for MemoryStore {
    fn write(&mut self, data: &str) { self.data = data.into(); }
    fn read(&self) -> String        { self.data.clone() }
}

// 加密装饰器（Caesar +1）
struct Encrypted<T: DataSource> {
    inner: T,
}

impl<T: DataSource> Encrypted<T> {
    fn new(inner: T) -> Self { Self { inner } }
}

impl<T: DataSource> DataSource for Encrypted<T> {
    fn write(&mut self, data: &str) {
        let enc: String = data.chars().map(|c| (c as u8 + 1) as char).collect();
        println!("  [Encrypt] '{}' -> '{}'", data, enc);
        self.inner.write(&enc);
    }
    fn read(&self) -> String {
        let raw = self.inner.read();
        let dec: String = raw.chars().map(|c| (c as u8 - 1) as char).collect();
        println!("  [Decrypt] '{}' -> '{}'", raw, dec);
        dec
    }
}

// 日志装饰器
struct Logged<T: DataSource> {
    inner: T,
    label: String,
}

impl<T: DataSource> Logged<T> {
    fn new(inner: T, label: &str) -> Self { Self { inner, label: label.into() } }
}

impl<T: DataSource> DataSource for Logged<T> {
    fn write(&mut self, data: &str) {
        println!("  [Log:{}] write {} bytes", self.label, data.len());
        self.inner.write(data);
    }
    fn read(&self) -> String {
        let result = self.inner.read();
        println!("  [Log:{}] read  {} bytes", self.label, result.len());
        result
    }
}

fn main() {
    println!("=== Decorator Pattern ===\n");

    println!("--- 裸存储 ---");
    let mut s = MemoryStore::new();
    s.write("hello");
    println!("read: {}\n", s.read());

    println!("--- 加密 ---");
    let mut s = Encrypted::new(MemoryStore::new());
    s.write("hello");
    println!("read: {}\n", s.read());

    println!("--- 日志 + 加密（嵌套）---");
    let mut s = Logged::new(Encrypted::new(MemoryStore::new()), "app");
    s.write("hello");
    println!("read: {}", s.read());
}

// Rust 关键差异：
// - 泛型装饰器 Logged<Encrypted<MemoryStore>> 是编译期确定的，零虚函数开销
// - TS 用接口 + class 继承，每次调用都是动态派发
