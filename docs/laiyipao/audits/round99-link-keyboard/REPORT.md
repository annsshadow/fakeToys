# R99 19 处 link 键盘可达 —— 真浏览器 QA 报告

- 轮次：R99（承接 t247 侦察 / t248 规范 / t249 实施；本任务 t250 实施的独立 QA）
- 任务：same-route-qa（t251），attempt_id=a352906e-8ee7-4d7a-b237-c89c876bc977
- 工作目录：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
- HEAD：05f2acf30950a377b681c09ae05d988f25eac866（未变）
- 验证对象：t250 在 4 页 vue 的 19 处可点 `text.link` 补 `role="button"` / `tabindex="0"` / `@keydown.enter=同名handler`（diff 19+/19-），契约 `laiyipao/miniapp/src/styles/r99LinkKeyboard.contract.test.ts`（24 it，readFileSync 真实读源码）。
- 唯一写域：docs/laiyipao/audits/round99-link-keyboard/（data/ 命中 .gitignore，screenshots/ UNTRACKED 证据不入库）。

## 1. 范围与方法

- 真浏览器：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800。
- H5 dev server：`pnpm run dev:h5 -- --host 127.0.0.1 --port 5174`（本轮自启 job，收官已 kill，P5174=000）。
- 后端 8080 未起：用 `open --init-script` 注入合成 config/战报/令牌（data/r99-init.js，占位令牌 `lyp-access-token = 'r99-qa-placeholder-token'`，零真值），使 link 自然渲染；XHR + fetch 双 patch 记录请求 URL 到 `window.__qaHits`。
- open --init-script 后视口被重置为 1258，walk 内重新 set viewport 后校验 iw=cw=sw=视口宽。
- 探针：data/r99-probe.js（linkCount + links[{tag,txt,role,tabAttr,tabIndex,cursor}] + innerW/clientW/scrollW/hOverflow）；data/probe-hit.js（极简 CTA_HIT/CTA_NO）；data/probe-focusring.js（outlineStyle/Width/Color）；data/enter-observer.js（document capture 监听 keydown 记 `__qaKeys`，patch uni.navigateTo 记 `__qaNav`，patch uni.setClipboardData 记 `__qaClip`）。
- AX：`agent-browser snapshot` 取 `button` 角色节点。
- 驱动：data/walk99.sh —— 三视口 × bag/me，open 空页 → set viewport → open --init-script → sleep5 → 重设 viewport → probe → observer → AX → screenshot → 14 次 Tab 循环，命中 CTA 即取焦点环并按 Enter。
- 证据文件：data/walk99-out.txt（188 行）、data/gates-out.txt（8755 行）、data/static-out.txt（29 行）。

## 2. 三视口实测数据（逐组，以落盘 walk99-out.txt 为准）

| 视口 | 页面 | linkCount | link 文本 → role/tabIndex | iw/cw/sw | hOverflow | AX button | Tab 命中 | 焦点环 | Enter 送达 |
|---|---|---|---|---|---|---|---|---|---|
| 390×844 | bag | 3 | 去商城买一颗/保存当前构筑/点这里填名字保存一套 → 全 button/0 | 390/390/390 | false | ✓ | TAB3/4/5/6 命中 | solid rgb(255,107,53) | isLink=true |
| 390×844 | me | 2 | 改名/复制 → 全 button/0 | 390/390/390 | false | ✓ | TAB3/5 命中 | solid rgb(255,107,53) | isLink=true |
| 768×1024 | bag | 4 | 3 处契约 link 全 button/0 + 第 4 处“已定位到上方输入框” role=null/-1 | 768/768/768 | false | ✓ | TAB3/4/7/8 命中 | solid rgb(255,107,53) | isLink=true |
| 768×1024 | me | 1 | 复制 → button/0 | 768/768/768 | false | ✓ | TAB4/9/14 命中 | solid rgb(255,107,53) | isLink=true |
| 1280×800 | bag | 4 | 同 768 组 | 1280/1280/1280 | false | ✓ | TAB3/4/7/8/12/13 命中 | solid rgb(255,107,53) | isLink=true |
| 1280×800 | me | 1 | 复制 → button/0 | 1280/1280/1280 | false | ✓ | 命中 | solid rgb(255,107,53) | isLink=true |

- 汇总：CTA_HIT 共 23 次，焦点环探针 23 条（全 `outlineStyle=solid` / `outlineColor=rgb(255,107,53)`，宽度 1px 或 3px，均为橙色焦点环）；AX `button` 节点 16 个（含去关卡页/保存当前构筑/点这里填名字保存一套/改名/复制/去专精页/‹ 返回）。
- 焦点环颜色 `rgb(255,107,53)` 即 tokens.css:35 `--accent:#ff6b35`。
- 第 4 处 link“已定位到上方输入框”SOURCE 见 bag.vue:424 `<text v-if="presetFocus" class="link">已定位到上方输入框</text>`，是点“点这里填名字保存一套”后的动态提示文本，**非契约 19 处可点 CTA**，无 role/tabindex 合规，不构成回退。
- 改前 R99-A 侦察基线（ROUND99_A_LINK_KEYBOARD_FOLLOWUP_RECON.md，t247）：link role=null / tabIndex=-1 / AX 无 button / 真 Tab 从不命中 —— 上述 6 组全部修复。
- 禁用态静态保持：mastery.vue:69 `resetMastery`、shop.vue:133 `doBuy(it)` 两处无 role（契约 it 静态守卫），符合本轮范围（禁用态不在 19 处内）。

## 3. 截图表（以落盘实物 sha256 为准）

| 文件 | sha256 |
|---|---|
| r99-bag-390.png | 52742f594f3b9ef8b4cab8fbaca9621db3e8120143bf71620bceeda55f111da0 |
| r99-bag-768.png | d8bd6b95ee87b1e54280c11d6d384253abee058ace29e33651758e6e18b79d97 |
| r99-bag-1280.png | bbcbf84abeead588b5bfd1739705cee33686dff79ab36e9705d79eced6ee8be4 |
| r99-me-390.png | 25b6d5fdf2df27eee028da15057ebe48b7723701fe08485326a0521afea690b4 |
| r99-me-768.png | eb2526f93512565ac7020a4672749d78e7ec42059720970de2cf9caa0a1347cb |
| r99-me-1280.png | 7295149c1c7e71f3c583ad7c6239bb52eeba821c26c80c940d7e6b01d52ff8b1 |

- 6 张全互异，UNIQUE=6/6。亲读 2 张非登录页：r99-bag-768.png（背包页，AI 空态 + “去关卡页 →” CTA + 出战技能空槽，“还没有可用技能”提示）；r99-me-1280.png（指挥官档案页，左上 ‹ 返回 + “去专精页 →” + 改名输入框 + “复制” link）。其余 4 张仅列 sha，未逐张亲读。
- 截图存于 docs/laiyipao/audits/round99-link-keyboard/screenshots/（UNTRACKED，作为审计证据保留，不入库）。

## 4. 门禁与静态回归（data/gates-out.txt、data/static-out.txt）

| 门禁 | 内容 | 结果 |
|---|---|---|
| G1 | r99LinkKeyboard.contract.test.ts | Tests 24 passed (24) RC=0 |
| G2 | r60A11yKeyboard.contract.test.ts | Tests 6 passed (6) RC=0 |
| G3 | me.test.ts 定向 | Tests 29 passed (29) RC=0 |
| G4 | miniapp 全量 vitest | Test Files 231 passed / 1 skipped (232)；Tests 2120 passed / 2 skipped (2122) RC=0 |
| G5 | vue-tsc (type-check) | RC=0 |
| G6 | build:h5 | RC=0 |
| G7 | git diff --check | RC=0 |

- 静态回归（static-out.txt）：tokens.css:35 `--accent:#ff6b35` / :40 `--bad:#ff7b72` / :44 `--ice:#58a6ff` 三锚点在；业务源码 `--r99-` NO_MATCH（零 --r99- 令牌）；19 处契约 CASES 逐项 role/tabindex/keydown 断言在；禁用态 mastery/shop 无 role 断言在；STATIC_DONE。
- 19 处改点清单（契约 readFileSync 真实读源码，均 role=button/tabindex=0/@keydown.enter=同名 handler）：announcements.vue:98；bag.vue:134/211/217/351/375/380/385/417/421/442/443/449；index.vue:185-188；me.vue:83/111/448/450。

## 5. 诚实边界

- 后端 8080 未起：全部数据来自 init-script mock（合成 config 2 技能/1 配方/1 装备/1 宝石、1 条合成战报、rating_weights 五维全 0）与占位令牌，**非真值**；未连接真实后端。
- 真浏览器完整坐实（role/tabIndex/AX/Tab/焦点环/Enter 送达）的仅 bag/me 两页可自然渲染的 link；index 页 bootstrap 仍失败（bodyLen=89 “无法连接服务器:request:fail”）CTA 未自然渲染、announcements 清除搜索 link 未自然复现、me 战报 link 默认页签未出 —— 这些页的其余 link 由契约 24/24 静态保证，未在真浏览器逐一命中，如实标注。
- bag 页 Enter 送达以 `__qaKeys` 中 `isLink=true` 为判据；uni.navigateTo / uni.setClipboardData 桥接的具体导航/剪贴值未在 h5 环境稳定捕获，故不对“Enter 后页面实际跳转/复制成功”作 DOM 断言。
- 截图亲读 2/6；焦点环宽度存在 1px 与 3px 两种（同一 solid 橙色样式，不同元素/时机），均判为可见焦点环。
- walk 输出首行因 open --init-script 视口重置曾出现 iw=1258 的 trial，正式 walk 重设视口后 6 组均 iw=cw=sw=视口宽，trial 数据不作为结论。

## 6. 判定与 hygiene

- hygiene：HEAD=05f2acf30950a377b681c09ae05d988f25eac866 未变；非 ?? 写域仅 .gitignore（队长预置）+ 4 页 vue（announcements/bag/index/me，预期 t250）；auth 四路径（.env.local.keys、laiyipao/deploy/.local.keys、laiyipao/admin/.env.local、keys/local.key）全 ABSENT；QA 库只读（历史 477 存在 / 104 无记录，本轮未写）；自启 5174 H5 dev server job 已 kill，P5174=000；零真值入库。
- 5 条验收全部满足：①三视口 role/tabIndex/AX button/Tab 命中/Enter 送达/焦点环/零横溢且逐组 raw 证据在 walk99-out.txt；②禁用态 2 处无 role + 令牌三锚点 + 业务零 --r99- + 零横溢；③门禁 G1-G7 全 RC0；④本报告含数据表/sha 表/诚实边界/门禁清单，6 图 UNIQUE 且亲读 2 张非登录页；⑤hygiene 与零真值全绿。
- **判定：PASS**。
