# H5 日历 data 信封类型错误修复（R300 / t25）

## 问题与根因
C1 全页实测（docs/ops/oa4rust-c1-full-browser-e2e-r300.md §二 P0-2）发现：H5 日历页 `#/pages/calendar/calendar` 调真实接口 `GET /api/calendar_assemble_control/calendar/list/my`，后端返回的 `data` 是对象信封：

```json
{ "data": { "myCalendars": [], "unitCalendars": [] }, "type": "success" }
```

而 `oa4rust/mobile/src/pages/calendar/calendar.vue` 旧代码 `calendars.value = (cals.data ?? []) as CalendarRow[]` 仅做 TypeScript 类型断言，运行期对象没有 `.map`，在 `Promise.allSettled(calendars.value.map(...))` 处抛 `S.value.map is not a function`，日历页完全不可用（UI 补数无法绕过）。这是确定性真 bug。

## 修复（test-first RED→GREEN）
1. **RED**：新增 `oa4rust/mobile/src/pages/calendar/calendar.envelope.contract.test.ts`，对归一化函数做行为断言——给定对象形 `{myCalendars, unitCalendars}` 必须合并为扁平数组可直接 `.map`，数组形原样透传，空值/畸形信封退化为 `[]`。首次运行模块不存在，1 failed（RED 确认）。
2. **GREEN**：新增 `oa4rust/mobile/src/pages/calendar/calendarEnvelope.ts` 的 `normalizeCalendarRows(data: unknown): CalendarRow[]`，运行时判定数组直接用、对象合并两字段、非法值退化为 `[]`；`calendar.vue` 改为 `calendars.value = normalizeCalendarRows(cals.data)`。不改后端接口契约、不改其他页面、不改业务逻辑。

## 验证
- 日历契约：`pnpm --filter @oa4rust/mobile exec vitest run src/pages/calendar` — 5 files / 22 tests passed。
- `pnpm --filter @oa4rust/mobile run typecheck` — 通过。
- `pnpm --filter @oa4rust/mobile build:h5` — 成功，产物 CSS `index-CdXXREF0.css`、calendar chunk `pages-calendar-calendar.1LLwp5D3.js`（含归一化逻辑）。
- 测试服部署：本地 tar → SSH 别名 BatchMode 备份 → scp /tmp → 原子替换 → 远端 grep 新 chunk 的 `myCalendars` 与 index CSS 哈希，均命中。

## 真浏览器实测（非物理真机）
Playwright Chromium headless，390×844 isMobile/hasTouch，真实 UI 登录流程 + 3 次进入日历页刷新。证据目录 `oa4rust/test-results/r300-h5-calendar-fix/`（gitignored）：`evidence.json` + 4 张 PNG 截图。实测捕获真实接口返回 `data:{myCalendars:[],unitCalendars:[]}` 对象形信封；3/3 次页面均渲染「今日暂无日程」正常空态，无「日程加载失败」、无 `.map is not a function` pageerror，0 API 失败。截图为真实浏览器渲染真实测试服页面，**非物理真机、非静态/假数据拼图**。

## 凭据与环境
测试服域名/主机仅用占位符 `<YOUR_TEST_DOMAIN>` 表示；账号口令运行时由 SSH 别名从本机凭据读取，不入库、不回显真值。

> 本项目为 ≥1000 轮持续迭代，本次修复只是其中一环，不代表项目结束。
