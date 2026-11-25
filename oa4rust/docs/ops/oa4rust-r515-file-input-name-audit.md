# R515 WCAG 4.1.2 文件输入控件可访问名称甄别报告（只读，不改码）

## 1. 依据与判据

- 轮次：R515；真父 commit 87aecfaabacf471d6eafe7aef0c204a06eef7faa（台账 §509）；工作分支 opt1000/oa4rust-pc-ai-refactor。
- 判据（WCAG 4.1.2 Name, Role, Value）：用户界面组件须有可程序化判定的名称/角色/值；本轮聚焦文件输入控件的**可访问名称**。队长派单预设：axName 为空或仅 UA 默认 Choose Files 才算缺陷，兄弟 label 非包裹不算程序化关联。
- 反预判铁律：派单判据仅为静态预设，须以真浏览器 DOM + CDP Accessibility.getPartialAXTree 实证为准；静态预判被实测推翻时据实记录，不机械套用。
- 锚点：frontend/apps/desktop/src/views/QueryManagerDeep.vue:205，「导入配置」面板内 <div class="ic-row"><label>源文件</label><input ref="importFileEl" type="file" class="ic-file" accept=".csv,.tsv,.txt"/>。源码层 label 无 for、input 无 id/aria-label/aria-labelledby。
- 对照：Personal.vue:21 <label class="upload-btn"> 包裹 input 且 aria-label="更换头像"；FileManager.vue:156 已由 R509 裁决 P2（提示用 <p> 而非 <label>，增强器不识别，axName 仅 UA 默认 Choose Files）。

## 2. 方法与自洽

- 工具：Playwright Chromium headless，1440x900 DPR1，light/dark 双主题，非物理真机；CDP DOM.querySelectorAll + Accessibility.getPartialAXTree 取 axRole/axName/axIgnored/nameSources/props.focusable；DOM 层取 outerHTML/id/aria-label/htmlFor/closest 标签关系。
- 入口障碍与处置：导入 tab 模板为 v-if="selected.importModel"（:121），而后端 query_list_all（query_assemble_designer/src/lib.rs:2076）只返回 id/name/category/creator/createTime，不含 importModel 字段，真实数据下 14 条列表项 selected.importModel 恒 undefined，导入面板无法进入。为先坐实 file input 本身名称语义，第二轮用 page.route 拦截 designer/list/all 响应注入 importModel=true（仅测试进程内存，不改后端、不入仓），走真实 UI 点击「导入配置」触达渲染路径后量测。
- 自洽校验：注入 aria-label="上传源文件" 后 axName 实时变为「上传源文件」，证明该控件的 AX 名可被程序化属性驱动（非不可变 UA 默认）；Personal 正面对照 axName=更换头像/inLabel=true 两次一致。
- 首轮（无注入）：遍历 14 条列表逐条 selectQuery，mode-tabs 恒为 SQL编辑/视图配置/表格设计/统计分析，导入 tab 0 次出现，found=-1；据此据实判 L1，未当作命名缺陷。

## 3. 逐项程序化证据

| 项 | light | dark | 判读 |
| --- | --- | --- | --- |
| 首轮 found（导入 tab） | -1（14/14 无导入配置） | -1（14/14 无导入配置） | 真实数据下入口不可达 |
| 注入后 routeHits / injected | 1 / 14 | 1 / 14 | 拦截生效，走真实 UI 触达 |
| 注入后 importTabCount / importVisible | 1 / 1 | 1 / 1 | 导入面板真实渲染 |
| DOM file.id | oa-label-control-000001 | oa-label-control-000001 | 运行时注入的 id |
| DOM label[for] | oa-label-control-000001（文本「源文件」） | 同 | label↔input 程序化关联成立 |
| DOM aria-label / aria-labelledby | 空 / 空 | 空 / 空 | 源码无显式 ARIA 名 |
| DOM rect | 878,668 500x20 | 同 | 控件可见可聚焦 |
| CDP axRole | button | button | 角色正确 |
| CDP axName | 源文件 | 源文件 | 真实业务名，非空、非 UA 默认 |
| CDP axIgnored / focusable | false / true | false / true | 未被无障碍树忽略 |
| 注入 aria-label 后 axName | 上传源文件 | 上传源文件 | 程序化属性可驱动名称 |
| Personal 对照 axName / inLabel | 更换头像 / true | 更换头像 / true | 正面对照一致 |
| pageerror | 0 | 0 | 无运行时异常 |

## 4. 三分判定

### 可达真缺陷：0 项

- QueryManagerDeep.vue:205 file input 的真实可访问名为「源文件」（非空、非 UA 默认 Choose Files），role=button/axIgnored=false/focusable=true，label↔input 程序化关联成立。符合 WCAG 4.1.2，不构成可达命名缺陷。
- 名称来源：全局 installLabelA11y()（frontend/apps/desktop/src/utils/labelA11y.ts:39，main.ts:618 安装）在运行时为孤儿 label 与其紧随的 labelable 控件建立 id/htmlFor 关联；labelA11y.contract.test.ts:43 契约坐实 input.id=label.htmlFor 且 label.control=input。该增强器是产品正式无障碍基础设施（R213 引入），属真实运行形态，非测试注入产物。

### 不可达 / L1：1 类

- 导入配置面板入口不可达：后端 query_list_all 不返回 importModel 字段，selected.importModel 恒 undefined，14 条列表项均无「导入配置」tab；需 route 注入方触达。属可达性障碍，已据实披露；非 4.1.2 命名问题，不夸大计入缺陷。

### 正常 / 规范预期：3 类

- Personal.vue:21 正面对照：axName=更换头像，label 包裹 input，inLabel=true（双主题一致）。
- 增强器 label↔control 关联：DOM 层 label[for]=input.id，CDP 读到真实业务名「源文件」，契约测试佐证。
- 控件基础语义：axRole=button、focusable=true、axIgnored=false、可见矩形 500x20。

## 5. 与 R509 同构对照（关键反转）

- R509 FileManager.vue:156 提示节点为 <p>拖放文件到此处…</p>，非 label 元素，增强器 findControl 仅从 label 出发，故不被识别，axName 停留 UA 默认 Choose Files → 判 P2。
- R515 QueryManagerDeep.vue:205 提示节点为 <label>源文件</label>，虽源码无 for，但增强器识别孤儿 label 并注入 id/htmlFor → axName=源文件 → 判 0 缺陷。
- 二者同为「源码层无显式名称属性」，但真实运行形态不同，结论不同；本轮据实测据实判定，不因同构而沿用 R509 结论。

## 6. 修复建议

- 无可达命名缺陷，无需为 4.1.2 修复。两条非阻断观察（不纳入本轮缺陷）：
- （a）源码层可为 input 显式加 aria-label="上传源文件" 或为 label 加 for + input 加 id，使名称不依赖运行时增强器，降低对 JS 注入的耦合；
- （b）若期望「导入配置」入口在真实数据下可达，后端 query_list_all 应返回 importModel 字段，否则该面板对用户长期不可见。

## 7. 证据 sha256（命令行核验）

| 文件 | sha256 |
| --- | --- |
| test-results/r515-file-input/report.json | f98dcfe388e18ac93838eec55adf4299187dd409eb2ffaef5b486db11fb2c08c |
| test-results/r515-file-input/supplement.json | fb5874c0ca39aff2595fe5f6868501f804c7bd65759a0df70d7426847db2093d |
| pc-qmd-import-light-sup-r515.png | c89bab381d9fef47b2ebd008ce402d5a71e8b32c42e9707033ade8f9f0c25153 |
| pc-qmd-import-dark-sup-r515.png | 70fd29aa61c5badc7205257d64328c6a497ad4944464c20752be0a80d0f5000b |
| QueryManagerDeep.vue | f356b2c85a187b4be7b6c2f9fd7caacfa493a1f1991c77d85639cba3b8aa0b1e |
| Personal.vue | 8a323a15ea9e3c66f8114dede3fbb38f403502a131c23c7fe75bdfe58088d9b1 |
| FileManager.vue | 832fc7d5f22637d383325d9bfafd189f22b7d68d6f384e486111bbe2bf38020b |
| labelA11y.ts | 0885bb1f2a91b12758431adbfea72387ecf32b9a073d19ba39fa8cdd8302f0ab |
| main.ts | e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0 |

## 8. 脱敏边界与结论

- 脱敏：真域名/主机一律占位 <YOUR_TEST_DOMAIN>/<TEST_HOST>；口令仅运行时从环境变量读取，不入仓；runner 真域名仅存在于 gitignored 本地脚本。证据 JSON 内 URL 为运行时记录，位于 gitignored test-results 目录，不入库。
- 截图来源：Playwright Chromium headless 1440x900 DPR1 真浏览器页面截图，非物理真机；已目视核对与文件名一致（查询管理深化页 + 导入配置面板 + Choose Files 控件，light/dark 双主题）。
- 结论：R515 file input 可访问名称甄别完成——可达真缺陷 0 项，不可达 L1 1 类，正常 3 类。核心反转：静态预设的「无 for 即无关联」被真浏览器实测（增强器注入关联 + axName=源文件）证伪，据实判 0 缺陷。项目持续迭代，本轮非终态。