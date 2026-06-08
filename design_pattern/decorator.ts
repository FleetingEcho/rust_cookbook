// ============================================================
// Decorator Pattern — 动态给对象添加职责，比继承更灵活
// 对比 Rust: 05_decorator.rs
// 运行: npx ts-node 05_decorator.ts
// ============================================================

interface DataSource {
  write(data: string): void;
  read(): string;
}

class MemoryStore implements DataSource {
  private data = "";
  write(data: string) { this.data = data; }
  read(): string      { return this.data; }
}

class Encrypted implements DataSource {
  constructor(private inner: DataSource) {}

  write(data: string) {
    const enc = data.split("").map(c => String.fromCharCode(c.charCodeAt(0) + 1)).join("");
    console.log(`  [Encrypt] '${data}' -> '${enc}'`);
    this.inner.write(enc);
  }
  read(): string {
    const raw = this.inner.read();
    const dec = raw.split("").map(c => String.fromCharCode(c.charCodeAt(0) - 1)).join("");
    console.log(`  [Decrypt] '${raw}' -> '${dec}'`);
    return dec;
  }
}

class Logged implements DataSource {
  constructor(private inner: DataSource, private label: string) {}

  write(data: string) {
    console.log(`  [Log:${this.label}] write ${data.length} bytes`);
    this.inner.write(data);
  }
  read(): string {
    const result = this.inner.read();
    console.log(`  [Log:${this.label}] read  ${result.length} bytes`);
    return result;
  }
}

// --- main ---
console.log("=== Decorator Pattern ===\n");

console.log("--- 裸存储 ---");
const s1 = new MemoryStore();
s1.write("hello");
console.log("read:", s1.read(), "\n");

console.log("--- 加密 ---");
const s2 = new Encrypted(new MemoryStore());
s2.write("hello");
console.log("read:", s2.read(), "\n");

console.log("--- 日志 + 加密（嵌套）---");
const s3 = new Logged(new Encrypted(new MemoryStore()), "app");
s3.write("hello");
console.log("read:", s3.read());
