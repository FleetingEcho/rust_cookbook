// ============================================================
// Adapter Pattern — 让不兼容的接口可以一起工作
// 对比 TS: 06_adapter.ts
// 运行: cargo run --bin adapter
// ============================================================

// 我们期望的新接口
trait PaymentProcessor {
    fn pay(&self, amount_cents: u64) -> bool;
    fn refund(&self, amount_cents: u64) -> bool;
}

// 旧的第三方 API（无法修改）
struct LegacyApi {
    merchant_id: String,
}

impl LegacyApi {
    fn new(id: &str) -> Self { Self { merchant_id: id.into() } }

    fn process_payment(&self, amount: f64, currency: &str) -> i32 {
        println!("[LegacyApi] merchant={} pay {:.2} {}", self.merchant_id, amount, currency);
        0
    }

    fn process_refund(&self, amount: f64, currency: &str) -> i32 {
        println!("[LegacyApi] merchant={} refund {:.2} {}", self.merchant_id, amount, currency);
        0
    }
}

// 适配器：把旧 API 包装成新接口
struct LegacyAdapter {
    api: LegacyApi,
    currency: String,
}

impl LegacyAdapter {
    fn new(merchant_id: &str, currency: &str) -> Self {
        Self { api: LegacyApi::new(merchant_id), currency: currency.into() }
    }
}

impl PaymentProcessor for LegacyAdapter {
    fn pay(&self, amount_cents: u64) -> bool {
        self.api.process_payment(amount_cents as f64 / 100.0, &self.currency) == 0
    }
    fn refund(&self, amount_cents: u64) -> bool {
        self.api.process_refund(amount_cents as f64 / 100.0, &self.currency) == 0
    }
}

// 业务代码只依赖新接口
fn checkout(processor: &dyn PaymentProcessor, amount_cents: u64) {
    println!("Checkout {} cents =>", amount_cents);
    let ok = processor.pay(amount_cents);
    println!("Result: {}\n", if ok { "success" } else { "failed" });
}

fn main() {
    println!("=== Adapter Pattern ===\n");

    let processor = LegacyAdapter::new("merchant-001", "USD");
    checkout(&processor, 1999);
    checkout(&processor, 5000);

    println!("Refund $19.99:");
    processor.refund(1999);
}
