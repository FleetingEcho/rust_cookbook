#!/bin/bash
set -e

# 如果 cargo 不在 PATH，自动从常见位置补上
if ! command -v cargo &>/dev/null; then
    for p in \
        "$HOME/.cargo/bin" \
        "/mnt/c/Users/$USER/scoop/shims/apps/rustup/current/.cargo/bin" \
        "/mnt/c/Users/$USER/.cargo/bin"
    do
        if [ -x "$p/cargo" ] || [ -x "$p/cargo.exe" ]; then
            export PATH="$PATH:$p"
            break
        fi
    done
fi

if ! command -v cargo &>/dev/null; then
    echo "Error: cargo not found" && exit 1
fi

BINS=(builder singleton factory newtype decorator adapter strategy observer state iterator command raii typestate)

echo "Building all..."
cargo build
echo ""

for bin in "${BINS[@]}"; do
    echo "========== $bin =========="
    cargo run --bin "$bin" 2>&1 | grep -v "^   Compiling\|^    Finished\|^     Running\|^warning:"
    echo ""
done

echo "All done. (${#BINS[@]} patterns)"
