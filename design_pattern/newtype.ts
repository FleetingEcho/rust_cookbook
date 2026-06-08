// ============================================================
// Newtype Pattern — 区分语义相同但含义不同的值，防止混用
// 对比 Rust: 04_newtype.rs
// 运行: npx ts-node 04_newtype.ts
// ============================================================

// TS 用 branded types 模拟（只是编译器技巧，运行时仍是 string/number）
type UserId    = string & { readonly _brand: "UserId" };
type OrderId   = string & { readonly _brand: "OrderId" };
type Meters    = number & { readonly _brand: "Meters" };
type Kilograms = number & { readonly _brand: "Kilograms" };

const makeUserId  = (id: string): UserId    => id as UserId;
const makeOrderId = (id: string): OrderId   => id as OrderId;
const makeMeters  = (n: number): Meters     => n as Meters;
const makeKg      = (n: number): Kilograms  => n as Kilograms;

function getUser(id: UserId)          { console.log("查询用户:", id); }
function getOrder(id: OrderId)        { console.log("查询订单:", id); }
function printDistance(d: Meters)     { console.log("距离:", d, "米"); }

// --- main ---
console.log("=== Newtype Pattern ===");

const uid = makeUserId("u-123");
const oid = makeOrderId("o-456");

getUser(uid);
getOrder(oid);

// TS 编译器会报错（取消注释验证）：
// getUser(oid);   // Argument of type 'OrderId' is not assignable to 'UserId'
// getOrder(uid);

console.log("\n--- 物理量防混用 ---");
const distance = makeMeters(100);
const weight   = makeKg(75.5);

printDistance(distance);
// printDistance(weight); // 编译错误

console.log("体重:", weight, "kg");
console.log("50m < 100m?", makeMeters(50) < makeMeters(100));

// Rust 关键差异：
// - Rust Newtype 是独立类型，零运行时开销
// - TS branded types 运行时 typeof uid === "string"，只是编译期幻觉
