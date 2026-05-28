// 运行命令：cargo run -p learning_notes --example rts_option_result
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // Option<T> 对应 TS 的 T | null | undefined
// function findUser(id: number): User | null {
//     return db.find(u => u.id === id) ?? null;
// }
// const user = findUser(1);
// if (user !== null) {
//     console.log(user.name);  // 类型收窄
// }
// const name = user?.name ?? "游客";     // 可选链 + 空值合并
// const upper = user?.name.toUpperCase(); // 可选链
//
// // Result<T, E> 对应 TS 的 try/catch 或 返回 Error
// function parseAge(s: string): number {
//     const n = parseInt(s);
//     if (isNaN(n)) throw new Error(`无效数字: ${s}`);
//     return n;
// }
// try {
//     const age = parseAge("abc");
// } catch (e) {
//     console.error((e as Error).message);
// }
//
// // 链式可能失败的操作
// async function getCity(userId: number): Promise<string | null> {
//     const user = await findUser(userId);
//     const address = user?.address;
//     return address?.city ?? null;
// }
// ============================================================

// 自定义错误类型（TS: class AppError extends Error）
#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(String),
    InvalidInput(String),
}

// 实现 Display，让错误可以打印（TS: error.message）
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound(s)      => write!(f, "未找到: {s}"),
            AppError::ParseError(s)    => write!(f, "解析错误: {s}"),
            AppError::InvalidInput(s)  => write!(f, "无效输入: {s}"),
        }
    }
}

fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        _ => None,   // TS: return null
    }
}

fn parse_age(s: &str) -> Result<u32, AppError> {
    s.parse::<u32>()
        .map_err(|_| AppError::ParseError(format!("'{s}' 不是有效年龄")))
}

fn validate_age(age: u32) -> Result<u32, AppError> {
    if age > 150 {
        Err(AppError::InvalidInput(format!("年龄 {age} 不合理")))
    } else {
        Ok(age)
    }
}

// ? 运算符：自动传播错误，相当于 TS 的 throw/rethrow
// TS 需要 try { const age = parseAge(s); validateAge(age); } catch(e) { throw e; }
fn parse_and_validate(s: &str) -> Result<u32, AppError> {
    let age = parse_age(s)?;       // 如果 Err，直接 return Err(...)
    let valid = validate_age(age)?;
    Ok(valid)
}

fn main() {
    // ============================================================
    // 一、Option<T> 基础
    // TS 对应：T | null | undefined
    // ============================================================
    println!("=== Option<T> ===");

    // Some / None 是 Option 枚举的两个变体
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    println!("Some: {:?}, None: {:?}", some_val, none_val);

    // --- match：最完整的处理方式 ---
    // TS: if (user !== null) { ... } else { ... }
    match find_user(1) {
        Some(name) => println!("找到用户: {name}"),
        None       => println!("用户不存在"),
    }

    // --- if let：只关心 Some 的情况 ---
    // TS: if (user !== null) { console.log(user.name) }
    if let Some(name) = find_user(2) {
        println!("if let: {name}");
    }

    // --- unwrap_or：提供默认值 ---
    // TS: findUser(99) ?? "游客"
    let name = find_user(99).unwrap_or(String::from("游客"));
    println!("unwrap_or: {name}");

    // --- unwrap_or_else：懒惰求值，适合默认值计算开销较大的情况 ---
    // TS: findUser(99) ?? expensiveCompute()
    let name2 = find_user(99).unwrap_or_else(|| String::from("计算出的默认值"));
    println!("unwrap_or_else: {name2}");

    // --- unwrap：直接取值，None 时会 panic（不安全，生产代码慎用）---
    // TS: user! （非空断言，同样不安全）
    let name3 = find_user(1).unwrap();
    println!("unwrap: {name3}");

    // --- map：对 Some 内的值做变换，None 直接穿透 ---
    // TS: user?.name.toUpperCase()
    let upper = find_user(1).map(|n| n.to_uppercase());
    println!("map: {:?}", upper);  // Some("ALICE")

    let upper_none = find_user(99).map(|n| n.to_uppercase());
    println!("map None: {:?}", upper_none);  // None

    // --- and_then：链式 Option 操作（可选链）---
    // TS: user?.address?.city
    fn get_address(name: &str) -> Option<String> {
        if name == "Alice" { Some(String::from("北京")) } else { None }
    }
    let city = find_user(1).and_then(|name| get_address(&name));
    println!("and_then: {:?}", city);   // Some("北京")，对应 TS: user?.address

    // --- filter：对 Some 的值加条件，不满足变为 None ---
    // TS: user !== null && user.age > 18 ? user : null
    let long_name = find_user(1).filter(|n| n.len() > 3);
    println!("filter: {:?}", long_name);

    // --- is_some / is_none ---
    println!("is_some: {}", find_user(1).is_some());   // TS: user !== null
    println!("is_none: {}", find_user(99).is_none());  // TS: user === null

    // --- ok_or：Option → Result ---
    let result: Result<String, AppError> = find_user(99)
        .ok_or(AppError::NotFound("id=99".to_string()));
    println!("ok_or: {:?}", result);

    // ============================================================
    // 二、Result<T, E> 基础
    // TS 对应：try/catch 或 T | Error
    // ============================================================
    println!("\n=== Result<T, E> ===");

    // Ok / Err 是 Result 枚举的两个变体
    let ok_val: Result<i32, String> = Ok(42);
    let err_val: Result<i32, String> = Err(String::from("出错了"));
    println!("Ok: {:?}, Err: {:?}", ok_val, err_val);

    // --- match ---
    // TS: try { ... } catch (e) { ... }
    match parse_age("25") {
        Ok(age)  => println!("解析成功: {age}"),
        Err(e)   => println!("解析失败: {e}"),
    }

    match parse_age("abc") {
        Ok(age)  => println!("解析成功: {age}"),
        Err(e)   => println!("解析失败: {e}"),
    }

    // --- if let ---
    if let Ok(age) = parse_age("30") {
        println!("if let Ok: {age}");
    }

    // --- unwrap_or ---
    // TS: try { parseAge(s) } catch { 0 }
    let age = parse_age("bad").unwrap_or(0);
    println!("unwrap_or: {age}");

    // --- map：变换 Ok 内的值 ---
    // TS: parseAge(s).then(n => n * 2) （Promise 风格类比）
    let doubled = parse_age("21").map(|n| n * 2);
    println!("map Ok: {:?}", doubled);

    // --- map_err：变换 Err 内的错误 ---
    let result: Result<i32, String> = "42".parse::<i32>()
        .map_err(|e| format!("解析失败: {e}"));
    println!("map_err: {:?}", result);

    // --- and_then：链式 Result ---
    // TS: parseAge(s).then(validateAge)（Promise 链）
    let validated = parse_age("25").and_then(|age| validate_age(age));
    println!("and_then: {:?}", validated);

    // --- ? 运算符（只能在返回 Result/Option 的函数中使用）---
    println!("? 传播: {:?}", parse_and_validate("25"));
    println!("? 传播: {:?}", parse_and_validate("200"));
    println!("? 传播: {:?}", parse_and_validate("abc"));

    // --- ok()：Result → Option（忽略错误）---
    let opt: Option<u32> = parse_age("18").ok();
    println!("ok(): {:?}", opt);

    // --- is_ok / is_err ---
    println!("is_ok: {}", parse_age("42").is_ok());
    println!("is_err: {}", parse_age("bad").is_err());

    // ============================================================
    // 三、批量处理（TS 需要手动 try/catch 循环）
    // ============================================================
    println!("\n=== 批量处理 ===");

    let inputs = vec!["10", "abc", "20", "bad", "30"];

    // 收集所有结果（包含 Ok 和 Err）
    let results: Vec<Result<u32, AppError>> = inputs.iter()
        .map(|s| parse_age(s))
        .collect();
    for r in &results {
        println!("  {:?}", r);
    }

    // 只收集成功的值（忽略错误）
    // TS: inputs.map(parseInt).filter(n => !isNaN(n))
    let successes: Vec<u32> = inputs.iter()
        .filter_map(|s| parse_age(s).ok())
        .collect();
    println!("只取成功: {:?}", successes);

    // 只要有一个失败就整体失败（collect::<Result<Vec<_>, _>>()）
    let all_valid: Result<Vec<u32>, AppError> = vec!["10", "20", "30"]
        .iter()
        .map(|s| parse_age(s))
        .collect();
    println!("全部成功: {:?}", all_valid);

    let any_invalid: Result<Vec<u32>, AppError> = vec!["10", "bad", "30"]
        .iter()
        .map(|s| parse_age(s))
        .collect();
    println!("有失败则整体失败: {:?}", any_invalid);
}
