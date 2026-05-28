// 模块用于组织代码；pub 控制可见性。
// 测试通常放在 #[cfg(test)] 模块里，只在 cargo test 时编译。

pub mod classroom {
    pub struct Student {
        pub name: String,
        score: u32,
    }

    impl Student {
        pub fn new(name: impl Into<String>, score: u32) -> Self {
            Self {
                name: name.into(),
                score,
            }
        }

        pub fn passed(&self) -> bool {
            self.score >= 60
        }
    }

    pub fn average_score(students: &[Student]) -> Option<f64> {
        if students.is_empty() {
            return None;
        }

        let total: u32 = students.iter().map(|student| student.score).sum();
        Some(total as f64 / students.len() as f64)
    }
}

pub fn module_demo() -> bool {
    let student = classroom::Student::new("Olivia", 95);

    // name 是 pub，可以直接访问；score 是私有字段，只能通过方法间接使用。
    println!("学生姓名: {}", student.name);
    student.passed()
}

#[cfg(test)]
mod tests {
    use super::classroom::{average_score, Student};

    #[test]
    fn checks_passed_status() {
        let student = Student::new("Teng", 88);
        assert!(student.passed());
    }

    #[test]
    fn calculates_average_score() {
        let students = [Student::new("A", 80), Student::new("B", 100)];
        assert_eq!(average_score(&students), Some(90.0));
        assert_eq!(average_score(&[]), None);
    }
}
