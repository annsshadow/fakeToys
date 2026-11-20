# R300 B1 — PC 日历页 TDZ 崩溃修复实测记录

## 问题与依据
t10 C1 真浏览器实测三次进入 `/app/calendar` 均触发 `Cannot access 'q' before initialization`（压缩产物变量 q 对应 `calListError`），页面出现「页面发生意外错误，请重试」。根因：`CalendarApp.vue` 在 `calListError = ref('')` 等响应式声明之前同步调用 `loadCalendars('my')`，而该函数首行写入 `calListError.value`，形成暂时性死区（TDZ）。

## 修复
- 先写 `CalendarApp.tdz.contract.test.ts` 做 RED 契约：真实编译 `<script setup>`，静态断言调用顺序，并在 vm 中模拟依赖执行 setup，锁定默认列表加载与无同步异常。首次运行 5 failed，证明缺陷真实存在。
- 将 `loadCalendars('my')` 从函数定义后移到所有 `ref` 声明之后；接口路径、加载逻辑、日历网格与 `businessToday` 计算位置均不变。契约转 GREEN。

## 验证
- `pnpm --filter @oa4rust/desktop exec vitest run src/views/CalendarApp --reporter=dot`：3 files / 11 tests 全通过。
- `pnpm run typecheck`（在 `oa4rust` 根执行，frontend 无 manifest）：exit 0。
- `pnpm --filter @oa4rust/desktop build`：exit 0；产物入口 `index-CAtVICEH.js`、`index-BOp8VUI6.css`。
- 按既有流程原子部署测试服（静态目录由环境变量/SSH 别名注入，真值不入库），远端校验新入口哈希存在。
- Playwright Chromium headless 真实登录测试服后连续 3 次进入 `/app/calendar`：
  - `pageerror=[]` ×3；无「意外错误/请重试」提示 ×3；日历标题与网格元素存在 ×3。
  - 截图：`test-results/r300-calendar-tdz-fix/calendar-run-{1,2,3}.png`，证据：同目录 `evidence.json`。
  - 来源如实标注：Playwright Chromium headless 对部署测试服真实页面截图，非物理真机；测试服域名与登录凭据均不写入仓库，凭据运行时从远端受控文件读取。

## 范围与后续
仅改 `CalendarApp.vue` 与新增 `CalendarApp.tdz.contract.test.ts`；未触碰 Dashboard、mobile、后端。项目为 ≥1000 轮持续迭代，本轮不是完成态。
