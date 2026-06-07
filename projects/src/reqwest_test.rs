use anyhow::Result;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task::JoinSet;

// ── Hacker News API types ──

#[derive(Debug, Deserialize)]
struct Story {
    id: u64,
    title: Option<String>,
    url: Option<String>,
    #[serde(rename = "by")]
    author: Option<String>,
    score: Option<i64>,
    time: Option<u64>,
    #[serde(rename = "descendants")]
    comments: Option<i64>,
}

// 这里拆成 StoryItem 主要是为了演示：API 原始字段名 (by, descendants, type)
// 和 Rust 惯用名 (author, comments, r#type) 之间的映射。
// 实际可以合并在 Story 上用 #[serde(rename = "...")] 一步到位。
#[derive(Debug, Deserialize)]
struct StoryItem {
    id: u64,
    title: Option<String>,
    url: Option<String>,
    by: Option<String>,
    score: Option<i64>,
    time: Option<u64>,
    descendants: Option<i64>,
    r#type: Option<String>,
}

// ── helpers ──

fn fmt_time(unix_ts: u64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let diff = now.saturating_sub(unix_ts);
    let hours = diff / 3600;
    if hours > 48 {
        format!("{} days ago", hours / 24)
    } else {
        format!("{} hours ago", hours)
    }
}

// ── main ──

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let base = "https://hacker-news.firebaseio.com/v0";

    // 1. Fetch top story IDs
    println!("Fetching top stories from Hacker News...\n");
    let ids: Vec<u64> = client
        .get(format!("{base}/topstories.json"))
        .send()
        .await?
        .json()     // reqwest::json() -> serde_json 反序列化成 Vec<u64>
        .await?;

    println!("Got {} top story IDs\n", ids.len());

    // 2. Take the first 15 IDs via vector slice & collect
    let sample_ids: Vec<u64> = ids.iter().take(15).copied().collect();
    let mut join_set: JoinSet<Result<Story>> = JoinSet::new();
    for &id in &sample_ids {
        let cl = client.clone();
        join_set.spawn(async move { fetch_story(&cl, base, id).await });
    }

    let mut stories: Vec<Story> = Vec::new();
    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(Ok(story)) => stories.push(story),
            Ok(Err(e)) => eprintln!("fetch error: {e}"),
            Err(e) => eprintln!("task panicked: {e}"),
        }
    }

    // 4. Vector operation: sort by score (descending)
    stories.sort_by(|a, b| b.score.unwrap_or(0).cmp(&a.score.unwrap_or(0)));

    // 5. Vector operation: filter out stories without titles
    let valid: Vec<&Story> = stories.iter().filter(|s| s.title.is_some()).collect();

    // 6. Vector operation: map to formatted strings
    let lines: Vec<String> = valid
        .iter()
        .enumerate()
        .map(|(i, s)| {

            let title = s.title.as_deref().unwrap_or("(no title)");
            let author = s.author.as_deref().unwrap_or("anonymous");
            let score = s.score.unwrap_or(0);
            let comments = s.comments.unwrap_or(0);
            let ago = s.time.map(fmt_time).unwrap_or_default();

            format!(
                "#{:<2} [{:>4} pts | {:>3} comments] {} — by {} ({})",
                i + 1,
                score,
                comments,
                title,
                author,
                ago
            )
        })
        .collect();

    // 7. Print
    println!("Top stories (sorted by score):\n");
    for line in &lines {
        println!("{line}");
    }

    // 8. Vector stats: 用 map + sum 算总分
    //    .score 是 Option<i64>，.unwrap_or(0) 兜底
    let total_score: i64 = stories.iter().map(|s| s.score.unwrap_or(0)).sum();
    let avg_score = total_score as f64 / stories.len() as f64;
    println!("\n── Stats ──");
    println!("Total stories: {}", stories.len());
    println!("Total scores: {total_score}");
    println!("Average score: {avg_score:.1}");
    println!(
        "Stories with URLs: {}",
        stories.iter().filter(|s| s.url.is_some()).count()
    );

    Ok(())
}

async fn fetch_story(client: &reqwest::Client, base: &str, id: u64) -> Result<Story> {
    let item: StoryItem = client
        .get(format!("{base}/item/{id}.json"))
        .send()
        .await?
        .json()
        .await?;

    // 从 StoryItem 映射到 Story，应用字段重命名
    Ok(Story {
        id: item.id,
        title: item.title,
        url: item.url,
        author: item.by,          // by -> author
        score: item.score,
        time: item.time,
        comments: item.descendants, // descendants -> comments
    })
}
