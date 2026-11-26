# R300 基线真浏览器实测与逐页批判清单

- 实测日期：2026-11-21（Asia/Shanghai）
- 分支：`opt1000/oa4rust-pc-ai-refactor`
- 来源声明：全部截图来自 tabbit 真浏览器（Tabbit Browser/Chromium）访问测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust`（PC）与 `https://<YOUR_TEST_DOMAIN>/oa4rust-h5`（H5），非本地 dev server、非静态假数据、非真机。PC/H5 均使用测试服真实账号 `admin` 经登录页 UI 登录；口令来自 `laiyipao/deploy/.local.keys`（不入仓、不在本文档回显）。
- 视口：无头浏览器标签 1529×930（H5 URL 未使用移动仿真，故 H5 截图为桌面宽视口渲染，仅作页面可达性与内容批判，不等同真机）。
- 证据目录：`Temp/qa-e2e-r300/`（运行产物，不入库）。PC 核心页 `live-*.png` + `pc-live-results.json`；H5 页 `h5-*.png` + `h5-results.json` / `h5-chain-results.json`。

## 一、PC 端逐页批判（16 个核心业务路由，登录后真浏览器采集）

统一导航：左侧为主菜单 + 管理员菜单，右侧为业务区；`/app/dashboard` 等 16 路由均可达、title 正确、无 200 遮罩/断页。以下问题按页列：

1. **工作台 `/app/dashboard`**：统计卡显示待审批 0 / 未读 0 / 已办结 0 / 今日会议 0；我的待办出现 `start:S3 运行闭环 work-1795166639123-yltovm 未知应用` 等原始 ID。批判：数据语义弱、ID 直接暴露，统计全零让首屏缺乏价值感。截图 `live-dashboard.png`。
2. **组织架构 `/app/org`**：`端点调试 98 展开查看全部端点`，主体仅 `Loading... / Select a node to view details`。批判：调试 UI 抢占版面、加载态长期停留，无法判断是接口慢还是无数据。截图 `live-org.png`。
3. **组织通讯录 `/app/directory`**：`共 0 人`，`暂无人员数据，请先搜索或加载名册`。批判：需要用户手动「加载名册」才能看到人员，首屏空。截图 `live-directory.png`。
4. **工作流 `/app/process`**：有待办数据但标题/ID 是 `S3 运行闭环 3848b8d6-cd2d-4179-85a6-b53b84b8601c` 之类原始串。批判：缺少可读业务标题与语义化状态。截图 `live-process.png`。
5. **消息 `/app/im`**：多条会话均 `暂无消息`。批判：会话列表有但内容空，缺乏空态引导。截图 `live-im.png`。
6. **个人中心 `/app/personal`**：手机号/邮箱未设置，授权委托均 0。批判：信息完整度低，缺少引导完善资料。截图 `live-personal.png`。
7. **系统设置 `/app/settings`**：基础配置可见，`已切换到：系统设置` 出现在可见文本。批判：无障碍 live 提示泄漏到视觉文本，设计噪声。截图 `live-settings.png`。
8. **日历 `/app/calendar`**：我的日历(0) / 公共日历(0)。批判：空数据无引导。截图 `live-calendar.png`。
9. **文件管理 `/app/file`**：我的文件/共享/回收三个分栏均空。批判：三栏空态重复，上传入口不够突出。截图 `live-file.png`。
10. **论坛 `/app/bbs`**：`暂无主题，来发起第一个讨论吧`。批判：版块导航为空，缺少示例/引导。截图 `live-bbs.png`。
11. **会议 `/app/meeting`**：`暂无会议`，会议室下拉需先选楼栋。批判：会议数据空、预订表单前置条件不直观。截图 `live-meeting.png`。
12. **考勤 `/app/attendance`**：统计 45 应出勤 / 42 实际 / 3 迟到 / 2 请假，但列表 `暂无考勤记录`。批判：统计与列表不一致，疑似统计接口与明细接口数据源不一致（需后续 test-first 验证）。截图 `live-attendance.png`。
13. **回收站 `/app/recycle`**：0 项目。批判：空态正常但缺少说明。截图 `live-recycle.png`。
14. **BAM `/app/bam`**：明确报错 `BAM 统计加载失败：["bam","list"] data is undefined`（出现两次）。批判：真 bug，接口返回结构与前端解析不匹配，需 RED→GREEN 修复。截图 `live-bam.png`。
15. **笔记 `/app/note`**：`📓 暂无笔记` + `端点调试 1`。批判：调试入口暴露给业务页。截图 `live-note.png`。
16. **全局搜索 `/app/search`**：仅 `输入关键词开始搜索`。批判：空态可接受但缺少热门搜索引导。截图 `live-search.png`。

## 二、H5 端逐页批判（登录后真浏览器采集）

H5 登录页 `#/pages/login/login` 使用真实口令登录成功，跳转 `#/`。快捷入口真实路由经 UI 点击枚举：消息/审批/文档/通讯录/考勤/发起流程/会议/日程/论坛/浏览/搜索/回收站。

- **工作台 `#/`**：`9 待办审批 / 0 今日出勤 / 📢 暂无公告`，快捷入口 12 项。批判：整体可用、统计有值；与 PC 工作台的 0 待审批不一致（H5=9），需核对数据源。截图 `h5-home.png`。
- **消息 `#/pages/message/message`**：批量采集时标题为「消息」，多次出现 `连接服务器超时，点击屏幕重试`。批判：消息页在测试服出现真实连接超时，需后续验证是否偶发/接口异常；截图采集受 tabbit 执行器超时影响未稳定落盘。证据：`h5-results.json`（stage=capture 记录）。
- **审批 `#/pages/process/process`**：有待办列表（`start:S3 运行闭环… active`），可驳回/通过。截图 `h5-process.png`。批判：同样存在原始 ID 展示。
- **文档 `#/pages/doc/doc`**：截图 `h5-doc.png` 已落盘（39KB），doc 1514×974。批判：需在评审中确认布局/内容。
- **通讯录 `#/pages/contacts/contacts`**：`E2E001…E2E014 / 13900000001…` 可见，人员数据存在。截图 `h5-contacts.png`。批判：H5 有人员数据而 PC 通讯录为 0，进一步证明 PC 端首屏未加载名册。
- **考勤打卡 `#/pages/attendance/attendance`**：`考勤打卡`，doc 1514×1024。截图 `h5-attendance.png`。
- **发起流程 `#/pages/process/start`**：`发起流程`，doc 1514×974。截图 `h5-process-start.png`。
- **会议 `#/pages/meeting/meeting`**：标题「会议」，截图采集多次因 tabbit 执行器超时未落盘；稳定性是问题。证据：`h5-chain-results.json`。
- **日历 `#/pages/calendar/calendar`**：标题「日历」，doc 1514×974，截图 `h5-calendar.png`。
- **论坛 `#/pages/bbs/bbs`**：`连接服务器超时，点击屏幕重试`，bodyTextLen=17。截图 `h5-bbs.png`。批判：H5 论坛真实不可用（超时态）。
- **浏览 `#/pages/browse/browse`**：多次采集超时，未稳定落盘。证据 `h5-chain-results.json`。
- **搜索 `#/pages/search/search`**：`连接服务器超时，点击屏幕重试`。截图 `h5-search.png`。批判：H5 搜索真实不可用（超时态）。
- **回收站 `#/pages/recycle/recycle`**：标题「回收站」，截图 `h5-recycle.png`。
- **我的 `#/pages/mine/mine`**：`连接服务器超时` + 底部 tabbar。截图 `h5-mine.png`。批判：我的页真实不可用（超时态）。

## 三、跨端共性缺陷与建议优先级

1. **P0 真 bug**：BAM 页 `["bam","list"] data is undefined`；H5 bbs/search/mine 的「连接服务器超时」；PC 考勤统计与列表不一致。均需 test-first RED→GREEN。
2. **P1 数据首屏**：PC org 一直 Loading、directory 0 人、file/bbs/meeting/note/recycle 全空；H5 有人员与待办数据，说明 PC 端的数据加载/呈现策略落后，应按 H5 的真实数据源补齐首屏。
3. **P2 设计/体验**：可见 `已切换到：XXX` 无障碍提示；流程/待办展示原始 UUID；调试入口（`端点调试`）侵入业务页；多处空态缺少引导。
4. **P3 工具链**：tabbit 的 page.goto 与 page.screenshot 在 H5 hash 路由上频发超时；CDP Page.captureScreenshot 可绕过截图超时。后续 E2E 应固定使用 CDP 截图与 hash 赋值，并在需要时用 `finish --task` 释放标签。

## 四、本轮结论

R300 基线已完成真浏览器逐页实测并形成批判清单；所有截图均来自测试服真浏览器，未使用 dev server 或静态假数据。未对业务代码做修改；下一步由队长据本清单安排 RED→GREEN 修复与 UI 重构轮次。本轮不是项目完成，仅为 1000 轮迭代中的基线记录。