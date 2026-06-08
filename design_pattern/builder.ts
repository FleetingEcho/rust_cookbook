// ============================================================
// Builder Pattern — 分步构造复杂对象，避免构造函数参数爆炸
// 对比 Rust: 01_builder.rs
// 运行: npx ts-node 01_builder.ts
// ============================================================

interface HttpRequest {
  url: string;
  method: string;
  headers: [string, string][];
  body?: string;
}

class HttpRequestBuilder {
  private url = "";
  private method = "GET";
  private headers: [string, string][] = [];
  private body?: string;

  setUrl(url: string): this {
    this.url = url;
    return this;
  }

  setMethod(method: string): this {
    this.method = method;
    return this;
  }

  setHeader(key: string, value: string): this {
    this.headers.push([key, value]);
    return this;
  }

  setBody(body: string): this {
    this.body = body;
    return this;
  }

  build(): HttpRequest {
    return { url: this.url, method: this.method, headers: this.headers, body: this.body };
  }
}

// --- main ---
const req = new HttpRequestBuilder()
  .setUrl("https://api.example.com/users")
  .setMethod("POST")
  .setHeader("Content-Type", "application/json")
  .setHeader("Authorization", "Bearer token-123")
  .setBody(JSON.stringify({ name: "Alice", age: 30 }))
  .build();

console.log("=== Builder Pattern ===");
console.log("URL:   ", req.url);
console.log("Method:", req.method);
console.log("Headers:");
req.headers.forEach(([k, v]) => console.log(`  ${k}: ${v}`));
console.log("Body:  ", req.body);

const getReq = new HttpRequestBuilder()
  .setUrl("https://api.example.com/users/1")
  .build();

console.log("\nGET request:");
console.log("URL:   ", getReq.url);
console.log("Method:", getReq.method);
console.log("Body:  ", getReq.body);

// Rust 关键差异：
// - TS 用 `this` 返回自身引用；Rust 用 `mut self` 消费并返回所有权
// - Rust 的链式调用每步都 move，编译器保证不会重用已消费的 builder
