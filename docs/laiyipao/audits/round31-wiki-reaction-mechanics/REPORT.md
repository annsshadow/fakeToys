# R31 复查：Wiki 机制字段真浏览器多端对账

## 一、结论摘要

PASS。以公开 `/api/v1/config` 为唯一权威，在真实 Chromium（agent-browser）中于 1440×900、375×390、390×844 三视口逐行对账反应表「层数传播 / 削甲 / 击退」三字段，7 行 × 3 字段与权威值零不符；页面级无横向溢出，表格自身可真实滚动至末列。目录深链、搜索命中与焦点 URL 传播均实测有效，「层数传播已实现」与「燃烧云仍未实现」文案共存且旧文案不存在。定向测试 46/46、admin 全量 779/779 通过。

本轮未改业务源码、未提交 git。含本地令牌的注入脚本已删除，本轮 data/ 与注入脚本已加入 .gitignore；QA 库 battle_records 保持基线 104 条 / max_id=477，未产生新战斗记录。

## 二、环境与权威数据

- 服务：本地 API、admin（5175）、H5（5174）、QA PostgreSQL（15440，容器）全部启动并核验就绪；admin `/wiki` 在本会话中因路由守卫要求令牌，使用本地临时 admin 令牌注入脚本进入（脚本用后即删，未入库）。
- 权威端点：公开 `GET /api/v1/config`，版本 `6237e0b623219b8b`，反应条目 7 条；快照落 D/data/config.json（已 gitignore）。
- 测试视口：1440×900（桌面）、375×390、390×844；均为真实 Chromium，非物理真机。

## 三、七项门禁验收

| 门禁 | 结果 | 证据 |
| --- | --- | --- |
| ① 三视口逐行三字段对账 | PASS | 三视口 rows 均 7 行；与权威 7×3 字段零不符，详见下表 |
| ② 表格可滚末列、页面无横溢 | PASS | 桌面 scrollLeft 0→206/max206；375 0→781/max781；390 0→766/max766；page scrollWidth=clientWidth=innerWidth |
| ③ 深链/搜索/文案共存 | PASS | 目录 href 14 项逐项可读；深链 `?tab=enemies&anchor=reactions` 落到敌人图鉴；搜索「腐蚀扩散」命中并写回 `?anchor=reactions&focus=reaction:corrosion_spread`、scrollY=708；hasImplement=true、hasBurnCloud=true、oldWording=false |
| ④ ≥4 张新图 sha 各异、亲读非登录页 | PASS | 5 张本轮新图，sha 两两不同；全部亲读为 Wiki 页面，已删除早期登录页误截图 |
| ⑤ 定向 46/46 与 admin 全量 779/779 | PASS | vitest 定向 3 files/46 tests；全量 69 files/779 tests |
| ⑥ 零真值清理与 QA 基线 | PASS（经 t243 更正后） | 令牌注入脚本删除、data/ 被 gitignore、审计脚本绝对路径污染已清除并由确定性测试守卫；QA records=104/max=477 与基线一致；详见第十节 |
| ⑦ 不改业务源码、不 git | PASS | QA 仅追加审计文档与 .gitignore 两行过滤规则；业务代码改动均为上游实施 |

## 四、逐行对账表（权威 / DOM 三视口一致）

| 反应 | 权威 spread_stacks / DOM 层数传播 | 权威 armor_shred_permille / DOM 削甲 | 权威 knockback / DOM 击退 |
| --- | --- | --- | --- |
| 蒸汽爆发 | 0 / — | 0 / — | 0 / — |
| 过热 | 0 / — | 0 / — | 0 / — |
| 燃烧云 | 0 / — | 0 / — | 0 / — |
| 超导 | 0 / — | 0 / — | 0 / — |
| 急速冻结 | 0 / — | 0 / — | 0 / — |
| 腐蚀扩散 | 1 / 传播 1 层 | 0 / — | 0 / — |
| 破甲击退 | 0 / — | 150 / 150‰ | 4000 / 4px |

口径：`spread_stacks` 正整数才显示「传播 N 层」；`armor_shred_permille` 正数显示 N‰（150‰=15%，页面未误写 15%）；`knockback` 为 ×1000 定点，4000 显示 4px。0/负/NaN/∞/非整数一律显示「—」，实测 `badToken=false`、`has15percent=false`、`has150permille=true`。

## 五、三视口几何与滚动证据

| 视口 | 页面 scrollWidth/clientWidth/innerWidth | 表格 wrapper | 实测 scrollLeft | 结论 |
| --- | --- | --- | --- | --- |
| 1440×900 | 1440/1440/1440 | sw=1090,cw=884,overflowX=auto | 0→206（max=206） | 页面无溢出，表格可滚末列 |
| 375×390 | 375/375/375 | sw=1090,cw=309,overflowX=auto | 0→781（max=781） | 页面无溢出，表格可滚末列 |
| 390×844 | 390/390/390 | sw=1090,cw=324,overflowX=auto | 0→766（max=766） | 页面无溢出，表格可滚末列 |

390×844 中将表格滚至末列后进一步量测：最后一格 `.el-table` 单元格 right=357，与 wrapper right=357 对齐，`docOverflowX=0`；证明「说明」末列可达且未越界。目录窄屏变顶部横条、正文单列，未在文档层产生横向滚动。

## 六、目录 / 深链 / 搜索与诚实文案

- 目录锚点 14 项：桌面视口 eval 读取每项文本与 href，示例「反应链一览」→`#/wiki?anchor=reactions`、「关卡一览」→`#/wiki?tab=levels`，与 `SECTIONS` 派生一致。
- 深链：在 390×844 打开 `#/wiki?tab=enemies&anchor=reactions`，URL 被归一化为 `#/wiki?tab=enemies`（非法/不适用 anchor 被忽略），落到敌人图鉴，124 个表行可见，`docOverflowX=0`；第四张截图证明不是登录页。
- 搜索：在搜索框填「腐蚀扩散」，命中 1 条「反应 / 腐蚀扩散 / 元素反应」；真实 CLI 点击后 URL 变为 `#/wiki?anchor=reactions&focus=reaction:corrosion_spread`，页面滚至反应表（scrollY=708），第五张截图可见该表末列与说明文本。焦点行本身在截图时已过短暂高亮窗口（`focusRows=0`），但 URL 传播与滚动已坐实；焦点高亮的单元测试见 `tests/WikiView.test.ts:364` 起，随全量 779/779 通过。
- 文案：探针 `hasImplement=true`（「层数传播已实现」）、`hasBurnCloud=true`（「燃烧云 burn_cloud 的火区仍未实现」）、`oldWording=false`（旧「没有特殊效果的只剩两条」不存在），`has150permille=true`、`has15percent=false`——诚实边界与单位口径均符合规范。

## 七、截图清单（本轮新图，均亲读非登录页）

| 文件 | 视口 | 内容 | sha256 |
| --- | --- | --- | --- |
| r31a-desktop-wiki.png | 1440×900 | 桌面 Wiki 与反应表 | 9237e65d4ce47f2431e8d1950d769408796808d31e0711f94da92ef224bf77b5 |
| r31a-mobile375-wiki.png | 375×390 | 窄屏首屏与搜索框 | de75ce6a358b7ed36d3c7f6adcc2e08a658e6d53e62279680df612f609a51853 |
| r31a-mobile390-wiki.png | 390×844 | 移动目录与反应表首屏 | e5f10e4d5cd531f2b5fdddac9188e1cbb7ca93ad71652c4a248b0101f4588407 |
| r31a-04-deeplink.png | 390×844 | 深链落到敌人图鉴 | 209f8af57d783a8ad53faed5cd85204a1a1bc1cee1dafd87859e691956b6a9a1 |
| r31a-05-search-focus.png | 390×844 | 搜索聚焦反应表末列 | ec8a0d6fcff44dd9e317bad4e33466fe5400e28a2b7dcf0e233b40d99ec58208 |

早期匿名会话因路由守卫跳登录页的截图及重复截图已删除，不作为证据。五个 sha 两两不同并逐张亲读确认内容差异：桌面全表、窄屏首屏、移动首屏、敌人图鉴、搜索聚焦末列。

## 八、测试、清理与命令

- 定向：在 `laiyipao/admin` 运行 `pnpm vitest run tests/WikiReactionMechanics.test.ts tests/WikiReactionDescr.test.ts tests/WikiView.test.ts --reporter=dot` → 3 files / 46 tests 全通过。
- 全量：`pnpm vitest run --reporter=dot` → 69 files / 779 tests 全通过（部分 router 警告为既有测试噪声，未失败）。
- 零真值：含本地 admin 令牌的 `admin-auth-init.js` 已删除；`.gitignore:366-367` 拦截本轮 `data/` 与注入脚本，`git check-ignore` 对两个路径均命中。
- QA 库：`docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*),coalesce(max(id),0) from battle_records'` → `104|477`，与复查前基线一致；本轮无战斗写入，无需 pg_restore。
- `git diff --stat -- laiyipao/admin/src ...` 显示 WikiView.vue 有上游实施改动（17 insertions/4 deletions），非本 QA 所为；本 QA 只新增审计文档与 .gitignore 过滤行，未提交。

## 九、局限与最终判定

- 测试在 headless Chromium 与 jsdom 中进行，非物理真机；三视口尺寸与 DOM 几何来自真实浏览器。
- 搜索焦点高亮的短暂窗口在截图时已过，截图证明滚动与 URL 传播，行高亮由单元测试覆盖；如实记录不冒充截图直接证据。
- 桌面视口 `/wiki` 在无令牌会话会被路由守卫送登录页；本轮为验证文档内容使用本地临时令牌进入，属于测试环境鉴权流程，非产品缺陷。

**最终判定：PASS。**七项门禁全部满足；未发现需要新建修复任务的真实产品缺陷。
## 十、t243 审计脚本零真值更正与复验（本轮修复后追加）

原报告第八节“零真值”结论仅覆盖令牌与 data/ 过滤，遗漏了三份待入库审计脚本中含本机个人绝对路径这一真值形态；因此原“零真值 0 命中”的表述不成立，现更正为：清理前存在本机绝对路径污染，清理后复验为零命中。

- 清理范围（仅审计产物与测试，不含产品业务源码）：e2e-viewports.sh、e2e-viewports-auth.sh、mint-admin-token.sh、WikiReactionMechanics.test.ts、本报告。
- RED 证据：先在 laiyipao/admin/tests/WikiReactionMechanics.test.ts 加入两条确定性守卫（禁止盘符/用户目录模式、必须使用 git rev-parse --show-toplevel），独立运行 pnpm exec vitest run tests/WikiReactionMechanics.test.ts 得 1 file failed / 2 tests failed；失败来自三份脚本中的本机绝对路径与缺少仓库根发现逻辑。
- GREEN 证据：三份脚本改用 cd "$(git rev-parse --show-toplevel)" || exit 1 后，守卫正则排除 URL scheme 的误报（避免把本地回环 http URL 当盘符），同命令复跑得 1 file passed / 18 tests passed；三份脚本 bash -n 均通过，git diff --check 通过。
- 本报告不回显任何真实绝对路径或令牌；实际路径仅存在于本地工作树脚本的历史状态中，现已替换为仓库根发现。
- 未修改 WikiView.vue、reactionMechanicsText.ts、engine/后端/replay/tokens；未执行 git add/commit/push。

更新后的最终判定仍为 PASS：产品功能门禁不受本次审计脚本清理影响，且零真值结论现由 RED→GREEN 测试与脚本复验共同支撑。
