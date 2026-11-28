# R104 骨架屏加载态 a11y 语义 · 真浏览器 QA 报告

- 任务：t284（R104），attempt bade381a
- 轮次：round104-loading-a11y
- 视口：390×844 / 768×1024 / 1280×800（真 Chromium，agent-browser 0.41.1）
- HEAD 基线：b4e0194ca46de1bfe022315b289ee5fc9e6a0a9c（全程未变）
- 契约：laiyipao/miniapp/src/styles/r104LoadingA11y.contract.test.ts（13 it，t282 RED 10 fail / t283 GREEN 13/13）
- SPEC：docs/laiyipao/specs/round104-loading-a11y/SPEC.md
- 改前基线：docs/laiyipao/recon/ROUND104_A_LOADING_A11Y_RECON.md（t280）

## 1. 范围与方法

R104 改点为 10 个业务页的加载根补 a11y 语义：

- A 类 7 页（announcements / bag / battles / mastery / rank / shop / signin）：v-if="loading" 文字加载根（class=card）补 role="status" + aria-busy="true" + aria-label="数据加载中"。
- B 类 3 页（equipment / reactions / skills）：v-if="r6Loading" 骨架根（class=r6-skeleton r6-enter）去 aria-hidden="true" 并补同三属性。
- 三属性静态写死，v-if 卸载节点天然等价 aria-busy=false（节点从 DOM 消失即不再广播加载态）。

方法：后端 8080 未起，用 init-script（data/r104-init.js）合成响应【合成】+ Vue 实例注入（见 §9 诚实边界）复现加载态；每视口每页先 set-loading 注入 loading/r6Loading=true → DOM 探针 probe.js → 截图 → unset-loading → DOM 探针坐实卸载。AX 语义用 snapshot --json 完整快照坐实（snapshot -i 仅交互元素会过滤 status 节点）。

## 2. 三视口 × 10 页加载期 DOM 三属性（全 PASS）

加载期每页 aria-busy=true 根恰 1 个，hidden=null，label=数据加载中；iw=cw=sw 视口宽、hOverflow=false、isLogin=false。

| 视口 | A 类 7 页（ann/bag/battles/mastery/rank/shop/signin） | B 类 3 页（equip/reactions/skills） |
|---|---|---|
| 390 | 各 busyCount=1，UNI-VIEW，cls=card，role=status，busy=true，hidden=null，label=数据加载中，skelCount=0 | 各 busyCount=1，cls=r6-skeleton r6-enter，role=status，busy=true，hidden=null，label=数据加载中，skelCount=1 |
| 768 | 同上（逐页实测一致） | 同上（逐页实测一致） |
| 1280 | 同上（逐页实测一致） | 同上（逐页实测一致） |

数据来源：data/w-390.txt / w-768.txt / w-1280.txt 每页 --LOAD_PROBE-- 段。三视口 30 页无一例外，改前基线（role/busy/live 全 null，r6 三页 aria-hidden=true）全部修复。

## 3. 卸载后加载语义从 DOM 消失（全 PASS）

unset-loading 设 loading/r6Loading=false 后，三视口 30 页 --UNLOAD_PROBE-- 全部 busyCount=0 且 skelCount=0：加载根 v-if 卸载后 role=status/aria-busy 节点确实从 DOM 消失。部分页面保留既有空态 role=status aria-live=polite（如 announcements「暂无公告」、equip「AI 因果链」），属 R101 既有语义，非加载节点，未误改。

## 4. AX 语义快照（768，完整 snapshot --json）

- equipment 加载期：- status "数据加载中"（与原 - status 空态节点并存）；announcements 加载期：- status "数据加载中" 下挂 - StaticText "加载中…"。
- 二者均出现 AX status 语义节点，加载态可被辅助技术感知；改前基线 AX 仅 generic/button/textbox、无加载节点。
- 说明：snapshot -i（交互元素快照）会过滤 role=status 节点，walk 内嵌 AX 段因此看不到 status；此结论以完整 snapshot --json 为准（data/ax-full-out.txt）。

## 5. 零横溢 / 令牌 / 不变量静态回归（全 PASS）

- 三视口 30 页加载期 iw=cw=sw=视口宽、hOverflow=false、isLogin=false。
- 令牌三锚点在 laiyipao/miniapp/src/styles/tokens.css：14 --lyp-token-version: lyp-tokens-v1、15 --lyp-r3-version: lyp-tokens-v3-r3、16 --lyp-r6-version: lyp-tokens-r6。
- 业务源码零 --r104-（唯一匹配在契约 it12 自身断言中，非业务代码）。
- R60/R100/R101 键盘三件套与 aria-expanded 抽查：由契约 G2（r101/r60/r61 三契约 20 passed）静态守住，本轮未重做真走查（属既有能力，未改动）。

## 6. 截图 sha256sum（30 张，逐张实测 = 落盘实物）

均经 sha256sum 对落盘 PNG 实测；文件在 docs/laiyipao/audits/round104-loading-a11y/screenshots/，UNIQUE 30/30。

| 页 | 390 | 768 | 1280 |
|---|---|---|---|
| announcements | `f13065253a2adb4723ea2c40c08b05444378709655c091a106af37bd62d481c4` | `64585f73296179e47db598a244003167e99777c48d33ca43b02b956122d53db4` | `8739b23e95ca04b6a2008dbe703f35df08d82fee1236eea696f9ef700fdb1b46` |
| bag | `156778bb245d4e401f2cad8174f20a9893b0d2697daa6427fad75f93489e17a8` | `d6fd3af0909609f8131e858fc63263391a2676231231faf135a824d5b1333c4f` | `742f27b6a41bb6931f324d25c9eec2b484b11272d24c03ee5d70f33adb7451ca` |
| battles | `081a1ccde28db89402ea5c5da8f28ea2daf5f9feaa66a49f082ce3c2bfb8762b` | `f837f053cd6e5390e0611f02061a3fca94d83c2349f97ac58242bddd2c8750bc` | `9742c94cda9c22f9f9787ad4479b4f479f075abadf5f7fe0dc3d79760171c3c9` |
| mastery | `8a5b233de766b0b49fa09b4f2ad09b82691246087e469ca490531ded3daad561` | `bd7a1e915577909a6663d4d308558b6d775886959c7a24023998fec0d561a941` | `09a954b83ae3e56808f3304f3845c65be154db85bf92ce7c97d3e190993e5067` |
| rank | `e0a79dae62034c9f55acda86201858371438f80ecdc3e762b3c4c49fd216185d` | `5fca71dee3ea0609812112c271f6a4c354c7ef6ef44eb1f23c8e76aef686d9e3` | `b0969c312113932410bd7e4275ea1c1042a4850c76c99d8e8457621286a37483` |
| shop | `75a1cecbecc96d802fc27a4155d374362d8bdb72853b7e36abcf75f18d26bab8` | `fe2b3bba163aadcf59550e81453a7639248d7c69ec061713408ec10c59614251` | `42399fbaf7ebc74729905b6bef8be9a9ab9d6998c2c18df40716743145d864b9` |
| signin | `0e1ea9cbcede24d3ad67d7950eb22f4fa4b2e03362791897234097472165885a` | `2c046dcf651592c62a40e2b8bf3952f27e568c8aca585f5dc851db87cdec60ee` | `61246485f0c530f9d7b4c1cf4074592f81ec7cb74e612dd2f5da25c9ab7a361f` |
| equipment | `6d050297e7f6706ab716c8cd21b0954210216da09b56a1089eb555b0136c3b41` | `31a03c0a9d3354af2d0fbc57d5450c19242cd1fe634fa4b498e29f34a5c1710d` | `a7491c5ffaa60e8793127cb4ac67ebdc9e8240844b7e2ad1550071f0141eaf59` |
| reactions | `77b32bd9e8b00ac119bdf0af8b958e5984c5062880dd6f363adc22ccd3cae27d` | `5087cda65350d373222511a310f5e21498ddbb4ee64896f92f1f23b32f184d7e` | `befe79bfa1c4365548f0042787f1e00b8253800b598d910cf5e79ffbc86f2f65` |
| skills | `39789bac7245b590b3d6e5686aabe5611b9d6b4783ba9e130bac6a74cc5e2b89` | `bb7f478fdb5398762cf1f4adc9eb4d4b1919418d215cf04b47a25bec6f8cfa64` | `ce09dc26461ba684324d278a78553f46174843ce20485c440e11aa638fb64681` |

## 7. 亲读截图（≥2 张非登录页）

- r104-load-announcements-768.png（768×1024，116922 B，sha 64585f73…）：公告页「加载中…」文字卡 + 三行骨架占位条，标题「公告与版本 RELEASE NOTES · RULE ENGINE」与 AI 徽标；非登录页，视觉与 DOM/AX 加载态一致。
- r104-load-equipment-1280.png（1280×800，89826 B，sha a7491c5f…）：装备图鉴骨架屏，紫框骨架块 + 「待配置 / 0/0 已拥有/装备总数 / 收集进度」+「AI 因果链」空态；非登录页，B 类 r6-skeleton 加载态视觉坐实。

## 8. 门禁（全绿，逐项真实 RC=0）

data/gates-out.txt（job 退出 0）：

| 项 | 内容 | 结果 | RC |
|---|---|---|---|
| G1 | r104LoadingA11y.contract.test.ts | 1 file / 13 tests passed | 0 |
| G2 | r101 + r60 + r61 三契约 | 3 files / 20 tests passed | 0 |
| G3 | 10 页单测（ann/bag/battles×2/equip/mastery/rank/shop/signin/reactions/skills） | 11 files / 143 tests passed | 0 |
| G4 | miniapp 全量 vitest | 236 files passed / 1 skipped；2175 tests passed / 2 skipped | 0 |
| G5 | pnpm run type-check | 通过 | 0 |
| G6 | pnpm run build:h5 | 通过 | 0 |
| G7 | git diff --check | 通过（仅 .gitignore LF→CRLF 提示） | 0 |

GATES_DONE。测试中 [Vue warn] Failed to resolve directive: aria-native、build.attacker 缺失 等 stderr 为既有告警，非本轮引入。

## 9. 诚实边界

1. 后端 8080 未起：全部网络响应由 init-script 合成（/announcements、/config、/wallet、/me、/battle/* 等），非真实服务端数据，非真值。
2. 加载态经 Vue 实例注入：__r104Find/__r104Set（data/r104-init.js）遍历 __vueParentComponent 链找到各页实例，注入 loading/r6Loading=true 复现加载根；非真实网络延迟自然触发。尤其 bag 页 loading=ref(false) 且源码无 loading.value=true，加载根不会自然出现，必须注入才能坐实其加载态三属性——此点为注入复现，非自然流程。
3. AX 验证范围：完整 snapshot --json 在 equip/ann 两页坐实 - status "数据加载中"；未对 10 页逐页做完整 AX 快照（DOM 三属性已逐页坐实）。walk 内嵌 -i 快照因过滤 status 节点不作为 AX 证据。
4. 零真值：占位令牌 r104-qa-placeholder-token（exp 2099）仅 localStorage 内存注入；auth 五路径全 ABSENT；QA 库本轮未连。
5. 改前基线引用：t280 recon 报告坐实改前 role/busy/live 全 null、r6 三页 aria-hidden=true、AX 无加载节点；本轮改后对照该基线，未自建改前浏览器对照。
6. 截图亲读 2/30：其余 28 张以 sha256sum + DOM 探针坐实。

## 10. 结论与 hygiene

- 结论：PASS。三视口 10 页加载期 role=status / aria-busy=true / aria-label=数据加载中（B 类 aria-hidden 缺席）全部坐实，卸载后语义节点从 DOM 消失；AX status 语义、零横溢、令牌三锚点与既有契约均保持；门禁 G1–G7 全绿。
- hygiene：HEAD=b4e0194ca46de1bfe022315b289ee5fc9e6a0a9c 未变；git status 非 ?? 项仅 .gitignore + 10 个预期业务 vue（t283 实施）；auth 五路径 ABSENT；零真值；后端 8080 未起；自启 H5 dev server（5174）于收尾 kill 并复验 P5174=000。
- 唯一入库写域：docs/laiyipao/audits/round104-loading-a11y/REPORT.md；data/ 与 screenshots/ 为 UNTRACKED 证据。
