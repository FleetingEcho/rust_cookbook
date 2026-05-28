#!/usr/bin/env bash
# 运行 main/src/ 下的任意学习文件
#
# 用法：
#   ./src.sh                              # 列出所有可用文件
#   ./src.sh basics/if_else               # 运行文件
#   ./src.sh types/array                  # 运行 fn example_* 函数（自动生成 main）
#   ./src.sh learning_additions/iterators # 运行 src/learning_additions/iterators.rs
#   ./src.sh learning_additions/iterators --test  # 只跑测试
#
# 运行模式（自动判断）：
#   fn main() × 1         → 复制为临时 example，cargo run
#   fn example_*() × N    → 自动生成 main 依次调用，cargo run
#   #[test] × N           → cargo test --nocapture
#   多个 fn main（注释中） → 仍可运行（仅以实际可编译的为准）

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
        local mains tests examples
        mains=$(count_pattern "^fn main()" "$f")
        tests=$(count_pattern "#\[test\]" "$f")
        examples=$(count_pattern "^fn example_" "$f")
        if   [ "$mains" -gt 1 ]; then
            printf "    %-42s ✅ 可运行（%s 个 main）\n" "$label" "$mains"
        elif [ "$mains" -eq 1 ] && [ "$examples" -gt 0 ]; then
            printf "    %-42s ✅ 可运行（main + %s 个 example）\n" "$label" "$examples"
        elif [ "$mains" -eq 1 ]; then
            printf "    %-42s ✅ 可运行\n" "$label"
        elif [ "$examples" -gt 0 ]; then
            printf "    %-42s ▶️  可运行（%s 个 example_*）\n" "$label" "$examples"
        elif [ "$tests" -gt 0 ]; then
            printf "    %-42s 🧪 有测试，用 --test\n" "$label"
        else
            printf "    %-42s 📋 纯模块（无 main/test）\n" "$label"
        fi
    }

    print_dir() {
        local dir="$1" title="$2"
        local files=()
        while IFS= read -r f; do
            files+=("$f")
        done < <(find "$SRC_ROOT/$dir" -maxdepth 1 -name "*.rs" ! -name "mod.rs" | sort)
        [ ${#files[@]} -eq 0 ] && return
        echo ""
        echo "  ── $title ──────────────────────────────────────────"
        for f in "${files[@]}"; do
            base=$(basename "$f" .rs)
            print_file "$f" "$dir/$base"
        done
    }

    print_dir "basics"        "basics/"
    print_dir "types"         "types/"
    print_dir "structs_enums" "structs_enums/"
    print_dir "traits"        "traits/"
    print_dir "ownership"     "ownership/"
    print_dir "errors"        "errors/"

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
    echo "  ── 其他子目录（直接指定路径）────────────────────────"
    for dir in base_type collections practice_core/core rust_by_example; do
        [ -d "$SRC_ROOT/$dir" ] || continue
        n=$(find "$SRC_ROOT/$dir" -maxdepth 1 -name "*.rs" ! -name "mod.rs" | wc -l | tr -d ' ')
        printf "    %-42s %s 个文件\n" "$dir/" "$n"
    done

    echo ""
    echo "  用法："
    echo "    ./src.sh basics/if_else                           # 运行"
    echo "    ./src.sh types/array                              # 运行所有 example_* 函数"
    echo "    ./src.sh learning_additions/iterators --test      # 只跑测试"
    echo "    cargo run -p learning_notes --example playground  # 自由练习"
    echo ""
    exit 0
fi

# -------------------------------------------------------
# 解析参数
# -------------------------------------------------------
TARGET="${1%.rs}"
MODE="${2:-}"

# 按优先级查找文件（新增 basics/ types/ structs_enums/ traits/ ownership/ errors/）
FILEPATH=""
for candidate in \
    "$SRC_ROOT/${TARGET}.rs" \
    "$SRC_ROOT/basics/${TARGET}.rs" \
    "$SRC_ROOT/types/${TARGET}.rs" \
    "$SRC_ROOT/structs_enums/${TARGET}.rs" \
    "$SRC_ROOT/traits/${TARGET}.rs" \
    "$SRC_ROOT/ownership/${TARGET}.rs" \
    "$SRC_ROOT/errors/${TARGET}.rs" \
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

MAIN_COUNT=$(count_pattern "^fn main()" "$FILEPATH")
TEST_COUNT=$(count_pattern "#\[test\]"  "$FILEPATH")
EXAMPLE_COUNT=$(count_pattern "^fn example_" "$FILEPATH")
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
if [ "$MAIN_COUNT" -eq 0 ] && [ "$TEST_COUNT" -gt 0 ] && [ "$EXAMPLE_COUNT" -eq 0 ]; then
    echo "  ℹ️  无 fn main，自动切换到 cargo test"
    echo ""
    cargo test -p learning_notes -- "$MODULE" --nocapture
    exit 0
fi

# -------------------------------------------------------
# 无 fn main + 有 fn example_* → 自动生成 main 依次调用
# -------------------------------------------------------
if [ "$MAIN_COUNT" -eq 0 ] && [ "$EXAMPLE_COUNT" -gt 0 ]; then
    echo "  模式: 自动生成 main（调用 $EXAMPLE_COUNT 个 example_* 函数）"
    echo ""
    # 提取所有 fn example_xxx 函数名
    mapfile -t FUNCS < <(grep "^fn example_" "$FILEPATH" | sed 's/fn \([^(]*\).*/\1/')
    {
        cat "$FILEPATH"
        echo ""
        echo "fn main() {"
        for fn_name in "${FUNCS[@]}"; do
            echo "    println!(\"\\n── $fn_name ──\");"
            echo "    $fn_name();"
        done
        echo "}"
    } > "$TMP_EXAMPLE"
    cargo run -p learning_notes --example _run
    exit 0
fi

# -------------------------------------------------------
# 无 fn main 也无测试、无 example → 纯模块
# -------------------------------------------------------
if [ "$MAIN_COUNT" -eq 0 ]; then
    echo "  ⚠️  无 fn main / #[test] / fn example_*（纯模块文件）"
    echo ""
    echo "  建议：在 playground.rs 中 use 并调用它的公开函数"
    exit 1
fi

# -------------------------------------------------------
# 单个或多个 fn main → 复制为临时 example 运行
# -------------------------------------------------------
if [ "$MAIN_COUNT" -gt 1 ]; then
    echo "  ⚠️  检测到 $MAIN_COUNT 个 fn main()（注释中的不算）"
    echo "  模式: 取第一个 fn main() 运行"
    echo ""
    # 只保留第一个 fn main 块（截取到第二个 fn main 前）
    first_main_line=$(grep -n "^fn main()" "$FILEPATH" | head -1 | cut -d: -f1)
    second_main_line=$(grep -n "^fn main()" "$FILEPATH" | sed -n '2p' | cut -d: -f1)
    if [ -n "$second_main_line" ]; then
        head -n $((second_main_line - 1)) "$FILEPATH" > "$TMP_EXAMPLE"
    else
        cp "$FILEPATH" "$TMP_EXAMPLE"
    fi
else
    echo "  模式: cargo run（临时 example）"
    echo ""
    cp "$FILEPATH" "$TMP_EXAMPLE"
fi
cargo run -p learning_notes --example _run
