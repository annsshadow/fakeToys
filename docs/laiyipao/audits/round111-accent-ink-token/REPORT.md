# R111 QA：--accent-ink 令牌生效真浏览器验收报告

- 任务：t329（verification）｜执行：same-route-qa｜基线 HEAD：d23ea384ce12d92404e4aaac895a4f6e2bb0cbd8（实施在工作树未提交，全程未变）
- 验证对象：R111 唯一改点 laiyipao/miniapp/src/styles/tokens.css:66 补 --accent-ink: #ff8659；三消费点（common.css 的 .r3-row-value / .r3-filter .r3-chip.active / .r3-empty-action）computed 文字色由回退白字变提亮橙。
- 三视口：390×844 / 768×1024 / 1280×800；真 Chromium（agent-browser 0.41.1）；三页 skills / equipment / shop。

## §1 范围与方法

- 后端 8080 未起，H5 dev server 自启 5174（pnpm run dev:h5 -- --host 127.0.0.1 --port 5174）。全部数据经 init-script 合成（patch XHR.open/send 直接回包 + patch fetch + localStorage 注入配置缓存/占位令牌），零真值。
- 探针 data/probe-ink.js 读 getComputedStyle(document.documentElement).getPropertyValue 取 --accent-ink、三消费点 computed color/bg、root 色变量、cw/sw/hOverflow，并在浏览器内按 WCAG 公式复算 accent-soft 叠 panel-2/panel-3 对比度。
- 每视口每页：close --all → open 页面路由 #/pages/<pg>/<pg>（带 init-script）→ open 后重设 viewport → sleep 2 → eval 探针 → 截图。9 组探针落 data/ink-<pg>-<W>.txt，9 张截图落 screenshots/。
- 改前强对照引用侦察 A 报告 docs/laiyipao/recon/ROUND111_A_VISUAL_RECON.md §3（改前 --accent-ink 计算值为空、.r3-chip.active color=rgb(230,237,243)），本轮未注入旧 CSS 重做 A/B。

## §2 三视口逐页实测表

| 视口 | 页 | root --accent-ink | .r3-chip.active computed color | 其余消费点 | WCAG soft+panel2 / panel3 | cw/sw | 横溢 | 登录页 |
|---|---|---|---|---|---|---|---|---|
| 390 | skills | #ff8659 | rgb(255,134,89) | row-value 0 / empty-action 0 | 5.19 / 4.62 | 390/390 | false | false |
| 390 | equipment | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 390/390 | false | false |
| 390 | shop | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 390/390 | false | false |
| 768 | skills | #ff8659 | rgb(255,134,89) | row-value 0 / empty-action 0 | 5.19 / 4.62 | 768/768 | false | false |
| 768 | equipment | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 768/768 | false | false |
| 768 | shop | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 768/768 | false | false |
| 1280 | skills | #ff8659 | rgb(255,134,89) | row-value 0 / empty-action 0 | 5.19 / 4.62 | 1280/1280 | false | false |
| 1280 | equipment | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 1280/1280 | false | false |
| 1280 | shop | #ff8659 | rgb(255,134,89) | empty-action 1 = rgb(255,134,89) | 5.19 / 4.62 | 1280/1280 | false | false |

- A 令牌定义：9/9 组 root --accent-ink = #ff8659（非空），--fire-ink = #ff8659、--accent = #ff6b35、--text = #e6edf3 同值稳定。改前强对照：recon A 报告改前全部为空。
- B 消费点生效：9/9 组 .r3-chip.active computed color = rgb(255,134,89)（提亮橙），改前 rgb(230,237,243) 白字回退已消除；equipment/shop 的 .r3-empty-action（第三消费点）6/6 组同样 = rgb(255,134,89) 额外坐实。.r3-row-value 本轮 9 组均未渲染（0 命中），按诚实边界标注，不冒充坐实；其消费关系由契约 it① 静态守。
- C 对比度：浏览器内按 WCAG 复算，#ff8659 文字叠 rgba(255,107,53,0.18) soft 后：panel-2 = 5.19，panel-3 = 4.62，均 ≥4.5，与权威 5.20/4.63 口径吻合；截图肉眼确认活动芯片为提亮橙字叠橙底（改前为暗淡白字）。

## §3 改前基线对照

| 项 | 改前（recon A） | 本轮实测 |
|---|---|---|
| root --accent-ink | 空字符串 | #ff8659 |
| .r3-chip.active color | rgb(230,237,243)（回退 --text） | rgb(255,134,89) |
| .r3-empty-action color | 回退白字（同缺失） | rgb(255,134,89) |
| 视觉层级 | 活动态白字叠橙底、无提亮 | 活动态提亮橙字叠橙底 |

## §4 截图 sha256sum（落盘实物实测，UNIQUE 9/9）

| 截图 | sha256 |
|---|---|
| r111-skills-390.png | 9e830257ead58928ebe2687a90f0695eb8d50ab661cdfc662fd27818ee611aa2 |
| r111-skills-768.png | 35ab86e10ab1e3462c24e23d346ad7e35f8e0d6b7033151990c0e62031d48b89 |
| r111-skills-1280.png | 2f364cc6ddd90ee1b8bc68d2f8c9188f85782e5cd055ee35d7d242ac1f1927f1 |
| r111-equipment-390.png | b1c7c45d6f567c63e235c5fe604f15bd63d60802e8f0b29feb78c0eff040e142 |
| r111-equipment-768.png | c30db8da940ff765526f056f620490d785dacba5348fec4ce951402b4f69a930 |
| r111-equipment-1280.png | 8e9ce1fef61df6942e87f47fa32fefb1aaba719b7287665688fdd314c330e0be |
| r111-shop-390.png | 5560972338931db9e2672a9173738b82152775d80b49425eced9e1f1853fcf98 |
| r111-shop-768.png | bf848c9e3ac64b46976779655ae9030abdd2e0de0b200b07b37cdbc709ff7c03 |
| r111-shop-1280.png | 2c69a604f586d26fe77639d71ee43e464688a51ef2182c6187f42104e9ced71a |

- 逐张由 sha256sum 对 screenshots/ 落盘实物实测，去重 9/9；队长机器可按实物核对。
- 亲读 2 张非登录页：r111-skills-768.png（768×1024，技能图鉴「技能构筑台」，收集进度活动芯片「全部 1」为提亮橙字叠橙底，其余「焰/冰/电/毒/动能」为常规灰字，层级清晰）；r111-equipment-1280.png（1280×800，装备图鉴「装备锻造台」，空态 CTA「去关卡看掉落 →」为提亮橙字叠橙底）。两图视觉与探针 computed 一致。

## §5 静态回归与门禁

- D 不变量：tokens.css:14 --lyp-token-version: lyp-tokens-v1、:15 --lyp-r3-version: lyp-tokens-v3-r3、:16 --lyp-r6-version: lyp-tokens-r6 三锚点不变；--fire-ink: #ff8659 原定义唯一（tokens.css:61，:66 新令牌注释引用同值）；admin laiyipao/admin/src/styles/tokens.css:58 --el-fire-ink: #ff8659 未改；common.css 三消费点 var(--accent-ink) 未改；业务零 --r111-（唯一 3 处命中在契约 r111AccentInkToken.contract.test.ts:76-78 自身断言）；三视口零横溢（cw=sw）。
- E 门禁（data/gates-out.txt，job exit 0，GATES_DONE）：G1 r111AccentInkToken 契约 8 passed (8) RC0；G2 r3/tokens 回归 20 passed (20) RC0；G3 全量 vitest 242 passed | 1 skipped files / 2245 passed | 2 skipped tests RC0；G4 vue-tsc(type-check) RC0；G5 build:h5 RC0；G6 git diff --check RC0。关键项 G1-G6 独立复跑全绿。

## §6 诚实边界

- 后端 8080 未起，全部数据由 init-script 合成（配置缓存/占位令牌 r111-qa-placeholder-token 仅内存注入），非真实业务流量；截图为合成数据下的真实渲染。
- 改前强对照引用 recon A 报告已落盘证据，本轮未注入旧 tokens.css 做 A/B 对照。
- .r3-row-value 消费点三视口均未渲染（9 组 0 命中），未在浏览器实测，仅由契约 it① 与源码静态保证；.r3-chip.active、.r3-empty-action 两消费点已实测坐实。
- WCAG 对比度为浏览器内按公式复算（accent-soft 叠 panel-2/panel-3），非独立第三方工具；色值 #ff8659 为侦察依据 WCAG 反推锁定（与 --fire-ink 同值），权威未逐字列色值字面。
- 截图亲读 2/9；三页 skills/equipment/shop 覆盖主消费点，其他含 r3 组件页面未逐一覆盖。
- 多门禁 job 并发写同文件时 tools.read 曾读到残缺段，最终以 ops_bash grep 核实 RC 行完整（G1-G6 全 RC0、GATES_DONE）。

## §7 结论与 hygiene

- 结论：PASS。三视口真 Chromium 坐实 R111 修复：--accent-ink 由空变 #ff8659，三页活动芯片文字由回退白字变提亮橙 rgb(255,134,89)，额外空态 CTA 同步生效，WCAG 5.19/4.62 达标；令牌三锚点、--fire-ink 唯一、admin --el-fire-ink、三消费点、零 --r111- 等不变量保持，门禁全绿。
- hygiene：HEAD=d23ea384ce12d92404e4aaac895a4f6e2bb0cbd8 全程未变；tracked 改动仅 .gitignore + tokens.css（预期 t328 实施）；审计产出 data/（.gitignore:692 拦）与 screenshots/ 全 UNTRACKED；auth 五路径（.env / laiyipao/admin/.env.local.keys / laiyipao/deploy/.local.keys / laiyipao/server/.local.keys / laiyipao/miniapp/.env.local）全 ABSENT；占位令牌零真值；自启 H5 dev server job 本轮末回收并复验 P5174=000；QA 库本轮未连。
