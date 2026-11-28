# R107 部署后复查报告：表单校验错误态 a11y 语义（五批发布 B5）

- 轮次：R107（t308 五批发布；前置 t302侦察→t303规范RED→t304实施GREEN→t305 QA PASS→t306终审 verdict=pass 六维全绿 8 条 low 不阻断）。
- 起点 HEAD：`528e6477d3f22d160bcbca21a65e3f1ecf773383`；五批精确提交链见 git log（B1 feat / B2 test / B3 docs / B4 audit / B5 本报告）。
- 复查对象：生产 H5 静态产物；三视口 414×896 / 820×1180 / 1440×900 错误态语义真浏览器验收。

## 1 H5 产物与后端

- `build:h5`（job `bash-9571b1da78234c56` exit 0）重建产物路径 `laiyipao/miniapp/dist/build/h5`；`index.html` 引用 `assets/index-DzYQdOVZ.js`。
- 主 bundle `index-DzYQdOVZ.js` sha256：`9ae50c57e5c2d208cf1015e3d7119c47c60335930fb05ee7548a255706efbed5`（逐字实测）。
- 后端/Admin 无本轮改动，沿用不回退；后端 8080 未起，全部响应 init-script 合成。

## 2 三视口错误态原生 input 属性坐实

- 静态服务：job `bash-04ea1b75766a4f79`，`python -m http.server 5201 --bind 127.0.0.1` on `laiyipao/miniapp/dist/build/h5`（HTTP 200）。
- init-script `r107-init.js`（占位令牌 `r107-postdeploy-placeholder-token`，exp2099，零真值）经 XHR+fetch 双 patch 合成 config/公告/战报/钱包；signin 的 `/redeem` 合成 HTTP 500 + error.message 触发错误态。
- driver：`docs/laiyipao/audits/round107-form-error-a11y-postdeploy/data/pd-driver.sh`（session `lyp-r107pd`，`@@` 标记分段；`pd-walk.txt` 490 行末 `@@PD_ALL_DONE`）。探针 `probe.js` 直接读**原生 input.uni-input-input** 的 aria-invalid / aria-describedby / aria-label（uni-app H5 白名单会丢弃外层 uni-input 的这两属性，靠 main.ts applyAriaNative 桥接）。
- 解析 `.r107-parse.py` 提取 9 组 × BEFORE/AFTER 共 18 行到 `pd-summary.txt`。

| 视口 | 页 | 原生 input 数 | BEFORE aria-invalid → AFTER | aria-describedby（恒指） | 错误节点 AFTER(role/id/text) | cw=sw / 横溢 / 登录页 |
|---|---|---|---|---|---|---|
| 414 | verify | 1 | false → **true** | verify-battle-err | alert / verify-battle-err / 请输入有效的 battle_id | 414=414 / 否 / 否 |
| 414 | login | 2 | false → **true**（用户名+口令） | login-err | alert / login-err / 请填写用户名与口令 | 414=414 / 否 / 否 |
| 414 | signin | 1 | false → **true** | signin-redeem-err | status / signin-redeem-err / 兑换码无效 | 414=414 / 否 / 否 |
| 820 | verify | 1 | false → **true** | verify-battle-err | alert / verify-battle-err / 请输入有效的 battle_id | 820=820 / 否 / 否 |
| 820 | login | 2 | false → **true** | login-err | alert / login-err / 请填写用户名与口令 | 820=820 / 否 / 否 |
| 820 | signin | 1 | false → **true** | signin-redeem-err | status / signin-redeem-err / 兑换码无效 | 820=820 / 否 / 否 |
| 1440 | verify | 1 | false → **true** | verify-battle-err | alert / verify-battle-err / 请输入有效的 battle_id | 1440=1440 / 否 / 否 |
| 1440 | login | 2 | false → **true** | login-err | alert / login-err / 请填写用户名与口令 | 1440=1440 / 否 / 否 |
| 1440 | signin | 1 | false → **true** | signin-redeem-err | status / signin-redeem-err / 兑换码无效 | 1440=1440 / 否 / 否 |

- 三视口 9 组错误态全部坐实：aria-invalid 由 false→true 字符串（!!errMsg / !!redeemMsg 条件绑定），aria-describedby 恒指对应唯一 id，错误节点 id 编译保留，零横溢，探针 isLogin=false。改前原生 ariaInvalid=null / ariaDescribedBy=null / alert id=null 的缺口全部修复。
- AX snapshot 坐实错误态控件与错误文本同时在无障碍树可见（verify「请输入有效的 battle_id」/ login「请填写用户名与口令」/ signin「兑换码无效」）。

## 3 Go 门禁与 QA 门禁

- Go job `bash-e2a3a049efd644f9` exit 0：`TestNoInfraIdentifiersInRepo` PASS 8.22s (cached)；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（`secrets_leak_test.go:182` 逐字「读不到 .local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」）。
- QA 门禁 job `bash-b232ae208b03493b` exit 0，G1–G6 全 RC0：G1 r107 契约 13/13、G2 r61 5/5、G3 全量 239 passed|1 skipped (240) files / 2209 passed|2 skipped (2211) tests、G4 type-check、G5 build:h5、G6 git diff --check。

## 4 部署后截图 sha256sum（落盘实物实测）

| 截图 | 尺寸 | sha256（实测） | 与 QA 九图 |
|---|---|---|---|
| pd107-verify-414.png | 414×896 | `40c4cf13c0e7d1cf63aa4d5629e28801052146c689b64db412d35890d8531783` | ≠ |
| pd107-verify-820.png | 820×1180 | `5da0698c71e931b29410236cffc678e96fa653bd7c5ed3049b694492ddd416f9` | ≠ |
| pd107-verify-1440.png | 1440×900 | `2c80037f14a5639a7fa0fa0050fd07742dc9e64e9dd789ec3e3e53066fe3bc39` | ≠ |
| pd107-login-414.png | 414×896 | `f16cfead7306d959a8da44a86088638580079508bfaa461b497706ff270232b7` | ≠ |
| pd107-login-820.png | 820×1180 | `6aab3c674416f5d6d742a153ca2b74713ba80721c863c082649750236bd876e5` | ≠ |
| pd107-login-1440.png | 1440×900 | `c8ef451f602b15a5a80aac975e69b2da3592485d106861e63f0bac849a6d40c4` | ≠ |
| pd107-signin-414.png | 414×896 | `0b4b2bc8c98410676ea3975d7e526d0e57f477fad1ece7d99394f3af6745a5b6` | ≠ |
| pd107-signin-820.png | 820×1180 | `c208f61ba72a6bc283e77909bc95dfec0e7602f6666af4df4351c3e981dec543` | ≠ |
| pd107-signin-1440.png | 1440×900 | `1068c3cc1942700559d94bf4e6677a288696343d08b500034ea14628c689b125` | ≠ |

- UNIQUE 9/9；与 QA 九图（verify e1d089ad…/de0e9b2d…/a5f2a22c…、login 33f719fc…/33c07f49…/84d1a697…、signin e5d6cf4c…/c9f9cfca…/c9a81c91…）交集为零。
- 亲读 3 张：`pd107-verify-414`（战绩验真页，battle_id 输入红框 + 「请输入有效的 battle_id」红字）、`pd107-login-820`（登录页错误态，账号提示）、`pd107-signin-1440`（签到页兑换码 INVALID-CODE + 「兑换码无效」红框），均为真实错误态页面，视觉与探针一致。

## 5 诚实边界

- 本轮错误态经**生产构建直接注入**验收成功：init-script 是纯网络层 XHR+fetch 双 patch 合成响应，不依赖 Vue 实例 setupState，故不落入 R103/R104/R105 的 setupState 闭包化无法注入限制；未使用 dev server。
- `:aria-describedby` 为**静态绑定**，无错误态时属性仍存在并指向未渲染的错误节点 id（契约同口径设计，QA 已披露 L2），错误节点本身 v-if 未渲染故不构成指向悬空 id 的实际播报。
- AX 快照文本本身未显式打印 described-by 引用标记；控件与错误文本的语义关联由原生 DOM 属性 aria-describedby=<错误节点 id> 坐实（QA L4）。
- signin 错误态由合成 HTTP 500 + error.message 触发（`/redeem`），非真实服务端错误（QA L3/重点⑤）；真实后端可用时 redeemMsg 仍由同一 error.message 路径产生，展示逻辑一致。
- 后端 8080 未起，全部响应 init-script 合成，占位令牌仅内存注入零真值。
- QA 三张 verify 截图与改前 recon 同名同 sha（纯 a11y 属性补全零视觉变化符合预期）；本部署后 verify 截图为不同视口（414/820/1440），语义差异以 DOM 属性 + AX 为准。
- 截图亲读 3/9；其余 6 张以 sha 重算 + DOM 探针佐证（同 QA L5 口径）。

## 6 hygiene 与终态

- auth 七路径全 ABSENT（`.env.local` / `.env.local.keys` / `laiyipao/miniapp/.env.local` / `laiyipao/admin/.env.local` / `laiyipao/admin/.env.local.keys` / `laiyipao/deploy/.local.keys` / `laiyipao/server/.local.keys`）。
- `.gitignore` 681 行，R107 六规则 676–681；`git check-ignore` 实测 `.gitignore:680` 拦 `round107-form-error-a11y-postdeploy/data/`。
- 本轮 B5 仅入仓本报告 + 9 张截图；postdeploy/data 脚本、recon 隐藏 `.r107-*` 均不入仓；B4 QA 三视口 9 图属契约预期入 audits。
- 生产 H5 产物 sha 与 §1 逐字一致；五批提交链完整，终态以 `git log` 与推送记录为准。

## 7 结论

- 三视口 9 组错误态原生 input 语义（aria-invalid false→true + describedby 指向唯一 id + 错误节点 id 保留）全部坐实，键盘/令牌不变量保持，零横溢，门禁全绿，截图与亲读齐备。
- R107 部署后复查判定 **PASS（含 §5 诚实边界）**；五批发布可交付。
