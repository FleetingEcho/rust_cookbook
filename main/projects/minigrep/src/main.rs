use clap::Parser;
use minigrep::{Config, run};
use std::process;

#[derive(Parser)]
#[command(name = "minigrep", about = "搜索文件或目录中的文本，支持彩色高亮和递归搜索")]
struct Args {
    /// 搜索关键词
    query: String,

    /// 文件路径或目录路径
    path: String,

    /// 忽略大小写
    #[arg(short, long)]
    ignore_case: bool,

    /// 禁用彩色输出（适合管道使用）
    #[arg(long)]
    no_color: bool,
}

/*
使用示例：
  cargo run -- "fn" src/lib.rs
  cargo run -- "async" src/ --ignore-case
  cargo run -- "fn" src/ --no-color
  IGNORE_CASE=1 cargo run -- "rust" src/

  cargo test
*/

fn main() {
    let args = Args::parse();

    let mut config = Config {
        query: args.query,
        path: args.path,
        ignore_case: args.ignore_case,
        no_color: args.no_color,
    };

    // 环境变量优先于 --ignore-case flag
    if std::env::var("IGNORE_CASE").is_ok() {
        config.ignore_case = true;
    }

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
