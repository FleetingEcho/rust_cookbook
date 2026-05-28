// 运行命令：cargo run -p learning_notes --example rts_error_handling_advanced
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 自定义错误类
// class AppError extends Error {
//     constructor(
//         message: string,
//         public readonly code: string,
//         public readonly cause?: Error,
//     ) {
//         super(message);
//         this.name = "AppError";
//     }
// }
//
// // 多错误类型
// class NetworkError extends Error { ... }
// class ParseError extends Error { ... }
// class NotFoundError extends Error { ... }
//
// // 错误包装与转换
// try {
//     const data = JSON.parse(input);        // 可能抛出 SyntaxError
//     const result = await fetch(url);       // 可能抛出 NetworkError
// } catch (e) {
//     if (e instanceof SyntaxError) { ... }
//     if (e instanceof NetworkError) { ... }
//     throw new AppError("操作失败", "ERR_001", e as Error); // 包装
// }
//
// // 错误联合类型（更 TS 风格）
// type Result<T, E extends Error = Error> =
//     | { ok: true; value: T }
//     | { ok: false; error: E };
// ============================================================

use std::fmt;
use std::num::ParseIntError;

// ============================================================
// 一、自定义错误类型（枚举）
// TS: class AppError extends Error { ... }
// Rust 惯用法：用枚举定义所有可能的错误变体
// ============================================================
#[derive(Debug)]
enum AppError {
    // 每个变体携带相关信息
    NotFound { resource: String, id: u32 },
    ParseError(ParseIntError),            // 包装标准库错误
    Network { url: String, status: u16 },
    InvalidInput(String),
    // 组合多种原因（类似 TS 的 cause）
    Internal { message: String, source: Box<dyn std::error::Error> },
}

// 实现 Display（让错误可以被 println! 打印，类似 TS 的 error.message）
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound { resource, id } =>
                write!(f, "未找到 {resource}（id={id}）"),
            AppError::ParseError(e) =>
                write!(f, "解析错误: {e}"),
            AppError::Network { url, status } =>
                write!(f, "网络错误 {status}: {url}"),
            AppError::InvalidInput(msg) =>
                write!(f, "无效输入: {msg}"),
            AppError::Internal { message, source } =>
                write!(f, "内部错误: {message}（原因: {source}）"),
        }
    }
}

// 实现 std::error::Error（让 AppError 可以作为 dyn Error 使用）
impl std::error::Error for AppError {
    // source() 返回底层错误（类似 TS 的 error.cause）
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ParseError(e) => Some(e),
            AppError::Internal { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

// ============================================================
// 二、From trait：自动错误类型转换（? 运算符的基础）
// TS: 不需要，catch 块直接处理所有类型
// Rust: 不同函数返回不同错误类型时，需要 From 转换
// ============================================================
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)  // 自动把 ParseIntError 包装成 AppError
    }
}

// 有了 From<ParseIntError>，? 运算符可以自动转换：
// "abc".parse::<i32>()? 在返回 Result<_, AppError> 的函数里可以用
fn parse_user_id(s: &str) -> Result<u32, AppError> {
    let id = s.parse::<u32>()?;  // ParseIntError 自动转换为 AppError::ParseError
    if id == 0 {
        return Err(AppError::InvalidInput("ID 不能为 0".to_string()));
    }
    Ok(id)
}

// ============================================================
// 三、错误传播（? 运算符）
// TS: try { ... } catch (e) { throw new AppError(..., e); }
// ============================================================
fn find_user(id_str: &str) -> Result<String, AppError> {
    let id = parse_user_id(id_str)?;  // 失败则立即返回错误

    // 模拟数据库查询
    match id {
        1 => Ok(String::from("Alice")),
        2 => Ok(String::from("Bob")),
        _ => Err(AppError::NotFound { resource: "用户".to_string(), id }),
    }
}

fn greet_user(id_str: &str) -> Result<String, AppError> {
    let name = find_user(id_str)?;      // 链式传播
    Ok(format!("你好，{name}！"))
}

// ============================================================
// 四、多种错误类型的统一处理
// TS: catch (e: unknown) { if (e instanceof X) ... }
// Rust 方案1：用枚举包装所有错误（推荐，类型安全）
// Rust 方案2：Box<dyn Error>（快速但失去类型信息）
// ============================================================
fn flexible_parse(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    // Box<dyn Error> 可以容纳任何错误类型（TS: any / unknown）
    let n: i64 = s.trim().parse()?;
    Ok(n * 2)
}

// ============================================================
// 五、错误处理组合子（方法链）
// TS: 通过 .catch() 或 try/catch 处理
// ============================================================
fn demonstrate_combinators() {
    // map_err：转换错误类型
    // TS: .catch(e => new AppError(e.message))
    let result: Result<i32, String> = "42"
        .parse::<i32>()
        .map_err(|e| format!("解析失败: {e}"));
    println!("map_err: {:?}", result);

    // and_then：成功后继续操作
    // TS: .then(n => validate(n))
    let validated: Result<i32, String> = "10"
        .parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| {
            if n > 0 { Ok(n) } else { Err("必须是正数".to_string()) }
        });
    println!("and_then: {:?}", validated);

    // unwrap_or / unwrap_or_else / unwrap_or_default
    let val = "bad".parse::<i32>().unwrap_or(0);         // TS: || 0
    let val2 = "bad".parse::<i32>().unwrap_or_else(|e| {
        eprintln!("解析失败: {e}");
        -1
    });
    println!("unwrap_or: {val}, unwrap_or_else: {val2}");

    // ok() / err()：Result ↔ Option 互转
    let opt: Option<i32> = "42".parse::<i32>().ok();     // Ok → Some, Err → None
    println!("ok(): {:?}", opt);

    // flatten：Option<Option<T>> 或 Result<Result<T, E>, E> → 展平
    let nested: Result<Result<i32, &str>, &str> = Ok(Ok(42));
    let flat = nested.flatten();
    println!("flatten: {:?}", flat);
}

// ============================================================
// 六、在 main 中返回 Result（Rust 支持，TS 不需要）
// ============================================================
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let n: i32 = "42".parse()?;  // 如果失败，程序以错误信息退出
//     println!("{n}");
//     Ok(())
// }

fn main() {
    // 成功情况
    match greet_user("1") {
        Ok(msg)  => println!("{msg}"),
        Err(e)   => println!("错误: {e}"),
    }

    // 各种错误情况
    let test_cases = vec!["abc", "0", "99", "2"];
    for s in test_cases {
        match greet_user(s) {
            Ok(msg)  => println!("✅ {msg}"),
            Err(ref e) => {
                println!("❌ {e}");
                // 可以检查具体错误类型（TS: instanceof 检查）
                match e {
                    AppError::ParseError(_)   => println!("  → 是解析错误"),
                    AppError::InvalidInput(_) => println!("  → 是无效输入"),
                    AppError::NotFound { .. } => println!("  → 是未找到错误"),
                    _ => {}
                }
            }
        }
    }

    println!();
    demonstrate_combinators();

    // Box<dyn Error> 灵活处理
    match flexible_parse("  21  ") {
        Ok(n)  => println!("flexible: {n}"),
        Err(e) => println!("flexible 错误: {e}"),
    }

    // ============================================================
    // 七、生产代码推荐：thiserror 和 anyhow（说明）
    // ============================================================
    println!("\n=== 生产代码错误处理库 ===");
    println!("thiserror：自动生成 Display/From/Error 实现（减少样板代码）");
    println!("  用法: #[derive(thiserror::Error)]");
    println!("  适合：库代码，需要精确错误类型");
    println!();
    println!("anyhow：快速错误处理，用 anyhow::Result<T> 代替 Result<T, E>");
    println!("  用法: fn foo() -> anyhow::Result<()> {{ ... }}");
    println!("  适合：应用代码，不在乎具体错误类型，只需传播和打印");
    println!();
    println!("TS 对比：");
    println!("  thiserror ≈ 自定义 class extends Error（有类型）");
    println!("  anyhow    ≈ catch (e: unknown) throw e（不在乎类型）");
}
