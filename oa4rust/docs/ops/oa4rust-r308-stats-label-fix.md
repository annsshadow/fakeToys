# R308 统计卡「加载中」误导标签修正（Hotpic / AppInfo）

## 来源声明
依据 \`docs/ops/oa4rust-r308-baseline-reaudit.md\` §3/§6（t62 R308 基线再审计）：
- 旧 loading 文本正则 \`/加载中|正在加载|Loading/\` 命中「PC loading 2 页」（19-admin-hotpic、21-admin-appinfo）是统计卡标签文案造成的假阳性——两页表格数据已正常渲染，无真实加载态。
- 新缺陷 L1：\`HotpicApp.vue:100\`、\`AppInfoApp.vue:105\` 的 stats computed 第四项 \`{ label: '加载中', value: loading.value ? 1 : 0 }\`，加载完成后值归 0，DOM 恒定显示「加载中 0」误导用户，也是历史假阳性根源。

## 修正前后
- 修正前：两页 stats computed 无条件包含 \`{ label: '加载中', value: loading.value ? 1 : 0, ... }\`，加载完成后该项仍渲染为「加载中 0」。
- 修正后：改为 \`...(loading.value ? [{ label: '加载中', value: 1, ... }] : [])\`——加载中时仍可呈现真实加载态提示，加载完成后统计卡数组不再含该项；其余统计项与业务接口完全不变。
- 真 bug test-first：新建 \`StatsCardLabel.R308.contract.test.ts\`，从源码提取 stats computed 数组表达式并用桩求值，真实验证 loading=false/true 下数组内容。RED（2 failed：两页 loading=false 仍含「加载中」项）→ 改实现 → GREEN（6/6 passed）。

## 真浏览器对照
构建 \`pnpm --filter @oa4rust/desktop build\` exit 0（新入口 \`index-kQdwUF6q.js\`），原子部署测试服后 Playwright Chromium headless 真登录逐页复验（1440x900）：

| 页面 | 路由 | 「加载中」文案数 | 统计卡数 | 表格行 | pageerror | API 失败 |
|---|---|---|---|---|---|---|
| Hotpic | /app/admin/hotpic | 0 | 3 | >0 | 0 | 0 |
| AppInfo | /app/admin/appinfo | 0 | 3 | >0 | 0 | 0 |

两页加载完成后 DOM 无「加载中」文案、统计数值正确、表格数据正常、0 错误。

## 证据索引
- \`oa4rust/test-results/r308-stats-label-fix/evidence.json\`（source：本地无头浏览器渲染，非物理真机）。
- \`oa4rust/test-results/r308-stats-label-fix/01-hotpic.png\` sha256:509e185b0d4d55f81b0553b3ab86d57361f0bcf73d49750bd5634b195d86b2a6
- \`oa4rust/test-results/r308-stats-label-fix/02-appinfo.png\` sha256:71f327d0b2f2460d0fc9b2212b96de952ddacc2eba4e5dedac799dcb1cba56f9

## 配置与脱敏
测试服地址（\`<YOUR_TEST_DOMAIN>\`）、主机（\`<YOUR_TEST_HOST>\`）、账号、口令仅运行期从凭据源读取；证据目录与部署脚本均为 gitignored，不入仓。本文 0 真值。

## 提交
精确 git add：HotpicApp.vue、AppInfoApp.vue、StatsCardLabel.R308.contract.test.ts、本文档；单提交推送，不改其他任务写域文件。项目持续迭代中（≥1000 轮，非完成态）。
