PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS recipes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT,
    category    TEXT    NOT NULL,
    difficulty  INTEGER,
    calories    REAL,
    cover_image TEXT,
    source      TEXT    NOT NULL,
    source_path TEXT    NOT NULL,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    UNIQUE(source, source_path)
);

CREATE TABLE IF NOT EXISTS ingredients (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    name      TEXT    NOT NULL,
    amount    TEXT,
    unit      TEXT,
    note      TEXT
);

CREATE TABLE IF NOT EXISTS steps (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id  INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    step_order INTEGER NOT NULL,
    content    TEXT,
    image_url  TEXT
);

CREATE TABLE IF NOT EXISTS nutrition (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id  INTEGER NOT NULL UNIQUE REFERENCES recipes(id) ON DELETE CASCADE,
    protein_g  REAL,
    fat_g      REAL,
    carbs_g    REAL,
    sodium_mg  REAL
);

CREATE TABLE IF NOT EXISTS tags (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    tag       TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_recipes_category   ON recipes(category);
CREATE INDEX IF NOT EXISTS idx_recipes_difficulty ON recipes(difficulty);
CREATE INDEX IF NOT EXISTS idx_recipes_calories   ON recipes(calories);
CREATE INDEX IF NOT EXISTS idx_ingredients_name   ON ingredients(name);
CREATE INDEX IF NOT EXISTS idx_tags_tag           ON tags(tag);

-- FTS5 full-text search (trigram tokenizer works well for Chinese)
CREATE VIRTUAL TABLE IF NOT EXISTS recipes_fts USING fts5(
    recipe_id UNINDEXED,
    name,
    description,
    ingredients_text,
    tokenize = 'trigram'
);
