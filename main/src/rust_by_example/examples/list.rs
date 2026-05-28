use crate::rust_by_example::examples::list::List::*;

enum List {
    // Cons：包含一个元素和指向下一节点指针的元组结构体
    Cons(u32, Box<List>),
    // Nil：表示链表末尾的节点
    Nil,
}

// 可以为枚举实现方法
impl List {
    // 创建空链表
    fn new() -> List {
        // `Nil` 的类型是 `List`
        Nil
    }

    // 消耗一个链表,并返回在其头部添加新元素后的链表
    fn prepend(self, elem: u32) -> List {
        // `Cons` 的类型也是 List
        Cons(elem, Box::new(self))
    }

    // 返回链表长度
    fn len(&self) -> u32 {
        // 需要对 `self` 进行匹配,因为方法的行为取决于 `self` 的变体
        // `self` 的类型是 `&List`,而 `*self` 的类型是 `List`
        // 匹配具体类型 `T` 优于匹配引用 `&T`
        // 在 Rust 2018 版本后,这里可以直接使用 self,下面可以使用 tail（无需 ref）
        // Rust 会自动推断为 &s 和 ref tail
        // 参见 https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // 无法获取 tail 的所有权,因为 `self` 是借用的；
            // 所以取 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本情况：空链表长度为零
            Nil => 0,
        }
    }

    // 返回链表的字符串表示（堆分配）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 类似于 `print!`,但返回堆分配的字符串,
                // 而不是打印到控制台
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn test() {
    // 创建空链表
    let mut list = List::new();

    // 在头部添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最终状态
    println!("链表长度为：{}", list.len());
    println!("{}", list.stringify());
}
