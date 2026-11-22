# R343 PC 查询设计器 queryKey 不匹配致写后列表不回显修复

> 证据来源：测试服 1440x900 Playwright Chromium headless 真浏览器实测（**非物理真机**）。域名/主机一律占位 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，0 真值入库；测试账号凭据经运行期 SSH 读取，不回显、不入库。

## 一、范围与依据

修复 t147 / R342 只读审计（commit c16d3a580，文档 oa4rust-r342-query-deep-audit.md 第四节）坐实的 P1 真缺陷。
依赖链：列表 useQuery 订阅 queryKey [query,defs]，而三处写操作失效打在无人订阅的 [qm,list] 上，写后不回显、刷新成死按钮。
inScope 仅三文件：QueryManagerDeep.vue、QueryManagerDeep.querykey.contract.test.ts、本文档。

## 二、根因与裁决（fail-loud）

- 根因：QueryManagerDeep.vue 列表 useQuery（:495-496）与 useListError（:510）订阅 queryKey [query,defs]；而新建 cm.onSuccess（:872）、删除 delM.onSuccess（:851）、刷新 refresh()（:883）三处全部 invalidateQueries({queryKey:[qm,list]})。两键完全不匹配，失效命中空订阅，TanStack Query 不触发重取。
- 后果：POST designer/create 200 且后端已落库（R342 evidence-v2 backend n=12），但列表不回显；点页面自带刷新按钮仍不更新；只有整页 reload 才显示。删除同理。
- 裁决（队长）：最小修复，仅把三处 invalidateQueries 的字符串字面量 [qm,list] 改为 [query,defs]，共 3 处（:851/:872/:883）。不改 useQuery 订阅键、不改端点调用/payload、不改其它页/后端/SDK/组件。
- 硬边界落实：git diff 确认源码仅 3 行字符串字面量替换；create/delete/refresh 的 api.post/api.delete/端点保持原样；实测 grep 全文件仅 3 处失效键、无第四处，未与裁决冲突，无需上报改向。
- fail-loud：若后续再出现写后不回显，须先核对 invalidate 键是否仍与订阅键一致，不得静默引入新键或回退旧键。

## 三、修复内容

- QueryManagerDeep.vue:851、:872、:883：qc.invalidateQueries({ queryKey: ['qm', 'list'] }) 改为 qc.invalidateQueries({ queryKey: ['query', 'defs'] })。
- 新增源码级契约 QueryManagerDeep.querykey.contract.test.ts（4 it，readFileSync 读源码 + queryKey 提取函数）：订阅键断言、三处失效键一致性、无孤立 [qm,list]、端点与函数保持。

## 四、RED → GREEN 台账

- RED：定点契约 2 failed / 2 passed —— 三处失效键为 qm,list（期望 query,defs），孤立 [qm,list] 计数 3（期望 0）。
- GREEN：定点契约 4/4 passed；全量 vitest 274 files / 2199 tests 0 failed；pnpm --filter @oa4rust/desktop typecheck exit 0。
- 部署：PC build 入口 index-CvBTgB9X.js，整目录 tar 原子替换远端 web 目录，sha256 校验一致，DEPLOY_R343_QUERYKEY_FULL_OK。踩坑：部署脚本 awk 字段名须用单引号，双引号会让 $1 被 shell 提前展开导致 hash mismatch。

## 五、真浏览器复验（1440x900 headless，非物理真机）

脚本 test-results/r343-querykey-check.cjs（gitignored）：登录 → /app/admin/query-manager-deep → 记录初始列表 12 条 → 真 UI 新建查询（填名称/SQL/分类，POST designer/create 200）→ 断言即时回显 13 条且新名可见、列表 GET 重取次数 +1 → 点页面刷新按钮后仍 13 条且新名可见 → 删查询 confirm 守卫点取消 0 DELETE。
7 断言全 true：create200 / listRefetchedAfterCreate / immediateVisible / refreshKeepsList / deleteGuardNoDelete / noPageErrors / noApiFailures。
证据：gitignored test-results/r343-querykey-fix/ 下 evidence.json（source 标非物理真机）与 3 张 1440x900 截图：01-create-immediate.png sha256 ecda91d1、02-refresh-keeps-list.png sha256 716d998f、03-delete-guard.png sha256 132cf4a5。

## 六、取证限制

- 破坏性真删未授权执行，仅验证 confirm 守卫点取消后 0 DELETE；如需坐实真删后列表消失须另行授权。
- 删除 onSuccess 的失效修复与新建/刷新同构（同一 queryKey 字面量替换），逻辑一致性由源码契约覆盖，未单独对真删做端到端实测。
- 63 写端点其余端点不在本轮修复范围，仅按 R342 既有代表性实测结论，未逐条重跑。
