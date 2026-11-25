# R43 真浏览器复查报告：战斗 HUD CTA 点击不再误暂停

- 任务：t45（attempt 5913a6ea-583e-40b8-9e10-792f598b54fa）
- 审计目录：docs/laiyipao/audits/round43-battle-hud-cta-honesty/
- 依赖：t43 实施（仅改 laiyipao/miniapp/src/pages/battle/battle.vue 四处，契约 33/33 绿）
- 规范：docs/laiyipao/specs/round43-battle-hud-cta-honesty/SPEC.md（9 节）

## 一、结论

**PASS（带诚实边界）**：R27 / R24 两条 AI 文案 CTA 已变为纯展示，真实浏览器坐标点击后不再打开图例帮助层、不再触发暂停，战斗计时继续推进；.help-chip 正对照仍能真实打开图例并冻结计时。所有可自动化的门禁为绿。以下第三、六节列明假阳性、无效证据与未验证项，不作为 PASS 依据。

## 二、范围与约束（已遵守）

- 未修改任何业务源码；未执行任何 git 写操作（无 commit/add/checkout）。
- 仅新增审计目录产物，并为该审计目录补 .gitignore 规则（:466-467，非业务配置，见第八节）。
- 所有浏览器交互由真实 Chromium 通过 agent-browser CLI 完成；最终 CTA 点击证据用真实底层鼠标事件（mouse move / mouse down left / mouse up left），不是 DOM eval 派发点击。
- 脚本全部以 git rev-parse --show-toplevel 推导仓库根，不写死绝对路径。
- 无字面凭据入库；令牌注入脚本 data/auth-init.js 运行时生成、报告与脚本中均不含令牌真值。

## 三、环境与页面进入

- QA 容器 lyp-qa-pg Up；API 服务（:8080，复用 round15-reaction-timeline/start-api.sh）与 H5 服务（:5174，pnpm run dev:h5 -- --host 127.0.0.1 --port 5174）均为 HTTP 200。
- 会话环境 QA_ADMIN_USER / QA_ADMIN_PASS 未设置；本轮不需要管理员口令，游戏侧经 /api/v1/auth/guest 自动访客登录进入 L5。
- 视口 390×844。必须先 open 再 set viewport：反序会静默得到 innerWidth=1258，本轮所有有效证据均为 innerW=390 / docW=390。
- 未登录深链进入 L5 显示「无法开始战斗 / 无权访问: 第 5 关未解锁（当前最高 0）」，属预期；使用访客登录后进入真实战斗态（data/auth-initial.json：波次 1/5、HP 1100/1100、击杀 1，r27/r24 文案均在场）。

## 四、真浏览器证据（最终判定）

证据文件：data/v2-r27-before.json / v2-r27-after.json、v2-r24-before.json / v2-r24-after.json；点击日志 data/v2-r27-click.txt / v2-r24-click.txt 各含 3 行 Done（move/down/up）。坐标来自 data/v2-r27-probe.json（r27 中心 196,168）与 v2-r24-probe.json（r24 中心 196,651）。

| 项目 | r27 点击前 → 后 | r24 点击前 → 后 |
| --- | --- | --- |
| href | 战斗页 level=5（不变） | 同左（不变） |
| 倒计时（距满分） | 138s → 135s | 133s → 130s |
| helpOverlay | false → false | false → false |
| hasTimeline | false → false | false → false |
| hasPause / hasResume | 0 / 0 → 0 / 0 | 0 / 0 → 0 / 0 |
| pauseLabels | [] → [] | [] → [] |
| r27 文案 | 继续清理场上敌人 →（不变） | 同左 |
| r24 文案 | 去释放对应元素技能 →（不变） | 同左 |
| hpNow / waveNow | 1100 / 1 → 1100 / 1 | 1100 / 1 → 1100 / 1 |

判定：点击两条 CTA 后战斗仍在推进（计时各减 3s），始终没有出现「继续战斗」按钮、没有图例帮助层、没有反应时间线层，即未误暂停、未误开帮助。页面始终处于战斗态（非结算态）：每帧 bodyHead 含「防线 / 波次 1/5 / 击杀 N / 钳分中」，无结算标志。

### 正对照（帮助层仍可真实打开并暂停）

- 证据：data/hc-before.json / hc-open-state.json / hc-closed-state.json，点击日志 data/hc-click.txt / hc-close.txt 均为 Done（真实选择器点击 .help-chip 与 .legend-close）。
- 点击 .help-chip：helpOverlay=false→true，bodyLen=1062→2123，倒计时停在 138s；截图 r43-hc-help-open.png 目视可见「战场图例」弹层。
- 点击 .legend-close：helpOverlay=true→false，bodyLen=2123→1062，倒计时恢复 137s。
- 这证明「帮助层能打开并冻结战斗、关闭后恢复」这一正路径仍然完好；R27/R24 CTA 的静默是定向去掉误暂停，不是全局禁用点击。

### 源码层共存核对（只读）

- laiyipao/miniapp/src/pages/battle/battle.vue：r27-ai-cta（约 :58）与 r24-ai-cta（约 :95）均无 @click，CSS 两项只留 color/font、无 pointer-events:auto；其宿主 .r27-ai-note / .overlay-top 为 pointer-events:none。
- r28-feed-cta 仍保留 @click.stop="openReactionTimeline" 与 pointer-events:auto；.help-chip（约 :113）、r26-ai-cta（约 :310）均保持原状；openHelp() 函数体未改。R23–R42 其余 HUD 元素（波次、击杀、钳分、AI 文案、反应时间线入口）在 v2 探针 bodyHead 中均可见，未发生回归。R28/R29 的真实点击开时间线本轮未单独执行浏览器用例，此项按源码只读核对结论，见第六节未验证项。

## 五、门禁结果

- R43 契约（battle_r27_wave_progress.contract.test.ts + battle_r24_element_hud.test.ts）：2 文件 / 33 通过，exit 0。
- battle.test.ts：61 通过，exit 0（stderr 仅有 [来一炮] build.attacker 缺失 占位告警，非失败）。
- pnpm run type-check：exit 0。
- pnpm run build:h5：exit 0，DONE Build complete（仅 uni-app Appid 提示）。
- miniapp 全量 pnpm test：Test Files 212 passed / 1 skipped，Tests 1988 passed / 2 skipped，exit 0。
- Go 守卫：TestNoInfraIdentifiersInRepo PASS（32.67s）；TestNoRealSecretsInRepo SKIP——不是 PASS。缺席真值文件 laiyipao/deploy/.local.keys，守卫输出含本机工作树绝对路径，此处以 WORKTREE_ROOT 占位、不回显真实路径；其原话为「读不到 WORKTREE_ROOT/laiyipao/deploy/.local.keys —— 真值扫描这条守卫没有执行，不是通过了」（原文含强调标记）。

## 六、假阳性、无效证据与未验证项（诚实边界）

1. r27 选择器点击失败是探针限制，不是产品缺陷：agent-browser click .r27-ai-cta 报「Element r27-ai-cta is covered by a div inside uni-canvas#battle-canvas at its click point」。原因：r27/r24 为只读 CTA，pointer-events 为 none（与规范一致），选择器点击被画布遮挡；CLI 无 force 参数。因此改用真实底层坐标鼠标事件作为最终证据。首次 coord-verify.sh 有 bash 变量拼接 bug（产出 MOVE=9140{stage}X... 与 Missing arguments for: mouse move），已由 coord-verify2.sh 修正，最终证据只采用 v2。
2. hasResume=0 / pauseLabels=[] 在图例打开时仍为假阴性：正对照中帮助层已打开、计时已冻结，但探针按精确文案匹配「继续战斗」未命中；可靠的暂停判据是 helpOverlay + 计时冻结/恢复 + 截图，而非该字段。
3. 无效截图：r43-v2-help-open.png 与 r43-v2-help-closed.png 字节相同（sha da0586…），是视口被重置成 1258×588 后的空白图，未作为任何证据；正对照改用 r43-hc-help-open.png / r43-hc-help-closed.png。
4. R28/R29 真实点击开时间线本轮未单独跑浏览器用例；仅源码只读确认 handler 保留。如需闭环，另开一轮浏览器用例。
5. TestNoRealSecretsInRepo 为 SKIP，不能计入「守卫全绿」；本报告不将其写作 PASS。

## 七、截图 sha256 清单（互异、非登录页）

| 文件 | sha256 |
| --- | --- |
| screenshots/r43-auth-initial.png | ac3c01b0385c809b3156f325b6bc2faa872f389d783c3509511f58a3e27db9a8 |
| screenshots/r43-r27-after.png | e80aa6d021df1fd8e503f5b2ee10dcb1459af180996e6372d5195c7791b4d9ec |
| screenshots/r43-r24-after.png | 2c93c3f3ecb06a12e378adf1ccbcf76053ad5b67f41dd36396d28f9e50b3769e |
| screenshots/r43-v2-r27-after.png | e19e773f46688d538c9d28430ec9314cd8c1d3a5a83362ed5b57b75b3d842696 |
| screenshots/r43-v2-r24-after.png | e714320eafa24a10783cf0e9e1d9686283f4c2aeb874e34ee6f0ad8623735625 |
| screenshots/r43-hc-help-open.png | ca853b93aa47472294befc270383f24f24cdffb5f822b032f8fada12eac7e776 |
| screenshots/r43-hc-help-closed.png | bc99c6ea08af378a3f4d7d6e28ed777596ea93ab6038bf6ce7e0a0bb828633ad |
| screenshots/r43-coord-r27-after.png | 78666d9f3c78399c67c4573d77b7923f03e207eda3d7930cf18ab9a3ba6919de |
| screenshots/r43-coord-r24-after.png | f66043eb9ef2285eb83c9b3a2d2a0d198382a0a3a02b2151deed7e1238107628 |
| screenshots/r43-initial.png | bd9a3a0d1a5eeb8251ae22c25ad64a5704e28022c24b914c9feb9d2d6bdc8e67 |
| screenshots/r43-stage.png | ee4a907382a957b20901ed519dc40ef767aeee5e25071367b348c1ed83be46fc |

≥4 张互异、非登录页截图满足要求（此处列 11 张有效互异图）。r43-v2-help-open/closed 为同一无效空白图，不计入；r43-coord-help-open/closed 为首次脚本缺陷产物，不作为证据。

## 八、密钥与清理

- 令牌注入脚本 data/auth-init.js 由运行时生成，未写入任何字面凭据；data/login.json 本轮未生成。
- .gitignore 新增 :466-467：docs/laiyipao/audits/round43-battle-hud-cta-honesty/data/ 与 .../admin-auth-init.js；git check-ignore -v 命中 .gitignore:466。
- 收尾删除 data/auth-init.js；静态密钥扫描见第九节复核结果；QA 库 battle_records 复核为 count=104 / max(id)=477。

## 九、收尾复核

- 删除 data/auth-init.js 后 git check-ignore -v 仍命中；git status 仅显示审计目录与 .gitignore 变更，无业务源码改动。
- 静态密钥扫描：审计目录文本文件中无字面 JWT / Bearer / 带口令连接串 / 本机绝对路径命中。
- 浏览器与服务端 job 全部 job_kill，无残留进程。
