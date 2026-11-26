# R71 验真页记录时间本地化 — 部署后真浏览器回归报告

- 任务：t149 / attempt 418bd146-351b-433b-9d38-ce4aab033ad2
- 工作树：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
- 起点 HEAD：a0e1ead084f7a0e1165af403919abfa936776406（R69 收官）
- 执行日期：2026-11-27（Asia/Shanghai，UTC+8）
- 结论：**PASS**（带诚实边界）。R71 修复在重建后的 H5 产物上、三视口真 Chromium 点击复验全部符合预期，回归锚点与门禁无回退。

## 1 五批发布链表
| 批次 | commit | 主题 |
|---|---|---|
| B1 feat | 2be0d839f | feat(laiyipao): R71 验真页记录时间本地化（y-mm-dd hh:mi） |
| B2 test | 5915d1902 | test(laiyipao): R71 记录时间本地化契约 5it |
| B3 docs | b4de5b734 | docs(laiyipao): R71 规范与独立终审 |
| B4 audit | f4b92655a | test(laiyipao): R71 真浏览器QA报告与截图 |
| B5 postdeploy | 本批 | chore(laiyipao): R71 部署后回归报告 |

每批均为逐文件路径精确 git add（禁 -A/点号），泄露守卫（带口令串连接/私钥/Bearer/Cookie）零命中，data/ 入库数 0，git diff --cached --check RC=0（仅 .gitignore LF→CRLF warning）。

## 2 产物 sha256（重建后实测，未回退）
| 产物 | sha256 | 说明 |
|---|---|---|
| H5 主 bundle index-BJrAi0qp.js | 3de2344dfbe062eec21fb6a40b0637a92221e083a69542aca6bc11c2194f4e57 | 本轮 pnpm run build:h5 重建（RC=0），index.html 引用；R69 为 index-2A8Oxcaj.js，已更替 |
| 后端 lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21 基线，未回退 |
| admin WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | 沿用基线，未回退（任务描述猜测的 JpQ4MFXA.js 本树不存在，以实际为准） |

## 3 部署后真浏览器复核（12 组：三视口 × 四状态）
服务：API 8080（go run ./cmd/api，/api/v1/config=200）、H5 静态 5201（python http.server，/=200）。鉴权经 gitignored data/auth-init.js 注入游客态（不回显、不入库）。验真链路为真点击：navigate 到 verify 页 → r71-click.js 填 battle_id=424242 并点「取复现信息」→ pd-probe.js 读 DOM。mock 数据由 init-script 构造（非真值）。视口 414×896 / 820×1180 / 1440×900 与 QA 的 390/768/1280 互异。

| 状态 | 三视口共同结果 |
|---|---|
| normal（合法 ISO，created_at=2026-11-27T01:23:45Z） | recordTime="记录时间2026-11-27 09:23"、recordRaw="2026-11-27 09:23"、hasT=false/hasZ=false、hashLine="服务端记录的哈希ABCD EF01 2345 6789"、snapshotCount=5、hitCount=1、hOverflow=0（innerW=docW=clientW）、errText=null |
| bad（非法 ISO，not-a-date） | recordTime="记录时间not-a-date"、recordRaw="not-a-date" 诚实回退、hasT=false/hasZ=false、snapshotCount=5、hitCount=1、hOverflow=0、errText=null |
| double（连点两次） | hitCount=1（第二次被 fetching 守卫拦截）、recordRaw="2026-11-27 09:23"、hOverflow=0、errText=null |
| fail（mock HTTP 500） | errText="请求失败 HTTP 500"、kvList=[]、hashLine=null、snapshotCount=0、r5Conclusion="复现信息尚未到位：先填 battle_id 取信息，再本地重放比对哈希。"（结论已清）、hitCount=1、hOverflow=0 |

12/12 组探针均符合预期，零 JS 错误、零横向溢出。

## 4 部署后新截图 sha256（12 张，实测）
| 文件 | sha256 |
|---|---|
| pd-normal-414.png | 89d9943411ea0780786b00b5af6df7cfce476fed36769ec67bf1e132853c3742 |
| pd-normal-820.png | ee0ddbabf345311b37f6d6a731f35408e7b3e3c334c972bc45363030a5778d36 |
| pd-normal-1440.png | e8c39dd37075443ba328281ee4c4c26cba2fd35a082eb5e6a3bdb2f65bfeae2a |
| pd-bad-414.png | 89d9943411ea0780786b00b5af6df7cfce476fed36769ec67bf1e132853c3742 |
| pd-bad-820.png | ee0ddbabf345311b37f6d6a731f35408e7b3e3c334c972bc45363030a5778d36 |
| pd-bad-1440.png | d5d43d76048e1b2f29c21707ab708f53f8864170a8f9da924e7d148084ecbb7b |
| pd-double-414.png | 89d9943411ea0780786b00b5af6df7cfce476fed36769ec67bf1e132853c3742 |
| pd-double-820.png | ee0ddbabf345311b37f6d6a731f35408e7b3e3c334c972bc45363030a5778d36 |
| pd-double-1440.png | e8c39dd37075443ba328281ee4c4c26cba2fd35a082eb5e6a3bdb2f65bfeae2a |
| pd-fail-414.png | 737bcda345fdba3145b9c13ce721fd918eacf88243b36fa7c1ea914d333611b2 |
| pd-fail-820.png | 13c68b2cec4761f6d697ec3298758c6e4116db4b0aedc696b624c0a3f4b5b83d |
| pd-fail-1440.png | f5d5d1b0e205400b7cb942a1b15d160dca13565b5b9e3bd5b17542966411bdb8 |

UNIQUE=9/12：normal/bad/double 在 414/820 共享 sha（记录时间行在首屏以下，可视区一致；bad-1440 因可见 not-a-date 而互异），1440 视口可见时间行故 normal-1440≠bad-1440；此现象与 QA 阶段 UNIQUE=6/12 同类，属视口可见性导致的确定性等价，DOM 探针才是差异证据。全部新图 sha 均 ≠ QA 12 图基线（视口互异）。亲读 pd-normal-1440.png / pd-bad-1440.png / pd-fail-820.png 三张，确认为战绩验真页（非登录页），分别可见本地化时间、not-a-date 回退、结论已清提示。

## 5 回归锚点（不回退）
- 令牌三锚点：tokens.css:14/15/16 逐字在（lyp-tokens-v1 / lyp-tokens-v3-r3 / lyp-tokens-r6）。
- --r71-：全 src 仅契约 r71VerifyTimeFormat.contract.test.ts:79 一处否定断言，业务源码 0 命中。
- R60 键盘：src/pages 下 tabindex="0" 48 处；R61 aria：v-aria-native 17 处；R55 placeholder-class="ph" 21 处。
- verify.vue：:101 模板绑定 fmtVerifyTime(replayInfo.created_at)，:433 函数定义；fetchReplay 入口守卫 / finally / catch 清结论保持。

## 6 门禁与收尾
- 契约 R71：5/5 passed（QA 阶段复跑）；全量 vitest 222 files passed | 1 skipped、2044 tests passed | 2 skipped；vue-tsc RC0；pnpm run build:h5 RC0（本轮重建）；git diff --check RC0（仅 .gitignore LF→CRLF warning）。
- Go：TestNoInfraIdentifiersInRepo PASS（21.33s，GO_INFRA_RC=0）；TestNoRealSecretsInRepo **SKIP 非 PASS**（缺 .local.keys，逐字「真值扫描这条守卫没有执行，不是『通过了』」，GO_SEC_RC=0）。
- QA 库：docker 容器 lyp-qa-pg，battle_records count|max(id)=104|477（只读，未改）。
- 零真值：两处 data/auth-init.js 在 finalize 删除；git ls-files 无本轮 auth/data；五批泄露扫描零命中。

## 7 诚实边界
1. 验真 replay 响应由 init-script mock 构造（非真值），仅为语义验证；后端 4 处 createdAt.UTC().Format 未改（R71 范围外）。
2. 12 图 UNIQUE=9/12，normal/bad/double 在 414/820 同视口共享 sha（时间行首屏以下，可视区一致），以 DOM 探针为差异证据。
3. 本地化按浏览器 UTC+8 解释，2026-11-27T01:23:45Z → 09:23；非法 ISO 回退原串。
4. Go TestNoRealSecretsInRepo 因缺 .local.keys SKIP 非 PASS，绝不记为通过。
5. guest_token 注入仅客户端会话，不改 QA 库数据。
6. 部署后报告自引用哈希悖论：本文件 sha 不纳入自身校验。

## 8 结论
R71 验真页记录时间本地化修复在重建后的 H5 产物、部署后三视口真 Chromium 点击复验中全部符合预期，四状态语义、回归锚点、门禁与零真值收尾均无回退，判定 PASS（带上述诚实边界）。
