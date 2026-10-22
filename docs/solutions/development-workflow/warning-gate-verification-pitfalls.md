---
title: "把「0 编译告警」做成门禁时踩的两个核验坑：ANSI 颜色码让 grep 漏报、CRLF 工作树让本地 lint 假红"
date: 2026-10-22
last_refreshed: 2026-10-22
category: development-workflow
module: 全仓（oa4rust / laiyipao / augmentor / auto-checkin / llm-lab / cool）+ .github/workflows + scripts
problem_type: workflow_improvement
component: build_verification
severity: medium
symptoms:
  - "写好的「构建日志 0 告警」断言脚本只数出 8 条，实际构建里有 9 条告警（漏掉 Vite 配置那条）"
  - "grep -E '^\\(!\\)' 对着一行肉眼可见的 `(!) Your Vite config uses ...` 返回 0 匹配"
  - "本地 pnpm lint（biome check --error-on-warnings）报 1 个格式错误，但同一份文件在 CI 上一直是绿的，git status 也是干净的"
root_cause: ansi_escapes_and_crlf_worktree
resolution_type: gate_hardening
tags: [warning-gate, ansi, grep, ere, crlf, autocrlf, gitattributes, biome, vite, lightningcss, fail-loud]
applies_when:
  - "给某个子项目加「构建日志必须无告警」的断言，且要 grep Vite/Rollup 的输出"
  - "本地 lint/格式化门禁报错，但 CI 绿、git status 干净，怀疑是工作树行尾问题"
related_components:
  - scripts/assert-no-warnings.sh
  - scripts/check-warnings.mjs
---

# 把「0 编译告警」做成门禁时踩的两个核验坑

## Context

2026-10-22 把全仓的编译告警清零（Rust/Go/TS×5/Python×2/Java 共 12 项检查），并把「0 告警」写进各工具链的配置。清零本身只动了 3 类存量告警，但**做门禁的过程**踩了两个坑，都是「门禁看起来在工作，其实在撒谎」这一类，值得单独记下来。

## 1. ANSI 颜色码 + ERE 括号：断言漏报一条

### 现象

`scripts/assert-no-warnings.sh` 第一版判据是：

```bash
grep -inE '^\(!\)|\[lightningcss|(^|[[:space:]])warning:' "$log"
```

对同一份构建日志，脚本报 8 条，实际是 9 条 —— 漏掉的正是 Vite 那条 `(!) Your Vite config uses features that are unsupported by configLoader: 'native'`。

### 两个独立原因，叠加

**原因一：ERE 里的 `\(` 不匹配字面括号。** 实测：

```bash
grep -n "(!)"  build.log | head -3   # 命中第 3 行
grep -cE '^\(!\)' build.log          # 0   ← 期望 1
grep -cE '^[(]!'  build.log          # 仍然 0（因为原因二）
```

在 POSIX ERE 里 `\(` 是**未定义行为**（`(` 本身已经是元字符，`\(` 想表达字面括号只能靠 `[(]`）。要用字符类 `[(]`。

**原因二（更隐蔽）：Vite/Rollup 的告警行前面有 ANSI 颜色码。** `od -c` 看那一行：

```
0000000 033 [ 3 3 m ( ! )   Y o u r   V i t e   c o n f i g ...
```

即 `\033[33m`（黄色）开头。任何 `^` 锚定的模式都匹配不到，除非先剥色。

### 修法

```bash
# 先剥 ANSI，再匹配；括号用字符类
sed $'s/\x1b\\[[0-9;]*m//g' "$log" \
  | grep -inE '^[(]!|\[lightningcss|(^|[[:space:]])warning:'
```

`scripts/check-warnings.mjs` 里同理：`output.replace(/\u001b\[[0-9;]*m/g, '')` 之后再逐行匹配。

### 为什么这条值得记

告警行的**颜色码是终端才有的东西**，本地跑有、CI 里 tee 到文件时可能没有 —— 于是同一个断言在本地和 CI 上的命中数不同，而两边都不会报错。判据必须在「剥色后」的文本上定义，否则门禁的严格程度取决于跑它的环境。

## 2. CRLF 工作树 vs LF 索引：本地 lint 假红

### 现象

`pnpm lint`（oa4rust，biome `check --error-on-warnings .`）报：

```
scripts\deepen_form.cjs format ━━━━━
  × Formatter would have printed the following content:
      1 │ - const·fs·=·require('node:fs')␍
```

即 biome 认为该文件需要重排（`␍` = CRLF）。但：

- `git status --porcelain oa4rust/scripts/deepen_form.cjs` → **空**（git 认为干净）；
- 该文件在 CI 上从来没红过。

### 判据：不要信 `git status`，要比对象哈希

```bash
git ls-files -s oa4rust/scripts/deepen_form.cjs   # 索引里的 blob：2fbe1cf0...
git hash-object  oa4rust/scripts/deepen_form.cjs  # 磁盘字节的 blob：63cf3c08...  ← 不一致
git cat-file -p HEAD:oa4rust/scripts/deepen_form.cjs | head -c 60 | od -c  # 索引侧是 LF
```

`git hash-object`（不带 `--path`）不做任何过滤，量的是**磁盘真实字节**。两者不等 = 工作树内容与索引不同，哪怕 `git status` 说干净。

### 修法

按索引内容把文件重写一遍（内容与索引完全一致，因此**不产生 diff**）：

```bash
git cat-file -p HEAD:oa4rust/scripts/deepen_form.cjs > oa4rust/scripts/deepen_form.cjs
# 校验：两个哈希应相同
git hash-object oa4rust/scripts/deepen_form.cjs
git ls-files -s oa4rust/scripts/deepen_form.cjs
```

之后 `pnpm lint` 恢复全绿（267 files, 0 error）。

### 为什么会出现

本机 `git config` 三层不一致：PortableGit 的**系统级** gitconfig 是 `core.autocrlf=true`，用户级与仓库级都是 `false`；而根 `.gitattributes` 只强制了 `*.sh/*.ts/*.tsx/*.vue/*.json/*.mjs/*.css/*.html/*.rs/*.toml/*.md/*.py/*.yml/*.yaml/*.sql`，**没有 `*.js` / `*.cjs`**。于是这几个后缀的文件既没有强制 LF 规则，又可能被某个环节（历史检出、脚本写入、编辑器）写成 CRLF。

**不要**为了让这一条消失就往 `.gitattributes` 里加 `*.js text eol=lf`：那会让所有现存 `*.js`/`*.cjs` 在下一次检出处触发重规范化，产生大规模无意义 diff。正确做法是按需把**被污染的那一个文件**按索引内容重写。

### 为什么这条值得记

「CI 绿 + 本地红」的组合最容易被误判成「本地环境坏了」，进而去改 `.gitattributes` / 关掉 lint。真正的判据是 `git hash-object` vs `git ls-files -s`，一行命令就能把「仓库问题」和「工作树污染」分开。

## 3. 顺带确立的两条门禁设计原则

1. **退出码不是判据。** Vite / Rollup / lightningcss / Python 的 `SyntaxWarning` 都**不改退出码**：构建照样 `exit 0`，告警只在日志里滚过去。所以「0 告警」必须显式断言日志（`scripts/assert-no-warnings.sh`），或把告警升级成错误（Rust `warnings = "deny"`、Java `-Werror`、Python `-W error::SyntaxWarning`）。
2. **门禁自身要有负向验证。** 断言脚本写完，用「修复前的日志」跑一遍确认它会红（本次实测：修复前 9 条 → exit 1，修复后 0 条 → exit 0）；Rust 侧注入一个 `let unused_variable = 1;` 确认 `cargo check` 以 `-D unused-variables implied by -D warnings` 失败（exit 101）。否则「一直是绿的」可能只是「从来没检查过」。

## 相关

- `scripts/check-warnings.mjs` —— 12 项一键核验（含「workspace 成员是否都继承 `[lints] workspace = true`」的静态守卫）
- `scripts/assert-no-warnings.sh` —— 构建日志的 0 告警断言（已接入 oa4rust-web / laiyipao-ci / ai-platform-ci）
- 仓库根 `README.md` § 编译告警零容忍
