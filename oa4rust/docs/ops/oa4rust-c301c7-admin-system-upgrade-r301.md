# R301 C7 管理区与系统域实质升级（脱敏记录）

## 目标
按设计评审 findings 对管理区/系统域做信息层级、表单可读性、权限可视化与表格密度的实质优化，不只改颜色。

## 改动
- 角色管理：成员按钮由仅 toast 人数改为角色权限明细弹窗（成员列表 / 空态解释 / 人数），角色 Flag 列增加可读「标识」标签，空态增加新建主行动；保留全部角色 CRUD 与端点。
- 三方成员：单位 ID 字段改为可读「所属单位」，表格单位列与状态列改为可读标签/徽标（已启用/已停用/待审核），状态输入改为下拉，保留搜索、CRUD 与端点。
- AdminTable 共享表格原语：新增标准/紧凑密度切换（data-testid=admin-table-density，admin-table--compact），保留 columns/rows/loading/error/skeletonRows/rowKeyField/retry。

## 契约与验证
- 新契约 src/views/AdminSystemUpgrade.C301.contract.test.ts：RED 4 failed/1 passed → GREEN 5/5。
- 相关回归 AdminSystemUpgrade + AdminZone B5 + ModernBusinessC + NonStyleFixes：21/21 passed。
- pnpm run typecheck：通过；pnpm --filter @oa4rust/desktop build：通过（index-BomljjCy.js / index-BMFkaD73.css）。
- 构建已原子部署到测试服（真实主机地址与口令仅从部署环境读取，不入库）。

## 真浏览器实测（非物理真机）
Playwright Chromium headless 访问已部署测试服，凭据在运行时从部署环境读取；逐页截图与 evidence.json 落 gitignored 的 oa4rust/test-results/r301-c301c7/。
- /app/role：角色 4 行，点击成员打开明细弹窗，成员数/空态可见，标识标签可见。
- /app/three-member：单位/状态 21 行均显示可读信息，表单单位字段与状态下拉可见（已启用/已停用/待审核），无「单位 ID」文案。
- /app/admin/process-application：AdminTable 密度控件可见，点击「紧凑」后 admin-table--compact 生效。
- pageerror=[]；结果 PASS。截图 01~06（1440x900）与 evidence.json 位于 gitignored 的 oa4rust/test-results/r301-c301c7/。

## 截图 sha256（非物理真机）
- 01-role-list.png: 05447b64bee2d1d6c4f20b8e8bdf3963061fe3b2e07b3b8112c13b4441ca6ea5
- 02-role-member-detail.png: 0e48028c521521fbed4f552dc67bdf053296f22b691aa75211e66a5c43867a62
- 03-three-member-list.png: a4818a1dd5d184524410729555f2230284f5413a86bea4c06244564814dc7002
- 04-three-member-form.png: 3e5e29248ac2561605821a5a2fe8e03fa6ada0729532049ddf41804ff413ae7f
- 05-admin-table-standard.png: 32a7c9b775ab2ceca6b39cbf7ceb4cf1df093a2d30865bef26d5e43ea9381968
- 06-admin-table-compact.png: 6b35fcff790551d2705ab11439444652f9371a1b3697b1fc1e5a3192a3b7713a

## 兼容性说明
页面标题保持无页面级 h1/使用 aria-label，以维持 shell 唯一 h1 与历史契约；端点调试能力由 import.meta.env.DEV 门控，本次未触碰。