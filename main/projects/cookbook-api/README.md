# cookbook-api

基于 [HowToCook](https://github.com/Anduin2017/HowToCook) 和 [CookLikeHOC](https://github.com/Gar-b-age/CookLikeHOC) 两个开源菜谱仓库数据构建的中文菜谱 RESTful API，使用 Rust + axum + SQLite。

## 数据概况

| 来源 | 菜谱数 |
|------|--------|
| HowToCook | 364 |
| CookLikeHOC | 336 |
| **合计** | **700** |

食材 4877 条，步骤 4155 条，营养数据 87 条，标签 1205 条。

## 快速启动

数据库已预 seed 到 `data/cookbook.db`，无需克隆原始 markdown 仓库。

```bash
# 1. 复制并编辑环境变量（默认端口 8080）
cp .env.example .env   # 或直接使用默认的 .env

# 2. 启动服务
cargo run --bin cookbook-api

# 3. 验证
curl http://localhost:8080/health
```

### 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `DATABASE_URL` | `sqlite://data/cookbook.db` | SQLite 数据库路径 |
| `PORT` | `8080` | 监听端口 |
| `RUST_LOG` | `info` | 日志级别 |

> **注意**：Windows 上端口 2942–3041 被系统保留，默认使用 8080。

### 重新生成数据库（可选）

```bash
# 克隆原始仓库
git clone --depth 1 https://github.com/Anduin2017/HowToCook data/HowToCook
git clone --depth 1 https://github.com/Gar-b-age/CookLikeHOC data/CookLikeHOC

# 运行 seed
cargo run --bin seed
```

---

## API 文档

Base URL: `http://localhost:8080/api/v1`

### 健康检查

```
GET /health
→ "ok"
```

---

### 菜谱列表

```
GET /api/v1/recipes
```

**Query 参数：**

| 参数 | 类型 | 说明 |
|------|------|------|
| `page` | int | 页码（默认 1） |
| `per_page` | int | 每页数量（默认 20，上限 100） |
| `category` | string | 分类过滤（如 `水产`、`早餐`） |
| `difficulty_min` | int | 难度下限（1-5） |
| `difficulty_max` | int | 难度上限（1-5） |
| `min_calories` | float | 热量下限（大卡） |
| `max_calories` | float | 热量上限（大卡） |
| `source` | string | 来源（`HowToCook` / `CookLikeHOC`） |
| `sort_by` | string | 排序字段：`calories` / `difficulty` / `name`（默认 `id`） |
| `order` | string | `asc` / `desc`（默认 `asc`） |

**示例：**
```bash
curl "http://localhost:8080/api/v1/recipes?category=水产&difficulty_max=2&sort_by=calories"
```

**响应：** `PagedResult<RecipeSummary>`

---

### 菜谱详情

```
GET /api/v1/recipes/:id
```

返回含食材、步骤、营养、标签的完整信息。

**示例：**
```bash
curl http://localhost:8080/api/v1/recipes/1
```

---

### 随机菜谱

```
GET /api/v1/recipes/random
```

| 参数 | 类型 | 说明 |
|------|------|------|
| `category` | string | 可选分类过滤 |
| `max_calories` | float | 可选热量上限 |

```bash
curl "http://localhost:8080/api/v1/recipes/random?category=早餐"
```

---

### 相似菜谱

```
GET /api/v1/recipes/:id/similar
```

同分类，按食材重合度降序排列。

| 参数 | 类型 | 说明 |
|------|------|------|
| `limit` | int | 返回条数（默认 5，上限 20） |

---

### 全文搜索

```
GET /api/v1/recipes/search
```

| 参数 | 类型 | 说明 |
|------|------|------|
| `q` | string | 查询词（必填） |
| `page` / `per_page` | int | 分页 |

查询词 ≥ 3 字符时使用 FTS5 trigram 搜索（按相关度排序），< 3 字符时 fallback 到菜名 LIKE 模糊匹配。

```bash
curl "http://localhost:8080/api/v1/recipes/search?q=红烧肉"
curl "http://localhost:8080/api/v1/recipes/search?q=红烧"  # 2字 LIKE fallback
```

---

### 按食材反查菜谱

```
GET /api/v1/recipes/by-ingredients
```

| 参数 | 类型 | 说明 |
|------|------|------|
| `ingredients` | string | 逗号分隔的食材名（支持模糊匹配） |
| `match` | string | `any`（含其中之一）/ `all`（全部包含），默认 `any` |
| `page` / `per_page` | int | 分页 |

```bash
curl "http://localhost:8080/api/v1/recipes/by-ingredients?ingredients=豆腐,鸡蛋&match=all"
```

---

### 食材列表

```
GET /api/v1/ingredients          # 全部食材分页（按出现次数降序）
GET /api/v1/ingredients/suggest  # 模糊搜索食材名
```

`suggest` 参数：`q`（查询词），`limit`（默认 10）

```bash
curl "http://localhost:8080/api/v1/ingredients/suggest?q=豆"
```

---

### 分类列表

```
GET /api/v1/categories
→ [{ "name": "荤菜", "count": 108 }, ...]
```

---

### 全局统计

```
GET /api/v1/stats
→ { "total_recipes": 700, "avg_calories": 761.7, "by_category": {...}, "sources": {...} }
```

---

### 餐单生成

```
POST /api/v1/meal-plan
Content-Type: application/json
```

**请求体：**

```json
{
  "days": 7,
  "people": 2,
  "max_calories_per_meal": 600,
  "max_difficulty": 3,
  "tags": ["低卡"]
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `days` | int | 天数（1-14，默认 7） |
| `people` | int | 人数（默认 2，仅记录，不影响选菜逻辑） |
| `max_calories_per_meal` | float? | 单道菜热量上限 |
| `max_difficulty` | int? | 难度上限（1-5） |
| `tags` | string[]? | 要求同时具有的标签（如 `低卡`、`新手友好`） |

**响应：**

```json
{
  "people": 2,
  "days": [
    {
      "day": 1,
      "breakfast": { ...RecipeSummary },
      "lunch": [ ...RecipeSummary, ...RecipeSummary ],
      "dinner": [ ...RecipeSummary, ...RecipeSummary ]
    }
  ]
}
```

每天早餐 1 道（来自早餐分类），午餐/晚餐各 2 道（荤菜/素菜/炒菜等），7 天内不重复。

---

## 数据模型

### RecipeSummary

```json
{
  "id": 42,
  "name": "清蒸鲈鱼",
  "category": "水产",
  "difficulty": 3,
  "calories": 385.0,
  "cover_image": null,
  "source": "HowToCook",
  "ingredient_count": 7
}
```

### RecipeDetail

```json
{
  "id": 42,
  "name": "清蒸鲈鱼",
  "description": "粤式经典蒸菜...",
  "category": "水产",
  "difficulty": 3,
  "calories": 385.0,
  "cover_image": null,
  "source": "HowToCook",
  "source_path": "aquatic/清蒸鲈鱼/清蒸鲈鱼.md",
  "created_at": "2026-06-17 07:15:40",
  "ingredients": [
    { "id": 1, "recipe_id": 42, "name": "鲈鱼", "amount": "1条", "unit": null, "note": null }
  ],
  "steps": [
    { "id": 1, "recipe_id": 42, "step_order": 1, "content": "姜切片...", "image_url": null }
  ],
  "nutrition": null,
  "tags": ["水产", "清蒸"]
}
```

### 错误格式

```json
{ "error": "not_found", "message": "not found" }
```

| 错误码 | HTTP 状态 | 说明 |
|--------|-----------|------|
| `not_found` | 404 | 资源不存在 |
| `bad_request` | 400 | 请求参数格式错误 |
| `validation_error` | 422 | 参数值不合法 |
| `internal_error` | 500 | 服务器内部错误 |

---

## 分类说明

| 分类 | 来源 | 说明 |
|------|------|------|
| 荤菜 / 水产 / 素菜 | HowToCook | 由英文目录名映射 |
| 早餐 / 汤 / 主食 / 甜点 / 饮品 | 两者均有 | |
| 炒菜 / 蒸菜 / 炖菜 / 卤菜 / 配料 | CookLikeHOC | 直接使用中文目录名 |

## 标签说明

标签由 seed 自动生成，规则如下：

- **分类名**：如 `水产`、`早餐`
- **烹饪方式**：从菜名提取 `红烧`、`清蒸`、`爆炒`、`凉拌`、`油炸`、`烤`、`炖`、`卤`
- **辣度**：`辣`、`微辣`
- **热量标签**：`低卡`（≤300 大卡）、`高热量`（≥600 大卡）
- **难度标签**：`新手友好`（难度=1）、`进阶`（难度≥4）
