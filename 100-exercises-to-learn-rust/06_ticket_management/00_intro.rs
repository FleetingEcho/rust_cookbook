// 🔑 要点：第六章构建了一个完整的票据管理系统
// 涉及 Vec、迭代器、HashMap、BTreeMap 等集合类型
// 以及 Index/IndexMut trait 的实现

fn intro() -> &'static str {
    "I'm ready to build a ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a ticket management system!");
    }
}
