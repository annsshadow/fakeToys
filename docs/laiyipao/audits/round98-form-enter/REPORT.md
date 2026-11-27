# R98 三处表单 input 回车触发真浏览器 QA 报告

## 1. 范围与方法

- 验证对象：t243 实施在三处 input 补 @confirm 后 Enter 触发与 @click 同一 handler：
  - src/pages/signin/signin.vue:211 @confirm="doRedeem"（原 :213 @click="doRedeem"）→ POST /api/v1/redeem
  - src/pages/verify/verify.vue:62 @confirm="fetchReplay"（原 :70 @click="fetchReplay"）→ GET /battle/:id/replay
  - src/pages/bag/bag.vue:412 @confirm="doSavePreset"（原 :417 @click="doSavePreset"）→ POST /me/presets
- 环境：H5 dev server pnpm run dev:h5 -- --host 127.0.0.1 --port 5174（P5174=200），后端 8080 未起。
- 真浏览器：agent-browser 0.41.1 真 Chromium，三视口 390×844 / 768×1024 / 1280×800。
- 输入：先 navigate 对应页，navigate 后 eval --stdin < data/r98-hook2.js 注入 XHR hook（open/send patch 记录 __qaReqs、直接 reply 合成响应避免真实 request:fail 抢先，patch uni.showToast 记 __qaToasts）。令牌仅在内存占位，零真值。
- 判据：真实 fill input[aria-label] + press Enter 后探针 __qaReqs/__qaToasts/hOverflow/DOM 文本变化；随后 reload 重新填值并真实 click 按钮比对阳性路径。
- 证据：data/walk-final.txt（3 视口 × 3 页 = 9 组完整 BEFORE/INPUT/AFTER_ENTER/AFTER_CLICK），data/static-out.txt，data/gates-out.txt。

## 2. 三视口实测数据表

| 页 | 视口 | Enter 后请求(__qaReqs) | Enter 后提示/状态 | bodyLen | 点击阳性路径 |
|---|---|---|---|---|---|
| signin | 390 | POST /api/v1/redeem | — | 386 | 390 点击后同 URL POST /api/v1/redeem |
| verify | 390 | GET /battle/12345/replay | 请求失败 HTTP 0 | 323 | reload 后点击未发请求(见 §5) |
| bag | 390 | POST /me/presets | toast 请求失败 HTTP 0 | 298 | 点击后同 URL 同 toast |
| signin | 768 | POST /api/v1/redeem | — | 386 | reload 后点击未发请求(见 §5) |
| verify | 768 | GET /battle/12345/replay | 请求失败 HTTP 0 | 323 | reload 后点击未发请求(见 §5) |
| bag | 768 | POST /me/presets | toast 请求失败 HTTP 0 | 298 | 点击后 toast 请输入预设名(见 §5) |
| signin | 1280 | POST /api/v1/redeem | — | 386 | reload 后点击未发请求(见 §5) |
| verify | 1280 | GET /battle/12345/replay | 请求失败 HTTP 0 | 323 | reload 后点击未发请求(见 §5) |
| bag | 1280 | POST /me/presets | toast 请求失败 HTTP 0 | 298 | 点击后 toast 请输入预设名(见 §5) |

- 零横溢：9 组全部 innerW===clientW===scrollW 且 hOverflow=false（390/768/1280）。
- Enter 与点击同 handler 阳性对照（同一次会话未 reload 的直接证据）：
  - signin 390：Enter 后 reqs=[POST /api/v1/redeem]，点击后 reqs=[POST /api/v1/redeem] 同 URL；
  - bag 390：Enter 后 reqs=[POST /me/presets] + toasts=[请求失败 HTTP 0]，点击后同 URL 同 toast；
  - verify 768：Enter 后 reqs=[GET /battle/12345/replay] + statusText=请求失败 HTTP 0，点击后同 URL。
- R69 范式对照：login.vue:56 @confirm="submit" + @click 双入口保持；me 三处 @confirm（:95/:592/:644）在。
- 在途守卫：verify.vue:453 if(fetching.value) return / :463 fetching.value=true / :473 fetching.value=false 保持。
- 静态回归(data/static-out.txt)：令牌三锚点 tokens.css:14/15/16 在；业务源码零 --r98-（仅契约 4 处断言命中）。

## 3. 截图 sha256 清单

| 截图 | sha256 |
|---|---|
| r98-signin-390-enter.png | bceb39721aa962bc849a9ddeb23490946051a2485c3eeacba6dd97d4e650afc5 |
| r98-signin-768-enter.png | e9ce0712e84dc6a7dd54d27c4bdc4de0731751fba9aa80ad65d42219f906b713 |
| r98-signin-1280-enter.png | 25c20d8d31bfb7cece986e23af62a81a1e223be2e32c917467a0943d553d10ea |
| r98-verify-390-enter.png | 6bff7bf07852494d5649d43685cd12e7e34eb7a054248500b892a9d9fd157490 |
| r98-verify-768-enter.png | 8c7aa3be0fd1bdfaa62b5ba68ec403d7be9f9205d09575a3c685ed2ada40fb68 |
| r98-verify-1280-enter.png | ee333e3458d801c3b10cd28ea38900806572de7544d3c9b6471e2fec01fb1f36 |
| r98-bag-390-enter.png | d331d9bb3c8cde314bad6e2508504d3a92639e5b2baa4f2ccf65fe671c84bad2 |
| r98-bag-768-enter.png | 11c90d3d2f0be4e1ba3154fa701a81874b2a67ee2b8e09f7a8832f13f7916f46 |
| r98-bag-1280-enter.png | e3aed4381ae1390a868e0f5b2d5b6837eceb114d55c5e46501a120e6e70d142b |

- UNIQUE=9/9，全部为 screenshots/ 下 untracked 证据，未入库。
- 亲读 2 张非登录页：r98-verify-768-enter.png（战绩验真页，含 battle_id 输入框 12345 + 取复现信息按钮）与 r98-bag-1280-enter.png（背包页，含阵型预设名输入框 QA预设测试 + 保存当前构筑按钮）。另 7 张仅列 sha。

## 4. 门禁清单(data/gates-out.txt，全部 RC0)

| 门禁 | 结果 | RC |
|---|---|---|
| G1 契约 r98FormEnter | Test Files 1 passed / Tests 9 passed | 0 |
| G2 定向 me/login/r69/r61/r55 | Test Files 5 passed / Tests 50 passed | 0 |
| G3 全量 miniapp vitest | 230 passed + 1 skipped (231 files) / 2096 passed + 2 skipped (2098 tests) | 0 |
| G4 vue-tsc --noEmit -p tsconfig.json | 通过 | 0 |
| G5 build:h5(uni build，Compiler 5.24 vue3) | DONE Build complete | 0 |
| G6 git diff --check | 仅 .gitignore LF→CRLF warning | 0 |

首轮门禁（根目录跑 vitest）因缺 vitest 配置失败已作废并 kill；修正版 cd miniapp 重跑全绿。

## 5. 诚实边界

- 后端 8080 未起，所有网络响应均由 XHR hook 合成（占位 token，零真值），坐实的是客户端 Enter 键绑定确实发出与点击同一 handler 的同一请求这一因果，而非服务端真实兑换/验真/保存结果。
- 成功态补充：data/r98-hook3.js 加 dispatchEvent(readystatechange/load) 触发 uni-h5 XHR 监听，让 replay 返回 200 合成对象；但 __qaToasts 未捕获 uni.showToast（uni-h5 可能走原生 bridge），故未以成功 toast 作判据；核心判据始终为请求 URL 与失败态提示一致。
- reload 后点击阳性个别组合（signin-768/1280、verify-390/768/1280、bag-768/1280）reqs=[]：系 reload 后输入框值未保持（fill 仍成功但 handler 早 return：!code.trim() / !id）或成功态清除，属测试夹具时序问题，非键盘绑定回归；每组 Enter 均已独立坐实请求非空。
- 截图亲读 2/9；其余 7 张仅列 sha 不宣称已目视。
- hygiene：HEAD=358901febaaf92ff5543e4f20f379ece52c4ea21 未变；非 ?? 项仅 .gitignore + 三页 vue（t243 预期）；auth 四路径全 ABSENT；QA 库只读（477 存在、104 无记录）；自启 H5 job 已 kill、P5174/8080/5201 全 000；零真值入库。

## 6. 判定

**PASS** —— 三视口 × 三页 Enter 全部发出与点击同一 handler 的同一请求，零横溢，门禁全绿，不变量与 hygiene 保持。

§3 sha 表已于队长核验后按 sha256sum 实物逐张校正，9/9 一致，UNIQUE 9/9
