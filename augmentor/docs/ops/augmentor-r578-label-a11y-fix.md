# R578 augmentor/web 数据增强页表单控件可访问名修复

## 问题与范围

R577 验证型甄别确认 augmentor/web（React 19 + antd 6 + Vite，跨子项目第一例修复）数据增强页 Augmentation.tsx 存在真缺陷 1 项（P2）：6 个表单控件使用裸 label（label 与控件无 for/id 关联、无 aria 关联），CDP AX 树中 name=null，辅助技术无法读出控件名（WCAG SC 3.3.2 / 1.3.1 / 4.1.2 A）。

任务 ID 的 antd Input（约 340-346 行）经 placeholder 自动补名（AX name=增强任务的任务 ID），据实判为正常，严禁重复补 aria-label。

## 方案取舍（Fail Loud）

逐控件显式补 aria-label（方案 B）。全局增强器方案因越界他页、动态名无法可靠派生，Fail Loud 排除。与 R561–R576 先例一致。

## 写域（精确三文件）

- M augmentor/web/src/pages/Augmentation.tsx（6 insertions / 4 deletions，纯 aria-label 属性追加；4 个 Switch 为单行整行替换）
- A augmentor/web/src/pages/R578.augmentationLabelA11y.contract.test.ts（纯字符串源码断言，遵循现有 vitest 惯例；8 用例）
- A augmentor/docs/ops/augmentor-r578-label-a11y-fix.md（本文档）

零改：handler（loadFiles / checkProgress / handleStart / loadReport）/ state / 样式 / 他页 / main.tsx / App.tsx / DataManagement.tsx 正面对照 / 既有 Augmentation.test.tsx / 6 条原交互绑定 / 任务 ID Input（placeholder 已补名，严禁 aria-label）/ 全局增强器 Fail Loud 排除。

真父 HEAD：c6c10726edb7ff86242f91060516e04612e5e5f6（测试全程未变）。

## 6 处修复

| 控件 | 行号 | 追加属性 |
| --- | --- | --- |
| antd Select 输入文件 | 约 211 | aria-label="输入文件" |
| 原生 input 输出文件 | 约 227 | aria-label="输出文件" |
| Switch 质量检查 | 约 242 | aria-label="质量检查" |
| Switch 智能去重 | 约 246 | aria-label="智能去重" |
| Switch 断点续传 | 约 250 | aria-label="断点续传" |
| Switch 异步引擎 | 约 254 | aria-label="异步引擎" |

@click / onChange 等原交互绑定全部保留，业务状态逻辑零改。

## 契约 RED → GREEN

契约 R578.augmentationLabelA11y.contract.test.ts 8 用例：4 核心（Select 开标签 aria-label + placeholder / 输出 input 开标签 aria-label / 四 Switch 完整行断言 / 全页 aria-label 恰 6 处各文案一次）+ 4 守卫（任务 ID Input 零改无 aria-label、placeholder 自动补名 / handler 零改 / 交互绑定 + import + api 零改无全局增强器 / DataManagement 正面对照 aria-label + 控件结构数量）。

踩坑：初版守卫 8 用 count(src, 小于 Select) = 1 失败（实际 2，Select.Option 也计入）→ 改为 count(src, 小于 Select 换行) = 1 排除，重跑 GREEN 8 pass。

- GREEN（修复版）EXIT=0，8 passed（日志见 B 组）
- RED（反向 ops_edit_file 移除 6 处 aria-label 回真父 revision=87677f7e…）EXIT=1，4 failed（核心 1-4）| 4 passed（守卫 5-8），随后 ops_edit_file 恢复修复版 revision=32ecf78b… 复核 GREEN EXIT=0 8 passed
- 据实披露：本轮未生成独立的修复前备份文件（RED 由反向编辑实现），真父 sha/ revision 由 git 与 ops_stat 记录坐实

## 三 verify（workdir augmentor/web，全 exit 0）

- V1 npx vitest run src/pages/R578.augmentationLabelA11y.contract.test.ts → EXIT=0，8 passed
- V2 npm run test → 首轮 EXIT=1（1 failed：src/pages/DataManagement.transform.test.tsx 分割卡 waitFor splitDataset 超时，与 R578 零关联 flaky）；单独重跑该文件 EXIT=0 13 passed；全量重跑 EXIT=0，27 files / 457 tests 全 passed
- V3 npm run typecheck（tsc --noEmit）→ EXIT=0，无错

据实披露 V2 首轮 flaky 与重跑通过。

## 真浏览器 CDP 决定性判据

自起 augmentor/web Vite dev 服务（端口见占位，初始探测各常用端口全空闲，服务 HTTP=200），Playwright chromium headless，viewport 1440x900 DPR1。CDP：DOM.getAttributes + DOM.describeNode 取 backendNodeId → Accessibility.getPartialAXTree / getFullAXTree 按 backendDOMNodeId 映射读 role/name；focused 过滤 focused=true 非 RootWebArea；严禁宿主 wrapper 假阳性（R535 教训），UNI-TEXT / UNI-VIEW 身份独立确认（本项目为原生 INPUT / BUTTON）。

6 组 case（withData / empty × light / dark + func + mouse）：0 fatal、0 pageerror。

- 6 控件 + 任务 ID 零回归：focused tag=INPUT（Select / 输出 / 任务 ID）或 BUTTON（4 Switch），isTarget=true、isBody=false；AX full role/name：Select=combobox/输入文件，输出=textbox/输出文件，四 Switch=switch/质量检查|智能去重|断点续传|异步引擎，任务 ID=textbox/增强任务的任务 ID（aria=null，placeholder 自动补名，零改）
- Tab 命中：select 4 / output 4 / 各 Switch 12 / taskId 3
- 注入对照：strip 全部 aria-label 后 select.full.name=null；restore 后=输入文件；注入 R578-inj-* 后 full role/name 达标
- 正面对照：自包含注入原生 button aria-label=R578正面对照控件 full role=button/name 达标；DataManagement.tsx select[aria-label="上传源格式"] full=combobox/上传源格式（跨页正面对照）
- 功能真命中：Select 展开 options=2（R578-seed-a.json / R578-seed-b.json）；质量检查 Space 由 true→false；智能去重 Enter 由 true→false；输出 input 键盘输入 R578-keyboard-output；鼠标点异步引擎由 false→true、鼠标填输出 R578-mouse-output
- 造数分层：Select Option 需 GET /api/data/list 造数据（withData 2 文件 / empty 0 文件）；Switch / 输出 / 任务 ID 零业务造数（纯前端状态）；进度 / 断点 stub 空态

## 截图目视

4 图 1440x900，名实相符：数据增强配置页（输入文件 Select / 输出文件 input / 四 Switch / 开始增强 / 增强进度 / 断点列表 / 增强报告 / 任务 ID）。H5/PC 无主题切换机制，light/dark 视觉一致、sha 微差据实披露。

- r578-withData-light.png / r578-withData-dark.png
- r578-empty-light.png / r578-empty-dark.png

## sha256 证据表

A 组 runner 紧凑 JSON（crypto.sha256(JSON.stringify(report))，与 R578_DONE 汇总行一致；非文件 sha256sum）：

- R578_DONE 汇总 sha：d0cbf8a8fdf2a4f984ebab620d41161c2d2dd9d76eb0794b7d45d557eb931250（cases=6 / fatals=0 / pe=0）

B 组文件 sha256sum（逐项核对）：

- augmentor/web/src/pages/Augmentation.tsx（修复版）：32ecf78b591b4e2c81c81b0a69899ecd80e34361eeed348ad160c71a1ccf295b
- 真父 revision（非文件 sha）：87677f7e68c25d6039d57ccf1532d6d89a03ad6b412b7b384aca5b9a0775869b
- augmentor/web/src/pages/R578.augmentationLabelA11y.contract.test.ts：7a51bc39cbe6e9929d4f6fab506c9995dac618d4e64869286373b5289bf30554
- oa4rust/test-results/r578/r578-probe.cjs：f4c28b361df5a05d07920cb8835009974775152dec9bb5c70dc5b2149e597f5f
- oa4rust/test-results/r578/r578-e2e.cjs：98961e301a2ac0530f35108d33af9bce1e756a5d1f2f264aa7bd15b3b5ed1ffa
- oa4rust/test-results/r578/r578-report.json：a4fa0537ee265e2c55da28576b2d365d1c425eef45bdf089094d8778cc4aee1c
- oa4rust/test-results/r578/r578-summary.cjs：09488aab0d875a2d3a72c742c6295c39cab8cb2f35ad04dc1b6b2481b5194857
- oa4rust/test-results/r578/r578-detail.cjs：8c3641faf38eda4d718ddf4c9402f35cc5564257a7b129a16f9945228a688e85
- oa4rust/test-results/r578/r578-withData-light.png：361f0ddbdf7d38013d11509e8fa8b0d26025e1af186042e18af518c99ff073cc
- oa4rust/test-results/r578/r578-withData-dark.png：27172bdd88970fad091a2c0666c1c142486ff07537884d7a449a0b0bf4bbbd4b
- oa4rust/test-results/r578/r578-empty-light.png：2d6833ba22d3febd6c90c3a7be584b272ae401f59c49d536ac03ca599afda34c
- oa4rust/test-results/r578/r578-empty-dark.png：9706002e31b08b2e34f974a3862161dbf2721a38bcd50ff359d90b4e6cfc0a0f
- /tmp/r578-red.log：e86b702101f1564d812ef924904b89c628343f0c0fab3fd474bcb11a31e2209c
- /tmp/r578-green.log：a5aaa79b2709c82e3fda55c0815c148901db98ac5843616da7eb70e53a6aa1d1
- /tmp/r578-green2.log：fbd38e5eca16c004f2a4a7eea39e50e01ae0b4d34415daa93d14a604a39c12d0
- /tmp/r578-v1.log：ed3f015e4d396df3cbe1c845b79f9c00f35063fd1285cda958c8b7f30c69040e
- /tmp/r578-v2b.log：034a9e127e0fb2d75424a4a342a8e5bd32d98e1f8871b11257ceb3046616ffed
- /tmp/r578-v3.log：308f72d15ad935773f505e41a22444c64b8cca155628fa2d60610233bf142ec1
- /tmp/r578-e2e.log：bf48b246e64cb8d930a2cc57a646219c5238a30e92b0857bc19f7b593dc6322a
- /tmp/r578-summary.log：34ef7f6aa38e7c8f375abe171785c85edf7a1d92e0e9e00fcbecdc513e13e25b

## 脱敏声明

本文件 0 真值：无密钥 / token / 口令 / 私钥 / 真实域名 / IP / 主机名 / SSH 端口 / 个人与环境信息。测试服务地址一律用 <TEST_HOST>:<PORT> 占位；证据文件位于 gitignored 目录（oa4rust/test-results/r578/ 与 /tmp/），未 track。本文件不含自身 sha（自引用悖论：写入自身 sha 会改变内容致 sha 失效）。
