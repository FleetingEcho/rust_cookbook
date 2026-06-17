-- ============================================================
-- 菜谱数据库验证测试
-- 运行方式: sqlite3 data/cookbook.db < data/test_queries.sql
-- ============================================================

.headers on
.mode column
.nullvalue NULL

-- ────────────────────────────────────────────────────────────
-- 1. 基础数量检查
-- ────────────────────────────────────────────────────────────
SELECT '=== 1. 基础数量 ===' AS test;

SELECT
    (SELECT COUNT(*) FROM recipes)     AS total_recipes,
    (SELECT COUNT(*) FROM ingredients) AS total_ingredients,
    (SELECT COUNT(*) FROM steps)       AS total_steps,
    (SELECT COUNT(*) FROM nutrition)   AS total_nutrition,
    (SELECT COUNT(*) FROM tags)        AS total_tags;

-- ────────────────────────────────────────────────────────────
-- 2. 数据来源分布
-- ────────────────────────────────────────────────────────────
SELECT '=== 2. 数据来源分布 ===' AS test;

SELECT source, COUNT(*) AS count
FROM recipes
GROUP BY source;

-- ────────────────────────────────────────────────────────────
-- 3. 分类分布
-- ────────────────────────────────────────────────────────────
SELECT '=== 3. 分类分布 ===' AS test;

SELECT category, COUNT(*) AS count
FROM recipes
GROUP BY category
ORDER BY count DESC;

-- ────────────────────────────────────────────────────────────
-- 4. 完整性检查：是否有空菜名、空分类
-- ────────────────────────────────────────────────────────────
SELECT '=== 4. 完整性检查 ===' AS test;

SELECT
    COUNT(*) FILTER (WHERE name IS NULL OR name = '')     AS empty_name,
    COUNT(*) FILTER (WHERE category IS NULL OR category = '') AS empty_category,
    COUNT(*) FILTER (WHERE source_path IS NULL)           AS empty_source_path,
    COUNT(*) FILTER (WHERE difficulty IS NULL)            AS no_difficulty,
    COUNT(*) FILTER (WHERE calories IS NULL)              AS no_calories,
    COUNT(*) FILTER (WHERE cover_image IS NOT NULL)       AS has_cover_image
FROM recipes;

-- ────────────────────────────────────────────────────────────
-- 5. 每道菜平均食材数 / 步骤数
-- ────────────────────────────────────────────────────────────
SELECT '=== 5. 平均食材 / 步骤数 ===' AS test;

SELECT
    ROUND(AVG(ing_count), 1) AS avg_ingredients,
    MIN(ing_count)           AS min_ingredients,
    MAX(ing_count)           AS max_ingredients,
    ROUND(AVG(step_count), 1) AS avg_steps,
    MIN(step_count)           AS min_steps,
    MAX(step_count)           AS max_steps
FROM (
    SELECT
        r.id,
        COUNT(DISTINCT i.id) AS ing_count,
        COUNT(DISTINCT s.id) AS step_count
    FROM recipes r
    LEFT JOIN ingredients i ON i.recipe_id = r.id
    LEFT JOIN steps s       ON s.recipe_id = r.id
    GROUP BY r.id
);

-- ────────────────────────────────────────────────────────────
-- 6. 菜谱详情查询（模拟 GET /recipes/:id）
-- ────────────────────────────────────────────────────────────
SELECT '=== 6. 菜谱详情 - 清蒸鲈鱼 ===' AS test;

SELECT id, name, category, difficulty, calories, source
FROM recipes
WHERE name = '清蒸鲈鱼';

SELECT i.name, i.amount, i.unit, i.note
FROM ingredients i
JOIN recipes r ON r.id = i.recipe_id
WHERE r.name = '清蒸鲈鱼';

SELECT step_order, content, image_url
FROM steps
WHERE recipe_id = (SELECT id FROM recipes WHERE name = '清蒸鲈鱼')
ORDER BY step_order;

-- ────────────────────────────────────────────────────────────
-- 7. 分页列表（模拟 GET /recipes?page=1&per_page=10）
-- ────────────────────────────────────────────────────────────
SELECT '=== 7. 分页列表 (page=1, per_page=10) ===' AS test;

SELECT
    r.id,
    r.name,
    r.category,
    r.difficulty,
    r.calories,
    COUNT(i.id) AS ingredient_count
FROM recipes r
LEFT JOIN ingredients i ON i.recipe_id = r.id
GROUP BY r.id
ORDER BY r.id
LIMIT 10 OFFSET 0;

-- ────────────────────────────────────────────────────────────
-- 8. 按分类 + 难度过滤（模拟 GET /recipes?category=水产&difficulty_max=3）
-- ────────────────────────────────────────────────────────────
SELECT '=== 8. 过滤：水产 & 难度<=3 ===' AS test;

SELECT name, difficulty, calories
FROM recipes
WHERE category = '水产' AND difficulty <= 3
ORDER BY difficulty, calories;

-- ────────────────────────────────────────────────────────────
-- 9. 卡路里范围过滤 + 排序（模拟 GET /recipes?max_calories=300&sort_by=calories）
-- ────────────────────────────────────────────────────────────
SELECT '=== 9. 低卡菜谱 (calories<=300, 按热量升序) ===' AS test;

SELECT name, category, calories, difficulty
FROM recipes
WHERE calories IS NOT NULL AND calories <= 300
ORDER BY calories ASC
LIMIT 10;

-- ────────────────────────────────────────────────────────────
-- 10. 全文搜索（模拟 GET /recipes/search?q=红烧）
-- 注意：系统 sqlite3 CLI 可能不支持 FTS5，用 sqlx 内置 sqlite 时正常
-- ────────────────────────────────────────────────────────────
SELECT '=== 10. FTS 搜索 "红烧"（fallback：菜名 LIKE 模糊匹配）===' AS test;

SELECT name, category, difficulty
FROM recipes
WHERE name LIKE '%红烧%'
LIMIT 10;

-- ────────────────────────────────────────────────────────────
-- 11. 食材模糊搜索（模拟 GET /ingredients/suggest?q=豆）
-- ────────────────────────────────────────────────────────────
SELECT '=== 11. 食材名含"豆"的 top10 ===' AS test;

SELECT name, COUNT(*) AS recipe_count
FROM ingredients
WHERE name LIKE '%豆%'
GROUP BY name
ORDER BY recipe_count DESC
LIMIT 10;

-- ────────────────────────────────────────────────────────────
-- 12. 按食材反查菜谱（模拟 GET /recipes/by-ingredients?ingredients=豆腐,鸡蛋&match=all）
-- ────────────────────────────────────────────────────────────
SELECT '=== 12. 含"豆腐"且含"鸡蛋"的菜谱 ===' AS test;

SELECT r.name, r.category
FROM recipes r
WHERE EXISTS (SELECT 1 FROM ingredients WHERE recipe_id = r.id AND name LIKE '%豆腐%')
  AND EXISTS (SELECT 1 FROM ingredients WHERE recipe_id = r.id AND name LIKE '%鸡蛋%')
ORDER BY r.name
LIMIT 10;

-- ────────────────────────────────────────────────────────────
-- 13. 随机推荐（模拟 GET /recipes/random?category=早餐）
-- ────────────────────────────────────────────────────────────
SELECT '=== 13. 随机早餐菜谱 ===' AS test;

SELECT id, name, calories, difficulty
FROM recipes
WHERE category = '早餐'
ORDER BY RANDOM()
LIMIT 1;

-- ────────────────────────────────────────────────────────────
-- 14. 相似菜谱（模拟 GET /recipes/:id/similar）
-- 同分类 + 食材重合度最高
-- ────────────────────────────────────────────────────────────
SELECT '=== 14. 与"清蒸鲈鱼"相似的菜谱 ===' AS test;

WITH target AS (
    SELECT id, category FROM recipes WHERE name = '清蒸鲈鱼'
),
target_ings AS (
    SELECT i.name FROM ingredients i JOIN target t ON i.recipe_id = t.id
)
SELECT
    r.name,
    r.category,
    COUNT(i.id) AS shared_ingredients
FROM recipes r
JOIN ingredients i ON i.recipe_id = r.id
JOIN target t ON r.category = t.category
WHERE i.name IN (SELECT name FROM target_ings)
  AND r.id != (SELECT id FROM target)
GROUP BY r.id
ORDER BY shared_ingredients DESC
LIMIT 5;

-- ────────────────────────────────────────────────────────────
-- 15. 全局统计（模拟 GET /stats）
-- ────────────────────────────────────────────────────────────
SELECT '=== 15. 全局统计 ===' AS test;

SELECT
    COUNT(*)                                               AS total_recipes,
    ROUND(AVG(calories), 1)                               AS avg_calories,
    ROUND(AVG(CASE WHEN difficulty IS NOT NULL THEN difficulty END), 2) AS avg_difficulty,
    COUNT(*) FILTER (WHERE calories IS NOT NULL)          AS recipes_with_calories,
    COUNT(*) FILTER (WHERE cover_image IS NOT NULL)       AS recipes_with_image
FROM recipes;

SELECT category, COUNT(*) AS count
FROM recipes
GROUP BY category
ORDER BY count DESC;

-- ────────────────────────────────────────────────────────────
-- 16. Tags 分布
-- ────────────────────────────────────────────────────────────
SELECT '=== 16. Tags 分布 top15 ===' AS test;

SELECT tag, COUNT(*) AS count
FROM tags
GROUP BY tag
ORDER BY count DESC
LIMIT 15;

-- ────────────────────────────────────────────────────────────
-- 17. 营养数据验证
-- ────────────────────────────────────────────────────────────
SELECT '=== 17. 营养数据样本 ===' AS test;

SELECT r.name, r.category, n.protein_g, n.fat_g, n.carbs_g, n.sodium_mg
FROM recipes r
JOIN nutrition n ON n.recipe_id = r.id
ORDER BY n.protein_g DESC NULLS LAST
LIMIT 8;

-- ────────────────────────────────────────────────────────────
-- 18. 餐单生成预检（覆盖早中晚三餐所需分类）
-- ────────────────────────────────────────────────────────────
SELECT '=== 18. 餐单分类覆盖检查 ===' AS test;

SELECT
    (SELECT COUNT(*) FROM recipes WHERE category = '早餐') AS breakfast_count,
    (SELECT COUNT(*) FROM recipes WHERE category IN ('荤菜','水产','素菜')) AS main_dish_count,
    (SELECT COUNT(*) FROM recipes WHERE category = '汤') AS soup_count,
    (SELECT COUNT(*) FROM recipes WHERE category = '主食') AS staple_count;

-- ────────────────────────────────────────────────────────────
-- 19. 外键完整性验证
-- ────────────────────────────────────────────────────────────
SELECT '=== 19. 孤立记录检查（应全为 0）===' AS test;

SELECT
    (SELECT COUNT(*) FROM ingredients WHERE recipe_id NOT IN (SELECT id FROM recipes)) AS orphan_ingredients,
    (SELECT COUNT(*) FROM steps      WHERE recipe_id NOT IN (SELECT id FROM recipes)) AS orphan_steps,
    (SELECT COUNT(*) FROM nutrition  WHERE recipe_id NOT IN (SELECT id FROM recipes)) AS orphan_nutrition,
    (SELECT COUNT(*) FROM tags       WHERE recipe_id NOT IN (SELECT id FROM recipes)) AS orphan_tags;

-- ────────────────────────────────────────────────────────────
-- 20. 重复数据检查
-- ────────────────────────────────────────────────────────────
SELECT '=== 20. 重复菜名检查 ===' AS test;

SELECT name, COUNT(*) AS count
FROM recipes
GROUP BY name
HAVING count > 1
ORDER BY count DESC
LIMIT 10;
