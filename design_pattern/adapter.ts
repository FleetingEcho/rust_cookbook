// ============================================================
// Adapter Pattern — 让不兼容的接口可以一起工作
// 对比 Rust: 06_adapter.rs
// 运行: npx ts-node 06_adapter.ts
// ============================================================

// 我们期望的新接口
interface PaymentProcessor {
  pay(amountCents: number): boolean;
  refund(amountCents: number): boolean;
}

// 旧的第三方 API（无法修改）
class LegacyApi {
  constructor(private merchantId: string) {}

  processPayment(amount: number, currency: string): number {
    console.log(`[LegacyApi] merchant=${this.merchantId} pay ${amount.toFixed(2)} ${currency}`);
    return 0;
  }

  processRefund(amount: number, currency: string): number {
    console.log(`[LegacyApi] merchant=${this.merchantId} refund ${amount.toFixed(2)} ${currency}`);
    return 0;
  }
}

// 适配器
class LegacyAdapter implements PaymentProcessor {
  private api: LegacyApi;

  constructor(merchantId: string, private currency: string) {
    this.api = new LegacyApi(merchantId);
  }

  pay(amountCents: number): boolean {
    return this.api.processPayment(amountCents / 100, this.currency) === 0;
  }

  refund(amountCents: number): boolean {
    return this.api.processRefund(amountCents / 100, this.currency) === 0;
  }
}

function checkout(processor: PaymentProcessor, amountCents: number) {
  console.log(`Checkout ${amountCents} cents =>`);
  const ok = processor.pay(amountCents);
  console.log("Result:", ok ? "success" : "failed", "\n");
}

// --- main ---
console.log("=== Adapter Pattern ===\n");

const processor = new LegacyAdapter("merchant-001", "USD");
checkout(processor, 1999);
checkout(processor, 5000);

console.log("Refund $19.99:");
processor.refund(1999);
