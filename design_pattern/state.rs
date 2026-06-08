// ============================================================
// State Pattern — 对象根据内部状态改变行为，状态转换显式化
// 对比 TS: 09_state.ts
// 运行: cargo run --bin state
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

struct Order {
    id: u64,
    #[allow(dead_code)]
    item: String,
    status: OrderStatus,
}

impl Order {
    fn new(id: u64, item: &str) -> Self {
        Self { id, item: item.into(), status: OrderStatus::Pending }
    }

    fn pay(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Paid;
                println!("[Order#{}] 支付成功 -> {:?}", self.id, self.status);
                Ok(())
            }
            _ => Err(format!("状态 {:?} 不允许支付", self.status)),
        }
    }

    fn ship(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Paid => {
                self.status = OrderStatus::Shipped;
                println!("[Order#{}] 已发货 -> {:?}", self.id, self.status);
                Ok(())
            }
            _ => Err(format!("状态 {:?} 不允许发货", self.status)),
        }
    }

    fn deliver(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Shipped => {
                self.status = OrderStatus::Delivered;
                println!("[Order#{}] 已送达 -> {:?}", self.id, self.status);
                Ok(())
            }
            _ => Err(format!("状态 {:?} 不允许确认收货", self.status)),
        }
    }

    fn cancel(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Delivered | OrderStatus::Cancelled => {
                Err(format!("状态 {:?} 不允许取消", self.status))
            }
            _ => {
                self.status = OrderStatus::Cancelled;
                println!("[Order#{}] 已取消 -> {:?}", self.id, self.status);
                Ok(())
            }
        }
    }
}

fn try_op(result: Result<(), String>) {
    if let Err(e) = result {
        println!("  [错误] {}", e);
    }
}

fn main() {
    println!("=== State Pattern ===\n");

    println!("--- 正常流程 ---");
    let mut order = Order::new(1001, "MacBook Pro");
    println!("初始: {:?}", order.status);
    try_op(order.pay());
    try_op(order.ship());
    try_op(order.deliver());

    println!("\n--- 非法转换 ---");
    let mut order2 = Order::new(1002, "iPhone");
    try_op(order2.ship());    // Pending 不能直接发货
    try_op(order2.pay());
    try_op(order2.pay());     // 不能重复支付
    try_op(order2.cancel());
    try_op(order2.cancel()); // 已取消不能再取消
}

// Rust 关键差异：
// - enum + match 表达状态机是 Rust 最惯用的写法
// - match 是穷举的，忘处理某个状态会编译报错
// - Result<(), String> 强制调用方处理错误
