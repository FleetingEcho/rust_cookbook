// ============================================================
// Observer Pattern — 状态变化时自动通知所有订阅者
// 对比 Rust: 08_observer.rs
// 运行: npx ts-node 08_observer.ts
// ============================================================

type Handler<T> = (event: T) => void;

class EventEmitter<T> {
  private handlers = new Map<number, Handler<T>>();
  private nextId = 0;

  on(handler: Handler<T>): number {
    const id = this.nextId++;
    this.handlers.set(id, handler);
    return id;
  }

  off(id: number) {
    this.handlers.delete(id);
    console.log(`[Emitter] 取消订阅 id=${id}`);
  }

  emit(event: T) {
    this.handlers.forEach(h => h(event));
  }
}

interface UserEvent {
  userId: number;
  action: string;
}

// --- main ---
console.log("=== Observer Pattern ===\n");

const emitter = new EventEmitter<UserEvent>();

const logId = emitter.on(e => {
  console.log(`[Log]   user=${e.userId} action=${e.action}`);
});

const _statId = emitter.on(e => {
  console.log(`[Stats] 记录事件: ${e.action}`);
});

const emailId = emitter.on(e => {
  if (e.action === "register") {
    console.log(`[Email] 发送欢迎邮件给 user=${e.userId}`);
  }
});

console.log("--- 触发 register ---");
emitter.emit({ userId: 1, action: "register" });

console.log("\n--- 触发 login ---");
emitter.emit({ userId: 1, action: "login" });

console.log("\n--- 取消 email + log 订阅 ---");
emitter.off(emailId);
emitter.off(logId);

console.log("\n--- 再次触发（只剩 stats）---");
emitter.emit({ userId: 2, action: "login" });
