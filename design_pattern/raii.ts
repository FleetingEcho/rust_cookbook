// ============================================================
// RAII in TypeScript — TS 无析构函数，需手动管理或用 using（ES2023）
// 对比 Rust: 12_raii.rs
// 运行: npx ts-node 12_raii.ts
// ============================================================

// 示例 1：连接池（手动归还）
class Connection {
  constructor(public readonly id: number) {}
  query(sql: string) { console.log(`  [Conn#${this.id}] ${sql}`); }
}

class Pool {
  private available: Connection[];

  constructor(size: number) {
    this.available = Array.from({ length: size }, (_, i) => new Connection(i + 1));
  }

  acquire(): Connection | undefined {
    const conn = this.available.pop();
    if (conn) console.log(`[Pool] 取出 Conn#${conn.id}`);
    return conn;
  }

  release(conn: Connection) {
    console.log(`[Pool] 归还 Conn#${conn.id}`);
    this.available.push(conn);
  }
}

// 用高阶函数模拟 RAII（确保归还）
function withConn<T>(pool: Pool, fn: (conn: Connection) => T): T {
  const conn = pool.acquire()!;
  try {
    return fn(conn);
  } finally {
    pool.release(conn); // 必须手动写 finally，Rust 里这行是自动的
  }
}

// 示例 2：计时器（手动 stop）
class Timer {
  private start = Date.now();
  constructor(private name: string) {
    console.log(`[Timer] '${name}' 开始`);
  }
  stop() {
    console.log(`[Timer] '${this.name}' 耗时 ${Date.now() - this.start}ms`);
  }
}

// --- main ---
console.log("=== RAII Pattern (TypeScript) ===\n");

console.log("--- 连接池（手动 finally）---");
const pool = new Pool(3);
withConn(pool, conn => {
  conn.query("SELECT * FROM users");
  conn.query("SELECT * FROM orders");
});
console.log("函数返回后，连接已手动归还\n");

console.log("--- 计时器（必须手动 stop）---");
const timer = new Timer("计算任务");
let sum = 0;
for (let i = 0; i < 1_000_000; i++) sum += i;
timer.stop(); // ← 忘写这行，就没有输出！Rust 的 Drop 不会忘

console.log(`
对比总结：
  Rust: 离开作用域 → Drop 自动执行 → 资源释放，不可绕过
  TS:   必须手动 finally / stop() / close() → 容易忘记 → 资源泄漏
  ES2023 "using" 关键字可以部分模拟，但需要宿主环境支持
`);
