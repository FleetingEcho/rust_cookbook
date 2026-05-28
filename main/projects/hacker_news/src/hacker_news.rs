use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::task::JoinSet;

const HACKER_NEWS_TOP_STORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";
const HACKER_NEWS_ITEM: &str = "https://hacker-news.firebaseio.com/v0/item/";

#[derive(Deserialize, Serialize, Debug)]
pub struct Story {
    pub id: u64,
    pub title: String,
    pub by: String,
    pub score: Option<u32>,
    pub url: Option<String>,
    pub descendants: Option<u32>, // comment count
    #[serde(rename = "type")]
    pub story_type: String,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}pts] {} (by {})\n  comments: {} | {}",
            self.score.unwrap_or(0),
            self.title,
            self.by,
            self.descendants.unwrap_or(0),
            self.url.as_deref().unwrap_or("(no url)"),
        )
    }
}

// JoinSet 版本：spawn 所有任务后统一 join，适合任务数量固定的场景
pub async fn fetch_stories_joinset(count: usize) -> Result<Vec<Story>> {
    let story_ids: Vec<u64> = reqwest::get(HACKER_NEWS_TOP_STORIES)
        .await?
        .json::<Vec<u64>>()
        .await?;

    let top_ids = story_ids.get(..count).unwrap_or(&story_ids);
    let mut tasks = JoinSet::new();

    for &id in top_ids {
        let url = format!("{HACKER_NEWS_ITEM}{id}.json");
        tasks.spawn(async move { reqwest::get(&url).await?.json::<Story>().await });
    }

    let mut stories = Vec::new();
    while let Some(result) = tasks.join_next().await {
        if let Ok(Ok(story)) = result {
            stories.push(story);
        }
    }
    Ok(stories)
}

// mpsc channel 版本：通过 channel 传递结果，适合生产者-消费者模式
// 区别：JoinSet 等所有任务完成后批量收集；mpsc 边生产边消费，内存压力更小
pub async fn fetch_stories_mpsc(count: usize) -> Result<Vec<Story>> {
    let story_ids: Vec<u64> = reqwest::get(HACKER_NEWS_TOP_STORIES)
        .await?
        .json::<Vec<u64>>()
        .await?;

    let top_ids = story_ids.get(..count).unwrap_or(&story_ids).to_vec();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Story>(count);

    for id in top_ids {
        let tx = tx.clone();
        let url = format!("{HACKER_NEWS_ITEM}{id}.json");
        tokio::spawn(async move {
            if let Ok(story) = reqwest::get(&url).await?.json::<Story>().await {
                let _ = tx.send(story).await;
            }
            Ok::<(), reqwest::Error>(())
        });
    }
    // 必须 drop 原始 sender，否则 rx.recv() 永远不会返回 None
    drop(tx);

    let mut stories = Vec::new();
    while let Some(story) = rx.recv().await {
        stories.push(story);
    }
    Ok(stories)
}
