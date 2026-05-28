#!/usr/bin/env bash
# ── cargo-watch 开发热重载 ─────────────────────────────────────────────────
# 需要先安装: cargo install cargo-watch
#
# 用法:
#   ./watch.sh                 # 默认: check + test (api)
#   ./watch.sh api             # 仅 API 项目
#   ./watch.sh web             # 仅 Web 前端 (vite)
#   ./watch.sh all             # API check + test + web dev 同时

set -euo pipefail
cd "$(dirname "$0")/main"

case "${1:-api}" in
  api)
    echo "▶ Watching API: check + test..."
    cargo watch -C projects/issue_tracker_api -x check -x test
    ;;
  web)
    echo "▶ Starting web dev server..."
    cd projects/issue_tracker_web
    npm run dev
    ;;
  all)
    echo "▶ Starting API watcher + web dev server..."
    cargo watch -C projects/issue_tracker_api -x check -x test &
    API_PID=$!
    cd projects/issue_tracker_web
    npm run dev &
    WEB_PID=$!
    trap "kill $API_PID $WEB_PID 2>/dev/null" EXIT
    wait
    ;;
  *)
    echo "Usage: $0 {api|web|all}"
    exit 1
    ;;
esac
