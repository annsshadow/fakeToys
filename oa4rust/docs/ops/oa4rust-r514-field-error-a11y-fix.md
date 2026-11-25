# R514 BBSForum 版块保存失败错误态程序化关联修复

## 1. 范围与基线

- 真父提交：12f8230ccd2343d880560991583e87aa2954f255（R513 甄别已 push，台账 §508）。
- 写域：本文件 + BBSForum.vue + R513.fieldErrorA11y.contract.test.ts，共 3 文件；formErrorA11y.ts / main.ts / Personal.vue / R489 契约零改。
- 缺陷：BBSForum.vue:200 版块保存失败 sectionError 错误 div 缺 data-oa-field-error 标记；R513 甄别故障注入实证 role 空 / axRole=generic / input aria-invalid=null / describedby=''，违反 WCAG 3.3.1 / 4.1.2，判 P2。

## 2. 修复

- 仅 BBSForum.vue:200 开标签加 data-oa-field-error：<div v-if="sectionError" class="error-msg" data-oa-field-error>{{ sectionError }}</div>。
- v-if / class / 插值文本 / catch 赋值（saveSection 第 400 行）/ @submit.prevent / label for / input 属性 / modal 结构 / 样式全部零改；复用 main.ts:614 已装全局增强器 formErrorA11y.ts 自动补 role=alert + 稳定 id + input aria-invalid=true / aria-describedby。
- git diff 核对：BBSForum.vue 完整 diff 仅上述 1 行改写。

## 3. RED → GREEN 与三 verify

- 契约 R513.fieldErrorA11y.contract.test.ts 5 it：真父定点 RED 1 failed（核心标记缺失）+ 4 passed（守卫无桩错），EXIT=1。
- GREEN 定点 5/5；三 verify 全 exit 0：① 定点 vitest 5/5；② desktop 全量 pnpm test 332 files / 3208 tests 0 failed；③ pnpm typecheck 6 项目通过。
- PC 构建 pnpm run build exit 0，入口 index-zXKy7wwn.js（/oa4rust/ 路径正常）；部署测试服经同一归档 sha256 比对，SHA_OK=7e821a5454999252e522fd712c97e362d390b28711921ea2a6bbc06c3a426f74，DEPLOY_R514_PC_FULL_OK。

## 4. 真浏览器实测（1440x900 DPR1 headless 非物理真机）

- 双主题 light/dark：page.route 拦截 section/create 返回 HTTP 500 触发 sectionError，CDP DOM + Accessibility.getPartialAXTree 坐实：错误文本 HTTP 500: Internal Server Error / marker=true / role=alert / id=oa-field-error-000001 / axRole=alert / axIgnored=false；#bbs-section-name aria-invalid=true、aria-describedby=oa-field-error-000001、label[for] 存在；modal axRole=dialog / axName=新建版块。
- 错误解除：取消故障注入、关闭后重开浮层，input aria-invalid=null / describedby='' / 错误 div 消失，属性还原。
- Personal.vue:50（R489）正面对照：真实触发两次密码不一致后 marker=role=alert/id=oa-field-error-000001，oldPwd/newPwd/confirmPwd 三框 aria-invalid=true + describedby 指向该 id，零回归。
- 两组 pageerror 均为 0；截图目视确认为真实 OA4RUST 论坛新建版块浮层错误态（浅色红字、深色红底提示框），非物理真机。
- 据实披露：故障注入为浏览器内请求拦截，未修改服务端数据；CDP error 节点 axName 为空（错误文本由 role=alert 的 live region 播报，非可访问名）。

## 5. 证据 sha256（命令行核验）

- report.json=1fe9d0b72245eb207a26ac57149512200dcb090feba86a68836fc0a2ff820c31
- pc-bbs-section-error-light-r514-sup.png=3e71d04dd27b10894bffb87b5f0fff1203d7de79e41a261cbb9d1dbd5b92bda4
- pc-bbs-section-error-dark-r514-sup.png=c9bf1d31972dd25e71f046076f0048f4b0abcf4f526d263b578dd2bfc1f0f348
- r514-red.log=3333804b2c70aa238d359b213ab0768e44a2db9527834349600203a9dc4a84bf
- r514-v1.log=55cb7ac80c8a6bdf6515d7957d2916f30223f1bdc20fe8288c799c612b08d892
- r514-v2-full.log=4e34669d81d39c6df0f33408dae0e3e50a356158d9c4ff4d2c583f961bdfe5f4
- r514-v3-typecheck.log=15a1cb81056b74929321923db42727ad98ce716deb7114421c20ddec7619c898
- r514-build.log=b5bae8c04db8bc2419a5e978ad2185d7d92d3b2d703bb3ab7d4c69c259d07cdd

## 6. 脱敏边界

- 证据与报告分离：真域名、账号、口令、SSH 信息仅存在于 gitignored 本地 runner 与证据 JSON，文档仅用 <YOUR_TEST_DOMAIN> / <TEST_HOST> 占位，无真值入库。
- 截图均为 headless 非物理真机，据实标注；E2E 脚本/报告/sha 日志入 gitignored oa4rust/test-results/r514-*，不随提交入库。
