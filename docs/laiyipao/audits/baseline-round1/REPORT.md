# 改造前基线实测 + 全栈真实缺陷审计（Round 1）

- 身份：qa-e2e（只读审计，不修改业务源码）
- 方法：真浏览器（headless Chrome 157，agent-browser 0.41.1）逐页打开真实 dev server 页面 + DOM/探针采集；截图均标注『非物理真机』（桌面 Chrome 模拟 H5 视口，两侧留黑边）。
- 测试环境：本机回环 dev server（H5 端 / admin 端）+ 独立 PostgreSQL 容器；均为临时测试实例，数据为种子/游客态，不代表生产。
- 探针：注入全局 init script 捕获 pageerror / unhandledrejection / console.error / 4xx-5xx fetch 与 XHR 失败。
- 说明：本文档不记录任何真实域名/IP/口令；占位符用 `<YOUR_TEST_DOMAIN>` / `<LOCAL_PORT>`。

## 一、采集覆盖与量化结果

| 端 | 覆盖 | 空页 | pageerror | API 失败 | 错误状态 |
|---|---|---|---|---|---|
| H5（miniapp） | pages.json 全部 19 页 | 0 | 0（静态逐页） | 0 | 0 |
| PC admin | 登录页 + 路由 10 视图 + 独立 wiki，共 12 视图 | 0 | 0（静态逐页） | 0 | 所有 /api/v1/admin/* 返回 200 |
| 核心玩法闭环 | 首页→关卡→第1关战斗→选卡→弃战 | — | 见缺陷 F-2 | 0 | — |

- H5 逐页 DOM：首页 textLen=629 / domElements=466；战斗页可见 Canvas 塔防渲染与 5 波次 HUD；关卡页难度曲线/地形索引；构筑评分雷达等均正常渲染。
- admin 逐页 DOM：跳转链接、侧边导航、表格列头/单元格均有无障碍引用；ECharts 容器存在。
- 原始日志：`data/h5-collection.log`、`data/admin-collection.log`；逐页截图见 `screenshots/`。

## 二、真实业务缺陷（F-1 / F-2，均真浏览器稳定复现）

### F-1【高】admin 数据看板：零战报时 stage_funnel=null 导致图表崩溃，并显示误导性『后端不可达』

- 现象：admin 数据看板页顶部恒定显示红色横幅 `后端不可达：Cannot read properties of null (reading 'slice') / 请先启动 server…`，关卡漏斗图表空白。截图：`screenshots/admin-01-admin_dashboard.png`（非物理真机）。
- 实测证据：带 token 的 `GET /api/v1/admin/dashboard` 返回 **HTTP 200**，响应包含全部 9 个字段，其中 `stage_funnel: null`（种子库 battle_records=0 行，漏斗查询 `HAVING SUM(attempts)>0` 无匹配 → Go nil slice → JSON null）。
- 根因：
  - 前端 `laiyipao/admin/src/views/DashboardView.vue:113` `const items = d.stage_funnel.slice(0, 30)` 对 null 无保护，在 `renderCharts()` 内抛错，被 `load()` 的 try/catch 捕获后写入 `loadError`。
  - 同一文件 `DashboardView.vue:81` 的 `funnelRows` computed 却写了 `(data.value?.stage_funnel ?? []).slice(...)`，两处消费同一字段一处安全一处崩溃，防御不一致。
  - `laiyipao/admin/src/api/types.gen.ts:422` 声明 `stage_funnel: StageFunnel[]`（非空类型），与实际可达的 null 值不符，类型契约失真。
  - 错误横幅文案把前端 JS 异常误标为『后端不可达 / 请启动 server / 执行迁移』，误导运维排障方向（后端实际健康）。
- 触发条件：任一合法空数据态（首次部署、清库、战报被清理）——非测试环境特有，线上新装同样触发。
- 修复建议（test-first RED→GREEN，交业务 owner）：① 前端两处统一 `?? []` 兜底；② 类型定义改为 `StageFunnel[] | null`；③ 错误横幅只在 loadError 确为网络/ApiError 时显示，渲染异常单独提示；④ 后端可将 nil slice 归一为 `[]`（二选一，不要两边都靠约定）。
- 状态：**未修复**（本次只读审计）。

### F-2【中】H5 战斗：高频音效 play()/stop() 竞态产生 unhandledrejection

- 现象：真实鼠标点击链路进入第 1 关战斗并运行约 12 秒后，捕获 5 条 `unhandledrejection: The play() request was interrupted by a call to pause()`（`screenshots/h5-24-battle-realclick.png`，非物理真机）。
- 根因：`laiyipao/miniapp/src/audio/sfx.ts` 的 `playSfx()` 每次 `a.stop(); a.play()` 连续调用；H5 端 uni `InnerAudioContext` 映射为 HTMLAudioElement，`play()` 返回 Promise，前一个尚未 resolve 时被 stop/pause 打断即 reject；`try/catch` 捕获不到异步 rejection。
- 测试缺口：`laiyipao/miniapp/src/audio/sfx.test.ts` 的 mock `play` 返回 undefined，不产生 Promise，故无法覆盖该路径。
- 影响：pageerror 级噪声污染错误监控；真机小程序端 InnerAudioContext 不返回 Promise，主要影响 H5 web 端。战斗逻辑本身不受影响（波次/击杀正常推进）。
- 修复建议（test-first）：把 `a.play()` 的返回值按 Promise 处理并 `.catch(() => {})`（注意 uni 类型声明 play 返回 void，需兼容写法）；补一条 mock play 返回 rejected Promise 的断言。
- 状态：**未修复**（本次只读审计）。

## 三、判定为假阳性 / 非缺陷（不提交修复）

1. H5 首页 console `[来一炮] build.attacker 缺失，正在使用本地占位属性…`：实测 `GET /me` 返回 200 且 `build.attacker.attack=190`（与首页『面板攻击力 190‰』一致），重载后该 warning 不再出现；`battle.vue:643 setBuild(bt.build)` 在开局注入权威快照，`game.test.ts:738` 已覆盖降级路径。判定为 build 初始为 null 时的首帧时序日志噪声，非业务缺陷。
2. 首次用 JS `element.click()` 进入战斗出现的 `play() failed because the user didn't interact with the document first`：改用 agent-browser 真实 trusted 点击后消失；属自动化合成点击未经用户手势激活的探针假阳性（与 F-2 的 play/pause 竞态是不同错误）。
3. `resize` 命令 Unknown：agent-browser 不支持该命令，非应用缺陷；H5 截图黑边是桌面 Chrome 下 uni-app 移动视口居中，已按『非物理真机』标注。
4. 数据看板 KPI 数值为 0：种子库零战报/零订单的真实数据，非渲染缺陷（F-1 的 null 崩溃另计）。

## 四、采集脚本与复现

- `collect-h5.sh` / `collect-admin.sh`：逐页 open→wait→DOM+探针+截图→errors/network 采集（已落盘于本目录）。
- `probe-init.js`：全局错误/失败请求探针。
- 复现 F-1：admin 登录后在零战报库打开数据看板；复现 F-2：真实点击进入第 1 关战斗并等待波次 1 运行 10 秒以上。

## 五、结论

改造前基线：19 个 H5 页 + 12 个 admin 视图可正常打开、核心闭环可走通；静态页面层无 pageerror/API 失败。真浏览器实测发现 2 个稳定复现的真实缺陷（F-1 高 / F-2 中），均已给出根因与 test-first 修复建议；4 类现象判定为假阳性。缺陷修复不在本任务写域内，已移交 captain 安排业务 owner 处理。
