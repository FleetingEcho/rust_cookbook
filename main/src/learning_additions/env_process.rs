// std::env  — 读取环境变量、命令行参数、当前目录等。
// std::process — 控制进程退出码、执行子进程。
//
// 这些是每个 CLI 工具或服务的必备基础。

use std::env;

// ── 环境变量 ──────────────────────────────────────────────────────────────────

pub fn read_env_vars() {
    // var()：读取单个环境变量。找不到或值不是有效 UTF-8 时返回 Err。
    match env::var("HOME") {
        Ok(home) => println!("HOME = {home}"),
        Err(e)   => println!("HOME 未设置: {e}"),
    }

    // var_os()：返回 OsString，不要求 UTF-8，适合路径类变量。
    if let Some(path) = env::var_os("PATH") {
        println!("PATH 前 50 字节: {:?}", &path.to_string_lossy()[..50.min(path.len())]);
    }

    // 读取自定义变量，缺失时用默认值（常见于配置）。
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("APP_PORT = {port}");

    // 遍历所有环境变量
    let count = env::vars().count();
    println!("当前环境变量数量: {count}");
}

// ── 命令行参数 ────────────────────────────────────────────────────────────────

pub fn read_args() {
    // args() 返回迭代器，第 0 个是程序名，第 1 个起是用户参数。
    let args: Vec<String> = env::args().collect();
    println!("参数数量（含程序名）: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  args[{i}] = {arg}");
    }

    // 跳过程序名，只看用户传的参数
    let user_args: Vec<String> = env::args().skip(1).collect();
    if user_args.is_empty() {
        println!("没有用户参数");
    } else {
        println!("用户参数: {user_args:?}");
    }
}

// ── 当前目录 ──────────────────────────────────────────────────────────────────

pub fn working_directory() {
    // 获取当前工作目录
    match env::current_dir() {
        Ok(path) => println!("当前目录: {}", path.display()),
        Err(e)   => println!("无法获取当前目录: {e}"),
    }

    // 获取可执行文件本身的路径
    match env::current_exe() {
        Ok(exe) => println!("可执行文件: {}", exe.display()),
        Err(e)  => println!("无法获取可执行文件路径: {e}"),
    }
}

// ── process::exit ─────────────────────────────────────────────────────────────

// exit(0)  — 成功退出（Unix/Linux 约定）
// exit(1)  — 一般错误
// exit(非零) — 其他错误码，Shell 可以用 $? 读取
//
// 注意：exit() 会立即终止进程，析构函数（Drop）不会运行。
//       如果需要清理资源，应该让 main 正常返回，或者在调用 exit 前手动清理。

pub fn show_exit_codes() {
    println!("退出码示例（实际不会真的退出，只是说明）：");
    println!("  std::process::exit(0)  → 成功");
    println!("  std::process::exit(1)  → 一般错误");
    println!("  std::process::exit(2)  → 参数错误（Unix 惯例）");
    // 如果你想真的退出，取消下面的注释：
    // std::process::exit(0);
}

// ── 实际用法：CLI 入口模式 ────────────────────────────────────────────────────
//
// 常见的 CLI 程序结构：
//
//   fn main() {
//       let args: Vec<String> = env::args().collect();
//       let config = Config::from_args(&args).unwrap_or_else(|e| {
//           eprintln!("错误: {e}");
//           process::exit(1);    // 参数错误，退出码 1
//       });
//
//       if let Err(e) = run(config) {
//           eprintln!("运行失败: {e}");
//           process::exit(2);    // 运行时错误，退出码 2
//       }
//   }
//
// 这个模式让 run() 只负责业务逻辑，main() 负责错误展示和退出码。

// ── 设置 / 删除环境变量（测试中常用）────────────────────────────────────────

pub fn mutate_env_demo() {
    // 设置一个变量（只影响当前进程）
    unsafe { env::set_var("MY_TEST_VAR", "hello") };
    println!("设置后: {:?}", env::var("MY_TEST_VAR")); // Ok("hello")

    // 删除变量
    unsafe { env::remove_var("MY_TEST_VAR") };
    println!("删除后: {:?}", env::var("MY_TEST_VAR")); // Err(...)
    // 注意：set_var / remove_var 在多线程环境下不安全，因此标记为 unsafe（Rust 1.81+）
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_var_missing_returns_err() {
        let result = env::var("THIS_VAR_DEFINITELY_DOES_NOT_EXIST_XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn env_var_with_default() {
        let val = env::var("THIS_VAR_DEFINITELY_DOES_NOT_EXIST_XYZ")
            .unwrap_or_else(|_| "default".to_string());
        assert_eq!(val, "default");
    }

    #[test]
    fn args_has_at_least_program_name() {
        // 测试环境里 args()[0] 是测试可执行文件的路径
        let args: Vec<String> = env::args().collect();
        assert!(!args.is_empty());
    }

    #[test]
    fn current_dir_exists() {
        let dir = env::current_dir();
        assert!(dir.is_ok());
    }

    // 注意：避免在测试里真正调用 process::exit()，
    // 那会直接终止整个测试进程。
    #[test]
    fn exit_code_is_just_a_number() {
        // 只验证类型，不真的退出
        let _code: i32 = 0;
        let _ = std::process::ExitCode::SUCCESS;
        let _ = std::process::ExitCode::FAILURE;
    }
}
