# R498 WCAG 2.2 SC 3.3.3 Error Suggestion（错误建议）甄别审计

> 本轮为只读甄别，不改任何源码/业务。测试服页面来源：Playwright Chromium headless 非物理真机（PC 1440×900 DPR1，light/dark 双主题）。所有真域名/IP/凭据均不入库，统一占位 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>`；口令仅由 runner 从环境变量注入。

## 1. 依据与判据

- 维度：WCAG 2.2 SC 3.3.3 Error Suggestion —— 当输入错误被**自动检测**且**纠正建议已知**时，应向用户提供具体建议；若建议会危及内容安全或目的（如密码、验证码）则属例外。
- 反预判铁律：①错误态必须真实出现（在触发校验时点取证，杜绝 R466 early-skeleton 名实不符）；②泛化「必填/不能为空」≠无建议——同页 label/placeholder/aria-describedby/紧邻文案若能给出具体格式或范围，须判**等价建议存在**；③密码/验证码类须区分安全例外；④带具体纠错建议的提示不算候选；⑤三分判定每项须有实测依据。
- 前序覆盖：R488/R489（3.3.1 错误识别语义）、R490/R491（3.3.2 必填标识）、R480（1.3.5 输入用途）均未触及 3.3.3 错误建议内容质量，本轮为全新维度。

## 2. 基线与取证方法

- 真父基线：`390bc498352dc4dde1e1e19b97ac8f10c764eb16`（台账 §493 已入账）。分支 `opt1000/oa4rust-pc-ai-refactor`，HEAD=UP=真父，AB=0/0，工作树干净。测试服 active、HTTP=200。
- 四轮真浏览器实测（证据全部 gitignored `oa4rust/test-results/r498-*`）：
  1. 主轮 `report.json`：12 组（6 动作 × 双主题），CDP DOM + Accessibility.getPartialAXTree + computedStyle。
  2. 补采 `supplement.json`：8 组（calendar/role/program/form × 双主题），触发后立即采样。
  3. 第三轮 `third.json`：4 组（role/form × 双主题），真实点击列表项/预览提交。
  4. 第四轮 `mobserver.json`：6 组（role/calendar/program × 双主题），**MutationObserver 在 toast 创建时点捕获文本+role+ariaLive**，解决 toast 3s 自动隐藏导致的采样时序矛盾。
- 四轮 pageerror 全为 0；statics 仅作候选锚点，全部结论以真浏览器实测为准。

## 3. 逐错误态程序化证据（与四轮 report 逐项一致）

错误态文本 sha 为纯文本 sha256（命令行 `printf %s | sha256sum`）；toast 捕获文本含图标「✗/ℹ」与「auto-hide」后缀，纯文本 sha 与之分列，二者不矛盾。

| 错误态 | 页/主题 | 真实触发 | 错误文本（去图标） | 纯文本 sha256 | 语义 (role / ariaLive / AX) | 同页等价建议来源 |
|---|---|---|---|---|---|---|
| 密码空提交 | personal / light+dark | 点「保存修改」 | 请填写完整密码 | `9ef4d3ca…c329282a` | `.error-msg` role=alert / ariaLive 空 / AX axRole=alert、non-ignored、color rgb(153,27,27)；R489 增强器已挂 `aria-describedby=oa-field-error-000001`×3、`aria-invalid=true`×3 | label 当前密码/新密码/确认密码；placeholder 请输入当前密码/请输入新密码/请再次输入（主轮 report labels/placeholders 实证） |
| 两次密码不一致 | personal / light+dark | 三框填不同值后保存 | 两次密码不一致 | `64eff9d1…f4965136` | role=alert / AX axRole=alert、non-ignored | 文本本身直指「两次密码」不一致，纠正方向明确；密码属安全例外 |
| 日程必填 | calendar / light+dark | 移除原生 required 后提交空表单 | 标题与起止时间必填 | `b2d651da…bd78c` | `.oa4-toast` role=alert / ariaLive=assertive / AX axRole=alert（第四轮 MO 捕获 `✗标题与起止时间必填auto-hide`） | form 内 placeholder「日程标题」+ aria-label「开始时间」「结束时间」+ required 标识；错误直指需补的三个字段 |
| Flag 为空 | role-manager / light+dark | 新建角色填名后空 Flag 保存 | Flag不能为空 | `19ee1514…cc2cabb` | `.oa4-toast` role=status / ariaLive=polite / AX axRole=status（第四轮 MO 捕获 `ℹFlag不能为空auto-hide`） | **同弹窗 flag 输入框 placeholder「如: admin, editor」给出具体格式示例**（第三轮 allNodes 实证 placeholders 含「如: admin, editor」）→ 等价建议存在 |
| 新口令过短 | program-center / light+dark | Config 面板点「改管理口令」，prompt 填 abc | 新口令至少 6 位 | `5081f54…075e0ae58` | `.oa4-toast` role=alert / ariaLive=assertive / AX axRole=alert（第四轮 MO 捕获 `✗新口令至少 6 位auto-hide`） | 错误文本本身含具体范围「至少6位」；触发 prompt 文案「新口令（至少 6 位）」（ProgramCenterApp.vue:1507）同给范围；密码属安全例外 |
| 表单预览空提交 | form-designer / light+dark | 选列表项→点预览→点提交 | （无可见错误） | — | 第三轮 meta：listItems=15、overlay=1、subCount=1、pvErrBefore=0、pvErrAfter=0；DOM 无 `.pv-error-msg` / 无 `[role=alert]` | validateFieldValue 会生成具体文本（`标题 不能为空`/`名称 至少2字符`/`编码 格式错误`，formFieldValidation.ts:47-61），但 preview 模板（FormDesigner.vue:119-167）未渲染 previewErrors，**错误态不可见** |

### 时序矛盾与补正（Fail Loud，不掩盖）

- 主轮/补采/第三轮对 role-flag 的即时采样 `visible=[]`，但主轮截图 `pc-role-flag-empty-light-r498.png` 目视右上角确有 toast「ℹFlag不能为空」——属 toast 3s 自动隐藏的采样时点差异，非名实不符。
- 第四轮 MutationObserver 在节点插入 body 的瞬间捕获，双主题均得 `ℹFlag不能为空auto-hide` / role=status / ariaLive=polite / AX axRole=status，程序化与截图目视自此一致。calendar/program 同样以 MO 捕获坐实。
- form-preview 三轮一致 `pvErrAfter=0`（overlay 与提交按钮都成功命中，非检测器失效）：错误态确实未渲染，本维度无法评估建议，据实判 L1（见 §4）。

## 4. 三分判定（本维度 WCAG 3.3.3）

- **可达真缺陷：0 项。** 逐一核对：personal 两提示（密码安全例外 + 「两次密码不一致」直指纠正方向 + 字段 label/placeholder 明确）；calendar（错误直指标题/起止时间三字段 + required 标识）；role-manager（同页 placeholder「如: admin, editor」提供具体格式等价建议，命中铁律②）；program-center（错误文本自身含「至少6位」具体范围，命中铁律⑤「带具体建议不算候选」+ 密码例外）——无任何一项满足「自动检测到错误、纠正建议已知、却未向用户提供」。
- **不可达 / 不适用 L1：1 类。** FormDesigner 预览提交：`validateFieldValue` 生成的具体错误文本从未渲染到界面（pvErrAfter=0、无 .pv-error-msg、无 role=alert），错误态对任何用户均不可见，本维度无「错误建议」可评估。此为 3.3.1 Error Identification 范畴（错误未被识别呈现），据实移交后续轮次，不在本维度臆造 3.3.3 缺陷。
- **正常规范预期：4 类（5 个错误态，双主题共 10 组实测）。** ① personal 密码空提交；② personal 两次不一致；③ calendar 日程必填；④ role-manager Flag 为空（等价建议）；⑤ program-center 口令过短（自带具体建议）。每类均有真实触发、错误文本 sha、AX 语义与等价建议来源四重实证。
- **反证边界：** 静态锚点 5 处覆盖 5 类错误态 + form 预览 1 类；未穷举全库校验提示，其余不预判为缺陷也不预判为正常。

## 5. 修复建议

- 本维度（3.3.3）无可达缺陷，**无需为错误建议内容发起修复**。
- 非阻断观察（移交 3.3.1 / 另轮次，不在本轮修改）：FormDesigner.vue:119-167 预览浮层未渲染 `previewErrors`，用户输入不合法后点提交无任何可见反馈；若后续立项，可在每个 `.pv-field` 下按字段 key 渲染错误文本（validateFieldValue 已提供含字段名与范围的 message），与 XformRuntime.vue:61 的 `data-oa-field-error` 增强器模式对齐，一并纳入 3.3.1 甄别。
- role-flag 使用 `toast.info`（role=status / polite）而非 error（role=alert / assertive），属 3.3.1 语义播报选择，非 3.3.3 建议缺失；本轮仅记录不判缺。

## 6. 截图与错误文本 sha（命令行核验）

全部 30 张 PNG 由 `sha256sum` 命令行核验，与四轮 report 内嵌 `shotSha` 三方一致；下表列代表性 20 张（其余同名脚本产物见证据目录）。

| 截图 | sha256 |
|---|---|
| `pc-personal-empty-light-r498.png` | `59f8d4054245ad2bde2f7e81d6364fd3be06b40d2a02e521df7f0e48c77ff746` |
| `pc-personal-empty-dark-r498.png` | `4e8da018138913b68bcd2b9958a0aaeb55809b8fc19efbc6fe5022d916121996` |
| `pc-personal-mismatch-light-r498.png` | `7af278ac98270a1e3c8210e2f97df00ccb5e229e7259ab29fee5ce77d5faffcd` |
| `pc-personal-mismatch-dark-r498.png` | `f3e1b7b98d068667bed85c0aa5ee513ac1690d1e65cddd3fc2144cd0b2f301bc` |
| `pc-calendar-forced-light-r498.png` | `8d8694de37850992579bc46f4d3f52827abeb1fb5a96a36f0397f70fbfba5fad` |
| `pc-calendar-forced-dark-r498.png` | `0075d87d57dc86a5bc3963fa095ecbabaa06562542fbaea5922b2234fe9f6c4b` |
| `pc-role-flag-empty-light-r498.png` | `6ced1258f3dbbf5de77eae3275ea234fa646e18ffd4771a9ba8ab39192a7902c` |
| `pc-role-flag-empty-dark-r498.png` | `7b4c93582c0ce2a56f396894bd6dd716383ddb1e6a963771ef3d90df7ccdb6c9` |
| `pc-program-short-pwd-light-r498.png` | `f71fb6ad8be8ab104eb061cf5ab2c16627c231fe4c03294bd3f452820e781146` |
| `pc-program-short-pwd-dark-r498.png` | `fd7a9aaa736504c27bc90ba44b0c8f6522894ea9f1f45b56f73b2b58ef907e9f` |
| `pc-form-preview-empty-light-r498.png` | `09aa4242ab6808dcb759f21388b505c959cf54427580f4a1c23a2d010206c11d` |
| `pc-form-preview-empty-dark-r498.png` | `48aa83fbb4ae6cb9c4797bedeadb1c5c442128cb8850643d312c04974e0d3b1f` |
| `mobserver-role-flag-light-r498.png` | `5c82820eaa528e922b70d122aba28657099d252aef8762291f32f36a4b5da566` |
| `mobserver-role-flag-dark-r498.png` | `9fe686b4b8ed89c6e1a93fe2a9fadecfa542b842dfb9317a7e7023971c0bba19` |
| `mobserver-calendar-forced-light-r498.png` | `7854e55f18fb0e540ee3e8ec04df7474c334219c35a98ce59e399d9c4eeaf558` |
| `mobserver-calendar-forced-dark-r498.png` | `b8bb0683e362def73b3cf9fd98105c1d7762ebd13fede155183f0889e3afa355` |
| `mobserver-program-short-pwd-light-r498.png` | `d84f29e611d22329c70c43498082bb33a3f770182295125c3631e64f7caec61e` |
| `mobserver-program-short-pwd-dark-r498.png` | `19758baaeb24620ce793084f3ff62a583c99b2f1f76236a715b4baea549ea773` |
| `third-form-preview-light-r498.png` | `c3a846ef23bcf57a0add982de3177cd2307cad01c8940fe57204848d27ed6041` |
| `third-form-preview-dark-r498.png` | `015d6ae6cbca6f197ec4b260a24182c26edbfcdc545cdc1bcdd63a686ac6f875` |

错误文本 sha256：请填写完整密码 `9ef4d3ca68473794db5a07c35184b15d4d10a6720ebe6d70ada46a07c329282a`；两次密码不一致 `64eff9d196466383b7e413239ddc2efd6a5e8fe1e944634523b2f331f4965136`；Flag不能为空 `19ee151461813937a8c559d7819568e3d92f87293282e3d8570816974cc2cabb`；标题与起止时间必填 `b2d651dacd0420dd0b0e6573917cdf28d4b91a2af9bf32119ab5ea9a6b1bd78c`；新口令至少 6 位 `5081f5482cc77607fcb2a9f9e279d1c723a813d4cad5a8ea48f6fdc075e0ae58`。

## 7. 脱敏与边界声明

- 脱敏：文档与所有入库证据不含真实域名、IP、主机名、端口、口令、证书、业务账号或个人路径；测试服地址以 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位，凭据由 runner 从环境变量注入。
- 截图来源如实标注：**headless 真浏览器渲染截图，非物理真机截图**，1440×900 DPR1。
- 本轮仅甄别不改码；唯一写域为本文档；四轮脚本与截图均 gitignored。≥1000 轮持续迭代，本轮非终态。
