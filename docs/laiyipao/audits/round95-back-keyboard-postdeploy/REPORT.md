# R95 部署后回归报告：我的页返回按钮键盘可达

> 轮次 R95 ｜ 任务 t225 ｜ attempt_id=cae5088f-5ca2-45c9-a09c-6d64276d2184 ｜ 角色 same-route-rel（release-ops）
> 工作目录 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 ｜ 起点 HEAD=639e77647bb93e3b5ec2c1cceb18031f76c15e85（R94 收官）AB=0/0
> 结论：可发布。生产静态构建三视口真 Chromium 回归坐实 —— 真菜单进入「我的」后返回按钮 role=button/tabindex=0，Tab 第 1 次命中且 AX 为 button，Enter 与真实 click 均回到 index；零横溢、非登录页。

## 1. 五批发布链

| 批次 | commit | 内容 |
|---|---|---|
| B1 | 05a9368535aca92ffbe649d0d8ae9c40eaa2a122 | feat：me.vue:11 单行 +1/-1（role=button / tabindex=0 / @keydown.enter=goBack） |
| B2 | e2d061ff1dd9acf368711979c3341fbd7f380ee4 | test：r95BackKeyboard.contract.test.ts 83 行 6it |
| B3 | dd601eb3b25e471aa54cd067a1a0bee97c3481c4 | docs：SPEC + ROUND95_DESIGN_REVIEW（124 行） |
| B4 | 7ba74e4dea71fa41c1aa36f2cbacb843a70c7a25 | audit：.gitignore + QA REPORT(74 行) + 8 脚本 + 3 截图（13 文件 278+） |
| B5 | 本报告 | chore：部署后回归报告 + 驱动脚本 + 3 张 pd3 截图 |

每批精确 git add 逐文件，泄露守卫 LEAK_OK、DATA_IN_STAGE=0、diff --check RC=0；未用 -A / 点号 / 目录通配。

## 2. 产物取证（不回退基线）

- H5 重建：`pnpm run build:h5` RC=0（job bash-c97aa73178204a11）；index.html:assets/index-D99R8srB.js。
- 主 bundle `assets/index-D99R8srB.js` sha256=2ad05f208a3864291d7da2c91aeeb7397c4facb7cdd58dd72580057c63a693b5（≠ R93 index-uFWsk6dV.js）。
- 页面 chunk `assets/pages-me-me.BQg0hlBh.js` sha256=acaef2bd1232ac1ea662ea0c0f88c5c47465ba5b9dd60b9d7b35d6ad4b2ee188。
- 后端 `laiyipao/deploy/bin/lyp-server-amd64` sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6，沿用不回退。
- 服务：API 8080（job bash-10cd85c0c9b949a5，`/api/v1/config`=200）；H5 静态 5201（job bash-7dbbea1556084db7，python http.server on dist/build/h5，=200）。

## 3. 部署后三视口探针（生产静态构建 + 真菜单点击）

| 视口 | 入口 index bodyLen | 到 me hash | back role/tab | TAB1 命中 | AX | Enter 后 | click 后 | iw=cw=sw | hOverflow |
|---|---|---|---|---|---|---|---|---|---|
| 414×896 | 976（15 grid-item） | #/pages/me/me | button / 0 | isBack=true | button "‹ 返回" | #/ | #/ | 414/414/414 | false |
| 820×1180 | 976 | #/pages/me/me | button / 0 | isBack=true | button "‹ 返回" | #/ | #/ | 820/820/820 | false |
| 1440×900 | 976 | #/pages/me/me | button / 0 | isBack=true | button "‹ 返回" | #/ | #/ | 1440/1440/1440 | false |

- 入口构造：后端就绪后 index 页真实渲染 15 个 `.grid-item`，点第 9 项「我的」进入 me 页（bodyLen=2321），非登录页（isLogin=false）。
- 焦点环：driver3 三视口 TAB1 即命中 back-btn（role=button、tabindex=0、文案 ‹ 返回、tag=UNI-TEXT），截图带橙色焦点框；driver2 全 8 次 Tab 呈 1/3/5/7 命中、2/4/6/8 落 BODY 的正常循环。
- AX 快照：`button "‹ 返回" [ref=e1]` + StaticText，与 DOM role 一致。
- Enter 与 click 均由 `#/pages/me/me` 返回 `#/`（index bodyLen=976、返回按钮消失、grid-item 重现）。

## 4. 部署后截图（3 张，UNIQUE 3/3，全 ≠ QA 三图）

| 截图 | sha256 | 亲读 |
|---|---|---|
| pd3-r95-414-focus.png | 2143db6ada29f3cdd62a417774f2fd3ec6dc816989abdde6c5a676cc5dc7e1ed | 是：指挥官档案页，左上「‹ 返回」橙色焦点框，非登录页 |
| pd3-r95-820-focus.png | e451dae48b805ef3feed01bd76e95eb4e56e447f0a35b1b2eb0739e8fc15e3e5 | 否（列 sha，未亲读，如实披露） |
| pd3-r95-1440-focus.png | 2a56a7da2096426db28495519e8a44eca371cbb3b347cc876c2a72b1ec8f859a | 是：指挥官档案页，左上「‹ 返回」橙色焦点框，非登录页 |

QA 三图基线（t223，dev server）：r95-390-focus=4475419e… / r95-768-focus=1da09f77… / r95-1280-focus=c8779d7e…，与上表三图全不同。
首轮失败态 pd-r95-{414,820,1440}-focus 与 driver2 的 pd2-r95-{414,820,1440}-focus 为诊断过程图，不作回归证据、不入库。

## 5. 门禁与静态回归

- QA 前置门禁（t223/t224 已录）：契约 6/6、me.test 29/29、全量 227 files passed | 1 skipped / 2073 tests passed | 2 skipped、vue-tsc RC0、build:h5 RC0、diff --check RC0。
- Go 门禁（正确模块根 laiyipao/server/go.mod，job bash-7c004c490b70454e EXIT=0）：TestNoInfraIdentifiersInRepo PASS 8.22s；TestNoRealSecretsInRepo SKIP 非 PASS（secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫没有执行，不是通过了」）。首跑在 worktree 根误报 "go: cannot find main module"，改到 server 目录后正确。
- 静态锚点：tokens.css:14 lyp-token-version / :15 lyp-r3-version / :16 lyp-r6-version 三锚点在；业务源码零 `--r95-`（仅契约否定断言命中）；me.vue:11 修复行经 tools.read 直证；R60/R61/R55 等历史不变量未回退。
- QA 库只读实测：users=750、battle_records=104（users 较前轮 747 增长系本轮及 E2E 游客账号注册，battle_records 稳定 104）。

## 6. 诚实边界

1. 首轮生产静态构建 `uni.navigateTo`/`getCurrentPages` 全 undefined（dev server 才有该 API），直接 navigate 到 me 页无法构造导航栈，返回按钮不渲染、6 张首轮图为 index/失败态，已作废；改用后端 + 真菜单点击重跑成功。
2. driver2 的 Enter 失败系 Tab 循环 8 次后焦点落 BODY（grep 转义致未提前 break），非业务缺陷；driver3 改为 TAB1 命中即停，Enter 成功返回。
3. 三张 pd3 图仅亲读 414/1440 两张，820 只列 sha，未亲读如实披露。
4. Go `TestNoRealSecretsInRepo` 为 SKIP 非 PASS，不冒充通过。
5. 游客账号由 E2E 注册，QA 库 users 计数增长；未写任何真值入库。
6. 探针脚本位于 gitignored `data/`，仓库内 root 副本（pd-probe.js / pd-click-me.js / pd-probe-active.js）供审计追溯；运行副本不入仓。

## 7. 结论

生产静态构建三视口真浏览器回归全部符合验收：键盘可达（Tab 命中 + AX button + Enter 返回）、真 click 返回一致、零横溢、非登录页、产物 sha 可核、真值零入库。R95 可发布。
