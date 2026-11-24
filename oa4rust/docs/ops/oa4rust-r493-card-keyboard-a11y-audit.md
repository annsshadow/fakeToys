# R493 WCAG 2.1.1 列表卡片键盘可达性甄别（只读审计）

## 1. 范围与判据
- 收敛 R492 §5 披露的 v-for 列表项纯 div/span/li @click 交互卡片，聚焦强候选：MeetingApp.vue:51 li.mc、ProcessWork.vue:40/62/84 li.item-card、FormApp.vue:23/50/87 div.item-card。
- 硬纪律：严禁仅凭源码 @click 预判；逐页以真浏览器键盘实证；同卡片/同行存在等价键盘入口（按钮同功能）不判缺陷；区分真实交互与装饰/遮罩/modalFocus 覆盖区。
- 环境：PC 1440×900 DPR1，Playwright Chromium headless 非物理真机，light/dark 双主题；CDP DOM + Accessibility.getPartialAXTree + 逐次 Tab 聚焦 + screenshot。测试地址 <YOUR_TEST_DOMAIN>；登录凭据仅从环境变量/部署环境注入。

## 2. 实测方法
- 主轮 r493-e2e.cjs：3 页 × 双主题 = 6 组，每组前 3 张卡片打 data-r493card 标记；采集 DOM（tag/class/role/tabindex/tabIndexProp/cursor/行内按钮）+ CDP AXTree（axRole/axName/axIgnored）+ 50 步 Tab 扫描（tabHitCard/tabHitCardButtons）+ .focus() 尝试 + 截图 SHA。
- 补采 r493-supplement.cjs：4 组，meeting 逐按钮 Enter 尝试打开详情浮层、鼠标点击标题对照 + 截图；form 卡片按钮 .focus()/Tab 序列 + 点击卡片文字区可见性检查 + 截图。
- 证据：\`test-results/r493-card-keyboard/report.json\`（6组）、\`supplement.json\`（4组）、\`analysis.txt\`、10 PNG——均 gitignored，不入库；口令/域名真值零入库，仅环境变量/运行期 runner 注入。

## 3. 逐项 programmatic 结果（与 JSON 一致）
### 3.1 主轮（report.json，6 组，0 pageerror）
| 组 | 卡片 tag/class | AX role / ignored | tabIndexProp / tabindex | 卡片本体 Tab 命中 | 卡片内按钮 Tab 命中 | 备注 |
|---|---|---|---|---|---|---|
| process/light | li.item-card | listitem / false | -1 / null | 否（tabHitCard=null） | card-1/2/3 全命中（Tab 序列 8×BUTTON.办理） | .focus() 落 BUTTON.btn-sm primary（按钮非卡片） |
| process/dark | 同上 | 同上 | 同上 | 否 | 同上 | 同上 |
| meeting/light | li.mc modern-card | listitem / false | -1 / null | 否 | card-1/2/3 全命中（每行7按钮×3） | .focus() 落 DIV.nav-item |
| meeting/dark | 同上 | 同上 | 同上 | 否 | 同上 | 同上 |
| form/light | div.item-card glass-card | generic / false | -1 / null | 否 | 首50步=[]（Tab 被左侧50+ admin-nav-item 占满，采样深度不足，非不可达） | 补采坐实行内按钮可达 |
| form/dark | 同上 | 同上 | 同上 | 否 | 同上 | 同上 |

- meeting 行内按钮文本（每组3卡一致）：加入、参会人、邀请、邀请管理、改时间、标记完成、签到码；process 行内按钮：办理；form 行内按钮：✏ 编辑、👁 预览、🗑。
- 全部 18 张卡片 axName 空、axIgnored=false；卡片本体 tabIndex=-1 不可聚焦，但判定以功能等价入口为准。

### 3.2 补采（supplement.json，4 组，0 pageerror）
- meeting/light、meeting/dark：卡片 cursor=pointer、tabindex=null/tabProp=-1、axRole=listitem，.focus() 落 MAIN.content（未激活卡片）；从行内首按钮起 7 次 Enter 实测 keyboardDetailOpened=false，键盘用户无法打开『会议详情』浮层。鼠标点击 li.mc .mt 后截图目视浮层出现（标题 R326审计会议_1795276849927、时间、创建者 admin、说明、控制项 0 项、关联『附件 0 · 后续会议 4 · 前序会议 0』、『关闭』按钮）——该详情为鼠标独有路径。（检测器 mouseAfter.count=0 系匹配文本『创建人』而浮层实际为『创建者』，用词偏差；以截图为实。）
- meeting 日历日程条目 .calendar-event：tag=div、tabindex=null、tabProp=-1、role 空——与列表卡片同为鼠标独有 viewMeeting 入口。
- form/light、form/dark：卡片内『✏ 编辑』按钮 .focus() 成功（becameActive=true）；连续 4 次 Tab 为 预览→🗑→编辑→预览，全部 inCard=true，编辑/预览/删除键盘完全可达。点击卡片文字区后 visibleOverlayLike=0、bodyTextHasDetail=false；源码 viewDetail() 仅设置未被模板渲染的 detailItem ref（FormApp.vue:188/287/290/294），点击无可见效果，非可感知功能被阻断。

## 4. 源码交叉坐实（决定性结构）
- MeetingApp.vue:51 \`<li v-for="m in meetings" class="mc" @click="viewMeeting(m)">\`；viewMeeting():274-300 打开 detail.open 浮层并发 5 个 api.get；行内 7 按钮 @click.stop 分别 joinMeeting / showParticipants / inviteParticipant / manageInvitee / modifyMeetingTime / markMeetingCompleted / showCheckinCode，**无一调用 viewMeeting**；:79 \`.calendar-event @click=viewMeeting\` 同为无 tabindex 的 div。MeetingApp.vue:98-112 浮层模板含标题/时间/创建者/说明/控制项/关联，仅『关闭』按钮可达（打开路径不可达）。
- ProcessWork.vue:40/62/84 卡片 \`@click=openWork(item)\`，行内按钮 :49『办理』/:71、:93『查看』均 \`@click.stop=openWork(item)\` → 同功能等价键盘入口成立，不判缺陷。
- FormApp.vue:23 卡片 \`@click=viewDetail(f)\`，行内 3 按钮调 editForm/previewForm/deleteForm（均 @click.stop，键盘可达）；detailItem ref 无模板消费 → 卡片点击无可感知效果。
- main.ts:609-623 装配 modalFocus/skipLink/formErrorA11y/tableA11y/labelA11y/loadingA11y/progressA11y/searchFieldA11y；modalFocus.ts 仅覆盖遮罩对话框，不覆盖裸卡片；R492 §5 已披露的 172 处 \`@click.self\` 遮罩属覆盖区，排除。

## 5. 三分判定
- **可达真缺陷 1 项（P2）**：MeetingApp 会议列表卡片（MeetingApp.vue:51）与日历日程条目（:79）唯一的『会议详情』打开路径为鼠标 @click；li/div 无 tabindex/role/@keydown，行内 7 按钮均不打开详情。键盘用户无法查看会议标题以外的时间、创建者、说明、控制项、关联信息，违反 WCAG 2.1.1。证据：主轮 6 组 AXTree + 补采 4 组 7×Enter 全 false + 鼠标截图对照。修复建议：:51/:79 增加 \`role="button" tabindex="0" @keydown.enter="viewMeeting(m)" @keydown.space.prevent="viewMeeting(m)"\`，或增加可见的『详情』按钮作为键盘等价入口；不改业务逻辑与现有 7 按钮。
- **不可达/不适用（L1）0 项**：form viewDetail 无可见效果不构成功能阻断；R492 已判不可达项不重复；装饰/遮罩区排除。
- **正常/规范预期若干**：process 三 tab 卡片的办理/查看按钮键盘可达且与卡片同功能；form 编辑/预览/删除按钮键盘可达；meeting 行内 7 按钮键盘可达；skip-link 与左导航装配完整。
- 反证披露：全 views 静态命中 34 处 v-for+@click；本轮实采 3 页 6 组 + 补采 4 组，其余候选未逐一实证，依任务硬纪律不预判、不扩判。

## 6. 截图 SHA256（命令行核验，与 JSON 逐项吻合；SHA_ALL_OK=true）
主轮 report.json：
- pc-process-card-light = 8c43b159ab95333a5ba5c856da4c2d8242e5dae60fb96728ee1f57530bf99be6
- pc-process-card-dark = 91d964d0e777cb5e6ceda00e8dd77bba8854f21d5b582a913fc3ba2166748b6d
- pc-meeting-card-light = 561b8ec09c267e4f63524ccb3e03e3955fd805339f602202ea2727cfd7f758f2
- pc-meeting-card-dark = f3faac831976be0ef3a9773f9bc54b11623b770fe587142121faf7b61c4e546e
- pc-form-card-light = 7fa0f02bbbbfe68bc9d1b26400e322999e70fb00f91a224c6b7ae072be903cdb
- pc-form-card-dark = 0c2a33050798949c2711b7cf692a7e55149bb868fa040404d8730bbee7ce3c6b
补采 supplement.json：
- pc-meeting-sup-light = ae826da66302a2fdf43f3490dee68710c1061dbb0496623a3e4125961ad91a72
- pc-meeting-sup-dark = 572209bab1ec0f3a629771921082716cae1609a0b5484cc036a4265e4977bb48
- pc-form-sup-light = 7ddafc6cd992eecfc72badf48daf7644a41a59e11da637526cd26718ac56fc81
- pc-form-sup-dark = bde62e1dad5fed784c5c9a4ffcae26bbdb819d84295237ffd59768c2a81052be
- 目视核验：process/form/meeting 主轮 light 图与 meeting 补采 light 图（会议详情浮层）均 1440×900，名实相符；截图均为 headless 非物理真机，不冒充真机。

## 7. 结论
R493 甄别完成：可达真缺陷 1 项 P2（MeetingApp 详情浮层鼠标独有），不可达 L1 0 项，正常若干。本轮只读不改码；缺陷与精确修复建议供后续轮次。项目继续迭代，非终态。
