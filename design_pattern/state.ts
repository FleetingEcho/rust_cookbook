// ============================================================
// State Pattern — 对象根据内部状态改变行为
// 对比 Rust: 09_state.rs
// 运行: npx ts-node 09_state.ts
// ============================================================

type OrderStatus = "Pending" | "Paid" | "Shipped" | "Delivered" | "Cancelled";

class Order {
  private status: OrderStatus = "Pending";

  constructor(public readonly id: number, public readonly item: string) {}

  pay(): void {
    if (this.status !== "Pending") throw new Error(`状态 ${this.status} 不允许支付`);
    this.status = "Paid";
    console.log(`[Order#${this.id}] 支付成功 -> ${this.status}`);
  }

  ship(): void {
    if (this.status !== "Paid") throw new Error(`状态 ${this.status} 不允许发货`);
    this.status = "Shipped";
    console.log(`[Order#${this.id}] 已发货 -> ${this.status}`);
  }

  deliver(): void {
    if (this.status !== "Shipped") throw new Error(`状态 ${this.status} 不允许确认收货`);
    this.status = "Delivered";
    console.log(`[Order#${this.id}] 已送达 -> ${this.status}`);
  }

  cancel(): void {
    if (this.status === "Delivered" || this.status === "Cancelled")
      throw new Error(`状态 ${this.status} 不允许取消`);
    this.status = "Cancelled";
    console.log(`[Order#${this.id}] 已取消 -> ${this.status}`);
  }

  getStatus(): OrderStatus { return this.status; }
}

const tryOp = (fn: () => void) => {
  try { fn(); } catch (e: any) { console.log("  [错误]", e.message); }
};

// --- main ---
console.log("=== State Pattern ===\n");

console.log("--- 正常流程 ---");
const order = new Order(1001, "MacBook Pro");
console.log("初始:", order.getStatus());
tryOp(() => order.pay());
tryOp(() => order.ship());
tryOp(() => order.deliver());

console.log("\n--- 非法转换 ---");
const order2 = new Order(1002, "iPhone");
tryOp(() => order2.ship());
tryOp(() => order2.pay());
tryOp(() => order2.pay());
tryOp(() => order2.cancel());
tryOp(() => order2.cancel());

// Rust 关键差异：
// - Rust 的 match 是穷举的，漏掉状态编译报错
// - TS 的 if/else 漏掉状态只是运行时 bug
