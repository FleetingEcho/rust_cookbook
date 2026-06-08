// ============================================================
// Factory Pattern — 工厂函数创建对象，调用方不感知具体类型
// 对比 Rust: 03_factory.rs
// 运行: npx ts-node 03_factory.ts
// ============================================================

interface Logger {
  log(level: string, msg: string): void;
}

class ConsoleLogger implements Logger {
  log(level: string, msg: string) {
    console.log(`[Console][${level}] ${msg}`);
  }
}

class FileLogger implements Logger {
  constructor(private path: string) {}
  log(level: string, msg: string) {
    console.log(`[File:${this.path}][${level}] ${msg}`);
  }
}

class JsonLogger implements Logger {
  log(level: string, msg: string) {
    console.log(JSON.stringify({ level, msg }));
  }
}

function createLogger(kind: string): Logger {
  switch (kind) {
    case "file": return new FileLogger("app.log");
    case "json": return new JsonLogger();
    default:     return new ConsoleLogger();
  }
}

// --- main ---
console.log("=== Factory Pattern ===");

for (const kind of ["console", "file", "json"]) {
  console.log(`\n[${kind}]`);
  const logger = createLogger(kind);
  logger.log("INFO", "server started");
  logger.log("ERROR", "connection refused");
}
