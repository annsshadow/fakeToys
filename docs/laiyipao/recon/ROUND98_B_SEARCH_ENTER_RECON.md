# ROUND98-B 搜索 Enter 提交与表单键盘流 只读侦察报告

- 轮次：ROUND98-B
- 执行：same-route-reviewer
- 基线 HEAD：358901febaaf92ff5543e4f20f379ece52c4ea21（tracked 清洁，全程未变）
- 性质：纯只读侦察，未改业务源码/契约/规范/.gitignore/令牌，未写真值，未写 git
- 结论：**列 1 个推荐候选（四条件全中）**——3 个「输入即提交」表单缺少 `@confirm` 键盘入口，Enter 键无任何反应

## 1. 视角与范围

小程序 H5 搜索输入框的 Enter/确认键提交路径与表单键盘流。`laiyipao/miniapp/src/pages` 下 19 个 vue，全仓 17 处 input。按用途分两类核查：

1. **搜索类**（7 页）：announcements(annKeyword:87)/enemies(34)/equipment(147)/gems(83)/reactions(185)/skills(159)/skins(39)。均只有 `v-model` + `v-aria-native` + `aria-label` + `class=search-inp` + `placeholder`，无 `@confirm`/`@keydown.enter`/`confirm-type`/`enterkeyhint`，也无独立搜索按钮（仅"清除搜索"链接）。过滤为实时 `computed`（announcements.vue:158-169 filteredItems、enemies.vue:204 filteredEnemies 等），输入即筛选无需 Enter。
2. **提交类**（10 处）：login 用户名(33)/口令(45, `@confirm=submit`:56)、me 昵称(95)/原口令(592)/升级口令(644)均带 `@confirm` + `@click` 双入口；signin 兑换码(203)、verify battleId(53)、bag 预设名(405) 仅 `@click` 无 `@confirm`。

全仓 `@confirm` 仅 4 处（login:56、me:95/592/644），`confirm-type`/`enterkeyhint` 源码零命中（真浏览器 DOM 中的 `enterkeyhint="done"` 是 uni-app 编译产物默认注入，非源码声明）。

## 2. 静态铁证

| 页面 | input | 行 | 键盘入口 | 提交入口 |
| --- | --- | --- | --- | --- |
| signin | code（兑换码） | 203-211 | 无 `@confirm` | :212 `@click="doRedeem"` |
| verify | battleId | 53-61 | 无 `@confirm` | 按钮 `@click="fetchReplay()"`（契约 r5:151 守卫） |
| bag | presetName | 405-411 | 无 `@confirm` | `@click="doSavePreset"` |
| login | username/password | 33/45 | password `:56 @confirm=submit` | `@click=submit` |
| me | nickname/oldPw/upgrade | 95/592/644 | 均有 `@confirm` | 均有 `@click` 双入口 |

**权威口径**：r69NicknameBusy.contract.test.ts:86-88 明文守卫 `@confirm="submitNickname"` 与 `@click="submitNickname"` 双入口并存——同仓表单范式是「Enter + 按钮」双路径，login.vue:56 亦遵循。r61FormA11y.contract.test.ts:43-61 只把 17 处 input 的 `aria-label` 列为不变量（含 signin code/verify battleId/bag presetName），未守卫 `@confirm` 属性集合——补 `@confirm` 不触碰该契约不变量。r55VisualReadability.contract.test.ts:57-62 守卫预设名 `placeholder="预设名（最多 20 字）"` 与 `placeholder-class="ph"`，补 `@confirm` 不影响。

**R79-B 边界**：docs/laiyipao/recon/ROUND79_B_INPUT_HINT_RECON.md（基线 74a89522，视角为输入引导可见性，结论无候选）已读，不覆盖 Enter 提交，本轮不重叠。

## 3. 真 Chromium 对照坐实

环境：H5 常驻回环 5174（API 8080）；agent-browser 0.41.1 命名 session；三视口 390×844 / 768×1024 / 1280×800。探针 hook XHR.open + uni.showToast 记录请求与提示；guest token 运行时注入 localStorage 不回显。

### 3.1 signin 兑换码框（三视口，输入非法码 INVALIDTEST98）

| 视口 | 按 Enter | 点「兑换」按钮 |
| --- | --- | --- |
| 390×844 | `reqs=[]`（无请求） | `/api/v1/redeem`（提示"兑换码无效或已启用"） |
| 768×1024 | `reqs=[]` | `/api/v1/redeem` |
| 1280×800 | `reqs=[]` | `/api/v1/redeem` |

三视口完全一致：Enter 无任何反应，按钮正常提交。signin-390-exact.png 亲读：Enter 后页面停在兑换码区无反馈，仅点击按钮才出现错误提示。

### 3.2 login 口令框阳性对照（390，有 `@confirm=submit`）

输入用户名/口令后在口令框按 Enter → `reqs=["/api/v1/auth/login"]`，显示"未认证或令牌已失效：用户名或密码错误"。login-390-enter.png 亲读确认。**证明 H5 编译产物下 `@confirm` 能被 Enter 键触发**，故"给缺 `@confirm` 的输入框补该属性"是经同仓阳性验证的有效修复路径。

### 3.3 verify battleId / bag 预设名（390，单视口）

输入值后按 Enter：`reqs=[]`、无导航、无提示，页面停留原处（verify-390-enter.png / bag-390-enter.png）。与 signin 同构，静态确认二者也仅 `@click` 无 `@confirm`。

### 3.4 搜索类反向印证

7 个搜索页输入即实时 computed 过滤，无需 Enter；无 `@confirm` 属设计取舍（非缺陷）。本视角候选仅针对「输入即提交」的 3 个表单。

### 3.5 截图 sha256（7 张全互异 UNIQUE=7/7）

| 截图 | sha256 |
| --- | --- |
| signin-390-exact.png | 06fd92e51d1a54a32e0d16b4670257c47f90be19faebc3e31a7d5779c919497c |
| signin-768-exact.png | 62436e4f70b457b2b924d71e87bba1cee76a3cb33f74556bb19d9f1392cad238 |
| signin-1280-exact.png | ad4d731a39fbda072f9b24f1178e40f2f8994143fa49d0ba5c579dee12127fc1 |
| signin-390-enter.png | 808064605e2efa19027546bf1c0d12b1f5ed5338bfaf1e05b40267644f8d00a8 |
| login-390-enter.png | 573fdcbd4c79e92bbad32cd1cb0f2049889854e40d9cbe98f468f31b692c4fdc |
| verify-390-enter.png | 4fc6e2f6212048b4a60c79fceeebde2a363481a645c5e7d06478e547f9ee7b51 |
| bag-390-enter.png | a88e7874cf776c627a471ec7961ea5d855319a9edef796047844e0d5323c38cb |

亲读 2 张（signin-390-exact、login-390-enter），其余 5 张仅列 sha（如实标注）。

## 4. 四条件逐条裁定（推荐候选）

**候选：signin 兑换码框(:203-211)、verify battleId(:53-61)、bag 预设名(:405-411) 补 `@confirm` 键盘提交入口。**

- **条件①（真实可复现用户可见缺陷）**：满足。三视口坐实 signin Enter 无请求（按钮有），verify/bag 同构。键盘用户填完表单按 Enter 无任何反应，与同仓 login/me 双入口范式不一致。
- **条件②（展示层/模板小改且有同仓权威口径）**：满足。仅在 3 个 `<input>` 各加一行 `@confirm="doRedeem"/"fetchReplay"/"doSavePreset"`，复用现有 handler；权威口径是 r69 契约守卫的「Enter+按钮双入口」范式 + login.vue:56。
- **条件③（不碰语义/令牌/测试不变量）**：满足。`@confirm` 仅补键盘入口，不改 handler 语义、不碰令牌、不触碰 r61（只守 aria-label）/r55（只守 placeholder）/r5:151（守 fetchReplay() 存在）等既有不变量。
- **条件④（改动面可控）**：满足。3 页各 +1 行模板属性，零新增 DOM/逻辑/依赖。

**四条件全中 → 1 个推荐候选**。最小修复：3 处 input 各补 `@confirm` 指向其现有 `@click` handler；建议由正式实施轮补 spec/契约/QA 后终审。

## 5. 诚实边界

1. verify/bag 仅 390 单视口实测 Enter 无反应（三视口仅 signin 做完整对照）；二者与 signin 同构（静态无 `@confirm`），裁定成立。
2. 真浏览器用 `/api/v1/redeem` 等真实请求前缀作为提交发生的判据，未提交无效兑换码成功流程（无需）。
3. `@confirm` 在 H5 下的有效性由 login 阳性对照证明，不推广到原生 App/小程序端（uni-app 各端 confirm 行为可能不同）。
4. 7 张截图亲读 2 张，其余 5 张仅列 sha。
5. 搜索类 7 页无 `@confirm` 是实时过滤的设计取舍，非缺陷，不列入候选。

## 6. 收尾 hygiene

- 基线 HEAD 358901febaaf92ff5543e4f20f379ece52c4ea21 未变
- 未改任何 tracked 文件（git status 非 ?? 项 NONE）
- 报告与脚本/截图均为 UNTRACKED，未写真值（guest token 运行时注入不回显）
- auth 四路径 ABSENT；自启 API/H5/采集 job 待收尾清理；QA 库只读
