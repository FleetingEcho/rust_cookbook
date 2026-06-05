// 标准库里除 Vec/HashMap 外还有四个常用集合：
// HashSet  — 去重集合，查找 O(1)
// BTreeMap — 有序键值表，查找 O(log n)
// BTreeSet — 有序去重集合，支持范围查询
// VecDeque — 双端队列，两头 push/pop 都是 O(1)

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

// ── HashSet ───────────────────────────────────────────────────────────────────

pub fn hashset_basics() {
    let mut langs: HashSet<&str> = HashSet::new();
    langs.insert("Rust");
    langs.insert("Go");
    langs.insert("Rust"); // 重复插入无效

    // 查询
    println!("包含 Rust: {}", langs.contains("Rust"));
    println!("元素数量: {}", langs.len()); // 2，不是 3

    // 集合运算
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();

    // 交集
    let inter: HashSet<_> = a.intersection(&b).collect();
    println!("交集: {:?}", inter); // {3, 4}

    // 并集
    let union: HashSet<_> = a.union(&b).collect();
    println!("并集: {:?}", union); // {1,2,3,4,5,6}

    // 差集（a 有 b 没有）
    let diff: HashSet<_> = a.difference(&b).collect();
    println!("差集 a-b: {:?}", diff); // {1, 2}
}

// ── BTreeMap ──────────────────────────────────────────────────────────────────

pub fn btreemap_basics() {
    let mut scores: BTreeMap<&str, u32> = BTreeMap::new();
    scores.insert("Alice", 90);
    scores.insert("Bob", 75);
    scores.insert("Carol", 85);

    // BTreeMap 遍历时键是有序的（字典序）
    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    // 范围查询：只看 B 开头到 C 之间的键
    for (name, score) in scores.range("B"..="C") {
        println!("范围内: {name} -> {score}");
    }

    // entry API 与 HashMap 完全一致
    scores.entry("Dave").or_insert(80);
    println!("Dave 的分数: {}", scores["Dave"]);
}

// ── BTreeSet ──────────────────────────────────────────────────────────────────

pub fn btreeset_basics() {
    let mut set: BTreeSet<i32> = BTreeSet::new();
    for n in [5, 2, 8, 1, 9, 3] {
        set.insert(n);
    }

    // 始终按升序迭代
    println!("有序集合: {:?}", set); // [1, 2, 3, 5, 8, 9]

    // 范围查询：3 到 8 之间（含两端）
    let range: Vec<_> = set.range(3..=8).collect();
    println!("3..=8 范围: {:?}", range); // [3, 5, 8]
}

// ── VecDeque ──────────────────────────────────────────────────────────────────

pub fn vecdeque_basics() {
    let mut dq: VecDeque<i32> = VecDeque::new();

    // 两端都可以 push
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(1); // 插到头部
    dq.push_front(0);

    println!("双端队列: {:?}", dq); // [0, 1, 2, 3]

    // 两端都可以 pop
    println!("pop_front: {:?}", dq.pop_front()); // Some(0)
    println!("pop_back:  {:?}", dq.pop_back()); // Some(3)

    // 当成普通 Vec 用也没问题
    dq.push_back(99);
    println!("长度: {}", dq.len());

    // 从 Vec 转换成 VecDeque
    let v = vec![10, 20, 30];
    let dq2: VecDeque<i32> = v.into();
    println!("从 Vec 转换: {:?}", dq2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashset_deduplicates() {
        let mut s: HashSet<i32> = HashSet::new();
        s.insert(1);
        s.insert(1);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn btreemap_is_sorted() {
        let map: BTreeMap<i32, &str> = [(3, "c"), (1, "a"), (2, "b")].into_iter().collect();
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, [&1, &2, &3]);
    }

    #[test]
    fn vecdeque_push_front() {
        let mut dq: VecDeque<i32> = VecDeque::new();
        dq.push_back(2);
        dq.push_front(1);
        assert_eq!(dq[0], 1);
        assert_eq!(dq[1], 2);
    }
}

// 📘 TypeScript 对比
// ====================
// Rust 集合 | TS 对应
// `Vec<T>` | `Array<T>`
// `VecDeque<T>` | 无（双端队列，可用数组模拟但性能差）
// `LinkedList<T>` | 无（JS 没有内置链表）
// `HashMap<K, V>` | `Map<K, V>` 或 `{}`
// `HashSet<T>` | `Set<T>`
// `BinaryHeap<T>` | 无（需第三方库）
//
// ⚠️ Rust 标准库集合比 TS 丰富得多。
//  TS 只有 Array/Map/Set/WeakMap/WeakSet。
//  Rust 还有 VecDeque、LinkedList、BinaryHeap、BTreeMap 等。
//
// 详细对照 → rust_vs_typescript.rs §13
