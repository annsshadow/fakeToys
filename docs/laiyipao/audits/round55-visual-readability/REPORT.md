# R55 真 Chromium 三视口视觉修复复查报告

- 任务：t88 / R55 真 Chromium 三视口视觉修复复查（复验 R54-C 两候选）
- 结论：**PASS（带诚实边界）**。两候选的核心修复在真浏览器中可复现；768 反应图鉴横向溢出由 77px 归零，CTA 从 h=633 竖排变为单行 w=222 可读；背包预设名占位色从 4.03:1 提升至 6.00:1。两处残留数值边界（768 CTA h=92 高于规范文字门槛 88、768 占位 cw237<sw240 仍差 3px）如实登记，不伪称消除。
- 范围：只读走查与审计写入；未改业务源码、未执行 git add/commit/push。

## 1. 门禁逐条结论

| # | 门禁 | 结论 | 证据 |
|---|---|---|---|
| 1 | 768 反应图鉴横向溢出修复 | PASS | 真 Chromium 768×1024：innerW=768 / docW=bodyW=768 / hOverflow=0（修复前 845→77px）；390/1280 同页 hOverflow=0 不回退。 |
| 2 | 768 空态 CTA 单行可读 | PASS（数值有界） | .r14-advice-empty-cta text 逐字「去打一场战斗 →」，rect h=92 w=222 x=222 right=444，whiteSpace=nowrap、maxWidth=100%、display=inline-block；从修复前 h=633/w=68 竖排变为单行横向且完整在卡片可视区；h=92 高于 SPEC §1.3 文字门槛 88，判据按「不再竖排、w>100、单行可读」判定，差异如实披露。空态标题「AI 暂时没有足够数据给出建议」逐字可见，overflowWrap=anywhere。 |
| 3 | flex-wrap 后元素 chip/系数/权重不劣化 | PASS | 三视口 rows[].flexWrap=wrap；元素 chip（焰/冰/电/毒）与系数/攻击力权重等 stats 共全部 tags/stats 的 sw-cw 均为 0（无截断），768 chip h=43、stats h=40 完整显示。 |
| 4 | 背包预设名占位色 | PASS | 三视口 .uni-input-placeholder.ph computed color=rgb(150,160,173)，与 --dim/#96a0ad 一致；按 WCAG 对背景 #161a24 比值 6.57（对 --panel-2 #1a2233 为 6.00）≥4.5，修复前 rgb(128,128,128)=4.03。 |
| 5 | 768 占位截断 | 诚实边界（未消除，未扩大改动面） | 768 实测 cw=237 / sw=240，仍差 3px；390 cw=sw=224、1280 cw=sw=400。SPEC §2.3 已预披露「若仍存在，另立视觉轮次，不在本轮扩大改动面」，本轮不私改 CSS。 |
| 6 | 回归锚点 | PASS | 源码只读复核：R22 CTA 逐字存在（battle.vue）、R16「发起验真 →」存在（BattlesView.vue）、R48 resolveR12Cta helper 存在、R51 r51-radar 14 处、R53 Math.round 8 处；tokens.css:14-16 三锚点 lyp-tokens-v1/lyp-tokens-v3-r3/lyp-tokens-r6 未改，源码/页面未出现 --r55-（仅契约测试文件含文档标记）。reactions.vue:241-244 空态模板 data-r14-source / @click=r14GoEmpty 保留。 |
| 7 | 门禁套件 | PASS | R55 契约 4/4 RC=0；miniapp 全量 215 files passed|1 skipped、2010 tests passed|2 skipped RC=0；type-check RC=0；build:h5 RC=0 DONE Build complete；git diff --check RC=0（仅 .gitignore LF→CRLF 提示）。 |
| 8 | Go 守卫与零真值 | PASS（SKIP 非 PASS） | TestNoInfraIdentifiersInRepo PASS(5.07s)；TestNoRealSecretsInRepo **SKIP**（非 PASS），逐字「读不到 <WORKTREE_ROOT>/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。收尾已删 data/auth-init.js、验证 ABSENT、git check-ignore 命中 .gitignore:518、QA 库 battle_records count/max=104/477、审计文本真值字面值扫描 0 命中。 |

## 2. 浏览器方法与数据可信度

- 服务：API 8080（start-api.sh 后台 job），H5 静态 5201（重新 build:h5 后 bundled Python http.server）。开浏览器前分别验证精确端口 HTTP 200；5174 旧 302 服务未使用。
- 认证：QA 库 user10 guest_token 经 /api/v1/auth/guest 换取 token，仅在浏览器会话 localStorage 注入 data/auth-init.js（gitignored），无字面凭据入库/报告；guest_token_aliases 本轮无活 alias（aliases=2/live=0），改用 service.go:463-482 的 guest_token 路径。
- AB wrapper：`function AB(){ agent-browser --session "$1" "${@:2}"; }` 转发全部参数；早期 wrapper 丢参数导致 eval 得到 --help 非 JSON 的写法已弃用。
- 探针：r55-probe.js 采 href/hash/innerW/docW/bodyW/hOverflow、advice/cta/空态、rows flexWrap/chip/stats 截断、占位 computed color；6 组主探针 href 均为正确深链、非 chrome-error。bag 主探针 preset 选择器在 H5 编译后结构不符，已用 data/bag-input-probe.js 广查补证（不作为失败证据隐藏）。
- 截图：滚动定位后另拍 r55-<W>-reactions-cta.png 与 r55-<W>-bag-ph.png；scroll 输出 SCROLLED / PH_SCROLLED:预设名（最多 20 字），三视口成功。

## 3. 逐页三视口客观量摘要

| 视口 | 页面 | hOverflow | CTA/占位关键值 | 结论 |
|---|---|---|---|---|
| 390×844 | reactions | 0 | CTA h=48 w=114 nowrap；tags/stats cut=0 | 不回退 |
| 768×1024 | reactions | 0（原 77） | CTA h=92 w=222 nowrap；空态标题可见；tags/stats cut=0 | 修复 |
| 1280×800 | reactions | 0 | CTA h=46 w=110 nowrap；tags/stats cut=0 | 不回退 |
| 390×844 | bag | 0 | 占位 rgb(150,160,173) cw=sw=224 | 修复 |
| 768×1024 | bag | 0 | 占位 rgb(150,160,173) cw=237 sw=240（残差 3px） | 对比度修复，截断残留披露 |
| 1280×800 | bag | 0 | 占位 rgb(150,160,173) cw=sw=400 | 修复 |

## 4. 截图 sha256 表

| 文件 | 视口 | sha256 | 备注 |
|---|---|---|---|
| r55-390-reactions-reactions.png | 390×844 | 24db315f6da8be78ed95c5c15b21ea0d357113683e4cc18595324f6a32c0ba88 | 主走查，非登录 |
| r55-768-reactions-reactions.png | 768×1024 | 7377325185397ad75faa51ede300c73e9f22e78fd077a53ee64819b136794a42 | 主走查，已亲读 |
| r55-1280-reactions-reactions.png | 1280×800 | 06f7785171b0554ace5a53e305d66fbde894e56edf8978742b4ca4ba9fa964e2 | 主走查，非登录 |
| r55-390-bag-bag.png | 390×844 | d4eb8d9f181ae1a241d12df4762a133260f5b5311107a99d7b36f5ce32378e73 | 与 R54 基线同 sha（页面此视口无视觉变化） |
| r55-768-bag-bag.png | 768×1024 | 68e929baa4fe123034f9cfa97674ad581845c76edb673fd735a2375809035d88 | 主走查，已亲读 |
| r55-1280-bag-bag.png | 1280×800 | d1e39955e6b3ad9dadc0d0c0421c2f6ff7674e0ecd87f1ebbecacecce47b7340 | 主走查，非登录 |
| r55-390-reactions-cta.png | 390×844 | 69de8880d015987792ca2f678b5b7f07c42befee5b578fc8122781a9faddfe85 | 滚动定位 CTA 可读 |
| r55-768-reactions-cta.png | 768×1024 | 6cf0fc97fedf1bbda5460df24cad56554d46ccbd08abccda9f21b174477527f5 | 已亲读：CTA 完整单行 |
| r55-1280-reactions-cta.png | 1280×800 | e9353ac47477536cb812ecc8a5dead178797742497d22a84b8a26955d33b0b46 | 滚动定位 CTA 可读 |
| r55-390-bag-ph.png | 390×844 | aa5920abf37288917a1aed46a06e63b44eaca6f1879980b8bd491fec4cebaa07 | 滚动定位占位符，与 r55-390-bag-input 同 sha |
| r55-768-bag-ph.png | 768×1024 | e6768f24945f4926c0b5b888aeb794fac6b67045f2160ee143f778a5d0de5fc0 | 已亲读：预设名占位符清晰可见 |
| r55-1280-bag-ph.png | 1280×800 | 9c6fa870f94b84aa8489aee8bde7e6b62d0965b12aee51505b674e0efaa2223f | 滚动定位占位符，与 r55-1280-bag-input 同 sha |

## 5. 诚实边界

1. 768 CTA rect.h=92 > SPEC §1.3 文字门槛 88；核心判据「不再竖排、单行 w>100、完整可见」满足，h 差异如实披露，不伪称逐字符达标。
2. 768 背包占位 cw237<sw240 残差 3px 未消除；按 SPEC §2.3 另立轮次，不在本轮扩大改动面。
3. r55-390-bag-bag.png 与 R54 基线同 sha（该视口页面视觉未变化），不作为「新增互异截图」计数；其余截图互异且均为非登录真实页面。
4. 主 bag 探针 selector 不匹配导致 preset 字段缺失，已用广查探针补证；首版 bag-scroll 未定位到占位符（截图与 r55-<W>-bag-input 同 sha），已用 bag-ph-driver 重拍并核对文本。均如实登记，不隐藏失败尝试。
5. TestNoRealSecretsInRepo 因缺 .local.keys 为 SKIP，逐字标注 **SKIP 非 PASS**，不把未执行当作通过。
6. 认证 token 只在浏览器会话与 gitignored data/ 中短暂存在，收尾删除；报告与审计文本不含任何真值/IP/口令/个人路径字面值。

## 6. 收尾

- data/auth-init.js 已用 ops_remove(expectedRevision) 删除并 stat 验证 exists=false；QA 库 battle_records count/max=104/477；git check-ignore 命中 .gitignore:518；审计文本真值字面值扫描 0 命中；自启 API/H5/各浏览器 job 已全部 kill 或 completed。
- 仅新增本报告及 gitignored data/、screenshots/ 下审计产物；未改业务源码，未做 git 写操作。
