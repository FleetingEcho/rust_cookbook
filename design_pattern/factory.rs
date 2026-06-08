// ============================================================
// Factory Pattern — 工厂函数创建对象，调用方不感知具体类型
// 对比 TS: 03_factory.ts
// 运行: cargo run --bin factory
// ============================================================

trait Logger {
    fn log(&self, level: &str, msg: &str);
}

struct ConsoleLogger;

struct FileLogger {
    path: String,
}

struct JsonLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: &str, msg: &str) {
        println!("[Console][{}] {}", level, msg);
    }
}

impl Logger for FileLogger {
    fn log(&self, level: &str, msg: &str) {
        println!("[File:{}][{}] {}", self.path, level, msg);
    }
}

impl Logger for JsonLogger {
    fn log(&self, level: &str, msg: &str) {
        println!(r#"{{"level":"{}","msg":"{}"}}"#, level, msg);
    }
}

fn create_logger(kind: &str) -> Box<dyn Logger> {
    match kind {
        "file" => Box::new(FileLogger { path: "app.log".into() }),
        "json" => Box::new(JsonLogger),
        _      => Box::new(ConsoleLogger),
    }
}

fn main() {
    println!("=== Factory Pattern ===");

    for kind in ["console", "file", "json"] {
        println!("\n[{}]", kind);
        let logger = create_logger(kind);
        logger.log("INFO", "server started");
        logger.log("ERROR", "connection refused");
    }
}

// Rust 关键差异：
// - Box<dyn Logger> 是运行时多态（堆分配），等价于 TS 接口
// - 若编译期类型已知，可用泛型 impl Logger，零堆分配
