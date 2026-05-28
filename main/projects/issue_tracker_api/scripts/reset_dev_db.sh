#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DB_DIR="$ROOT_DIR/data"
UPLOAD_DIR="$ROOT_DIR/storage/uploads"
DB_PATH="$DB_DIR/issue_tracker.db"

mkdir -p "$DB_DIR" "$UPLOAD_DIR"
rm -f "$DB_PATH"
find "$UPLOAD_DIR" -type f ! -name ".gitkeep" -delete

sqlite3 "$DB_PATH" < "$ROOT_DIR/migrations/0001_init.sql"
sqlite3 "$DB_PATH" < "$ROOT_DIR/scripts/seed.sql"

echo "Seeded $DB_PATH"

