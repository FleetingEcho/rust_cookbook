// ============================================================
// Newtype Pattern — 区分语义相同但含义不同的值，防止混用
// 对比 TS: 04_newtype.ts
// 运行: cargo run --bin newtype
// ============================================================

// 零成本抽象：编译后与裸 String/f64 完全一样，无运行时开销
#[derive(Debug, Clone)]
struct UserId(String);

#[derive(Debug, Clone)]
struct OrderId(String);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilograms(f64);

impl UserId {
    fn new(id: &str) -> Self { UserId(id.into()) }
}

impl OrderId {
    fn new(id: &str) -> Self { OrderId(id.into()) }
}

fn get_user(id: &UserId) {
    println!("查询用户: {}", id.0);
}

fn get_order(id: &OrderId) {
    println!("查询订单: {}", id.0);
}

fn print_distance(d: Meters) {
    println!("距离: {} 米", d.0);
}

fn main() {
    println!("=== Newtype Pattern ===");

    let uid = UserId::new("u-123");
    let oid = OrderId::new("o-456");

    get_user(&uid);
    get_order(&oid);

    // 取消注释可验证编译错误：
    // get_user(&oid);   // error: expected `UserId`, found `OrderId`
    // get_order(&uid);  // error: expected `OrderId`, found `UserId`

    println!("\n--- 物理量防混用 ---");
    let distance = Meters(100.0);
    let weight   = Kilograms(75.5);

    print_distance(distance);
    // print_distance(weight); // 编译错误！

    println!("体重: {} kg", weight.0);
    println!("50m < 100m? {}", Meters(50.0) < Meters(100.0));
}

// Rust 关键差异：
// - Newtype 是零成本抽象，编译后无额外开销
// - TS 的 branded types 只是编译器技巧，运行时仍是普通值
