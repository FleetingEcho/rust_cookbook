# 菜谱 API 项目计划

## 项目概述

基于 [HowToCook](https://github.com/Anduin2017/HowToCook) 和 [CookLikeHOC](https://github.com/Gar-b-age/CookLikeHOC) 两个开源菜谱仓库的数据，用 Rust + axum 构建一个中文菜谱 RESTful API 服务。

**技术栈：**
- Web 框架：`axum`
- 数据库：SQLite（`sqlx` 0.8）
- FTS5 中文搜索：trigram tokenizer（内置，无需 jieba-rs）
- Markdown 解析：行解析（无需 pulldown-cmark）
- 目录遍历：`walkdir`
- 序列化：`serde` / `serde_json`
- 错误处理：`thiserror` + `anyhow`
- 日志：`tracing` + `tracing-subscriber`
- 测试：`axum-test`

---

## 数据来源分析

### HowToCook

- 路径结构：`dishes/{category}/{recipe_name}.md`
- 分类（英文）：`aquatic`、`breakfast`、`condiment`、`dessert`、`drink`、`meat_dish`、`semi-finished`、`soup`、`staple`、`vegetable_dish`
- Markdown 结构（固定格式）：
  ```
  # 菜名
  描述段落
  预估烹饪难度：★★★
  预估卡路里：385 大卡
  ## 必备原料和工具
  - 食材名（备注）
  ## 计算
  每份：
  - 食材 用量
  ## 操作
  1. 步骤文字
  2. ![图片描述](./图片.jpg)
  ## 附加内容
  - 技巧说明
  ```

### CookLikeHOC

- 路径结构：`{category}/{recipe_name}.md`
- 分类（中文）：`主食`、`凉拌`、`卤菜`、`早餐`、`汤`、`炒菜`、`炖菜`、`炸品`、`烤类`、`烫菜`、`煮锅`、`砂锅菜`、`蒸菜`、`配料`、`饮品`
- Markdown 结构：
  ```
  # 菜名
  ## 原料
  - 食材 用量
  ## 步骤
  - 1. 步骤文字
  ## 营养成分
  | 项目 | 每 100g 含量 |
  | 热量 | 59 Kcal |
  | 蛋白质 | 3.0 g |
  | 脂肪 | 2.7 g |
  | 碳水化合物 | 5.7 g |
  | 钠 | 497 mg |
  ```

---

## 数据分析报告（实际 clone 后整理）

### 数量统计

| 来源 | 菜谱数 |
|------|--------|
| HowToCook | 364 |
| CookLikeHOC | 336 |
| **合计** | **700** |

### HowToCook 实际数据格式

**格式高度统一：**
- 所有 364 条菜谱都有 `预估烹饪难度` 和 `预估卡路里`，无需容错
- 难度分布：★(33) ★★(92) ★★★(131) ★★★★(89) ★★★★★(20)

**食材用量情况：**
- 总食材行 5482 条，约 24%（1327条）有明确数字用量，其余为无量描述（如 `- 盐`、`- 适量生姜`）
- `## 必备原料和工具` 和 `## 计算` 两个 section 都包含食材，需要合并处理
- 两个 section 内部有时还有 `### 主食材`、`### 辅料`、`### 副食材`、`### 必备工具` 等子标题，解析时跳过以 `### 工具` 开头的子标题下的内容（工具不是食材）

**步骤中的图片：**
- 图片格式：`![描述](./图片名.jpg)`，相对路径
- 需拼接为 GitHub raw URL：`https://raw.githubusercontent.com/Anduin2017/HowToCook/master/dishes/{category}/{recipe_dir}/{image}`

### CookLikeHOC 实际数据格式

**各分类菜谱数量：**

| 分类 | 数量 | | 分类 | 数量 |
|------|------|-|------|------|
| 炒菜 | 70 | | 卤菜 | 11 |
| 蒸菜 | 49 | | 煮锅 | 11 |
| 配料 | 40 | | 汤 | 5 |
| 早餐 | 35 | | 凉拌 | 4 |
| 主食 | 31 | | 烤类 | 1 |
| 饮品 | 21 | | 炸品 | 18 |
| 砂锅菜 | 15 | | 炖菜 | 13 |

**食材 section 标题有 6 种写法（需归一化处理）：**

| 原始标题 | 出现次数 | 处理方式 |
|----------|----------|----------|
| `## 配料` | 251 | 识别为食材 |
| `## 已知成分` | 39 | 识别为食材（用于配料/酱料，成分不完整披露） |
| `## 原料` | 15 | 识别为食材 |
| `## 原料：` / `## 原料:` / `## 原料: ` | 24 | trim + 去冒号后识别为食材 |
| `## 配料\t`（含 tab） | 2 | trim 后识别为食材 |

**步骤 section 标题同样有变体：**
`## 步骤` / `## 步骤：` / `## 步骤:` / `## 步骤\t` → 统一 trim + 去冒号处理

**营养成分覆盖率：**
- 有 `## 营养成分` 表格：87 条（26%）
- 无营养数据：249 条（74%）

**食材用量情况：**
- 总食材行 2738 条，约 22%（610条）有数字用量
- 大量食材只列名称，无用量（例：`- 盐`、`- 鸡精`）
- 食材行中存在内部链接引用，需提取名称：
  `- [老鸡汤](/汤/老鸡汤.md)` → 食材名为 `老鸡汤`
  `- [炒菜基料](/配料/炒菜基料.md)` → 食材名为 `炒菜基料`

**图片：**
- 格式：`![菜名](../images/菜名.png)`，位于菜谱文件开头
- GitHub raw URL：`https://raw.githubusercontent.com/Gar-b-age/CookLikeHOC/main/images/{image}`

### 解析策略总结

```
HowToCook 食材解析：
  合并 "## 必备原料和工具" 和 "## 计算" 两节内容
  跳过 "### 工具" / "### 必备工具" 子节
  其他 ### 子节（主食材/辅料/副食材等）正常解析，食材不分主副

CookLikeHOC 食材 section 识别（伪代码）：
  header.trim().trim_end_matches(['：', ':', '\t'])
  匹配 "配料" | "原料" | "已知成分" → 识别为食材节

内部链接处理：
  正则 `\[([^\]]+)\]\([^)]+\)` 提取显示文本作为食材名

步骤解析：
  HowToCook：有序列表 `1. 步骤` 和图片 `![](./xxx)` 交替出现
  CookLikeHOC：无序列表 `- 1. 步骤文字`（前缀 `- ` 需去掉）
```

---

## 数据模型（SQLite）

### 表结构

```sql
-- 菜谱主表
CREATE TABLE recipes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT,
    category    TEXT NOT NULL,           -- 标准化后的中文分类
    difficulty  INTEGER,                 -- 1-5，NULL 表示未知
    calories    REAL,                    -- 大卡，NULL 表示未知
    cover_image TEXT,                    -- CookLikeHOC 菜品封面图 URL（HowToCook 为 NULL）
    source      TEXT NOT NULL,           -- "HowToCook" | "CookLikeHOC"
    source_path TEXT NOT NULL,           -- 原始文件路径（用于去重）
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(source, source_path)          -- 幂等导入约束
);

-- 食材表
CREATE TABLE ingredients (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    name      TEXT NOT NULL,
    amount    TEXT,                      -- 原始用量字符串，如 "10-15ml"
    unit      TEXT,                      -- 解析出的单位，如 "ml"
    note      TEXT                       -- 备注，如 "害怕杀鱼可让店家处理"
);

-- 步骤表
CREATE TABLE steps (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id  INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    step_order INTEGER NOT NULL,
    content    TEXT,                     -- 纯图片步骤时为 NULL
    image_url  TEXT                      -- GitHub raw 图片链接
);

-- 营养成分表（来自 CookLikeHOC，每 100g）
CREATE TABLE nutrition (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id   INTEGER NOT NULL UNIQUE REFERENCES recipes(id) ON DELETE CASCADE,
    protein_g   REAL,
    fat_g       REAL,
    carbs_g     REAL,
    sodium_mg   REAL
);

-- 标签表
-- 自动生成规则（seed 时写入）：
--   1. 分类名直接作为 tag（如 "水产"、"早餐"）
--   2. 从菜名提取烹饪方式关键词（红烧 / 清蒸 / 爆炒 / 凉拌 / 油炸 / 烤 / 炖 / 卤）
--   3. 从菜名/描述提取辣度（辣、微辣、不辣；默认不打辣度 tag）
--   4. 从卡路里推断热量标签（≤300 低卡 / ≥600 高热量）
--   5. 难度标签（difficulty=1 → 新手友好 / difficulty≥4 → 进阶）
CREATE TABLE tags (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    tag       TEXT NOT NULL
);

-- 索引（高频过滤/查询字段）
CREATE INDEX idx_recipes_category   ON recipes(category);
CREATE INDEX idx_recipes_difficulty ON recipes(difficulty);
CREATE INDEX idx_recipes_calories   ON recipes(calories);
CREATE INDEX idx_ingredients_name   ON ingredients(name);
CREATE INDEX idx_tags_tag           ON tags(tag);

-- 全文搜索虚拟表（SQLite FTS5）
-- trigram tokenizer 对中文字符做 n-gram 切割，无需 jieba-rs 分词
-- 独立表（非 content= 方式），seed 完成后执行一次 DELETE + bulk INSERT
CREATE VIRTUAL TABLE recipes_fts USING fts5(
    recipe_id   UNINDEXED,
    name,
    description,
    ingredients_text,
    tokenize = 'trigram'
);

-- seed 完成后执行（全量重建）：
-- DELETE FROM recipes_fts;
-- INSERT INTO recipes_fts SELECT id, name, description, ingredients_text FROM recipes;
```

### 分类映射（HowToCook 英文 → 中文标准化）

| 原始分类 | 标准中文分类 |
|----------|-------------|
| aquatic | 水产 |
| breakfast | 早餐 |
| condiment | 调味料 |
| dessert | 甜点 |
| drink | 饮品 |
| meat_dish | 荤菜 |
| semi-finished | 半成品 |
| soup | 汤 |
| staple | 主食 |
| vegetable_dish | 素菜 |
| CookLikeHOC 直接使用中文分类 | — |

---

## 项目结构

```
cookbook-api/
├── Cargo.toml
├── .env                        # DATABASE_URL=sqlite://data/cookbook.db（不提交）
├── migrations/
│   └── 0001_init.sql           # 一个文件包含所有表 + 索引 + FTS5（已完成）
├── data/
│   ├── cookbook.db             # 预 seed 的数据库（已提交，700 条菜谱）
│   ├── test_queries.sql        # SQL 验证测试（20 条查询）
│   ├── HowToCook/              # 克隆的仓库（.gitignore 排除）
│   └── CookLikeHOC/            # 克隆的仓库（.gitignore 排除）
├── src/
│   ├── main.rs                 # 占位符（待实现 AppState、路由注册）
│   ├── config.rs               # 配置（端口、数据库路径等）
│   ├── error.rs                # 统一错误类型 AppError -> axum IntoResponse
│   ├── router.rs               # 路由注册汇总
│   ├── models/
│   │   ├── mod.rs
│   │   ├── recipe.rs           # Recipe, RecipeDetail, RecipeSummary
│   │   ├── ingredient.rs       # Ingredient
│   │   ├── step.rs             # Step
│   │   ├── nutrition.rs        # Nutrition
│   │   └── pagination.rs       # PaginationParams, PagedResult<T>
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── recipes.rs          # list, get, random, search
│   │   ├── categories.rs       # list_categories
│   │   ├── ingredients.rs      # suggest, by_ingredients
│   │   └── meal_plan.rs        # generate_meal_plan
│   └── db/
│       ├── mod.rs
│       ├── recipes.rs          # DB 查询函数
│       ├── ingredients.rs
│       └── search.rs           # FTS5 搜索逻辑
└── bin/
    └── seed.rs                 # 数据导入 CLI（已完成）
```

---

## API 设计

### 基础端点

```
GET  /api/v1/recipes
     ?page=1&per_page=20          # 分页，per_page 上限 100
     ?category=汤                 # 分类过滤
     ?difficulty_min=1            # 难度下限（1-5）
     ?difficulty_max=3            # 难度上限（1-5）
     ?min_calories=200            # 热量下限
     ?max_calories=500            # 热量上限
     ?source=HowToCook            # 数据来源过滤
     ?sort_by=calories            # 排序字段：calories / difficulty / name（默认 id）
     ?order=asc                   # asc / desc（默认 asc）
     → PagedResult<RecipeSummary>

GET  /api/v1/recipes/random
     ?category=早餐               # 可选过滤
     ?max_calories=400
     → RecipeDetail

GET  /api/v1/recipes/:id
     → RecipeDetail（含食材、步骤、营养）

GET  /api/v1/recipes/:id/similar
     ?limit=5                     # 默认 5
     → [RecipeSummary]
     # 同分类优先，按食材重合度排序（共享食材数 / 总食材数）

GET  /api/v1/categories
     → [{ name, count }]

GET  /api/v1/stats
     → {
         total_recipes: 358,
         by_category: { "水产": 42, "早餐": 30, ... },
         avg_calories: 420.5,
         sources: { "HowToCook": 200, "CookLikeHOC": 158 }
       }
```

### 搜索端点

```
GET  /api/v1/recipes/search
     ?q=红烧肉                    # 查询词，FTS5 trigram 搜索
     ?page=1&per_page=20
     → PagedResult<RecipeSummary>

GET  /api/v1/ingredients
     ?page=1&per_page=50
     → PagedResult<{ name, recipe_count }>   # 所有食材及出现次数

GET  /api/v1/ingredients/suggest
     ?q=豆&limit=10
     → [{ name, recipe_count }]  # 食材名模糊匹配

GET  /api/v1/recipes/by-ingredients
     ?ingredients=豆腐,鸡蛋,酱油  # 逗号分隔
     ?match=any                   # any=含其中之一 / all=全部包含
     ?page=1&per_page=20
     → PagedResult<RecipeSummary>
```

### 组合功能端点

```
POST /api/v1/meal-plan
     Body: {
       "days": 7,
       "people": 2,
       "max_calories_per_meal": 600,   # 可选，替代自由文本偏好
       "max_difficulty": 3,            # 可选
       "tags": ["低卡", "新手友好"]     # 可选，匹配 tags 表中的标签
     }
     → {
         days: [
           {
             "date": "2024-01-01",
             "breakfast": RecipeSummary,
             "lunch": [RecipeSummary, RecipeSummary],
             "dinner": [RecipeSummary, RecipeSummary]
           }
         ]
       }
     # 早餐从"早餐"分类选 1 道
     # 午/晚餐各从荤菜/素菜/汤中随机搭配 2-3 道
     # 7 天内同一道菜不重复
```

### 响应格式

```jsonc
// RecipeSummary（列表用）
{
  "id": 42,
  "name": "清蒸鲈鱼",
  "category": "水产",
  "difficulty": 3,
  "calories": 385.0,
  "source": "HowToCook",
  "ingredient_count": 7
}

// RecipeDetail（详情用）
{
  "id": 42,
  "name": "清蒸鲈鱼",
  "description": "粤式经典蒸菜，鱼肉细嫩爽滑...",
  "category": "水产",
  "difficulty": 3,
  "calories": 385.0,
  "source": "HowToCook",
  "ingredients": [
    { "name": "鲈鱼", "amount": "1条", "unit": null, "note": "可让店家帮忙杀" },
    { "name": "蒸鱼豉油", "amount": "10-15ml", "unit": "ml", "note": null }
  ],
  "steps": [
    { "order": 1, "content": "姜切片切丝...", "image_url": null },
    { "order": 4, "content": null, "image_url": "https://raw.githubusercontent.com/..." }
  ],
  "nutrition": {
    "protein_g": null,
    "fat_g": null,
    "carbs_g": null,
    "sodium_mg": null
  },
  "tags": ["清蒸", "鱼", "粤式"]
}

// PagedResult<T>
{
  "data": [...],
  "page": 1,
  "per_page": 20,
  "total": 358,
  "total_pages": 18
}

// 错误格式
{
  "error": "not_found",       // not_found | bad_request | validation_error | internal_error
  "message": "Recipe with id 999 not found"
}
```

### HTTP 状态码约定

| 场景 | 状态码 |
|------|--------|
| 正常返回 | 200 OK |
| `:id` 不存在 | 404 Not Found |
| 参数格式错误（非数字等） | 400 Bad Request |
| 参数值不合法（per_page > 100 等） | 422 Unprocessable Entity |
| 服务器内部错误 | 500 Internal Server Error |
```

---

## 实现步骤

### Step 1：项目初始化 ✅

- [x] `cargo new cookbook-api`
- [x] 配置 `Cargo.toml` 依赖（sqlx 0.8、tokio、tracing、walkdir 等；axum 待 Step 4 加入）
- [x] 创建 `.env`，配置 `DATABASE_URL=sqlite://data/cookbook.db`
- [x] 创建 `data/` 目录

### Step 2：数据库迁移 ✅

- [x] 安装 `sqlx-cli`：`cargo install sqlx-cli --no-default-features --features sqlite`
- [x] 编写单一 migration 文件 `migrations/0001_init.sql`（包含所有表 + 索引 + FTS5）
- [x] 运行 `sqlx migrate run`
- [x] 验证表结构正确

### Step 3：数据导入（`bin/seed.rs`）✅

- [x] 浅克隆两个仓库（`--depth 1` 避免下载完整历史）：
  ```
  git clone --depth 1 https://github.com/Anduin2017/HowToCook data/HowToCook
  git clone --depth 1 https://github.com/Gar-b-age/CookLikeHOC data/CookLikeHOC
  ```
  运行 seed 前手动 clone，脚本检查目录已存在时跳过
- [x] 递归扫描 `.md` 文件，跳过 README 和模板文件
- [x] 实现 HowToCook Markdown 解析器（行解析，非 pulldown-cmark）：
  - 菜名：取一级标题，strip `的做法` 后缀
  - 描述：取标题后、`预估烹饪难度` 前的第一段正文
  - 难度：正则 `预估烹饪难度：(★+)` → 数 `★` 个数（1-5）
  - 卡路里：正则 `预估卡路里：(\d+(?:\.\d+)?) 大卡`
  - 食材：合并解析 `## 必备原料和工具` 和 `## 计算` 两节，跳过 `### 工具`/`### 必备工具` 子节下的条目
  - 步骤：解析 `## 操作` 下有序列表；图片 `./xxx.jpg` → GitHub raw URL
- [x] 实现 CookLikeHOC Markdown 解析器：
  - 菜名：取一级标题，`resolve_links()` 去除 `[name](url)` 包装
  - 分类：从文件路径的父目录名取得
  - 封面图：解析菜名标题后第一行的 `![](../images/xxx.png)` → GitHub raw URL
  - 食材：`clhoc_section()` 归一化识别 6 种 section 标题变体；内部链接提取显示文本
  - 步骤：识别 `## 步骤` 变体，去掉列表前缀 `- `
  - 营养：`## 营养成分` 下的 Markdown 表格，解析蛋白质/脂肪/碳水/钠四项
- [x] 自动生成 tags（分类名 / 烹饪方式关键词 / 辣度 / 热量标签 / 难度标签）
- [x] 去重逻辑：先检查 `(source, source_path)` 是否存在再 INSERT，幂等导入
- [x] 导入完成后全量重建 FTS5 索引（DELETE + bulk INSERT，无需 rebuild 命令）
- [x] 打印导入统计（700 条导入，0 条跳过）
- [x] 预 seed 的 `data/cookbook.db` 已提交，他人无需自行 clone markdown 仓库

### Step 4：核心 API 骨架

- [ ] 实现 `AppState`（包含 `SqlitePool`）
- [ ] 实现统一错误类型 `AppError`（NotFound / BadRequest / Internal）
- [ ] 注册路由，各 handler 先返回占位符
- [ ] 实现健康检查端点 `GET /health`

### Step 5：基础 CRUD 接口

- [ ] `GET /api/v1/recipes`：分页列表 + 过滤（category / difficulty_min~max / calories / source）+ 排序（sort_by / order）
- [ ] `GET /api/v1/recipes/:id`：详情（JOIN 三张表）
- [ ] `GET /api/v1/recipes/:id/similar`：同分类、食材重合度排序
- [ ] `GET /api/v1/recipes/random`：随机一条（可带分类/热量过滤）
- [ ] `GET /api/v1/categories`：分类及菜谱数量
- [ ] `GET /api/v1/stats`：全局统计数据

### Step 6：搜索功能

- [ ] 实现 FTS5 搜索查询（`recipes_fts MATCH ?`，trigram 自动处理中文）
- [ ] `GET /api/v1/recipes/search`：FTS5 全文搜索
- [ ] `GET /api/v1/ingredients`：所有食材分页列表
- [ ] `GET /api/v1/ingredients/suggest`：食材名模糊匹配
- [ ] `GET /api/v1/recipes/by-ingredients`：按食材反查菜谱（any / all 模式）

### Step 7：组合功能

- [ ] `POST /api/v1/meal-plan`：按天数、人数、偏好生成食谱计划
  - 早餐从 `早餐` 分类选，午/晚餐从其他分类各选 2-3 道
  - 同一道菜在 7 天内不重复出现
  - 偏好关键词（"不辣" / "低卡"）做标签或卡路里过滤

### Step 8：测试

- [ ] 单元测试：Markdown 解析器（覆盖各种边界情况）
- [ ] 集成测试：用 `axum-test` 测试各接口（搜索、分页、详情）
- [ ] 用 `httpie` 或 `curl` 手动验证所有接口

### Step 9：完善

- [ ] 添加请求日志中间件（`tracing`）
- [ ] 添加 CORS 中间件（`tower-http`）
- [ ] 添加请求超时中间件
- [ ] 编写 `README.md`，含接口文档和启动说明

---

## Cargo.toml 依赖参考

```toml
[dependencies]
# API 框架（Step 4 加入）
axum            = { version = "0.7", features = ["macros"] }
tokio           = { version = "1", features = ["full"] }
serde           = { version = "1", features = ["derive"] }
serde_json      = "1"
tower-http      = { version = "0.5", features = ["cors", "trace", "timeout"] }
thiserror       = "1"

# 数据库 + 工具（已有）
sqlx            = { version = "0.8", features = ["runtime-tokio", "sqlite", "migrate"] }
anyhow          = "1"
regex           = "1"
walkdir         = "2"
tracing         = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenvy         = "0.15"

[[bin]]
name = "seed"
path = "bin/seed.rs"
```

---

## 注意事项

1. **SQLite 连接配置**：启动时执行以下 PRAGMA，并用连接池选项控制并发：
   ```rust
   // main.rs 中连接池初始化
   let pool = SqlitePoolOptions::new()
       .max_connections(5)
       .after_connect(|conn, _| Box::pin(async move {
           conn.execute_batch("
               PRAGMA journal_mode=WAL;
               PRAGMA foreign_keys=ON;
               PRAGMA synchronous=NORMAL;
           ").await?;
           Ok(())
       }))
       .connect(&database_url).await?;
   ```
2. **FTS5 中文搜索**：使用 `trigram` tokenizer，对字符串按 3 字符滑动窗口建立倒排索引，无需 jieba-rs 分词，支持任意子串匹配。查询时直接传入原始查询词即可，sqlx 内置的 sqlite 已包含 FTS5 支持（系统 sqlite3 CLI 可能没有）。
3. **图片 URL**：HowToCook 中图片路径为相对路径（`./图片.jpg`），需在 seed 时拼接为 GitHub raw URL：`https://raw.githubusercontent.com/Anduin2017/HowToCook/master/dishes/{category}/{recipe}/{image}`。
4. **解析容错**：两个仓库的 Markdown 格式并不完全统一，解析器要做容错处理，解析失败的字段记 `NULL` 而不是整条跳过。
5. **去重**：两个仓库可能有相同菜名，用 `(source, source_path)` 联合唯一索引去重，不用菜名去重。
