---
title: "批量改写 Git 历史去真值：机制、六个坑与验证配方"
date: 2026-10-21
last_refreshed: 2026-10-21
category: development-workflow
module: 仓库根（git 历史 / .gitignore / .gitattributes）
problem_type: workflow_pitfall
component: repo_policy
severity: high
symptoms:
  - "已跟踪文件里的真实域名 / IP / 私钥路径被清掉了，但 git log 里仍然能翻出来"
  - "提交信息里回显了真值 —— 文件改了，信息还留在仓库里"
  - "重写历史后 3939 个文件、61 万行纯空白 diff，内容却一个字没变"
  - "跑完历史重写工具，磁盘上被 .gitignore 忽略的工作目录整个消失了"
  - "平时 git status 干净，索引一重建（reset --hard / filter-repo）几千个文件同时变成已修改"
root_cause: tooling_and_index_semantics
resolution_type: documented_procedure
tags: [git, filter-repo, rebase, gitignore, gitattributes, crlf, secrets, history-rewrite, backup]
applies_when:
  - "仓库推到公开远端，需要把已推送历史里的真值清掉"
  - "批量替换仓库内的绝对路径 / 域名 / 私钥路径"
  - "提交信息里误写了不该公开的内容"
  - "用 git filter-repo 或交互式 rebase 改写历史"
related_components:
  - ../security-issues/secret-and-environment-info-governance.md
  - AGENTS.md
  - .gitignore
  - .gitattributes
---

# 批量改写 Git 历史去真值

## Problem

治理文档（`../security-issues/secret-and-environment-info-governance.md`）给的是**规则与判据**：
什么算真值、用什么占位符、守卫怎么写。它没覆盖**机制**：

- 提交信息也是仓库内容，`git log` 公开可见 —— 文件脱敏了，信息里还留着真值；
- 已经推送的历史不会因为改了当前文件而消失，GitHub 上按 SHA 仍可达；
- 改写历史的工具（`git filter-repo`、交互式 rebase）**有自己的脾气**，
  在 Windows 上踩了六个坑，其中两个造成了**大范围损坏**。

本篇把机制、可复制的操作顺序、六个坑与验证配方写下来。

## Resolution

### 操作顺序（先备份，再动刀，最后对账）

```bash
# 1) 全量备份（所有 refs；包括你没打算改的那些）
git bundle create "$TEMP/opencode/pre-scrub-$(date +%F).bundle" --all

# 2) 记下改写前的基线，供事后逐字节对账
PRE=$(git rev-parse HEAD)
git diff --stat $PRE HEAD          # 改写后应只剩「该改的那些」

# 3) 真值清单（literal 逐条；正则要写清转义）
cat > /tmp/scrub-expressions.txt <<'EOF'
literal:real.example.com==><YOUR_TEST_DOMAIN>
literal:203.0.113.10==><TEST_HOST>
literal:C:/Users/<REAL_USER>==><HOME>
EOF

# 4) 关掉 autocrlf（见坑 1），再重写
git config core.autocrlf false
git filter-repo --force \
  --replace-text  /tmp/scrub-expressions.txt \
  --replace-message /tmp/scrub-expressions.txt \
  --refs $(git for-each-ref --format='%(refname)' refs/heads refs/remotes)
git config core.autocrlf true
```

### 验证配方（每一条都要跑，不能只跑一条）

```bash
# A. 真值还在不在（历史 + 当前树 + 所有远端跟踪分支）
git log --all --oneline -S '<真值>'          # 应为空
git grep -l -F '<真值>' HEAD -- .            # 应为空
for r in $(git for-each-ref --format='%(refname:short)' refs/remotes); do
  git log "$r" --oneline -S '<真值>' | head -1
done

# B. 改写是否只动了该动的（**忽略空白**后必须为空）
git diff -w --stat $PRE HEAD

# C. 提交信息里有没有
git log --all --format='%H %s %b' | grep -F '<真值>'

# D. 工作区与索引是否干净
git status --porcelain
```

## 六个坑（全部实测踩过）

### 坑 1：Windows 上 `core.autocrlf=true` 会把 blob 写成 CRLF

**症状**：重写后 `git diff --stat` 显示 3939 个文件、611338 行增删，
但 `git diff -w` **完全为空** —— 内容一个字没变，只有行尾变了。

**根因**：filter-repo 把 blob 往返经过工作树转换，LF 被写成了 CRLF。
于是「存储形态」被整体改写，等于给全仓加了一次无意义的空白 diff。

**处置**：跑之前 `git config core.autocrlf false`，跑完恢复。
判据是 B（`git diff -w` 为空）。

### 坑 2：filter-repo 的清理会 `git clean`，连被 ignore 的本地目录一起删

**症状**：`.workbuddy/`、`.workbuddy-ai/`、`.zcode/` 在磁盘上变成 **0 个文件**，
但 git 里它们只是「untracked」，看起来一切正常。

**根因**：工具的 cleanup 阶段清掉未跟踪/被忽略的文件。
`.gitignore` 保护的是「不被提交」，**不是**「不被删」。

**处置**：
- 跑之前把这类目录**挪出工作树**（或记下文件数，跑完对账）；
- 已经删了就从历史恢复：`git checkout <sha> -- <dir>` 再 `git rm -r --cached` 保持不跟踪；
- 未提交的本地修改只会留在对象库里（`git fsck --dangling`），**捞回来要靠签名匹配**，别指望目录还在。

### 坑 3：`--refs` 不会重写 `refs/remotes/*`

**症状**：本地分支干净了，`origin/*` 仍有命中。

**处置**：单独再跑一遍只针对远端跟踪引用，然后把它们推回去：

```bash
git filter-repo --force --replace-text ... --refs $(git for-each-ref --format='%(refname)' refs/remotes)
git push --force origin 'refs/remotes/origin/<group>/*:refs/heads/<group>/*'
```

### 坑 4：`.gitignore` 只对**未跟踪**文件生效

想真正停止跟踪，光加规则没用（已跟踪的文件照旧提交）：

```bash
git rm -r --cached <dir>      # 停止跟踪，文件留在磁盘
git add -f <path>              # 反向：强行恢复跟踪
```

### 坑 5：仓库里潜伏着「blob 含 CRLF、而 .gitattributes 要求 LF」的不一致

**症状**：平时 `git status` 干净；一旦索引被重建（`reset --hard` /
filter-repo / `add --renormalize`），几千个文件同时显示"已修改"。

**根因**：git 平时靠 index 的 stat 缓存（mtime/size）**跳过内容比对**，
索引一重建就真去比 —— 于是「一直存在但一直隐形」的不一致被暴露。

**处置**：`git add --renormalize <path>` 后单独提交（纯空白），
并用 `git diff -w --cached` 确认零内容差异。

### 坑 6：`git rebase -i HEAD~N` 的范围是**容易数错一位**的

`HEAD~N` 是**基准**（不重放），重放的是它**之后**的 N 个提交。
想包含某条指定提交，锚点要用它的父提交：

```bash
git rebase -i '<sha>^'          # 包含 <sha> 本身
```

连错两次的后果是「改完再改」：第一遍漏掉了真正含真值的那条，
第二遍才发现它还在历史里。

## 附带两个 rebase 小坑

### 脚本里 `git add -A` 会把别人的工作树改动卷进提交

rebase 的 `exec` 钩子里做内容改写时，**只 `git add` 真正改过的文件**。
`git add -A` 会把使用者同时在编辑的文件一起提交走 —— 实测就这么吞掉过别人的改动。

### todo 编辑器脚本里的 awk 会重复打印

```awk
/^pick / { print; print "exec " scrub; next }   # ← 缺 next 就重复
{ print }
```

缺 `next` ⇒ 每个 `pick` 打印两次 ⇒ 第二个 pick 必然是「空提交」，
rebase 会停下来报 `The previous cherry-pick is now empty`。

## 提交信息里的真值：单独处理

文件脱敏不等于脱敏完成 —— 提交信息同样公开。

```bash
# 用 GIT_EDITOR 脚本统一清洗提交信息（幂等：已是占位符的不动）
GIT_SEQUENCE_EDITOR='sh todo-editor.sh' GIT_EDITOR='sh msg-scrub.sh' \
  git rebase -i '<sha>^'
```

改写时若内容也变了，后续提交的补丁会与已脱敏的内容**冲突**：
解法是取「已经脱敏的那一侧」（`git checkout --theirs -- <file>`）——
两边都是占位符版本，取谁都安全。

## 回滚

```bash
# 从 bundle 恢复所有 refs
git fetch "$BUNDLE" '+refs/heads/*:refs/backup-bundle/*'
for r in $(git for-each-ref --format='%(refname)' refs/backup-bundle); do
  n=${r#refs/backup-bundle/}
  git update-ref "refs/heads/$n" "$r"
done
git reset --hard main
```

## When to Apply

- 仓库推到公开远端，且真值/定位信息已经进过历史；
- 批量替换仓库内的绝对路径、域名、主机名；
- 提交信息误写了不该公开的内容；
- 任何要动历史（filter-repo / rebase / filter-branch）的场合。

## Related

- 规则与判据：`../security-issues/secret-and-environment-info-governance.md`
- 守卫形态：`laiipao/server/internal/domain/secrets_leak_test.go`
- 仓库根规则：`AGENTS.md`「Secrets & Environment Data」
