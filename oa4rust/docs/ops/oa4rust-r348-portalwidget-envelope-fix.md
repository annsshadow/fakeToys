# R348 PC 门户组件设计器双层信封列表恒空修复

> 证据来源：测试服 1440x900 Playwright Chromium headless 真浏览器实测（**非物理真机**）。域名/主机占位 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST>，0 真值入库；测试账号凭据经运行期 SSH 读取，不回显、不入库。

## 一、范围与依据

修复 t154 / R347 只读审计（文档 oa4rust-r347-remaining-audit.md，evidence-v2.json 深探）坐实的 P1 真缺陷。
根因：PortalWidgetDesignerApp.vue:90 queryFn 仅解析单层信封 Array.isArray(r?.data)，而后端 GET /api/portal/assemble/designer/widget/list 返双层信封 {data:{count:N,data:[...]},type:success}，r.data 为对象→返回 []→列表恒空、新建成功不可见。
inScope 仅三文件：PortalWidgetDesignerApp.vue、新增契约 PortalWidget.envelope.contract.test.ts、本文档。

## 二、根因与裁决（fail-loud）

- 根因：单层信封假设与双层信封实际返回不匹配，Array.isArray(r.data) 对对象为 false，列表数据源恒为空数组。
- 裁决（队长）：与 R341 三页同范式，queryFn 改为兼容双层信封并保留单层：先取 const d=(r as any)?.data，return Array.isArray(d)?d:(Array.isArray(d?.data)?d.data:[])。不改后端/SDK/AdminTable/其它页，不改 create/save/delete 端点与 payload。
- 硬边界落实：git diff 确认源码仅 :89-91 queryFn 三行改动（const d + 双层返回），listEp/createEp/qk、表单与写端点全部保持。
- fail-loud：若后续该页列表再恒空，须先用裸探针看 list 响应信封层级，不得臆造新解析分支；双层内层非数组仍返回 []（空态可区分于 500 兜底）。

## 三、修复内容

- PortalWidgetDesignerApp.vue:89-91：新增 const d=(r as any)?.data，返回 Array.isArray(d)?(d as Item[]):(Array.isArray(d?.data)?(d.data as Item[]):[])。
- 新增源码级契约 PortalWidget.envelope.contract.test.ts（5 it，readFileSync + bodyOf 花括号配对 + new Function 真执行 queryFn，剥离 TS 断言并注入 api/loading/listEp 桩）：双层信封返回内层数组、单层信封兼容、非数组安全返回空、listEp 端点保持、写链保持。

## 四、RED → GREEN 台账

- RED：定点契约 2 failed / 3 passed —— 双层信封场景返回 []（期望内层数组），故列表恒空可复现。
- GREEN：定点契约 5/5 passed；全量 vitest 276 files / 2210 tests 0 failed；pnpm --filter @oa4rust/desktop typecheck exit 0。
- 部署：PC build 入口 index-DIgRQMur.js、PortalWidget chunk PortalWidgetDesignerApp-CQ36S6__.js，整目录 tar 原子替换远端 web 目录，sha256 校验一致，DEPLOY_R348_PORTALWIDGET_FULL_OK。

## 五、真浏览器复验（1440x900 headless，非物理真机）

脚本 test-results/r348-portalwidget-check.cjs（gitignored）：登录 → /app/admin/portal-widget-designer → 记录初始列表 2 行 → 真 UI 新建门户组件（填名称/门户ID/分类/配置，POST designer/widget/create 200）→ 断言即时回显 3 行且新名可见、列表 GET 重取 1→2 → 点刷新按钮后仍 3 行且新名可见 → 删组件 confirm 守卫点取消 0 delete 调用。
8 断言全 true：listRowsPresent / create200 / listRefetchedAfterCreate / immediateVisible / refreshKeepsList / deleteGuardNoDelete / noPageErrors / noApiFailures。行数 2→3→3、list GET 1→2、createStatus 200、delete 调用 0→0、pageErrors 与 apiFailures 为空。
证据：gitignored test-results/r348-portalwidget/ 下 evidence.json（source 标非物理真机）与 3 张 1440x900 截图，sha256 均为 6de6edc28c5ac5dadf93b646283170b3a0d4774ca37fa6f4b9cf965eea8d1b77。三张图内容一致属真实情况：新建成功后、点刷新后、删除守卫取消后页面均为同一稳定 3 行列表，UI 无视觉变化，非复制冒充；断言均基于 DOM 计数与 API 捕获，不依赖截图像素差异。

## 六、取证限制

- 破坏性真删未授权执行，仅验证 confirm 守卫点取消后 0 delete 调用；删除 onSuccess 的双层信封读取与新建共用同一 queryFn，由源码契约覆盖。
- 列表既有 2 行含 R347 审计期写入的记录；本轮新建记录用于证明即时回显，未清理（测试服数据），不影响功能判定。
- 其余写端点（save/save/{id}）不在本轮修复范围，仅保持原样未重跑端到端。
