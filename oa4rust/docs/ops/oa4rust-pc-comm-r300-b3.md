# R300 B3 沟通协作域页面 AI 风格重构与测试服实测

## 范围与依据
- 本轮属于 oa4rust PC 端 ≥1000 轮持续迭代的 R300 B3 环节，仅重构沟通协作域四页：会议管理 `/app/meeting`、组织通讯录 `/app/directory`、工作日历 `/app/calendar`、工作审批待办 `/app/process`。
- 依据 t1 的 `--ai-*` 双主题设计令牌与 t4 的 AppShell 外壳视觉语言；只追加视觉类、渐变/玻璃覆盖与更明确的空态引导文案，未改动任何业务逻辑、接口调用、路由或数据结构。
- IM 消息中心由并行 B2 契约覆盖，避免写范围冲突；审批中心的「端点调试」侵入等非本域核心问题留后续轮次。

## 契约（RED → GREEN）
- 新增 `frontend/apps/desktop/src/views/Pages.aiStyle.B3.contract.test.ts`，4 项契约：会议 hero/ai-page/空态引导/令牌，通讯录面板/头像/栅格/testid，日程渐变标题/今日高亮/栅格/testid，审批表面渐变/玻璃卡片/主按钮/空态引导。
- 首次执行 RED：4/4 失败（exit 1）；四页改造后 GREEN：4/4 通过（exit 0）。
- `pnpm --filter @oa4rust/desktop typecheck` 与 `pnpm --filter @oa4rust/desktop build` 均通过。

## 测试服真浏览器实测（非物理真机）
- 部署：构建产物经 gitignored 部署脚本推送至测试服静态目录并原子替换，远端 index.html 校验新哈希；本文件不记录任何主机名、账号或口令。
- 验证脚本使用 Playwright Chromium headless 访问已部署页面，真实填写登录表单进入 `/app/dashboard`，再逐页导航；证据来源标注为「Playwright Chromium headless against deployed test server; not a physical device」。
- 四页 DOM 实测：每页均存在 `.ai-page`（meeting/directory/calendar `aiPanel=1`，process `aiPanel=2`），渐变节点数分别为 4/4/4/13；会议页「新建会议」按钮可见且点击后对话框出现。
- 真实 UI 补数据流程：通讯录点击「加载名册」后列表从 `共 0 人` 回显为 `共 21 人 · 第 1 / 2 页`；会议创建因表单需先选择楼栋与会议室，本轮未提交成功（如实记录，不作假数据蒙混），空态引导 `暂无会议，点击「新建会议」预订第一场` 正常；日程保持 `我的日历（0）公共日历（0）`，未编造数据。
- 截图与机器证据（本地 gitignored，不入仓）：`test-results/r300-b3-comm/`（6 张页面与对话框截图 + evidence.json）、`test-results/r300-b3-flow/`（3 张补数据流程截图 + evidence.json）。

## 结论
- 四页视觉与令牌对齐 t1/t4，全部契约与类型检查通过，真实浏览器流程确认无回归、接口与业务逻辑保持不变。
- 遗留：会议创建需楼栋/会议室完整链路，日历事件为空，审批中心端点调试侵入；均留待后续轮次按评审结论处理。
- 项目处于 ≥1000 轮持续迭代中，本轮不是完成态。
