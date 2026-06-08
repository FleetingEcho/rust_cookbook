// ============================================================
// Typestate Pattern (TypeScript) — 只能运行时检查，对比 Rust 编译期版本
// 对比 Rust: 13_typestate.rs
// 运行: npx ts-node 13_typestate.ts
// ============================================================

type State = "Disconnected" | "Connected" | "Authenticated";

class Client {
  private state: State = "Disconnected";

  constructor(private host: string, private port: number) {}

  connect(): this {
    if (this.state !== "Disconnected") throw new Error(`无法连接：${this.state}`);
    this.state = "Connected";
    console.log(`[${this.host}:${this.port}] 已连接`);
    return this;
  }

  authenticate(token: string): this {
    if (this.state !== "Connected") throw new Error(`无法认证：${this.state}`);
    this.state = "Authenticated";
    console.log(`[${this.host}:${this.port}] 认证成功 token=${token}`);
    return this;
  }

  send(data: string): void {
    if (this.state !== "Authenticated") throw new Error(`无法发送：${this.state}`);
    console.log(`[${this.host}:${this.port}] 发送: ${data}`);
  }

  receive(): string {
    if (this.state !== "Authenticated") throw new Error(`无法接收：${this.state}`);
    const msg = `来自 ${this.host}:${this.port} 的响应`;
    console.log(`[${this.host}:${this.port}] 接收: ${msg}`);
    return msg;
  }
}

// --- main ---
console.log("=== Typestate Pattern (TypeScript) ===\n");

console.log("--- 正确流程 ---");
const client = new Client("api.example.com", 443);
client.connect().authenticate("Bearer xyz");
client.send('{"action":"getUsers"}');
client.receive();

console.log("\n--- 运行时错误（TS 无法在编译期阻止）---");
const bad = new Client("x.com", 80);
try {
  bad.send("data"); // 运行时才抛出！
} catch (e: any) {
  console.log("错误:", e.message);
}

// 两者对比：
// Rust 版本中，未认证的 Client<Disconnected> 根本没有 .send() 方法
// 调用会在编译时报错，程序根本跑不起来
// TS 版本在编译时看起来没问题，只有运行时才发现
