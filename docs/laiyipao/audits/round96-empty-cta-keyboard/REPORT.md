# R96 六页空态 CTA 键盘可达 —— 真浏览器 QA 报告

## 1 范围与方法

- 任务：对 t229 实施独立 QA，验证六页空态 CTA 补 `role="button"` / `tabindex="0"` / `@keydown.enter` 后，真浏览器无障碍树、Tab 命中、Enter 触发、焦点环、零横溢均成立，且不变量与门禁不回退。
- 环境：agent-browser 0.41.1 真 Chromium；三视口 390×844 / 768×1024 / 1280×800；H5 dev server `http://127.0.0.1:5174`（`pnpm run dev:h5 -- --host 127.0.0.1 --port 5174`，后台作业，收尾已 kill）。
- 六页：bag / battles / enemies / me / rank / skins，各 1 处 `class="r3-empty-action"` 空态 CTA。
- 空态来源：**全部为自然空态**。后端 8080 未启动、`uni.request` 失败时六页本地兜底均渲染空态，recon5-driver 实测六页 `ctaCount=1`、`emptyExists=true`；未使用任何 mock 或人为构造，故无「人为构造」样本需披露。
- 方法：每视口每页独立会话，`open` 空页预热（规避带 URL 的 `--init-script` 挂起）、`navigate` 到目标 hash、`eval --stdin < probe-*.js` 注入探针；Enter 观测用 `document` capture `keydown` 监听 + patch `uni.navigateTo` 记录 `__qaNav`；截图用位置参数 `screenshot <完整路径>`。
- 探针字段：CTA 四属性（role / tabindex / tabIndex / cursor）、iw/cw/sw 与 `hOverflow`、AX `button` 角色、Tab 命中序号、焦点环 `outlineStyle/Width/Color`、Enter 事件 `isCta` 与导航记录。

## 2 三视口 × 六页实测

18 组组合全部：`role=button`、`tabindex="0"`、`tabIndex=0`、`cursor=pointer`；`iw=cw=sw`（分别为 390 / 768 / 1280）、`hOverflow=false`；AX 树可见 `button` 角色；Tab 命中 CTA（me 页因先命中 R95 返回按钮需 2 次，其余页 1 次）；`activeIsCta=true`；焦点环 `outlineStyle=solid`、`outlineColor=rgb(255,107,53)`（390 与 1280 宽 1px、768 宽 3px）；Enter 事件 `target.isCta=true` 确实送达 CTA handler。

| 视口 | 页面 | CTA role/tab/tabIdx/cursor | AX button | Tab 命中 | 焦点环 | Enter 送达 | hOverflow / iw=cw=sw | 导航结果 |
|---|---|---|---|---|---|---|---|---|
| 390 | bag | button/0/0/pointer | 去关卡页 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | → /pages/stage/stage |
| 390 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | → /pages/stage/stage |
| 390 | enemies | button/0/0/pointer | 重新加载配置 | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | no-op（见边界） |
| 390 | me | button/0/0/pointer | 去专精页 →（另有返回按钮） | 2 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | no-op（见边界） |
| 390 | rank | button/0/0/pointer | 去冲榜关卡 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | → /pages/stage/stage |
| 390 | skins | button/0/0/pointer | 重新加载配置 | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 390=390=390 | no-op（见边界） |
| 768 | bag | button/0/0/pointer | 去关卡页 → | 1 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | → /pages/stage/stage |
| 768 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | → /pages/stage/stage |
| 768 | enemies | button/0/0/pointer | 重新加载配置 | 1 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | no-op（见边界） |
| 768 | me | button/0/0/pointer | 去专精页 →（另有返回按钮） | 2 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | no-op（见边界） |
| 768 | rank | button/0/0/pointer | 去冲榜关卡 → | 1 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | → /pages/stage/stage |
| 768 | skins | button/0/0/pointer | 重新加载配置 | 1 | solid/3px/rgb(255,107,53) | isCta=true | false / 768=768=768 | no-op（见边界） |
| 1280 | bag | button/0/0/pointer | 去关卡页 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | → /pages/stage/stage |
| 1280 | battles | button/0/0/pointer | 去关卡页挑一关开打 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | → /pages/stage/stage |
| 1280 | enemies | button/0/0/pointer | 重新加载配置 | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | no-op（见边界） |
| 1280 | me | button/0/0/pointer | 去专精页 →（另有返回按钮） | 2 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | no-op（见边界） |
| 1280 | rank | button/0/0/pointer | 去冲榜关卡 → | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | → /pages/stage/stage |
| 1280 | skins | button/0/0/pointer | 重新加载配置 | 1 | solid/1px/rgb(255,107,53) | isCta=true | false / 1280=1280=1280 | no-op（见边界） |

改前 R96-A 侦察基线（t214）：battles 三视口自然空态 `role=null` / `tabIndex=-1` / Tab×8 全停 BODY / AX 仅 generic 非 button；本轮 18 组全部修复。其余五页改前因本地兜底曾报 `ctaCount=0`，改后六页全部自然复现空态 CTA。

## 3 截图证据

18 张截图 `screenshots/r96-{page}-{view}.png`，SHA-256 如下（UNIQUE=18/18）：

| 文件 | SHA-256 |
|---|---|
| r96-bag-390.png | cde82e5c3a4b5638a1995726c18e1b48d564ef8a99ef6e1a85e3500ea9b8f876 |
| r96-bag-768.png | 747537e0242c376d758a409748c70d520941fc294c0dffeca3caa9425ef37fe6 |
| r96-bag-1280.png | 8f8acd2b6df9d53765b428a492eb41c26bf2f05c4fd291b32e27eafb57634f78 |
| r96-battles-390.png | 3adf666740ff1f8d96a7db199469dcf810fcf63c19e018cc2cda53669021359f |
| r96-battles-768.png | 0636b865cba13b72f0cb5096514a44e9e5465178dc156feacabcc11913ad4d1f |
| r96-battles-1280.png | 5581d69a00e7652d0c436244fd791e2590f5d6b472b23f4bef1777c965bed468 |
| r96-enemies-390.png | 3cc38c3b21e366e22df2d3711a5f93ec495023f8092b84d773b863d919b92451 |
| r96-enemies-768.png | 1688559d79a295f3831c7f8c574dd994d24bba3deeb61abccc033694a05a4820 |
| r96-enemies-1280.png | 55eaeb8716f93a32f56007907b8700cef308d4a22f3c49ffd1a74d4f672bf3c6 |
| r96-me-390.png | 5cf2d533dd228cda08b41e94f9b739aea4f05febfa662bba92e6b607eece7092 |
| r96-me-768.png | 93da9a03c4e13fb0ba67a21cb21d1655f412a116fab65e81a09c1f843f2e77fc |
| r96-me-1280.png | f465c90576b2090014166245406b9aa5050d5bc4cab1c3c2a41d746aabc286b5 |
| r96-rank-390.png | 03a9834b6c8fea86327934504971d4657ea05ff8cca2477512e85de869a033b7 |
| r96-rank-768.png | 445b682941d4cd16c86e208ff83fb8a97def079b18540071c8e40ce5b22397c7 |
| r96-rank-1280.png | 41464d2c3863b371701c093c23069bf143e2b96a4085a3ac553a2daeac0256c7 |
| r96-skins-390.png | c43532cfc1f30fb14c97c2b73b8783d4a5d0994d5ccebb5a340aa7adb0e12f05 |
| r96-skins-768.png | 85fc705f5cdef6def1cc4b26a28fb133bb1d904b30861741a004fb24b6ff549c |
| r96-skins-1280.png | 215b772d420529f7c1c6618b78ba4ebdfcdb4dbb286504beaab9c3de98971e10 |

亲读 2 张（均非登录页）：`r96-battles-768.png` 显示「我的战报」空态、筛选 chips、CTA「去关卡页挑一关开打 →」带橙色焦点环；`r96-me-1280.png` 显示指挥官档案（含左上「‹ 返回」）与「去专精页 →」CTA。其余 16 张仅列 SHA-256，未逐张亲读。`screenshots/` 未被 gitignore 忽略，作为 untracked 证据保留、不入库。

## 4 不变量与门禁

- 源码级：六页 `class="r3-empty-action"` 行均按 `role="button"` → `@click` → `tabindex="0"` → `@keydown.enter` 顺序补齐，焦点函数 `r4GoBag` / `r4GoBattle` / `loadPage` / `r4GoMe` / `r4GoRank` / `loadPage` 声明保留；空态四段槽位顺序未变；令牌三锚点 `tokens.css:12/13/14` 在（`--lyp-token-version` / `--lyp-r3-version` / `--lyp-r6-version`）；业务源码零 `--r96-`（仅契约断言文本命中）；焦点环 CSS `common.css:914/918` 在。
- 前端门禁（job bash-a12b9f79b1bc41af，exit 0，data/gates-out.txt）：G1 R96 契约 8/8；G2 `me.test` 29/29；G3 `r60A11yKeyboard.contract.test.ts` 6/6；G4 `r95BackKeyboard.contract.test.ts` 6/6；G5 全量 228 files passed | 1 skipped（229）、2081 tests passed | 2 skipped（2083）；G6 `vue-tsc` RC0；G7 `build:h5` RC0；G8 `git diff --check` RC0（仅 .gitignore LF→CRLF warning）；`GATES_ALL_DONE`。首轮 `--reporter=basic` 报 `Failed to load custom Reporter from basic`，改 default 重跑全绿。
- Go 守卫（job bash-f328d5f7c6c84d9e，`cd laiyipao/server`）：`TestNoInfraIdentifiersInRepo` PASS 8.22s（cached）；`TestNoRealSecretsInRepo` **SKIP 非 PASS**，逐字「读不到 …/laiyipao/deploy/.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」。

## 5 hygiene

- HEAD=`85d5b0d91f384fdb0bda1625d9893680a9d3dfbf`，与派单基线一致、未变。
- 非 `??` 写域仅 `M .gitignore`（队长预置）+ 六个业务 vue：`bag/bag.vue`、`battles/battles.vue`、`enemies/enemies.vue`、`me/me.vue`、`rank/rank.vue`、`skins/skins.vue`。
- auth 四路径全 ABSENT：`laiyipao/miniapp/.env.local`、`.env.local.keys`、`laiyipao/admin/.env.local.keys`、`laiyipao/deploy/.local.keys`。零真值写入。
- QA 库 `lya_qa` 只读：`users` id=477 存在（username null）、104 无记录；本轮未写。
- 自启 H5 作业 `bash-01958b7b0c574d13` 收尾 kill；无遗留服务端口。

## 6 诚实边界与判定

- **Enter 导航差异**：bag/battles/rank 三页 Enter 后确实跳转 `/pages/stage/stage`（`__qaNav` 记录坐实）；enemies/skins 的 `loadPage()` 因 `pageLoaded` 幂等为 no-op、me 的 `r4GoMe()` 因空态 `r4AiMe=null` 提前 return，三页 × 三视口 Enter 事件均已 `target.isCta=true` 送达 CTA handler，但无 DOM/导航变化。这是既有业务守卫，不是键盘绑定失效；R96 验证目标是「键盘可聚焦可激活」，已达成。
- 后端未启动，空态来自本地兜底（非 API 返回的空列表）；因六页均自然复现空态，未使用 mock，零人为构造。
- 焦点环宽度 390/1280 为 1px、768 为 3px，系 `3rpx` 在不同 DPR/视口下的渲染差异，颜色与样式一致（`solid` / `rgb(255,107,53)`）。
- 三个作废脚本版本（首版 8 次 Tab 偏离 CTA、转义 JSON 匹配失败、未注入 Enter observer）均已重写，最终版驱动全 18 组通过；仅最终版数据计入结论。
- 截图亲读 2/18，其余仅列 SHA-256。

**判定：PASS。** 六页空态 CTA 在三视口均满足键盘可达（role/tabindex/Enter）、AX button、焦点环可见、零横溢；门禁与不变量全绿；hygiene 合规。
