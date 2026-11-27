# R95 真浏览器 QA 复查报告：我的页返回按钮键盘可达

> 轮次 R95 ｜ 任务 t223 ｜ attempt_id=c374e255-1eb9-44d1-b689-8ab00d90e0e9 ｜ 角色 same-route-qa
> 工作目录 D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 ｜ HEAD=639e77647bb93e3b5ec2c1cceb18031f76c15e85（未变）
> 承接 t221 规范 RED（3 fail / 3 pass）+ t222 实施 GREEN（me.vue:11 单行 +1/-1，契约 6/6）。唯一业务写域 laiyipao/miniapp/src/pages/me/me.vue。

## 1. 范围与方法

- 工具：agent-browser 0.41.1 真 Chromium，三视口 390x844 / 768x1024 / 1280x800。
- 前端：uni-app H5 dev server（pnpm run dev:h5 -- --host 127.0.0.1 --port 5174），本地回环，无真值。
- 后端未启动：index 页 bodyLen=89「无法连接服务器」、gridLabels=[]，无法从菜单点进「我的」。故用 uni 全局（探针坐实 typeof uni=object）在页内 uni.navigateTo({url:/pages/me/me}) 构造 getCurrentPages() 栈长 2 → canGoBack=true 渲染返回按钮；直接 navigate 到该页栈长 1 → canGoBack=false 不渲染。此为构造导航栈手段，非业务回退。
- 双 open 预热 + 独立会话串行执行（多会话并发曾致 daemon 竞态 Invalid response EOF，已全部改串行）。
- eval 走 eval --stdin < file（命令替换 $(<file) 会挂起）；screenshot 用位置参数完整路径。
- 探针字段：url/hash/bodyLen/gcp/uniGcp/backExists/backRole/backTab/backText/backTag/backCls/isLogin/iw/cw/sw/hOverflow/gridLabels；焦点探针另取 activeElement 的 tag/cls/role/tab/txt/isBack。

## 2. 三视口实测（walk2，job bash-95cbb555b15749d2 RC0）

| 视口 | 导航前 gcp | 到 me 后 gcp | backExists | role | tabindex | 文案 | tag | isLogin | iw=cw=sw | hOverflow |
|---|---|---|---|---|---|---|---|---|---|---|
| 390x844 | 1 | 2 | true | button | 0 | ‹ 返回 | UNI-TEXT | false | 390/390/390 | false |
| 768x1024 | 1 | 2 | true | button | 0 | ‹ 返回 | UNI-TEXT | false | 768/768/768 | false |
| 1280x800 | 1 | 2 | true | button | 0 | ‹ 返回 | UNI-TEXT | false | 1280/1280/1280 | false |

- 焦点环：三视口均 TAB1/3/5/7 命中 back-btn（isBack=true、role=button、tab=0），TAB2/4/6/8 落 BODY，焦点循环正常、无死循环。
- canGoBack=false：直接导航 me 页（gcp=1）三视口均 backExists=false，返回按钮不渲染（v-if=canGoBack 生效）。
- 零横溢：三视口全程 innerWidth=clientWidth=scrollWidth、hOverflow=false（仅 me 页 bodyLen=626 状态）。
- 改前基线（R94-A recon，t214）：同一返回按钮 role=null / tabindex=-1、Tab 6 次恒 BODY、AX 树仅 StaticText "‹ 返回"；本轮全部修复。

## 3. 决定性验证：Enter 返回 / 鼠标 click 返回 / AX button 角色

首轮 walk 脚本的 Enter 通道有测试缺陷：循环 8 次 Tab 后焦点停在 BODY 才按 Enter，自然不触发返回；768 组另遇 daemon 竞态，该组作废。改用 clean 串行脚本 final-verify.sh（单视口 768：AT ME → TAB1 → Enter → RE-NAV → 真实 click → AX），逐字结果见 data/final-768.txt：

- AT ME：hash=#/pages/me/me、gcp=2、backExists=true、role=button、tab=0、text=‹ 返回、hOverflow=false。
- TAB1：activeElement=UNI-TEXT.back-btn.tap、role=button、tab=0、txt=‹ 返回、isBack=true。
- ENTER 后：hash 从 #/pages/me/me 变为 #/、gcp 2→1、backExists=false、hOverflow=false —— Enter 触发返回坐实。
- RE-NAV + 真实 click .back-btn：CLICK_RC=0，hash 同样回到 #/、gcp=1 —— 鼠标 click 通道保持、无回退。
- 另补 ax-driver.sh 抓 me 页未返回前 AX 快照（data/ax-768.txt）：` - button "‹ 返回" [ref=e1]`，其下 StaticText "‹ 返回"。改前该节点为纯 StaticText、无 button 角色，本轮 AX button 语义坐实。
- 说明：final-verify.sh 内 AX 快照抓在返回之后（落在 index 页，显示「无法连接服务器」），已由上条 me 页快照补足；此点如实披露。

## 4. 截图证据

三视口各 1 张焦点态截图，sha256 全互异 UNIQUE=3/3（文件位于本审计目录 screenshots/，未进 .gitignore，作 untracked 证据保留不入库）：

| 截图 | sha256 |
|---|---|
| r95-390-focus.png | 4475419e989840f4b99d8be65b21ebc09f61c5bb9bd5feeffd586953b9307b18 |
| r95-768-focus.png | 1da09f778ab70be2c416f70a096251ef5378628decbfb1f5c72af2a9f1a4e122 |
| r95-1280-focus.png | c8779d7edcd42873f8ec370dfe46af326925fb607803a8dafe65bfbb294f1a3f |

亲读 2 张非登录页：r95-768-focus.png（768x1024，指挥官档案/玩家/AI 等待数据/资源/设置，左上角「‹ 返回」清晰，非登录页）、r95-1280-focus.png（1280x800，同 me 页布局，左上角返回按钮可见，非登录页）。第三张 390 仅列 sha 未亲读，如实披露。

## 5. 门禁与静态回归

门禁 7 项（data/gates-out.txt，job bash-3af12a9994374b43 exit0；首轮因 cwd 设审计目录致 gates95.sh 找不到 RC127 作废，改仓库根 workdir + 绝对路径重跑全绿）：

| 项 | 内容 | 实测 | RC |
|---|---|---|---|
| G1 | R95 契约 r95BackKeyboard.contract.test.ts | Test Files 1 passed / Tests 6 passed | 0 |
| G2 | me.test.ts | Test Files 1 passed / Tests 29 passed（4.16s） | 0 |
| G3 | miniapp 全量 vitest | 227 passed / 1 skipped (228)；2073 passed / 2 skipped (2075)（28.47s） | 0 |
| G4 | vue-tsc | 类型检查通过 | 0 |
| G5 | build:h5 | 构建成功 | 0 |
| G6 | git diff --check | 仅 .gitignore LF→CRLF warning | 0 |
| — | GATES_ALL_DONE | 全部完成 | 0 |

静态回归（OPS grep，data/static-out 同源）：tokens.css:14/15/16 三锚点在（--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6）；业务源码零 --r95-（唯一命中为 r95BackKeyboard.contract.test.ts 契约断言，业务文件 0）；me.vue 不变量 me.vue:10 v-if=canGoBack、:755 const canGoBack、:757 getCurrentPages().length > 1、:764 uni.navigateBack({ delta: 1 })、:1776 .back-btn:active、:1777 transform: translateX(-6rpx) 全在。

## 6. 不变量、hygiene 与诚实边界 + 判定

- 不变量：canGoBack 判定、goBack 行为、:active 左移降透明样式、令牌三锚点均保持；R60 键盘范式由契约 it5 源码级抽查 announcements.vue / equipment.vue ≥2 处仍具 role+tabindex+@keydown.enter（源码级对照，非真浏览器逐页对照，如实披露）。
- back-btn:active 左移效果为源码级确认（me.vue:1776-1777），未做真浏览器按下态截图（:active 伪类难以稳定抓取），如实披露。
- hygiene：HEAD=639e77647bb93e3b5ec2c1cceb18031f76c15e85 未变；非 untracked 写域仅 M .gitignore（队长预置）+ M laiyipao/miniapp/src/pages/me/me.vue（唯一业务实施）；本轮审计产物均 UNTRACKED；auth 四路径 ABSENT（.env.local / admin/.env.local / laiyipao/deploy/.local.keys / miniapp/.env.local 均不存在）；QA 库只读查 users id=477 存在、104 无记录，本轮未写；自启 H5 job 已纳入清理。
- 诚实边界：后端未启动，全链用 uni.navigateTo 构造导航栈（非真值、非菜单点击路径）；首轮 walk95.sh 依赖菜单点击因后端未起作废；walk 脚本 Enter 测试因焦点停 BODY 缺陷作废，由 final-verify.sh 干净串行坐实；AX 首抓落在返回后 index 页，已补 me 页快照；截图仅亲读 2/3；无真值入库。
- 判定：6 条验收全部满足 → **PASS**。