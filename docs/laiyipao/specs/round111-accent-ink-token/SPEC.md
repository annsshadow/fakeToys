# R111 规范：--accent-ink 令牌真源缺失（test-first RED）

## §0 立轮依据

- 侦察 A 报告：docs/laiyipao/recon/ROUND111_A_VISUAL_RECON.md（四条件全中，判立轮）。
- 基线 HEAD：d23ea384ce12d92404e4aaac895a4f6e2bb0cbd8（未提交，全程未变）。
- 权威唯一真源：docs/laiyipao/design/AI_VISUAL_SYSTEM.md 第 212 / 268 / 277 / 896 / 949 行，逐字指定 --accent-ink。
- B 路（对比度 chip 角度）已证伪，无候选；本轮聚焦令牌真源缺失。

## §1 缺陷陈述

- --accent-ink 在全部 laiyipao 目录零定义（ops_grep 正则 --accent-ink\s*: 0 命中）。
- 却被 laiyipao/miniapp/src/styles/common.css:1286（.r3-row-value）、:1304（.r3-filter .r3-chip.active）、:1355（.r3-empty-action，任务描述称 .r3-cta，以源码为准）消费为 color: var(--accent-ink)。
- tokens.css:57 注释声明「文字用 --accent-ink」，但 ink 色段（:61–:65）只有 fire/ice/lightning/corrosion/kinetic，无 accent-ink，变量未定义导致文字回退继承色 --text:#e6edf3。
- 真 Chromium 坐实：skills/equipment/shop 三页 .r3-chip.active computed color=rgb(230,237,243)、--accent-ink 计算值为空；活动筛选态失去提亮橙层级提示。
- 假守卫：laiyipao/miniapp/src/styles/r3SecondaryPages.contract.test.ts:33/39 只断言 css 含 var(--accent-ink) 字样，不验证令牌在 tokens.css 中真实定义——令牌缺失时契约仍绿。

## §2 修复目标（写入规范，实施轮执行；本轮不改任何业务文件）

1. laiyipao/miniapp/src/styles/tokens.css ink 色段（:65 --kinetic-ink 行后）补一行：--accent-ink: #ff8659;，注明「= --fire-ink 同值，活动态/CTA 文字专用」。
2. common.css:1286/1304/1355 三个消费点不动（定义补齐即生效）。
3. tokens.css:14-16 令牌三锚点（v1 / v3-r3 / r6）不动。
4. --fire-ink: #ff8659 原定义行不改（新值与其同值，但属不同令牌）。
5. admin 端 tokens.css:58 --el-fire-ink: #ff8659 与 main.css:551 已合规，不动。
6. 业务样式零 --r111-。

## §3 唯一色值依据（WCAG 复算，tokens.css 实际色值）

| 文字色 | soft+panel | soft+panel-2 | soft+panel-3 | 判定 |
|---|---|---|---|---|
| #ff8659（fire-ink / 拟补 accent-ink） | 5.80 | 5.19 | 4.62 | 吻合权威 5.20/4.63，合格 |
| #ff6b35（accent） | 4.88 | 4.36 | 3.88 | 未过 4.5 红线，权威明令禁止 |
| #e6edf3（当前回退 --text） | 11.71 | 10.47 | 9.32 | 数值过线但视觉层级错误（白字非橙字） |

权威第 212 行给的 5.20/5.23 与 4.63/4.67 只能由 #ff8659 满足；色值唯一锁定。

## §4 契约结构（r111AccentInkToken.contract.test.ts）

- 真实 fs.readFileSync 读 tokens.css、common.css、r3SecondaryPages.contract.test.ts、admin tokens.css。
- 缺口 it ×2（改前必须真实失败）：
  - G1：tokens.css 的 :root 色段内必须存在 --accent-ink: #ff8659;（正则锚在 :root 块内）。
  - G2：--accent-ink 值必须与 --fire-ink 同值（权威双端同值口径）。
- 堵假守卫 it ×1（改前通过）：记录 r3SecondaryPages.contract.test.ts 只查 var(--accent-ink) 字样不查定义属盲区，本契约补上定义检查。
- 不变量 it ×5（改前全部通过）：
  - ① common.css 三消费点 var(--accent-ink) 仍在且不动；
  - ② tokens.css 三锚点（v1 / v3-r3 / r6）不变；
  - ③ --fire-ink: #ff8659 原定义不变且唯一；
  - ④ admin --el-fire-ink: #ff8659 不被改；
  - ⑤ 业务样式零 --r111-。

## §5 验证命令与预期

命令：cd laiyipao/miniapp && npx vitest run src/styles/r111AccentInkToken.contract.test.ts --reporter=verbose

- RED 预期：exit 1，2 failed | 6 passed (8)；两条失败均为真实缺口（G1/G2），六条不变量/守卫全绿，无 PARSE_ERROR。
- GREEN 预期（实施轮）：exit 0，8 passed (8)。
- 队长独立复核：读 RED-raw.log 逐字日志 + 复跑。
