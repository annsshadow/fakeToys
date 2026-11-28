# R107 QA 复查报告：表单校验错误态的屏幕阅读器语义

- 轮次：R107（真浏览器 QA 复查）
- 任务：t306 / attempt_id=cf729105-8ee9-4bdb-90e5-8628aeb3886b
- 基线 HEAD：`528e6477d3f22d160bcbca21a65e3f1ecf773383`（全程未变）
- 三视口：390×844 / 768×1024 / 1280×800（真 Chromium，agent-browser 0.41.1）
- 验证对象：3 页 4 个错误态 input（login×2 / verify×1 / signin×1）+ 3 个错误节点 id + main.ts v-aria-native 桥接扩展

## 1. 范围与方法

R107 专治表单校验错误与输入控件的语义关联：错误态 input 补 :aria-invalid 失败态绑定 + :aria-describedby 指向错误节点唯一 id，错误节点补 id，扩展 main.ts applyAriaNative 把两属性桥接到原生 input.uni-input-input。

关键机制：uni-app H5 属性白名单会把 aria-invalid / aria-describedby 从外层 uni-input 丢弃，故必须直接读**原生 input.uni-input-input 的 DOM 属性**验证（而非外层自定义元素）。改前基线（t302 recon `docs/laiyipao/recon/ROUND107_A_FORM_ERROR_A11Y_RECON.md`）坐实三视口 verify 错误态视觉生效（红框红字）但原生 ariaInvalid=null / ariaDescribedBy=null / alert id=null，AX 中 spinbutton 与 alert 彼此孤立。

方法：自启 H5 dev server（5174），后端 8080 未起，全量用 init-script 合成响应（cfg version r107-qa-v1 + 占位令牌 r107-qa-placeholder-token，仅内存注入零真值）。每视口每页：open(init-script) → 重设 viewport → probe BEFORE → 触发错误 → probe AFTER → AX snapshot → 截图。

## 2. 三视口 × 三页原生 input 属性坐实

探针直接读 input.uni-input-input 的 ariaInvalid / ariaDescribedBy / ariaLabel；错误节点读 role / id / text。BEFORE 为进入页面初始态，AFTER 为触发校验错误后。

| 视口 | 页 | 态 | aria-invalid | aria-describedby | aria-label | 原生 input 数 | 错误节点(role/id/text) | cw/sw | 横溢 | 登录页 |
|---|---|---|---|---|---|---|---|---|---|---|
| 390 | verify | BEFORE | false | verify-battle-err | battle_id | 1 | 无 | 390/390 | 否 | 否 |
| 390 | verify | AFTER | **true** | verify-battle-err | battle_id | 1 | alert / verify-battle-err / 请输入有效的 battle_id | 390/390 | 否 | 否 |
| 390 | login | BEFORE | false | login-err | 用户名 | 2 | status/无id/请填写用户名与口令后可登录 | 390/390 | 否 | 否 |
| 390 | login | AFTER | **true** | login-err | 用户名 | 2 | 新增 alert / login-err / 请填写用户名与口令 | 390/390 | 否 | 否 |
| 390 | signin | BEFORE | false | signin-redeem-err | 输入兑换码 | 1 | status/无id/AI今日任务已领完… | 390/390 | 否 | 否 |
| 390 | signin | AFTER | **true** | signin-redeem-err | 输入兑换码 | 1 | 新增 status / signin-redeem-err / 兑换码无效 | 390/390 | 否 | 否 |
| 768 | verify | AFTER | **true** | verify-battle-err | battle_id | 1 | alert / verify-battle-err / 请输入有效的 battle_id | 768/768 | 否 | 否 |
| 768 | login | AFTER | **true** | login-err | 用户名 | 2 | alert / login-err / 请填写用户名与口令 | 768/768 | 否 | 否 |
| 768 | signin | AFTER | **true** | signin-redeem-err | 输入兑换码 | 1 | status / signin-redeem-err / 兑换码无效 | 768/768 | 否 | 否 |
| 1280 | verify | AFTER | **true** | verify-battle-err | battle_id | 1 | alert / verify-battle-err / 请输入有效的 battle_id | 1280/1280 | 否 | 否 |
| 1280 | login | AFTER | **true** | login-err | 用户名 | 2 | alert / login-err / 请填写用户名与口令 | 1280/1280 | 否 | 否 |
| 1280 | signin | AFTER | **true** | signin-redeem-err | 输入兑换码 | 1 | status / signin-redeem-err / 兑换码无效 | 1280/1280 | 否 | 否 |

三视口 9 组错误态全部坐实：aria-invalid 由 false → true 字符串（!!errMsg / !!redeemMsg 条件绑定），aria-describedby 恒指对应唯一 id，错误节点 id 编译保留，零横溢且均非登录页。改前 ariaInvalid=null / ariaDescribedBy=null / alert id=null 的缺口全部修复。

## 3. 核心桥接与 AX 语义

main.ts:9-21 applyAriaNative 扩展后，mounted/updated 均把外层 uni-input 上编译保留的 aria-label / aria-invalid / aria-describedby setAttribute 到原生 input.uni-input-input；无效值走 removeAttribute 分支。三视口 trial 坐实核心机制：768 verify 清空 battle_id 点「取复现信息」后，原生 input ariaInvalid false→true / ariaDescribedBy=verify-battle-err，alert 节点 id=verify-battle-err。

AX snapshot（三视口一致）坐实错误态控件与错误文本同时在无障碍树可见：

- verify：`- spinbutton "battle_id" [ref=e3]` + `- StaticText "请输入有效的 battle_id"`
- login：`- textbox "用户名" [ref=e1]` / `- textbox "口令" [ref=e2]` + `- StaticText "请填写用户名与口令"`
- signin：`- textbox "输入兑换码" [ref=e4]: INVALID-CODE` + `- StaticText "兑换码无效"`

诚实说明：AX 快照文本本身未显式打印 described-by 引用标记；控件与错误文本的语义关联由原生 input 的 DOM 属性 aria-describedby=<错误节点 id> 坐实（见 §2），AX 树证明两者均被暴露为可感知节点。

## 4. 无错误态口径（诚实披露）

- :aria-describedby 为**静态绑定**，恒指错误节点 id（如 login-err / verify-battle-err / signin-redeem-err），因此无错误态时该属性仍存在。此属源码设计与 R107 契约同口径，非缺陷；错误节点本身在无错误态未渲染（v-if="errMsg" / "redeemMsg"），故不构成指向悬空 id 的实际播报。
- main.ts removeAttribute 分支对本项目四个 input 因静态绑定不触发；aria-invalid 由双叹号产生 "false"/"true" 字符串，无陈旧 true 残留（BEFORE 全为 false，见 §2）。
- signin 错误态经 mock 让 /redeem 返回 HTTP 500 + error.message 触发 client.ts reject → signin.vue:481 redeemMsg=兑换码无效（因 request() 对 HTTP 200 的 ok:false 直接 resolve 不抛错，故不能只返回 200+ok:false）。此为合成故障，非真实服务端错误。

## 5. 不变量与静态回归

- 令牌三锚点 tokens.css:14-16 在：--lyp-token-version: lyp-tokens-v1 / --lyp-r3-version: lyp-tokens-v3-r3 / --lyp-r6-version: lyp-tokens-r6。
- 业务零 --r107-：laiyipao/miniapp/src 下 vue/css NO_MATCH（唯一 3 处命中在契约 r107FormErrorA11y.contract.test.ts 自身断言）。
- R61：四 input 保留 aria-label + v-aria-native，桥接仍 setAttribute aria-label。
- R60/R100/R101/R104：index.vue aria-expanded="String(expanded.has(a.id))" 在；equipment.vue r6-skeleton role=status aria-busy=true 在；me back-btn 键盘三件套由契约不变量 it 守卫。
- 零横溢：三视口九组 cw=sw、isLogin=false（见 §2）。

## 6. 截图 sha256sum（落盘实物实测）

9 张截图位于 docs/laiyipao/audits/round107-form-error-a11y/screenshots/，sha256sum 实物实测，UNIQUE 9/9：

| 截图 | sha256 |
|---|---|
| r107-verify-390.png | e1d089adcf642e4271065b3d5bf57ca09bd2a67f566a84f63ac1e04d57ccfa63 |
| r107-verify-768.png | de0e9b2d46e245c452e8c312acb5287a107877cfd331fd5376b9db161d05f7a8 |
| r107-verify-1280.png | a5f2a22c4265551782217a25d37f35fc1df495bf598bf0043d30b811448129b0 |
| r107-login-390.png | 33f719fc04d18f02b3c1309802457fdd948257798fe6b1392926c11ee5e6e75d |
| r107-login-768.png | 33c07f493efb9b4eb912c00ca51fc12d5752acbd71603b6a28a2a6280724094c |
| r107-login-1280.png | 84d1a6974c6cbf1df3719a97e3501e5f4aa44654c3d77196e30c3977fc53ef8f |
| r107-signin-390.png | e5d6cf4c5706bc3f001ee71e31288979bc79cb519e03417b9bc41e73cf693996 |
| r107-signin-768.png | c9f9cfca2f222dadfbb1169296ba07e710d3e58440995f0999c67cff222da943 |
| r107-signin-1280.png | c9a81c91e614951fc27deb7d114d13b0b295ee109b2ce4f6d74d31ee977ef747 |

说明：三张 verify 截图 sha 与改前 recon 同名截图一致（verify-390=e1d089ad… / 768=de0e9b2d… / 1280=a5f2a22c…）——R107 为纯 a11y 属性语义补全、零视觉变化，错误态视觉（红框红字）与改前相同符合预期；语义差异以 §2 DOM 属性 + §3 AX 为准。

亲读 2 张非登录页：r107-verify-768.png（768×1024，战绩验真页，battle_id 输入框红框错误态 + AI 复核提示）与 r107-signin-1280.png（1280×800，签到页兑换码 INVALID-CODE 红框 + 红字「兑换码无效」），视觉与探针一致。

## 7. 门禁

独立门禁脚本 gates-final.sh（仓库根 workdir，输出到 data/gates-final.txt）全绿：

- G1 r107FormErrorA11y.contract.test.ts：13 passed / 13，RC=0
- G2 r61FormA11y.contract.test.ts：5 passed / 5，RC=0
- G3 全量 vitest：239 passed | 1 skipped (240) files，2209 passed | 2 skipped (2211) tests，RC=0
- G4 pnpm run type-check（vue-tsc）：RC=0
- G5 pnpm run build:h5：RC=0
- G6 git diff --check：RC=0
- GATES_DONE

说明：G3 中 [Vue warn] Failed to resolve directive: aria-native 与 [来一炮] build.attacker 缺失为测试既有 stderr 告警，非本轮引入，不影响 RC=0。

## 8. 诚实边界

- 后端 8080 未起，全量用 init-script 合成响应（config/token/wallet/me/redeem 等），占位令牌 r107-qa-placeholder-token 仅 localStorage 内存注入，零真值。
- signin 兑换失败态由合成 HTTP 500 触发，非真实服务端错误；改前 recon 该按钮未命中未触发，本轮经修正 mock 成功坐实。
- describedby 无错误态仍静态存在（§4）属源码设计，不冒充「无陈旧属性」全绿。
- AX 快照未显式打印 described-by 引用标记，关联由原生 DOM 属性坐实（§3）。
- 截图亲读 2/9；三张 verify 截图视觉与改前相同（纯属性改动零视觉变化）。

## 9. 结论与 hygiene

R107 表单校验错误 a11y 语义在真 Chromium 三视口坐实：4 个错误态 input 的 aria-invalid 由 false→true、aria-describedby 指向唯一错误节点 id、3 个错误节点 id 与 role/live 编译保留、AX 可见控件与错误文本，改前缺口全部修复。不变量与令牌未回退，门禁全绿。判定 PASS。

hygiene：HEAD=528e6477d3f22d160bcbca21a65e3f1ecf773383 未变；git status 非 ?? 项仅 M .gitignore / main.ts / login.vue / signin.vue / verify.vue（全为预期 t305）；auth 五路径全 ABSENT；产出 REPORT.md 入库，data/ 与 screenshots/ 为 UNTRACKED；占位令牌零真值。
