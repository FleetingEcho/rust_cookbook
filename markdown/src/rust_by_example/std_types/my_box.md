# Box

## 堆分配

```rust
use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn test() {
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point 在栈上占用 {} 字节", mem::size_of_val(&point));
    println!("Rectangle 在栈上占用 {} 字节", mem::size_of_val(&rectangle));
    println!("装箱的 point 在栈上占用 {} 字节", mem::size_of_val(&boxed_point));
    println!("装箱的 rectangle 在栈上占用 {} 字节", mem::size_of_val(&boxed_rectangle));

    let unboxed_point: Point = *boxed_point;
    println!("未装箱的 point 在栈上占用 {} 字节", mem::size_of_val(&unboxed_point));
}
```

> Box 的大小等于指针大小，不论它包装的数据有多大。
