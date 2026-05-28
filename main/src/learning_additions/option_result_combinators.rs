// Option 和 Result 的组合子（combinator）让错误处理可以写成流式链式调用，
// 避免大量嵌套的 match/if let。
//
// 核心思路：把"可能没有值"或"可能出错"的操作串联起来，
//           只在最终一次解包，中间全部用组合子传递。

// ── Option 组合子 ─────────────────────────────────────────────────────────────

pub fn option_combinators() {
    let some_val: Option<i32> = Some(10);
    let none_val: Option<i32> = None;

    // map：有值时转换值，None 直接传递。
    println!("{:?}", some_val.map(|x| x * 2)); // Some(20)
    println!("{:?}", none_val.map(|x| x * 2)); // None

    // and_then（flatMap）：有值时执行可能失败的操作，避免 Some(Some(...)) 嵌套。
    let result = some_val.and_then(|x| if x > 5 { Some(x * 10) } else { None });
    println!("{:?}", result); // Some(100)

    // or / or_else：None 时提供备选值。
    println!("{:?}", none_val.or(Some(42)));              // Some(42)
    println!("{:?}", none_val.or_else(|| Some(99)));      // Some(99)

    // unwrap_or / unwrap_or_else / unwrap_or_default：解包，None 时给默认值。
    println!("{}", none_val.unwrap_or(0));                // 0
    println!("{}", none_val.unwrap_or_else(|| 2 + 2));   // 4（惰性求值）
    println!("{}", none_val.unwrap_or_default());         // 0（i32 的 Default）

    // filter：有值时按条件过滤，不满足则变 None。
    println!("{:?}", some_val.filter(|&x| x > 5));  // Some(10)
    println!("{:?}", some_val.filter(|&x| x > 50)); // None

    // zip：把两个 Option 合并成一个 Option<(A, B)>，任一为 None 则结果为 None。
    let a = Some("hello");
    let b = Some(42);
    println!("{:?}", a.zip(b));          // Some(("hello", 42))
    println!("{:?}", a.zip(none_val));   // None

    // flatten：Option<Option<T>> → Option<T>。
    let nested: Option<Option<i32>> = Some(Some(7));
    println!("{:?}", nested.flatten()); // Some(7)

    // is_some / is_none：简单判断，不解包。
    println!("some_val 有值: {}", some_val.is_some());
    println!("none_val 为空: {}", none_val.is_none());
}

// ── Result 组合子 ─────────────────────────────────────────────────────────────

pub fn result_combinators() {
    let ok_val: Result<i32, &str> = Ok(10);
    let err_val: Result<i32, &str> = Err("出错了");

    // map：Ok 时转换值，Err 直接透传。
    println!("{:?}", ok_val.map(|x| x * 3));   // Ok(30)
    println!("{:?}", err_val.map(|x| x * 3));  // Err("出错了")

    // map_err：Err 时转换错误，Ok 直接透传（常用于统一错误类型）。
    println!("{:?}", err_val.map_err(|e| format!("错误: {e}"))); // Err("错误: 出错了")

    // and_then：Ok 时执行下一步可能失败的操作（链式调用的核心）。
    let chained = ok_val
        .and_then(|x| if x > 5 { Ok(x * 2) } else { Err("太小了") })
        .and_then(|x| Ok(x + 1));
    println!("{:?}", chained); // Ok(21)

    // or / or_else：Err 时提供备选值。
    println!("{:?}", err_val.or(Ok::<i32, &str>(0)));            // Ok(0)
    println!("{:?}", err_val.or_else(|_| Ok::<i32, &str>(99))); // Ok(99)

    // unwrap_or / unwrap_or_else：解包，Err 时给默认值（不 panic）。
    println!("{}", err_val.unwrap_or(0));
    println!("{}", err_val.unwrap_or_else(|_| -1));

    // ok() / err()：Result ↔ Option 互转。
    println!("{:?}", ok_val.ok());  // Some(10)
    println!("{:?}", err_val.ok()); // None
    println!("{:?}", err_val.err()); // Some("出错了")

    // is_ok / is_err
    println!("ok_val 成功: {}", ok_val.is_ok());
    println!("err_val 失败: {}", err_val.is_err());
}

// ── Option ↔ Result 互转 ──────────────────────────────────────────────────────

pub fn conversions() {
    // ok_or / ok_or_else：Option → Result，None 时给出具体的错误。
    let opt: Option<i32> = Some(5);
    let res: Result<i32, &str> = opt.ok_or("没有值");
    println!("{:?}", res); // Ok(5)

    let none: Option<i32> = None;
    println!("{:?}", none.ok_or("没有值")); // Err("没有值")

    // transpose：Option<Result<T,E>> ↔ Result<Option<T>, E>。
    let opt_res: Option<Result<i32, &str>> = Some(Ok(42));
    let res_opt: Result<Option<i32>, &str> = opt_res.transpose();
    println!("{:?}", res_opt); // Ok(Some(42))
}

// ── 实际链式调用示例 ──────────────────────────────────────────────────────────

fn parse_and_double(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()                    // 可能失败：ParseIntError
        .map_err(|e| format!("解析失败: {e}")) // 统一为 String 错误
        .and_then(|n| {
            if n >= 0 {
                Ok(n * 2)
            } else {
                Err("不能是负数".to_string())
            }
        })
}

pub fn show_chain() {
    println!("{:?}", parse_and_double("21"));   // Ok(42)
    println!("{:?}", parse_and_double("abc"));  // Err("解析失败: ...")
    println!("{:?}", parse_and_double("-5"));   // Err("不能是负数")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_then_short_circuits_on_none() {
        let x: Option<i32> = None;
        let y = x.and_then(|n| Some(n * 10));
        assert_eq!(y, None);
    }

    #[test]
    fn parse_and_double_happy_path() {
        assert_eq!(parse_and_double("10"), Ok(20));
    }

    #[test]
    fn parse_and_double_negative() {
        assert!(parse_and_double("-1").is_err());
    }

    #[test]
    fn option_zip() {
        assert_eq!(Some(1).zip(Some("a")), Some((1, "a")));
        assert_eq!(Some(1).zip(None::<&str>), None);
    }
}

// 📘 TypeScript 对比
// ====================
// Rust 组合子 ≈ 链式调用，TS 没有直接等价。
//
// | Rust 组合子 | 作用 | TS 对应 |
// |------------|------|---------|
// | `.map()` | 转换 Some/Ok 的值 | `if(x != null) return f(x)` |
// | `.and_then()` | flatMap——链式可能失败 | `try { return f(x) } catch {}` |
// | `.or()` | None/Err 时用备选值 | `x ?? fallback` |
// | `.unwrap_or()` | 解包或默认值 | `x ?? default` |
// | `.filter()` | 条件过滤 Some | `if (cond) x else null` |
//
// ⚠️ TS 没有这些是因为 null/undefined 没有"包装器"。
//    Rust 的 Option/Result 是真正的类型，编译器强制你处理。
//
// 详细对照 → rust_vs_typescript.rs §12
