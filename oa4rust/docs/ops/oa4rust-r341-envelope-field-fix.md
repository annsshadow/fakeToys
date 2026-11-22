# R341 PC 双层信封列表恒空（P1×3）+ CmsColumnManager 字段映射（P2）修复

> 证据来源：测试服 1440x900 Playwright Chromium headless 真浏览器实测（**非物理真机**）。域名/主机一律占位 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`，0 真值入库。测试账号凭据经运行期 SSH 读取，不回显、不入库。

## 一、范围与依据

依赖 R340 t144 只读审计（commit 287ee9be0，文档 `oa4rust/docs/ops/oa4rust-r340-crud-pages-audit.md` §三）。本轮只改 4 个 .vue + 1 个源码级契约 + 本文档，共 6 文件；不改后端、SDK、AdminTable 组件、其它页，不改 4 页 create/save/delete 端点调用与逻辑，破坏性真删未授权不执行。

## 二、根因与裁决

- **P1（3 页列表恒空）**：ProcessDictDesignerApp.vue:119 / ProcessFormDesignerApp.vue:138 / ProcessXformApp.vue:124 的 queryFn 统一 `Array.isArray(r?.data) ? (r.data as Item[]) : []`；这三页后端 list 返回双层信封 `{"data":{"count":N,"data":[...]},"type":"success"}`，`r.data` 是对象 → 为 false → [] → 列表恒空、create 200 成功但用户完全不可见、编辑/删除无行可触达（对照 cms-module 单层信封 `{"data":[...]}` 正常）。裁决：queryFn 兼容双层信封并保留单层——`const d=(r as any)?.data; return Array.isArray(d)?d:(Array.isArray(d?.data)?d.data:[])`。
- **P2（列显示破折号）**：CmsColumnManagerApp.vue list 返回小写字段 `columnid/personid/createtime/updatetime`，列定义/具名插槽用驼峰 `columnId/personId/updateTime`（:29/:32/:38），取 `row.columnId` 为 undefined → 显示「—」。裁决：queryFn 读取记录（兼容单层/双层）后映射驼峰 `columnId/personId/createTime/updateTime`，列 key 与表头/具名插槽保持不动，不破坏 AdminTable 匹配。
- fail-loud：任一页实测信封形状与裁决不同须回报队长，不静默改向；本轮 3 页实测均为裁决所述双层信封，无冲突。

## 三、修复内容

| 文件 | 改动 |
|---|---|
| ProcessDictDesignerApp.vue | queryFn 双层信封兼容（保留单层） |
| ProcessFormDesignerApp.vue | 同上 |
| ProcessXformApp.vue | 同上 |
| CmsColumnManagerApp.vue | queryFn 取行后映射小写→驼峰字段 |
| ProcessEnvelopeField.contract.test.ts | 新增源码级契约 9 it |

## 四、RED → GREEN 台账

- 契约 `ProcessEnvelopeField.contract.test.ts`：readFileSync 读 4 页源码，bodyOf 花括号配对抓取 queryFn 函数体，剥离 TypeScript 断言后 `new Function` 真执行（注入 api/loading/listEp 桩），断言双层信封返回内层数组、单层兼容、小写字段映射驼峰、列 key 与插槽保持、写端点保持。函数参数均带 `: string` 标注防 TS7006。
- 首次运行 7 failed 为**契约执行器缺陷**（new Function 直接执行含 `as Item[]` / `(row: any)` 的 TS 源码报 SyntaxError），非业务 RED；修正剥离逻辑并注入桩后确立真实 RED = **4 failed / 5 passed**（3 页双层信封返回 []、CMS 映射列 undefined）。
- GREEN：定点契约 **9/9 passed**；全量 `pnpm exec vitest run` **273 files / 2195 tests 0 failed**；`pnpm --filter @oa4rust/desktop typecheck` exit 0。

## 五、部署与真浏览器复验

- PC 构建入口 `index-EwvT83IV.js`；整目录 tar 原子替换远端 web 目录，脚本 `test-results/deploy-pc-r341-envelope.sh`，执行 `DEPLOY_R341_ENVELOPE_FULL_OK`（hash 比对通过）。
- 真浏览器脚本 `test-results/r341-envelope-field-check.cjs`（gitignored，1440x900 headless）：登录 → 3 页逐页「+ 新建」填 marker 保存 → 断言列表可见新行 → CMS 列管理器断言栏目 ID/成员 ID 非「—」、编辑/删除入口可触达；10 断言全 true：dictVisible/formVisible/xformVisible/createAll200/cmsRowsPresent/cmsIdsReal/cmsEditReachable/cmsDeleteReachable/noPageErrors/noApiFailures。
- 逐页实测：dict/form/xform 列表 3→4 行、marker 可见 1、POST create 均 200；CMS 3 行、编辑 3、删除 3，栏目 ID 列显示 `R34011479387-cms-column-manager` 等真实值（成员 ID 空值仍显示「—」属后端数据本身为空，非映射缺陷）。0 pageerror、0 API 失败。
- 截图（均 1440x900，gitignored `test-results/r341-envelope-field-fix/`）：`01-dict-visible.png` sha256 36bbd3ebfe92016e848f57781dbdab692dbbe6d0265ee3571ba64a930775699b；`02-form-visible.png` sha256 ab84cf52a4fb2dbfa08f04e3351224748af2735bce5c04ae71acad05e7520ba0；`03-xform-visible.png` sha256 63b5a4297aaad26de221b45792fec42e24de53320119d047c75e8758cab69e67；`04-cms-fields.png` sha256 3eede764a28ec0fd5eccc5064ea90997ba0c8d3685d4710e39a205ddd7eebe2e。`evidence.json` source 标注非物理真机。破坏性真删未授权未执行。

## 六、取证限制与结论

- 取证限制：破坏性真删（点「确认」）未授权执行，仅验证编辑/删除入口可触达；CMS 成员 ID 空值行的「—」是后端数据本身为空，未伪造数据填充。
- 结论：3 页 P1 双层信封列表恒空与 1 页 P2 字段映射列空均修复闭环，单层信封兼容未破坏，test-first RED→GREEN 成立，真浏览器 10 断言全绿、0 pageerror；无新增 P0。
