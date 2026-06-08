// ============================================================
// Singleton Pattern — 全局唯一实例，延迟初始化
// 对比 Rust: 02_singleton.rs
// 运行: npx ts-node 02_singleton.ts
// ============================================================

class Config {
  private static instance: Config;
  private data: Map<string, string>;

  private constructor() {
    console.log("[Config] 初始化（只执行一次）");
    this.data = new Map([
      ["env", "production"],
      ["db_url", "postgres://localhost/mydb"],
      ["max_conn", "100"],
    ]);
  }

  static getInstance(): Config {
    if (!Config.instance) {
      Config.instance = new Config();
    }
    return Config.instance;
  }

  get(key: string): string | undefined {
    return this.data.get(key);
  }
}

// --- main ---
console.log("=== Singleton Pattern ===");

const c1 = Config.getInstance();
console.log("env:     ", c1.get("env"));
console.log("db_url:  ", c1.get("db_url"));

const c2 = Config.getInstance(); // 不会再次初始化
console.log("max_conn:", c2.get("max_conn"));

console.log("\n同一实例？", c1 === c2);

// Rust 关键差异：
// - TS 单例在 Web Worker / 多线程中不安全
// - Rust OnceLock 线程安全，'static 生命周期由编译器保证
