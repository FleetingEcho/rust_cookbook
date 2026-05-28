#!/usr/bin/env bash
# 运行 Rust vs TypeScript 对照示例
# 用法：./rts.sh <主题名>
# 例如：./rts.sh ownership_borrowing
#       ./rts.sh strings
#       ./rts.sh          （无参数则显示所有可用主题）

set -e

TOPICS=(
    "primitives          基础类型：整数、浮点、bool、char、类型转换"
    "variables           let/let mut/const/static、遮蔽（shadowing）、解构"
    "control_flow        if表达式、loop返回值、while let、for、范围、标签循环"
    "strings             String vs &str、切片、查找、替换、分割"
    "arrays              [T;N] vs Vec<T>、增删查排序、二维数组"
    "tuples              元组、解构、多返回值、元组结构体"
    "structs             结构体（interface/class）、impl、更新语法"
    "hashmaps            HashMap、HashSet、BTreeMap、entry API"
    "enums               枚举（Discriminated Union）、带数据变体"
    "ownership_borrowing 所有权与借用：Move/Clone/Copy、&/&mut、借用规则"
    "lifetimes           生命周期：'a标注、函数/结构体引用、'static"
    "pattern_matching    模式匹配：守卫、@绑定、嵌套解构、let else"
    "traits              Trait（interface）、默认方法、dyn Trait"
    "generics            泛型、trait bound、where子句、关联类型"
    "modules             mod/pub/use（import/export）、可见性"
    "option_result       Option<T>（T|null）、Result<T,E>（try/catch）、?"
    "error_handling_advanced  自定义错误、From转换、Box<dyn Error>"
    "closures_iter       闭包（箭头函数）、Fn/FnMut/FnOnce、迭代器链"
    "smart_pointers      Box/Rc/RefCell/Arc/Mutex（对应JS GC）"
    "async_await         async/await、tokio、join!/spawn、Future惰性"
    "macros              println!/format!/vec!/dbg!/assert!/macro_rules!"
)

# 无参数：显示菜单
if [ -z "$1" ]; then
    echo ""
    echo "  Rust vs TypeScript 对照示例"
    echo "  用法: ./rts.sh <主题>"
    echo ""
    printf "  %-30s %s\n" "主题" "内容"
    printf "  %-30s %s\n" "------------------------------" "--------------------"
    for entry in "${TOPICS[@]}"; do
        topic=$(echo "$entry" | awk '{print $1}')
        desc=$(echo "$entry" | cut -d' ' -f2-)
        printf "  %-30s %s\n" "$topic" "$desc"
    done
    echo ""
    echo "  示例: ./rts.sh ownership_borrowing"
    echo ""
    exit 0
fi

TOPIC="$1"

# 检查是否是有效主题
VALID=0
for entry in "${TOPICS[@]}"; do
    t=$(echo "$entry" | awk '{print $1}')
    if [ "$t" = "$TOPIC" ]; then
        VALID=1
        break
    fi
done

if [ "$VALID" -eq 0 ]; then
    echo "❌ 未知主题: $TOPIC"
    echo "   运行 ./rts.sh 查看所有可用主题"
    exit 1
fi

echo ""
echo "  运行: rts_$TOPIC"
echo ""
cargo run -p learning_notes --example "rts_$TOPIC"
