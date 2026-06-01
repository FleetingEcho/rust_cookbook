// 🔑 要点：综合练习——实现一个完整的 Order 类型
// 需要实现：构造函数、getter、setter、验证、总价计算
// 注意可见性：集成测试只能访问 pub 的 API

// TODO: 定义 Order 类型
// 三个字段：product_name (String), quantity (u32), unit_price (u32)
// 验证规则：
// - product_name 不能为空，不能超过 300 字节
// - quantity 必须大于 0
// - unit_price 必须大于 0（单位：分）
// 方法：
// - total() 返回总价（quantity × unit_price）
// - 每个字段的 getter 和 setter

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if product_name.is_empty() { panic!("Product name cannot be empty"); }
        if product_name.len() > 300 { panic!("Product name cannot be longer than 300 bytes"); }
        if quantity == 0 { panic!("Quantity must be greater than zero"); }
        if unit_price == 0 { panic!("Unit price must be greater than zero"); }
        Self { product_name, quantity, unit_price }
    }

    pub fn product_name(&self) -> &str { &self.product_name }
    pub fn quantity(&self) -> &u32 { &self.quantity }
    pub fn unit_price(&self) -> &u32 { &self.unit_price }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        if name.is_empty() { panic!("Product name cannot be empty"); }
        if name.len() > 300 { panic!("Product name cannot be longer than 300 bytes"); }
        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        if quantity == 0 { panic!("Quantity must be greater than zero"); }
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, price: u32) {
        if price == 0 { panic!("Unit price must be greater than zero"); }
        self.unit_price = price;
    }
}

// 内联的集成测试
#[cfg(test)]
mod integration_tests {
    use super::Order;

    #[test]
    fn test_order() {
        let mut order = Order::new("Rusty Book".to_string(), 3, 2999);

        assert_eq!(order.product_name(), "Rusty Book");
        assert_eq!(order.quantity(), &3);
        assert_eq!(order.unit_price(), &2999);
        assert_eq!(order.total(), 8997);

        order.set_product_name("Rust Book".to_string());
        order.set_quantity(2);
        order.set_unit_price(3999);

        assert_eq!(order.product_name(), "Rust Book");
        assert_eq!(order.quantity(), &2);
        assert_eq!(order.unit_price(), &3999);
        assert_eq!(order.total(), 7998);
    }

    #[test]
    #[should_panic]
    fn test_empty_product_name() {
        Order::new("".to_string(), 3, 2999);
    }

    #[test]
    #[should_panic]
    fn test_long_product_name() {
        Order::new("a".repeat(301), 3, 2999);
    }

    #[test]
    #[should_panic]
    fn test_zero_quantity() {
        Order::new("Rust Book".to_string(), 0, 2999);
    }

    #[test]
    #[should_panic]
    fn test_zero_unit_price() {
        Order::new("Rust Book".to_string(), 3, 0);
    }
}
