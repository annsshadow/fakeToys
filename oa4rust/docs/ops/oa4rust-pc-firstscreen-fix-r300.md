# R302 PC 首屏数据缺失与 BAM 加载失败复核（t34）

> 本轮为 oa4rust PC 端 AI 风格重构 ≥1000 轮持续迭代中的一环，非完成态。
> 证据来自真实测试服页面 + Playwright Chromium headless 真实 UI 流程，均为本地渲染，非物理真机、非静态/假数据截图。

## 1. 来源与目标
- A3 P0-1：BAM 加载失败（"data is undefined"，注明可能是部署滞后）。
- A3 P1-2 / C1：PC 首屏数据缺失、部分空态为死胡同。
- 目标：复核并修复 BAM 加载失败；确保目标首屏页自动加载真实数据或给出明确引导空态，无长期 loading。

## 2. BAM 加载失败复核结论
- 先按既定权威步骤原子重部署，使测试服产物与源码一致（产物入口 index-3yaPonHY.js / index-BOp8VUI6.css 校验一致）。
- 真浏览器登录并访问 BAM：页面无 "data is undefined"、无加载错误条、无长期 loading；统计卡显示 90/0/22/10；summary 接口 HTTP 200 且信封 data 完整。
- 结论：该 P0 属历史部署滞后导致，当前源码逻辑已正常，本轮未修改 BamApp.vue 与 SDK api.ts，避免无依据改动。
- 证据：`oa4rust/test-results/r34-bam-recheck/01-bam-recheck.png`（1440x900，真实测试服页面）。

## 3. 首屏五页真浏览器探测
- 对 file / bbs / meeting / note / recycle 五页逐页真实 UI 登录访问，2.5s 后采集 DOM 指标：五页 loading=0、skeleton=0、errorText=false，均自动加载真实数据或进入正常空态，无长期 loading。
- 发现真实缺陷：文件管理与笔记管理的空态只有一句文案（"当前文件夹为空" / "暂无笔记"），没有下一步可操作入口，用户进入空态即死胡同，不符合"明确引导空态"验收。
- 会议、论坛空态已有引导（"点击新建会议" / "来发起第一个讨论吧"），回收站为空是正常态，均不改动。
- 探测证据：`oa4rust/test-results/r34-firstscreen-probe.cjs`（本地 gitignored 脚本）。

## 4. RED → GREEN（test-first）
- 新增契约 `oa4rust/frontend/apps/desktop/src/views/FirstScreenData.contract.test.ts`（4 条）：
  1. 五页首屏自动加载（源码匹配 loadFiles()/onMounted(loadMeetings)/loadItems()/useQuery）；
  2. 文件管理空态窗口内需复用 handleUpload 且含"上传"引导；
  3. 笔记空态窗口内需复用 openCreate 且含"新建"引导；
  4. 会议/论坛既有引导回归保护。
- RED：`pnpm --filter @oa4rust/desktop exec vitest run src/views/FirstScreenData.contract.test.ts --reporter=dot` → exit 1，2 failed / 2 passed，失败正是文件管理缺 handleUpload、笔记缺 openCreate（真缺陷，非空断言）。
- GREEN 修复（不改接口/业务）：
  - FileManager.vue 空态新增 `<button class="empty-cta" @click="handleUpload">📤 上传文件，开始协作</button>`，复用既有 handleUpload 打开上传。
  - NoteApp.vue 空态新增 `<button class="empty-cta" @click="openCreate">✏️ 新建第一条笔记</button>`，复用既有 openCreate 打开新建。
  - 两页 .empty-cta 样式消费 t1 AI 令牌：--ai-gradient-brand 品牌渐变、--radius-md、--ai-motion-fast + --ai-ease-spring 悬停微动效。
- GREEN：同一契约命令 exit 0，4/4 passed。

## 5. 校验
- typecheck：`pnpm --filter @oa4rust/desktop typecheck` exit 0。
- build：`pnpm --filter @oa4rust/desktop build` exit 0，产物入口 index-BM6UG1wT.js / index-BOp8VUI6.css。
- 全量 `vitest run src/views`：本任务相关 FirstScreenData 契约 4/4 通过；全量套件另有 11 个失败文件（CalendarApp 无障碍、CMS w13-w14 收敛、IMChat 业务、ModernUI/B/C、popupSurfaces、ProcessFold 等），均为队友并行未提交改动所致，与本任务改动文件（FileManager.vue / NoteApp.vue）无关，按"fail loud"如实记录，留给相应任务处理。

## 6. 测试服部署与真浏览器复验
- 原子部署：build → tar -C frontend/dist/web → SSH BatchMode 备份 web → scp /tmp → web.incoming-20261121035000 → 原子替换 web/web.old → 远端 grep 校验 index-BM6UG1wT.js + index-BOp8VUI6.css，exit 0。
- 真浏览器：Playwright Chromium headless，1440x900，真实 UI 登录测试服（<YOUR_TEST_DOMAIN>/oa4rust），逐页访问五页并截图、采集 DOM 指标。
- 结果：R34_FIRSTSCREEN_BROWSER_DONE，五页 loading=0、skeleton=0、errorText=false；文件页与笔记页各出现 1 个 .empty-cta，计算背景均为 linear-gradient(135deg, rgb(79,70,229), rgb(8,145,178)) 品牌渐变，与 t1 AI 令牌一致。
- 证据：`oa4rust/test-results/r34-firstscreen/evidence.json` + 01-file.png / 02-bbs.png / 03-meeting.png / 04-note.png / 05-recycle.png（均 1440x900，source 字段标注 "Playwright Chromium headless against deployed test server; not a physical device"），目录 gitignored，不入仓。
- 截图人工复核：文件页空态品牌渐变按钮 "上传文件，开始协作" 清晰可见；笔记页空态按钮 "新建第一条笔记" 清晰可见。

## 7. 边界与脱敏
- 本轮仅改 FileManager.vue、NoteApp.vue、FirstScreenData.contract.test.ts 与本文档；未碰 Dashboard.vue / OrgDirectoryApp.vue / EndpointFold.vue（t26/t27/t28 范围），未碰后端源码与 H5。
- 文档不含真实域名/IP/主机/SSH 别名/口令/私钥，均用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位；测试服凭据仅运行时由本机脚本从远端读取，未回显、未入库。
