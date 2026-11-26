# R307 改造后组织树真浏览器独立复查（qa-e2e / t60）

## 来源与边界声明

- 测试对象：部署于测试服 `<YOUR_TEST_DOMAIN>` 的 OA4Rust PC 端（路径前缀 `/oa4rust`）；测试服目录 `<YOUR_TEST_HOST>`。
- 浏览器：Playwright Chromium headless，1440×900 视口，真 UI 登录（用户名/口令在运行时由远端凭据源读取，仅存内存，不入库、不回显）。
- **截图为测试服真浏览器渲染，非物理真机**、非静态假数据；证据落 gitignored 的 `oa4rust/test-results/r307-org-after/`，不入库。
- 本轮为只读独立复查：未改任何源码、契约测试或清单；仅新增本文档。项目处于 ≥1000 轮持续迭代中，非终态。

## 复查时序与产物

- 前置：t59（page-comm）完成组织/部门新建入口与后端 unit_create 绑定修复，commit `d828a8b04`，原子部署测试服后经队长放行。
- 复查时测试服新产物已上线：`admin-CpsrAJgl.js`、`OrgViewer-BKtyqMaC.css`（改造前为 `index-BiyhjNmp.js`）。
- 复查时代码基线：HEAD=`a05554f7e175ec9a499ef114198ad67b4d4cc0a9`，工作树干净。

## 复查口径（队长给定）

真登录测试服访问 `/app/org`（组织架构）与 `/app/directory`（组织通讯录），抓取组织组树 `group/list/like` 与部门树 `departments/tree` 接口 data 结构（脱敏摘要）+ DOM 节点数 > 0、空态遮蔽、中文错误条、pageerror、API 失败；与 t58 基线（data 为空数组）量化对照。

## 量化对照（t58 基线 → t60 改造后）

| 指标 | 页面 | t58 基线（改造前） | t60 改造后 | 结论 |
| --- | --- | --- | --- | --- |
| `PUT .../group/list/like` 接口 data | /app/org | HTTP200/success，`data:[]`（长度 0） | HTTP200，`dataArrayLen=8` | 已收敛：组织组数据非空 |
| 组织组树 DOM 节点数 | /app/org | 0（空态「暂无组织架构数据，请点击重试加载」） | 8，无空态、无错误条 | 已收敛 |
| `GET .../departments/tree` 接口 data | /app/directory | HTTP200/success，`data:[]`（长度 0） | HTTP200，`dataArrayLen=1` | 已收敛：部门树数据非空 |
| 部门树 DOM 节点数 | /app/directory | 0（空态「暂无部门树数据，请点击加载部门树」） | 1（显示 `R307部门…`），无空态、无错误条 | 已收敛 |
| 部门筛选 chip / 人员 | /app/directory | 5 chip / 21 条人员 | 5 chip / 21 条人员（保持不变） | 未回退 |
| `POST .../person/list/filter` | /app/directory | HTTP200，data={count,data,page,size}，21 条 | HTTP200，data keys={count,data,page,size} | 未回退 |
| pageerror | 两页 | 0 | 0 | 无回归 |
| API 失败（HTTP≥400） | 两页 | 0（组/部门接口 200 但空数据） | 0 | 无回归 |
| 中文错误条（加载失败/超时/异常/data is undefined） | 两页 | 无（仅空态提示） | 无（chineseErrorText=[]） | 无回归 |

## 真浏览器实测结论

- `R307_AFTER_DONE`：`pages=2, responses=3, errors=0, apiFailures=0, pass=true`。
- `/app/org`（title「组织架构 · OA4Rust」）：组织组树 8 节点（如 `R307组织组…`），`orgTreeEmpty=false`、`orgTreeError=false`、`orgTreeRetry=false`、`errorText=false`；截图目视左侧树栏 8 组、右侧「选择节点查看详情」。
- `/app/directory`（title「组织通讯录 · OA4Rust」）：部门树 1 节点（`R307部门…`），`deptEmpty=false`、`deptError=false`、`deptLoading=false`；左侧部门筛选 5 chip、人员列表「共 21 人 · 第 1/2 页」；截图目视部门树面板已显示真实部门、人员卡片 E2E001…E2E020。
- 三项组织/人员接口均 HTTP200 且结构脱敏摘要无真实姓名/手机/邮箱/令牌入库。

## 证据（gitignored，非物理真机）

- `oa4rust/test-results/r307-org-after/evidence.json`：含 source 声明（NOT physical device）、host、逐页 DOM 诊断、三条接口脱敏摘要、pageerror/API 失败、pass=true。
- `oa4rust/test-results/r307-org-after/org-after.png`：1440×900，组织架构页改造后截图。
- `oa4rust/test-results/r307-org-after/directory-after.png`：1440×900，组织通讯录页改造后截图。
- 复查脚本：`oa4rust/test-results/r307-org-after.cjs`（真登录、结构脱敏、逐页截图、判定落盘）。

## 判定

t58 记录的「后端组织组与部门数据真空」已由 t59 修复并在本次独立复查中确认收敛：组织组树与部门树 data 由空数组变为非空、DOM 节点由 0 变正、空态消失，且未引入 pageerror、API 失败或中文错误条，人员数据与部门筛选无回退。对照队长 t59 复核（部门树 0→1、组织树 7→8），本轮独立实测组织组树为 8 节点（与之一致量级）、部门树 1 节点（一致）。组织域数据真实性缺陷在 R307 收敛。
