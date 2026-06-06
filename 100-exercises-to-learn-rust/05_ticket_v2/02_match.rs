// 🔑 要点：match 是 Rust 最强大的控制流表达式
// 必须穷举所有分支（exhaustive）
// 每个分支返回一个表达式值

enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

impl Shape {
    pub fn n_sides(&self) -> u8 {
        // match 必须覆盖所有枚举变体
        match self {
            Shape::Circle => 0,
            Shape::Square => 4,
            Shape::Rectangle => 4,
            Shape::Triangle => 3,
            Shape::Pentagon => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        assert_eq!(Shape::Circle.n_sides(), 0);
    }
    #[test]
    fn test_square() {
        assert_eq!(Shape::Square.n_sides(), 4);
    }
    #[test]
    fn test_rectangle() {
        assert_eq!(Shape::Rectangle.n_sides(), 4);
    }
    #[test]
    fn test_triangle() {
        assert_eq!(Shape::Triangle.n_sides(), 3);
    }
    #[test]
    fn test_pentagon() {
        assert_eq!(Shape::Pentagon.n_sides(), 5);
    }
}
