#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // Vec 的默认扩容策略是翻倍：2 → 4
        // ⚠️ 这是当前实现，未来可能变化
        assert_eq!(v.capacity(), 4);
    }
}
