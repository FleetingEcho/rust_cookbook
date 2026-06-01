// 🔑 要点：if let 是 match 的语法糖——只匹配一个模式
// 适合只关心一个变体、忽略其余的情况
// 这里只关心 Circle 变体才能获取半径

enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    pub fn radius(&self) -> f64 {
        // 使用 if let 只匹配 Circle
        if let Shape::Circle { radius } = self {
            *radius
        } else {
            panic!("Not a circle!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle { width: 1.0, height: 2.0 }.radius();
    }
}
