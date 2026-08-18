# fakeToys 仓库长期笔记

## Git 协作：pull/push 失败的真正根因（2026-08-16 起多次修正，最终定论见下）
- **最终根因（2026-08-17 真机 fetch 暴露，确定）**：真机本地存在一个**名尾带 `?` 的损坏分支 ref `refs/heads/feat/ai?`**，指向不存在的对象（bad object）。它导致 `git fetch` 末尾 `fatal: bad object refs/heads/feat/ai?` → `failed to perform geometric-repack` → fetch 被标记为失败；并污染对象库/ref 遍历，使 `git rev-parse --verify HEAD` 失败、整树 staged-A 假象。**之前的"本地领先6/远端领先14 分叉 + pull.ff=only"理论（2026-08-16）是错误的**——真机 .git 确属损坏态，只是坏在一个具体垃圾 ref，而非 packed-refs 整体或单纯分叉。
- **坏 ref 处理要点**：`?` 在 bash/PowerShell 都是通配符，必须用字面量——`git update-ref -d "refs/heads/feat/ai?"`（引号传参，git 把 refname 当字面量，不会 glob）最安全；勿裸写 `?` 当文件名。
- **修复流程（真机执行，已脚本化 `git-branch-tidy-safe/scripts/fix_bad_ref.ps1`）**：① 扫描 `refs/heads/*` 找 `rev-parse --verify` 失败的坏 ref → `git update-ref -d` 删之（需 'y' 确认）；② `git fetch origin`（geometric repack 不再遇坏 ref，应成功）；③ 若 HEAD 可解析则 stash 保全真实未提交改动；④ `git checkout -B feat/oa4rust-completion-phase0-1 origin/feat/oa4rust-completion-phase0-1` re-anchor 清假象。绝不 `git add -A` / 盲 `reset --hard`。
- **pack-refs 损坏的终极修法（2026-08-17 定论）**：反复"原地重写 `.git/packed-refs` 文本"极不可靠（会连踩 BOM / 空行 / `#` 头三道边角）。正确做法：**一旦 `git for-each-ref` 因 packed-refs 报错，就把 `.git/packed-refs` 整体移开（先留 `.bak`）**，git 改用 loose refs，fetch 即通；随后 `git pack-refs --all --prune` 让 git 自己生成全新规范 packed-refs。不丢任何对象。**一键脚本 `D:\WORKSPACE\fakeToys\recover_repo.ps1`**（备份→清 `feat/ai?`→移开坏 packed-refs→fetch→从 origin/* 重建 4 分支→stash-safe re-anchor→pack-refs→校验），真机跑：`powershell -ExecutionPolicy Bypass -File "D:\WORKSPACE\fakeToys\recover_repo.ps1"`。
- **PowerShell 脚本坑（重要）**：中文 Windows 下 PS 5.1 默认按 GBK 读**无 BOM** 的 `.ps1`，会把 UTF-8 中文误读、破坏双引号边界报"字符串缺少终止符"。含中文的 .ps1 **必须带 UTF-8 BOM**：`[System.IO.File]::WriteAllBytes($p, ([System.Text.Encoding]::UTF8.GetPreamble() + [System.Text.Encoding]::UTF8.GetBytes($text)))`（`WriteAllText` 在本环境不加 BOM）。注意区分：这是**脚本文件自身**的 BOM 要求；而 `.git/packed-refs` 内容则**禁止** BOM（git 拒读），写回须 `UTF8Encoding($false)` 无 BOM。
- **远端 tip 仍有效**：`git ls-remote` 核实远端 tip = `9f6422f9`（2026-08-16 detached-HEAD 合并并 push 的结果），re-anchor 目标即此。
- **沙箱伪影（勿再追，但勿误判真机）**：本沙箱 `.git` 是 overlay 占位桩（54 条 ref 全零 SHA + CRLF），且抑制对 .git 的写入（不持久）。**agent 无法在沙箱替用户在真机 .git 上落地删 ref / checkout -B 等写操作**——凡涉及 `.git` 结构的修复须用户在真机终端执行（agent 可写脚本落真机目录供其运行）。注意：HEAD 不可解析 + 整树 staged-A 假象在真机 refs 损坏时也会出现，勿仅凭症状武断判定为沙箱；须先向用户确认运行环境。
- **配置建议（已实装）**：`pull.ff` 于 2026-08-17 由 `only` 改为 `false`（分叉时走 merge，根治 pull 直接 abort）；`pull.rebase=true` 为备选（线性历史但需干净工作树）。`core.autocrlf` 保持 false。**agent 遗留未提交改动 + 本地提交未推送 + 远端有新提交 = 分叉 → pull/push 同时失败**，标准解法：`stash` 脏树 → `rebase origin/<branch>` → `push`（fast-forward）→ `stash pop`。
- **危险假象**：损坏 refs 下 `git status` 会显示整个工作树 staged-A（branch has no commits yet）。**切勿 `git commit`**，那会造出游离根提交。

## 环境约束（沙箱）
- 沙箱抑制 `.git` 写入（loose ref / reset 跨命令不持久），但**工作树文件写入持久**。故涉及 `.git` 结构的修复必须在用户真机终端执行。
- **勿武断归因**：本 agent 沙箱 `.git` 虽是 overlay stub，但"HEAD 不可解析 + 整树 staged-A 假象"在用户**真机**也会出现（真机 refs 损坏/混乱时）。2026-08-17 曾连续误判用户真机 PowerShell 运行为沙箱，须**先向用户确认运行环境**再下结论，而非仅凭症状猜测。
- 跨空间上下文：用户 annsshadow 关注"AI 是否看得到其他空间上下文"（见 2026-08-13.md）；用 WorkBuddy 时各 workspace 记忆隔离，需主动说明。
