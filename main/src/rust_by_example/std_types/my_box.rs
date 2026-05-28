use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// 可以通过指定左上角和右下角在空间中的位置来定义矩形
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个点，并返回指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn test() {
    // （所有的类型标注都不是必须的）
    // 栈分配的变量
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // 堆分配的矩形
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // 函数的输出可以被装箱
    let boxed_point: Box<Point> = Box::new(origin());

    // 双重间接引用
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point 在栈上占用 {} 字节", mem::size_of_val(&point));
    println!("Rectangle 在栈上占用 {} 字节", mem::size_of_val(&rectangle));

    // box 大小 == 指针大小
    println!(
        "装箱的 point 在栈上占用 {} 字节",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "装箱的 rectangle 在栈上占用 {} 字节",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "装箱的 box 在栈上占用 {} 字节",
        mem::size_of_val(&box_in_a_box)
    );

    // 将 `boxed_point` 中的数据复制到 `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "未装箱的 point 在栈上占用 {} 字节",
        mem::size_of_val(&unboxed_point)
    );
}
