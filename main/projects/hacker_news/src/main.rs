mod hacker_news;
use clap::{Parser, ValueEnum};
use hacker_news::{fetch_stories_joinset, fetch_stories_mpsc};
use serde_json;
use tokio::fs;

#[derive(Parser)]
#[command(name = "hacker_news", about = "Fetch top Hacker News stories")]
struct Args {
    /// Number of stories to fetch
    #[arg(short, long, default_value_t = 10)]
    count: usize,

    /// Output JSON file path
    #[arg(short, long, default_value = "data.json")]
    output: String,

    /// Concurrency model to use
    #[arg(short, long, default_value = "joinset")]
    mode: Mode,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    /// JoinSet: spawn all tasks then collect results
    Joinset,
    /// mpsc channel: producer-consumer pattern
    Mpsc,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!(
        "Fetching top {} stories (mode: {:?})...",
        args.count,
        match args.mode {
            Mode::Joinset => "joinset",
            Mode::Mpsc => "mpsc",
        }
    );

    let result = match args.mode {
        Mode::Joinset => fetch_stories_joinset(args.count).await,
        Mode::Mpsc => fetch_stories_mpsc(args.count).await,
    };

    match result {
        Ok(stories) => {
            for story in &stories {
                println!("{story}\n");
            }

            let json = serde_json::to_string_pretty(&stories).expect("Failed to serialize JSON");
            if let Err(e) = fs::write(&args.output, json).await {
                eprintln!("Failed to save {}: {:?}", args.output, e);
            } else {
                println!("Saved {} stories to {}", stories.len(), args.output);
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
