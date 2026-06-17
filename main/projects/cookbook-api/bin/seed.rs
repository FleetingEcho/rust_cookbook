use anyhow::{Context, Result};
use regex::Regex;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

// ── Data structures ───────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct RecipeData {
    name: String,
    description: Option<String>,
    category: String,
    difficulty: Option<i32>,
    calories: Option<f64>,
    cover_image: Option<String>,
    source: String,
    source_path: String,
    ingredients: Vec<IngredientData>,
    steps: Vec<StepData>,
    nutrition: Option<NutritionData>,
    tags: Vec<String>,
}

#[derive(Debug, Default)]
struct IngredientData {
    name: String,
    amount: Option<String>,
    unit: Option<String>,
    note: Option<String>,
}

#[derive(Debug, Default)]
struct StepData {
    order: i32,
    content: Option<String>,
    image_url: Option<String>,
}

#[derive(Debug, Default)]
struct NutritionData {
    protein_g: Option<f64>,
    fat_g: Option<f64>,
    carbs_g: Option<f64>,
    sodium_mg: Option<f64>,
}

// ── Parsing helpers ───────────────────────────────────────────────────────────

/// Resolve markdown links `[name](url)` → `name`
fn resolve_links(text: &str) -> String {
    let re = Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap();
    re.replace_all(text, "$1").to_string()
}

/// Split `text（note）` into `("text", Some("note"))`.
/// Handles both fullwidth `（）` and ASCII `()`.
fn split_note(text: &str) -> (String, Option<String>) {
    let open = text.find(|c| c == '（' || c == '(');
    let Some(open_pos) = open else {
        return (text.trim().to_string(), None);
    };
    let before = text[..open_pos].trim().to_string();
    let rest = &text[open_pos..];
    let open_char_len = rest.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    let inner_start = open_pos + open_char_len;
    let close = text[inner_start..].find(|c| c == '）' || c == ')');
    match close {
        Some(rel) => {
            let note = text[inner_start..inner_start + rel].trim().to_string();
            (before, if note.is_empty() { None } else { Some(note) })
        }
        None => (before, None),
    }
}

/// Parse a `- ingredient [amount] （note）` list line into IngredientData.
fn parse_ingredient_line(line: &str) -> Option<IngredientData> {
    let text = line
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim();

    if text.is_empty() || text.starts_with('#') {
        return None;
    }
    // Skip non-ingredient lines
    if text.starts_with("每份") || text.starts_with("每人") || text.starts_with("以下") {
        return None;
    }

    // Resolve internal links first
    let text = resolve_links(text);
    let text = text.trim();

    // Extract parenthetical note
    let (body, note) = split_note(text);
    let body = body.trim();

    // Split on first whitespace: name | amount
    let (name, amount) = match body.find(char::is_whitespace) {
        Some(pos) => {
            let n = body[..pos].trim().to_string();
            let a = body[pos..].trim().to_string();
            if a.is_empty() {
                (body.to_string(), None)
            } else {
                (n, Some(a))
            }
        }
        None => (body.to_string(), None),
    };

    if name.is_empty() {
        return None;
    }

    let unit = amount.as_deref().and_then(extract_unit).map(str::to_string);

    Some(IngredientData { name, amount, unit, note })
}

fn extract_unit(amount: &str) -> Option<&str> {
    // Longer patterns first to avoid partial matches (e.g. "毫升" before "升")
    const UNITS: &[&str] = &[
        "毫升", "ml", "mL", "千克", "kg", "汤匙", "茶匙",
        "升", "L", "克", "g", "个", "条", "根", "片",
        "块", "颗", "勺", "杯", "碗", "把", "束", "朵",
        "瓣", "头", "只", "包", "袋",
    ];
    UNITS.iter().find(|&&u| amount.contains(u)).copied()
}

/// Auto-generate tags from recipe metadata.
fn generate_tags(recipe: &RecipeData) -> Vec<String> {
    let mut tags = vec![recipe.category.clone()];

    let text = format!(
        "{} {}",
        recipe.name,
        recipe.description.as_deref().unwrap_or("")
    );

    const METHODS: &[&str] = &[
        "红烧", "清蒸", "爆炒", "凉拌", "油炸", "炖", "卤",
        "烤", "炸", "煎", "焖", "溜", "熘",
    ];
    for m in METHODS {
        if recipe.name.contains(m) {
            tags.push(m.to_string());
        }
    }

    if text.contains("麻辣") {
        tags.push("麻辣".to_string());
    } else if text.contains("辣") && !text.contains("不辣") {
        tags.push("辣".to_string());
    }

    if let Some(cal) = recipe.calories {
        if cal <= 300.0 {
            tags.push("低卡".to_string());
        } else if cal >= 600.0 {
            tags.push("高热量".to_string());
        }
    }

    match recipe.difficulty {
        Some(1) => tags.push("新手友好".to_string()),
        Some(5) => tags.push("高难度".to_string()),
        _ => {}
    }

    tags.sort();
    tags.dedup();
    tags
}

// ── HowToCook parser ──────────────────────────────────────────────────────────

const HTC_CATEGORIES: &[(&str, &str)] = &[
    ("aquatic", "水产"),
    ("breakfast", "早餐"),
    ("condiment", "调味料"),
    ("dessert", "甜点"),
    ("drink", "饮品"),
    ("meat_dish", "荤菜"),
    ("semi-finished", "半成品"),
    ("soup", "汤"),
    ("staple", "主食"),
    ("vegetable_dish", "素菜"),
];

fn htc_category(dir: &str) -> &str {
    HTC_CATEGORIES
        .iter()
        .find(|(k, _)| *k == dir)
        .map(|(_, v)| *v)
        .unwrap_or("其他")
}

fn parse_htc_file(path: &Path, repo_root: &Path) -> Option<RecipeData> {
    let content = std::fs::read_to_string(path).ok()?;
    let rel = path.strip_prefix(repo_root).ok()?;
    let rel_str = rel.to_string_lossy().replace('\\', "/");

    // Category: second component of dishes/{category}/…
    let parts: Vec<&str> = rel_str.splitn(3, '/').collect();
    if parts.len() < 2 {
        return None;
    }
    let category = htc_category(parts[1]).to_string();

    // Directory containing recipe file (for resolving relative image paths)
    let recipe_dir = path
        .parent()
        .and_then(|p| p.strip_prefix(repo_root).ok())
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();

    const RAW_BASE: &str =
        "https://raw.githubusercontent.com/Anduin2017/HowToCook/master";

    let re_diff =
        Regex::new(r"预估烹饪难度：(★+)").unwrap();
    let re_cal =
        Regex::new(r"预估卡路里：(\d+(?:\.\d+)?)\s*大卡").unwrap();
    let re_step =
        Regex::new(r"^\d+\.\s*(.+)").unwrap();
    let re_img =
        Regex::new(r"!\[[^\]]*\]\(\./([^)]+)\)").unwrap();

    #[derive(PartialEq)]
    enum Sec { Header, Ingredients, Amounts, Steps, Other }

    let mut section = Sec::Header;
    let mut recipe = RecipeData {
        source: "HowToCook".to_string(),
        source_path: rel_str,
        category,
        ..Default::default()
    };

    let mut desc_lines: Vec<String> = Vec::new();
    let mut in_tool_subsec = false;
    let mut tool_ings: Vec<IngredientData> = Vec::new();
    let mut amt_ings: Vec<IngredientData> = Vec::new();
    let mut step_order = 0i32;

    for line in content.lines() {
        let t = line.trim();

        // H2 section switch
        if let Some(h) = t.strip_prefix("## ") {
            in_tool_subsec = false;
            section = match h.trim() {
                "必备原料和工具" => Sec::Ingredients,
                "计算"         => Sec::Amounts,
                "操作"         => Sec::Steps,
                _              => Sec::Other,
            };
            continue;
        }
        // H3 sub-section
        if let Some(h) = t.strip_prefix("### ") {
            in_tool_subsec = h.contains("工具");
            continue;
        }

        match section {
            Sec::Header => {
                if t.starts_with("# ") && recipe.name.is_empty() {
                    let raw = t[2..].trim();
                    recipe.name = raw.trim_end_matches("的做法").trim().to_string();
                    continue;
                }
                if let Some(cap) = re_diff.captures(t) {
                    recipe.difficulty =
                        Some(cap[1].chars().filter(|&c| c == '★').count() as i32);
                    continue;
                }
                if let Some(cap) = re_cal.captures(t) {
                    recipe.calories = cap[1].parse().ok();
                    continue;
                }
                if !t.is_empty() && !t.starts_with('#') && !t.starts_with("预估") {
                    desc_lines.push(t.to_string());
                }
            }
            Sec::Ingredients => {
                if !in_tool_subsec && t.starts_with("- ") {
                    if let Some(ing) = parse_ingredient_line(t) {
                        tool_ings.push(ing);
                    }
                }
            }
            Sec::Amounts => {
                if t.starts_with("- ") {
                    if let Some(ing) = parse_ingredient_line(t) {
                        amt_ings.push(ing);
                    }
                }
            }
            Sec::Steps => {
                if let Some(cap) = re_step.captures(t) {
                    step_order += 1;
                    let body = cap[1].trim();
                    if let Some(img_cap) = re_img.captures(body) {
                        let url =
                            format!("{}/{}/{}", RAW_BASE, recipe_dir, &img_cap[1]);
                        recipe.steps.push(StepData {
                            order: step_order,
                            content: None,
                            image_url: Some(url),
                        });
                    } else {
                        // Strip any inline image from the text
                        let clean = re_img.replace_all(body, "").trim().to_string();
                        recipe.steps.push(StepData {
                            order: step_order,
                            content: if clean.is_empty() { None } else { Some(clean) },
                            image_url: None,
                        });
                    }
                }
            }
            Sec::Other => {}
        }
    }

    // Merge: amt_ings is base; add tool_ings entries not already present,
    // and copy notes from tool_ings into matching amt_ings.
    for ti in tool_ings {
        match amt_ings.iter_mut().find(|a| a.name == ti.name) {
            Some(ai) if ti.note.is_some() && ai.note.is_none() => {
                ai.note = ti.note;
            }
            Some(_) => {}
            None => amt_ings.push(ti),
        }
    }
    recipe.ingredients = amt_ings;

    if !desc_lines.is_empty() {
        recipe.description = Some(desc_lines.join(" "));
    }

    recipe.tags = generate_tags(&recipe);

    if recipe.name.is_empty() {
        return None;
    }
    Some(recipe)
}

fn parse_howtocook(repo_root: &Path) -> Vec<RecipeData> {
    let dishes = repo_root.join("dishes");
    let mut recipes = Vec::new();

    for entry in walkdir::WalkDir::new(&dishes)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            p.extension().and_then(|s| s.to_str()) == Some("md")
                && !p
                    .file_name()
                    .map(|n| n.to_string_lossy().to_uppercase().starts_with("README"))
                    .unwrap_or(false)
                && !p.to_string_lossy().contains("template")
        })
    {
        match parse_htc_file(entry.path(), repo_root) {
            Some(r) => recipes.push(r),
            None => warn!("skip (parse failed): {:?}", entry.path()),
        }
    }

    info!("HowToCook: parsed {} recipes", recipes.len());
    recipes
}

// ── CookLikeHOC parser ────────────────────────────────────────────────────────

/// Normalize CookLikeHOC H2 section headers to canonical tags.
fn clhoc_section(header: &str) -> &'static str {
    let h = header.trim().trim_end_matches(['：', ':', ' ', '\t']);
    match h {
        "配料" | "原料" | "已知成分" => "ingredients",
        "步骤"                       => "steps",
        "营养成分"                   => "nutrition",
        _                            => "other",
    }
}

fn parse_clhoc_file(path: &Path, repo_root: &Path) -> Option<RecipeData> {
    let content = std::fs::read_to_string(path).ok()?;
    let rel = path.strip_prefix(repo_root).ok()?;
    let rel_str = rel.to_string_lossy().replace('\\', "/");

    // Category from parent directory name
    let category = path
        .parent()?
        .file_name()?
        .to_string_lossy()
        .to_string();

    const RAW_BASE: &str =
        "https://raw.githubusercontent.com/Gar-b-age/CookLikeHOC/main";

    let re_cover = Regex::new(r"!\[[^\]]*\]\(\.\./images/([^)]+)\)").unwrap();
    let re_step  = Regex::new(r"^-\s*\d+[.、。]\s*(.+)").unwrap();
    let re_nutr  = Regex::new(r"^\|\s*(.+?)\s*\|\s*([\d.]+)\s*\w*\s*\|").unwrap();

    #[derive(PartialEq)]
    enum Sec { Header, Ingredients, Steps, Nutrition, Other }

    let mut section = Sec::Header;
    let mut recipe = RecipeData {
        source: "CookLikeHOC".to_string(),
        source_path: rel_str,
        category,
        ..Default::default()
    };

    let mut nutrition = NutritionData::default();
    let mut has_nutrition = false;
    let mut step_order = 0i32;

    for line in content.lines() {
        let t = line.trim();

        if let Some(h) = t.strip_prefix("## ") {
            section = match clhoc_section(h) {
                "ingredients" => Sec::Ingredients,
                "steps"       => Sec::Steps,
                "nutrition"   => { has_nutrition = true; Sec::Nutrition }
                _             => Sec::Other,
            };
            continue;
        }

        match section {
            Sec::Header => {
                if t.starts_with("# ") && recipe.name.is_empty() {
                    recipe.name = resolve_links(t[2..].trim()).trim().to_string();
                    continue;
                }
                if recipe.cover_image.is_none() {
                    if let Some(cap) = re_cover.captures(t) {
                        recipe.cover_image =
                            Some(format!("{}/images/{}", RAW_BASE, &cap[1]));
                    }
                }
            }
            Sec::Ingredients => {
                if t.starts_with("- ") {
                    if let Some(ing) = parse_ingredient_line(t) {
                        recipe.ingredients.push(ing);
                    }
                }
            }
            Sec::Steps => {
                if let Some(cap) = re_step.captures(t) {
                    step_order += 1;
                    recipe.steps.push(StepData {
                        order: step_order,
                        content: Some(cap[1].trim().to_string()),
                        image_url: None,
                    });
                } else if !t.is_empty() && !t.starts_with('#') {
                    // Continuation / sub-step: append to last step
                    let body = t.trim_start_matches("- ").trim();
                    if !body.is_empty() {
                        if let Some(last) = recipe.steps.last_mut() {
                            if let Some(ref mut c) = last.content {
                                c.push(' ');
                                c.push_str(body);
                            }
                        }
                    }
                }
            }
            Sec::Nutrition => {
                if let Some(cap) = re_nutr.captures(t) {
                    let field = cap[1].trim();
                    let val: f64 = cap[2].parse().unwrap_or(0.0);
                    match field {
                        "蛋白质"     => nutrition.protein_g  = Some(val),
                        "脂肪"       => nutrition.fat_g      = Some(val),
                        "碳水化合物" => nutrition.carbs_g    = Some(val),
                        "钠"         => nutrition.sodium_mg  = Some(val),
                        _ => {}
                    }
                }
            }
            Sec::Other => {}
        }
    }

    if has_nutrition {
        recipe.nutrition = Some(nutrition);
    }
    recipe.tags = generate_tags(&recipe);

    if recipe.name.is_empty() {
        return None;
    }
    Some(recipe)
}

fn parse_cooklhoc(repo_root: &Path) -> Vec<RecipeData> {
    const SKIP: &[&str] = &["docker_support", "docs", "images", ".git"];

    let mut recipes = Vec::new();

    for entry in walkdir::WalkDir::new(repo_root)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                return false;
            }
            if p.file_name()
                .map(|n| n.to_string_lossy().to_uppercase().starts_with("README"))
                .unwrap_or(false)
            {
                return false;
            }
            // Skip files whose parent dir is in SKIP list
            let parent = p
                .parent()
                .and_then(|pr| pr.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            !SKIP.contains(&parent.as_str())
        })
    {
        match parse_clhoc_file(entry.path(), repo_root) {
            Some(r) => recipes.push(r),
            None => warn!("skip (parse failed): {:?}", entry.path()),
        }
    }

    info!("CookLikeHOC: parsed {} recipes", recipes.len());
    recipes
}

// ── DB operations ─────────────────────────────────────────────────────────────

/// Returns `true` if the recipe was newly inserted, `false` if it already existed.
async fn insert_recipe(pool: &SqlitePool, r: &RecipeData) -> Result<bool> {
    // Idempotency check
    let exists: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM recipes WHERE source = ? AND source_path = ?",
    )
    .bind(&r.source)
    .bind(&r.source_path)
    .fetch_optional(pool)
    .await?;

    if exists.is_some() {
        return Ok(false);
    }

    // Insert recipe row
    let res = sqlx::query(
        "INSERT INTO recipes (name, description, category, difficulty, calories, cover_image, source, source_path)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&r.name)
    .bind(&r.description)
    .bind(&r.category)
    .bind(r.difficulty)
    .bind(r.calories)
    .bind(&r.cover_image)
    .bind(&r.source)
    .bind(&r.source_path)
    .execute(pool)
    .await
    .context("insert recipe")?;

    let recipe_id = res.last_insert_rowid();

    // Ingredients
    for ing in &r.ingredients {
        sqlx::query(
            "INSERT INTO ingredients (recipe_id, name, amount, unit, note) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(recipe_id)
        .bind(&ing.name)
        .bind(&ing.amount)
        .bind(&ing.unit)
        .bind(&ing.note)
        .execute(pool)
        .await?;
    }

    // Steps
    for step in &r.steps {
        sqlx::query(
            "INSERT INTO steps (recipe_id, step_order, content, image_url) VALUES (?, ?, ?, ?)",
        )
        .bind(recipe_id)
        .bind(step.order)
        .bind(&step.content)
        .bind(&step.image_url)
        .execute(pool)
        .await?;
    }

    // Nutrition
    if let Some(n) = &r.nutrition {
        sqlx::query(
            "INSERT INTO nutrition (recipe_id, protein_g, fat_g, carbs_g, sodium_mg) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(recipe_id)
        .bind(n.protein_g)
        .bind(n.fat_g)
        .bind(n.carbs_g)
        .bind(n.sodium_mg)
        .execute(pool)
        .await?;
    }

    // Tags
    for tag in &r.tags {
        sqlx::query("INSERT INTO tags (recipe_id, tag) VALUES (?, ?)")
            .bind(recipe_id)
            .bind(tag)
            .execute(pool)
            .await?;
    }

    Ok(true)
}

async fn rebuild_fts(pool: &SqlitePool) -> Result<()> {
    sqlx::query("DELETE FROM recipes_fts").execute(pool).await?;
    sqlx::query(
        "INSERT INTO recipes_fts (recipe_id, name, description, ingredients_text)
         SELECT r.id,
                r.name,
                COALESCE(r.description, ''),
                COALESCE(
                    (SELECT GROUP_CONCAT(i.name, ' ')
                     FROM ingredients i WHERE i.recipe_id = r.id),
                    ''
                )
         FROM recipes r",
    )
    .execute(pool)
    .await?;
    Ok(())
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();

    // Always resolve paths relative to the project root
    let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(&project_root).context("set working dir")?;

    dotenvy::dotenv().ok();

    let data_dir = project_root.join("data");
    std::fs::create_dir_all(&data_dir).ok();

    let db_path = data_dir.join("cookbook.db");
    let db_url = format!("sqlite:{}", db_path.display());

    let connect_opts = SqliteConnectOptions::from_str(&db_url)
        .context("parse db url")?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connect_opts)
        .await
        .context("connect to SQLite")?;

    sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("run migrations")?;
    info!("migrations applied");

    let htc_root  = PathBuf::from("data/HowToCook");
    let clhoc_root = PathBuf::from("data/CookLikeHOC");

    if !htc_root.exists() {
        anyhow::bail!(
            "data/HowToCook not found – run:\n  git clone --depth 1 https://github.com/Anduin2017/HowToCook data/HowToCook"
        );
    }
    if !clhoc_root.exists() {
        anyhow::bail!(
            "data/CookLikeHOC not found – run:\n  git clone --depth 1 https://github.com/Gar-b-age/CookLikeHOC data/CookLikeHOC"
        );
    }

    let all_recipes: Vec<RecipeData> = parse_howtocook(&htc_root)
        .into_iter()
        .chain(parse_cooklhoc(&clhoc_root))
        .collect();

    info!("total recipes to process: {}", all_recipes.len());

    let (mut inserted, mut skipped, mut failed) = (0usize, 0usize, 0usize);

    for recipe in &all_recipes {
        match insert_recipe(&pool, recipe).await {
            Ok(true)  => inserted += 1,
            Ok(false) => skipped += 1,
            Err(e) => {
                warn!("failed '{}': {:#}", recipe.name, e);
                failed += 1;
            }
        }
    }

    info!("rebuilding FTS index…");
    rebuild_fts(&pool).await.context("rebuild FTS")?;

    // Summary stats
    let total_recipes: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM recipes").fetch_one(&pool).await?;
    let total_ingredients: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ingredients").fetch_one(&pool).await?;
    let total_steps: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM steps").fetch_one(&pool).await?;
    let total_nutrition: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM nutrition").fetch_one(&pool).await?;
    let total_tags: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM tags").fetch_one(&pool).await?;

    info!("─────────────────────────────────");
    info!("inserted : {}", inserted);
    info!("skipped  : {}", skipped);
    info!("failed   : {}", failed);
    info!("─────────────────────────────────");
    info!("recipes     : {}", total_recipes);
    info!("ingredients : {}", total_ingredients);
    info!("steps       : {}", total_steps);
    info!("nutrition   : {}", total_nutrition);
    info!("tags        : {}", total_tags);

    Ok(())
}
