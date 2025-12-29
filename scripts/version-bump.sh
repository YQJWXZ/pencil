#!/usr/bin/env bash
set -euo pipefail

# 错误处理
error() { echo "ERROR: $*" >&2; exit 1; }
info() { echo "INFO: $*"; }

# 读取 VERSION 文件
VERSION_FILE="VERSION"
[[ -f "$VERSION_FILE" ]] || error "VERSION file not found"

current_version=$(cat "$VERSION_FILE" | tr -d '[:space:]')
[[ -n "$current_version" ]] || error "VERSION file is empty"

# 解析版本号（支持 pre-release）
# 格式: MAJOR.MINOR.PATCH 或 MAJOR.MINOR.PATCH-prerelease
if [[ $current_version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)(-([a-zA-Z0-9.]+))?$ ]]; then
    major="${BASH_REMATCH[1]}"
    minor="${BASH_REMATCH[2]}"
    patch="${BASH_REMATCH[3]}"
    prerelease="${BASH_REMATCH[5]}"
else
    error "Invalid version format: $current_version"
fi

info "Current version: $current_version (major=$major, minor=$minor, patch=$patch, pre=$prerelease)"

# 获取 commit message（优先从参数，否则从 .git/COMMIT_EDITMSG）
if [[ $# -gt 0 ]]; then
    commit_msg="$1"
else
    commit_msg_file=".git/COMMIT_EDITMSG"
    [[ -f "$commit_msg_file" ]] || error "No commit message provided and $commit_msg_file not found"
    commit_msg=$(cat "$commit_msg_file" | head -1)
fi

info "Commit message: $commit_msg"

# 判断版本递增类型
bump_type="none"

# 检查 BREAKING CHANGE
if [[ $commit_msg =~ ^[a-z]+(\(.+\))?!: ]] || echo "$commit_msg" | grep -q "BREAKING CHANGE"; then
    bump_type="major"
    info "Detected BREAKING CHANGE → major version bump"
# 检查 feat
elif [[ $commit_msg =~ ^feat(\(.+\))?: ]]; then
    bump_type="minor"
    info "Detected feat → minor version bump"
# 检查 fix
elif [[ $commit_msg =~ ^fix(\(.+\))?: ]]; then
    bump_type="patch"
    info "Detected fix → patch version bump"
else
    info "Commit type does not require version bump (docs/style/refactor/test/chore)"
    exit 0
fi

# 执行版本递增
case $bump_type in
    major)
        major=$((major + 1))
        minor=0
        patch=0
        ;;
    minor)
        minor=$((minor + 1))
        patch=0
        ;;
    patch)
        patch=$((patch + 1))
        ;;
esac

# 构建新版本号（暂不处理 pre-release 递增，清除 pre-release 标识）
new_version="$major.$minor.$patch"

info "New version: $new_version"

# 写回 VERSION 文件
echo "$new_version" > "$VERSION_FILE"

# 同步到 Cargo.toml
bash scripts/sync-version.sh

# 自动 stage 修改的文件（在 prepare-commit-msg 阶段可以修改 staging area）
git add "$VERSION_FILE" rust/Cargo.toml rust/Cargo.lock

info "✓ Version bumped from $current_version to $new_version"
info "Files staged: VERSION, Cargo.toml, Cargo.lock"
