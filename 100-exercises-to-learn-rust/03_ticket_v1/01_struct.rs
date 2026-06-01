// 🔑 要点：struct 是 Rust 自定义数据类型
// 使用 impl 块为 struct 添加方法
// 方法的第一个参数是 self（或 &self、&mut self）

struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    // &self 是对 self 的不可变引用，不获取所有权
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
