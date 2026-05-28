// 与 C/C++ 不同，Rust 中函数定义的顺序没有限制
pub fn test() {
    // 我们可以在这里使用函数，并在稍后的某处定义它
    // fizzbuzz_to(100);
    // test2();
    test3();
}

// 返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 特殊情况，提前返回
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，此处不需要 `return` 关键字
    lhs % rhs == 0
}

// "无返回值"的函数实际上返回单元类型 `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，可以在函数签名中省略返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

// 关联函数和方法
// 某些函数与特定类型相关联。这些函数有两种形式：关联函数和方法。关联函数是在类型上定义的函数，而方法是在类型的特定实例上调用的关联函数。

struct Point {
    x: f64,
    y: f64,
}

// 实现块，所有 `Point` 的关联函数和方法都在此处定义
impl Point {
    // 这是一个"关联函数"，因为这个函数与特定类型 Point 相关联。
    //
    // 关联函数不需要通过实例来调用。
    // 这些函数通常用作构造函数。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另一个接受两个参数的关联函数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个方法
    // `&self` 是 `self: &Self` 的语法糖，其中 `Self` 是调用者对象的类型。
    // 在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用对象是可变的
    // `&mut self` 是 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有两个堆分配的整数资源
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会"消耗"调用对象的资源
    // `self` 是 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("正在销毁 Pair({}, {})", first, second);

        // `first` 和 `second` 超出作用域并被释放
    }
}

fn test1() {
    let rectangle = Rectangle {
        // 使用双冒号调用关联函数
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 使用点运算符调用方法
    // 注意，第一个参数 `&self` 是隐式传递的
    // 即 `rectangle.perimeter()` 等同于 `Rectangle::perimeter(&rectangle)`
    println!("矩形周长：{}", rectangle.perimeter());
    println!("矩形面积：{}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 错误！`rectangle` 是不可变的，但这个方法需要可变对象
    //rectangle.translate(1.0, 0.0);
    // TODO ^ 尝试取消这行的注释

    // 正确！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！之前的 `destroy` 调用已经"消耗"了 `pair`
    //pair.destroy();
    // TODO ^ 尝试取消注释这一行
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

//高阶函数
// Rust 提供了高阶函数（Higher Order Functions，HOF）。这些函数接受一个或多个函数作为参数，并/或产生一个更有用的函数。HOF 和惰性迭代器赋予了 Rust 函数式编程的特性。
fn test2() {
    println!("找出所有平方为奇数且小于 1000 的数字之和");
    let upper = 1000;

    // 命令式方法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：从 0, 1, 2, ... 到无穷大
    for n in 0.. {
        // 计算数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 如果超过上限则跳出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数，则累加值
            acc += n_squared;
        }
    }
    println!("命令式风格：{}", acc);

    // 函数式方法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数的平方
        .take_while(|&n_squared| n_squared < upper) // 小于上限
        .filter(|&n_squared| is_odd(n_squared)) // 筛选奇数
        .sum(); // 求和
    println!("函数式风格：{}", sum_of_squared_odd_numbers);
}

//发散函数
// 发散函数永不返回。它们使用 ! 标记，这是一个空类型。

fn foo() -> ! {
    panic!("此调用永不返回。");
}

// 与所有其他类型相反，这个类型不能被实例化，因为这个类型可能拥有的所有可能值的集合是空的。注意，它与 () 类型不同，后者恰好有一个可能的值。
// 例如，这个函数像往常一样返回，尽管返回值中没有任何信息。

fn some_fn() {
    ()
}

fn test3() {
    let _a: () = some_fn();
    println!("这个函数返回了，你可以看到这一行。");

    // let x: ! = panic!("此调用永不返回。");
    // println!("你永远不会看到这一行！");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回类型必须是 u32，
            // 因为 "addition" 变量的类型是 u32。
            let addition: u32 = match i % 2 == 1 {
                // "i" 变量的类型是 u32，这完全没问题。
                true => i,
                // 另一方面，"continue" 表达式不返回 u32，
                // 但这仍然可以，因为它永远不会返回，
                // 因此不违反 match 表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("9 以下（不包括 9）的奇数之和：{}", sum_odd_numbers(9));
}
