// ============================================================
// Typestate Pattern — 用类型系统在编译期强制状态顺序
// 对比 TS: 13_typestate.ts
// 运行: cargo run --bin typestate
// ============================================================

use std::marker::PhantomData;

// 状态标记（零大小类型，纯编译期信息，无运行时开销）
struct Disconnected;
struct Connected;
struct Authenticated;

struct Client<State> {
    host: String,
    port: u16,
    _state: PhantomData<State>,
}

// 只有 Disconnected 状态才有 connect 方法
impl Client<Disconnected> {
    fn new(host: &str, port: u16) -> Self {
        Self { host: host.into(), port, _state: PhantomData }
    }

    fn connect(self) -> Client<Connected> {
        println!("[{}:{}] 已连接", self.host, self.port);
        Client { host: self.host, port: self.port, _state: PhantomData }
    }
}

// 只有 Connected 状态才有 authenticate 方法
impl Client<Connected> {
    fn authenticate(self, token: &str) -> Client<Authenticated> {
        println!("[{}:{}] 认证成功 token={}", self.host, self.port, token);
        Client { host: self.host, port: self.port, _state: PhantomData }
    }

    #[allow(dead_code)]
    fn disconnect(self) -> Client<Disconnected> {
        println!("[{}:{}] 已断开", self.host, self.port);
        Client { host: self.host, port: self.port, _state: PhantomData }
    }
}

// 只有 Authenticated 状态才能 send / receive
impl Client<Authenticated> {
    fn send(&self, data: &str) {
        println!("[{}:{}] 发送: {}", self.host, self.port, data);
    }

    fn receive(&self) -> String {
        let msg = format!("来自 {}:{} 的响应", self.host, self.port);
        println!("[{}:{}] 接收: {}", self.host, self.port, msg);
        msg
    }
}

fn main() {
    println!("=== Typestate Pattern ===\n");

    println!("--- 正确流程 ---");
    let client = Client::<Disconnected>::new("api.example.com", 443);
    let client = client.connect();
    let client = client.authenticate("Bearer xyz");
    client.send(r#"{"action":"getUsers"}"#);
    client.receive();

    println!("\n--- 编译期阻止非法调用（取消注释验证）---");
    // let c = Client::<Disconnected>::new("x.com", 80);
    // c.send("data");
    // ^ error[E0599]: no method named `send` found for `Client<Disconnected>`
    //
    // let c = Client::<Disconnected>::new("x.com", 80).connect();
    // c.send("data");
    // ^ error[E0599]: no method named `send` found for `Client<Connected>`
    //   必须先 authenticate，否则编译不过

    println!("Typestate 保证：未认证的客户端根本没有 send/receive 方法");
    println!("非法调用是编译错误，不是运行时异常");
}

// Rust 关键差异：
// - PhantomData<State> 是零成本的，编译后状态类型不存在于二进制中
// - 状态转换消费 self（move 语义），防止持有多个状态的引用
// - TS 只能运行时 throw，Rust 在编译期就拒绝
