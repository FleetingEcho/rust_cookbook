// ============================================================
// Singleton Pattern — 全局唯一实例，线程安全延迟初始化
// 对比 TS: 02_singleton.ts
// 运行: cargo run --bin singleton
// ============================================================

use std::collections::HashMap;
use std::sync::OnceLock;

struct Config {
    data: HashMap<String, String>,
}

impl Config {
    fn instance() -> &'static Config {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            println!("[Config] 初始化（只执行一次）");
            let mut data = HashMap::new();
            data.insert("env".into(), "production".into());
            data.insert("db_url".into(), "postgres://localhost/mydb".into());
            data.insert("max_conn".into(), "100".into());
            Config { data }
        })
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

fn main() {
    println!("=== Singleton Pattern ===");

    let c1 = Config::instance();
    println!("env:      {:?}", c1.get("env"));
    println!("db_url:   {:?}", c1.get("db_url"));

    let c2 = Config::instance(); // 不会再次初始化
    println!("max_conn: {:?}", c2.get("max_conn"));

    println!(
        "\n同一实例？ {}",
        std::ptr::eq(c1 as *const Config, c2 as *const Config)
    );
}

// Rust 关键差异：
// - OnceLock 是线程安全的，TS 的静态变量在多线程中不安全
// - Rust 的 &'static 引用保证全局生命周期，编译期即验证
