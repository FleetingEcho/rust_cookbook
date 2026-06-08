// ============================================================
// RAII Pattern — 资源生命周期绑定作用域，Drop 自动释放
// 对比 TS: 12_raii.ts
// 运行: cargo run --bin raii
// ============================================================

use std::sync::Mutex;

// 示例 1：连接池 Guard
struct Connection { id: u32 }

impl Connection {
    fn query(&self, sql: &str) { println!("  [Conn#{}] {}", self.id, sql); }
}

struct Pool { available: Vec<Connection> }

impl Pool {
    fn new(size: u32) -> Self {
        Self { available: (1..=size).map(|id| Connection { id }).collect() }
    }

    fn acquire(&mut self) -> Option<PoolGuard<'_>> {
        let conn = self.available.pop()?;
        println!("[Pool] 取出 Conn#{}", conn.id);
        Some(PoolGuard { conn: Some(conn), pool: self })
    }

    fn release(&mut self, conn: Connection) {
        println!("[Pool] 归还 Conn#{}", conn.id);
        self.available.push(conn);
    }
}

struct PoolGuard<'a> {
    conn: Option<Connection>,
    pool: &'a mut Pool,
}

impl<'a> std::ops::Deref for PoolGuard<'a> {
    type Target = Connection;
    fn deref(&self) -> &Connection { self.conn.as_ref().unwrap() }
}

impl<'a> Drop for PoolGuard<'a> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            self.pool.release(conn); // 自动归还，无法绕过
        }
    }
}

// 示例 2：计时器 Guard
struct Timer { name: String, start: std::time::Instant }

impl Timer {
    fn new(name: &str) -> Self {
        println!("[Timer] '{}' 开始", name);
        Self { name: name.into(), start: std::time::Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("[Timer] '{}' 耗时 {:?}", self.name, self.start.elapsed());
    }
}

fn main() {
    println!("=== RAII Pattern ===\n");

    println!("--- 连接池 Guard ---");
    let mut pool = Pool::new(3);
    {
        let guard = pool.acquire().unwrap();
        guard.query("SELECT * FROM users");
        guard.query("SELECT * FROM orders");
        // guard 在这里离开作用域 → Drop → 自动归还连接
    }
    println!("作用域结束，连接已自动归还\n");

    println!("--- 计时器 Guard ---");
    {
        let _t = Timer::new("计算任务");
        let _sum: u64 = (0..1_000_000u64).sum();
        // _t 在这里 drop，自动打印耗时
    }

    println!("\n--- Mutex Guard（标准库 RAII）---");
    let counter = Mutex::new(0i32);
    {
        let mut g = counter.lock().unwrap();
        *g += 100;
        println!("counter = {}", *g);
        // MutexGuard drop → 自动释放锁
    }
    println!("锁已释放，再次读取: {}", *counter.lock().unwrap());
}

// Rust 关键差异：
// - Drop 是语言级保证，不可绕过，不需要 try/finally
// - 所有权系统保证 guard 不会被复制，资源只释放一次
