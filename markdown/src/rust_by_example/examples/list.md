# 链表

## 使用枚举实现链表

```rust
use crate::rust_by_example::examples::list::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

pub fn test() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("链表长度为：{}", list.len());
    println!("{}", list.stringify());
}
```

## 链表结构

| 变体 | 含义 |
|------|------|
| `Cons(u32, Box<List>)` | 包含元素和指向下一个节点的指针 |
| `Nil` | 表示链表末尾 |
