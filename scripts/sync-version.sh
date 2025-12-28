#!/usr/bin/env bash
set -euo pipefail

# 错误处理函数
error() { echo "ERROR: $*" >&2; exit 1; }
info() { echo "INFO: $*"; }

# 检查 VERSION 文件是否存在
VERSION_FILE="VERSION"
[[ -f "$VERSION_FILE" ]] || error "VERSION file not found"

# 读取版本号
VERSION=$(cat "$VERSION_FILE" | tr -d '[:space:]')
[[ -n "$VERSION" ]] || error "VERSION file is empty"

# 验证版本号格式（Semantic Versioning）
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
    error "Invalid version format: $VERSION (expected: x.y.z or x.y.z-prerelease)"
fi

info "Current version: $VERSION"

# 同步到 Rust 项目
CARGO_TOML="rust/Cargo.toml"
if [[ -f "$CARGO_TOML" ]]; then
    info "Syncing version to $CARGO_TOML..."
    # 使用 sed 替换 version 字段（仅替换第一个匹配项）
    sed -i "0,/^version = \".*\"/s//version = \"$VERSION\"/" "$CARGO_TOML"
    info "✓ Synced to Rust project"
else
    info "⚠ $CARGO_TOML not found, skipping Rust sync"
fi

# TODO: 未来扩展 - 同步到其他子项目
# if [[ -f "frontend/package.json" ]]; then
#     jq ".version = \"$VERSION\"" frontend/package.json > frontend/package.json.tmp
#     mv frontend/package.json.tmp frontend/package.json
# fi

info "Version sync completed"
