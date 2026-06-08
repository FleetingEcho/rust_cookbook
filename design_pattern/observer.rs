// ============================================================
// Observer Pattern — 状态变化时自动通知所有订阅者
// 对比 TS: 08_observer.ts
// 运行: cargo run --bin observer
// ============================================================

use std::collections::HashMap;

type HandlerId = u64;

struct EventEmitter<T> {
    next_id: HandlerId,
    handlers: HashMap<HandlerId, Box<dyn Fn(&T)>>,
}

impl<T> EventEmitter<T> {
    fn new() -> Self {
        Self { next_id: 0, handlers: HashMap::new() }
    }

    fn on(&mut self, handler: impl Fn(&T) + 'static) -> HandlerId {
        let id = self.next_id;
        self.next_id += 1;
        self.handlers.insert(id, Box::new(handler));
        id
    }

    fn off(&mut self, id: HandlerId) {
        self.handlers.remove(&id);
        println!("[Emitter] 取消订阅 id={}", id);
    }

    fn emit(&self, event: &T) {
        for handler in self.handlers.values() {
            handler(event);
        }
    }
}

#[derive(Debug)]
struct UserEvent {
    user_id: u64,
    action: String,
}

fn main() {
    println!("=== Observer Pattern ===\n");

    let mut emitter = EventEmitter::<UserEvent>::new();

    let log_id = emitter.on(|e| {
        println!("[Log]   user={} action={}", e.user_id, e.action);
    });

    let _stat_id = emitter.on(|e| {
        println!("[Stats] 记录事件: {}", e.action);
    });

    let email_id = emitter.on(|e| {
        if e.action == "register" {
            println!("[Email] 发送欢迎邮件给 user={}", e.user_id);
        }
    });

    println!("--- 触发 register ---");
    emitter.emit(&UserEvent { user_id: 1, action: "register".into() });

    println!("\n--- 触发 login ---");
    emitter.emit(&UserEvent { user_id: 1, action: "login".into() });

    println!("\n--- 取消 email + log 订阅 ---");
    emitter.off(email_id);
    emitter.off(log_id);

    println!("\n--- 再次触发（只剩 stats）---");
    emitter.emit(&UserEvent { user_id: 2, action: "login".into() });
}
