#!/usr/bin/env bash
# 运行 main/src/ 下的任意学习文件
#
# 用法：
#   ./src.sh                              # 列出所有可用文件
#   ./src.sh ownership                    # 运行 src/ownership.rs
#   ./src.sh learning_additions/iterators # 运行 src/learning_additions/iterators.rs
#   ./src.sh advanced/compiled_examples   # 运行 src/advanced/compiled_examples.rs
#   ./src.sh learning_additions/iterators --test  # 只跑测试
#
# 原理：
#   单个 fn main → 复制为临时 examples/_run.rs，用 cargo run 执行后删除
#   无 fn main + 有 #[test] → 自动用 cargo test
#   多个 fn main → 笔记风格，提示用 playground.rs

set -e

SRC_ROOT="main/src"
TMP_EXAMPLE="main/examples/_run.rs"

# grep -c 在无匹配时返回 exit code 1，用这个函数安全计数
count_pattern() {
    grep -c "$1" "$2" 2>/dev/null || true
}

cleanup() { rm -f "$TMP_EXAMPLE"; }
trap cleanup EXIT

# -------------------------------------------------------
# 无参数：列出所有文件
# -------------------------------------------------------
if [ -z "$1" ]; then
    echo ""
    echo "  main/src/ 下的学习文件"
    echo ""

    print_file() {
        local f="$1" label="$2"
        local mains tests
        mains=$(count_pattern "^fn main" "$f")
        tests=$(count_pattern "#\[test\]" "$f")
        if   [ "$mains" -gt 1 ]; then
            printf "    %-38s ⚠️  多个 fn main（笔记，用 playground）\n" "$label"
        elif [ "$mains" -eq 1 ]; then
            printf "    %-38s ✅ 可直接运行\n" "$label"
        elif [ "$tests" -gt 0 ]; then
            printf "    %-38s 🧪 有测试（自动用 --test）\n" "$label"
        else
            printf "    %-38s 📋 纯模块（无 main/test）\n" "$label"
        fi
    }

    echo "  ── 顶层文件 ─────────────────────────────────────────"
    for f in "$SRC_ROOT"/*.rs; do
        base=$(basename "$f" .rs)
        [[ "$base" == "lib" || "$base" == "main" ]] && continue
        print_file "$f" "$base"
    done

    echo ""
    echo "  ── learning_additions/ ──────────────────────────────"
    for f in "$SRC_ROOT"/learning_additions/*.rs; do
        base=$(basename "$f" .rs)
        [[ "$base" == "mod" ]] && continue
        print_file "$f" "learning_additions/$base"
    done

    echo ""
    echo "  ── advanced/ ────────────────────────────────────────"
    for f in "$SRC_ROOT"/advanced/*.rs; do
        base=$(basename "$f" .rs)
        [[ "$base" == "mod" ]] && continue
        print_file "$f" "advanced/$base"
    done

    echo ""
    echo "  ── 其他子目录（文件较多，直接指定路径）────────────"
    for dir in base_type collections practice_core/core rust_by_example; do
        [ -d "$SRC_ROOT/$dir" ] || continue
        n=$(find "$SRC_ROOT/$dir" -maxdepth 1 -name "*.rs" ! -name "mod.rs" | wc -l | tr -d ' ')
        printf "    %-38s %s 个文件\n" "$dir/" "$n"
    done

    echo ""
    echo "  示例："
    echo "    ./src.sh basics/variable"
    echo "    ./src.sh learning_additions/iterators"
    echo "    ./src.sh learning_additions/ownership_borrowing --test"
    echo "    ./src.sh advanced/compiled_examples --test"
    echo ""
    exit 0
fi

# -------------------------------------------------------
# 解析参数
# -------------------------------------------------------
TARGET="${1%.rs}"
MODE="${2:-}"

# 按优先级查找文件
FILEPATH=""
for candidate in \
    "$SRC_ROOT/${TARGET}.rs" \
    "$SRC_ROOT/learning_additions/${TARGET}.rs" \
    "$SRC_ROOT/advanced/${TARGET}.rs" \
    "$SRC_ROOT/base_type/${TARGET}.rs" \
    "$SRC_ROOT/collections/${TARGET}.rs" \
    "$SRC_ROOT/practice_core/core/${TARGET}.rs" \
    "$SRC_ROOT/rust_by_example/examples/${TARGET}.rs"
do
    if [ -f "$candidate" ]; then
        FILEPATH="$candidate"
        break
    fi
done

if [ -z "$FILEPATH" ]; then
    echo "❌ 找不到文件: $TARGET"
    echo "   运行 ./src.sh 查看所有可用文件"
    exit 1
fi

MAIN_COUNT=$(count_pattern "^fn main" "$FILEPATH")
TEST_COUNT=$(count_pattern "#\[test\]"  "$FILEPATH")
MODULE=$(basename "$TARGET")

echo ""
echo "  文件: $FILEPATH"

# -------------------------------------------------------
# --test 模式
# -------------------------------------------------------
if [ "$MODE" = "--test" ]; then
    echo "  模式: cargo test (过滤: $MODULE)"
    echo ""
    cargo test -p learning_notes -- "$MODULE" --nocapture
    exit 0
fi

# -------------------------------------------------------
# 无 fn main + 有测试 → 自动 test
# -------------------------------------------------------
if [ "$MAIN_COUNT" -eq 0 ] && [ "$TEST_COUNT" -gt 0 ]; then
    echo "  ℹ️  无 fn main，自动切换到 cargo test"
    echo ""
    cargo test -p learning_notes -- "$MODULE" --nocapture
    exit 0
fi

# -------------------------------------------------------
# 多个 fn main → 笔记文件
# -------------------------------------------------------
if [ "$MAIN_COUNT" -gt 1 ]; then
    echo "  ⚠️  该文件有 $MAIN_COUNT 个 fn main（笔记风格，无法直接编译）"
    echo ""
    echo "  建议：把要测试的片段复制到 playground.rs，然后运行："
    echo "    cargo run -p learning_notes --example playground"
    echo ""
    exit 1
fi

# -------------------------------------------------------
# 无 fn main 也无测试 → 纯模块
# -------------------------------------------------------
if [ "$MAIN_COUNT" -eq 0 ]; then
    echo "  ⚠️  无 fn main 也无 #[test]（纯模块文件）"
    echo ""
    echo "  建议：在 playground.rs 中 use 并调用它的公开函数"
    exit 1
fi

# -------------------------------------------------------
# 单个 fn main → 复制为临时 example 运行
# -------------------------------------------------------
echo "  模式: cargo run（临时 example）"
echo ""
cp "$FILEPATH" "$TMP_EXAMPLE"
cargo run -p learning_notes --example _run
