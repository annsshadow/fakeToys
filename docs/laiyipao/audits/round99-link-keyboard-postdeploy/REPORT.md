# R99 19处 link 键盘可达 —— 部署后真浏览器复查报告（postdeploy）

- 轮次：R99 五批发布 B5（承接 t249规范→t250实施→t251 QA→t252终审）
- 任务：same-route-rel（t253），attempt_id=6335beba-02ca-451f-9f3d-93cdd3c97bef
- 工作目录：D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
- 起点 HEAD：05f2acf30950a377b681c09ae05d988f25eac866；本报告对应 B5 提交
- 复查对象：生产构建 H5 下 bag/me 页可点 text.link 的 role=button / tabIndex=0 / AX button / 真 Tab 命中 / 焦点环 / Enter 送达 / 零横溢

## 1. 五批链（commit 前缀 feat/test/docs/audit/chore）

| 批次 | sha256 | 说明 |
|---|---|---|
| B1 feat | 8bc9850a6094e0ba5e69f2ebf01171533d60c24d | 4页vue 19处link补三属性 |
| B2 test | 169f2f720029f6d2cad4b1bccf83172a2bc550c7 | r99LinkKeyboard.contract.test.ts 24 it |
| B3 docs | 0b71d7d428b696b69641936f8cd97179cf7b16de | 两份侦察+终审+SPEC+RED证据 |
| B4 audit | 2c9c64d3bae6dead73a5d805112f38f3e4c7fa9d | .gitignore+QA报告+脚本+截图 |
| B5 chore | 以最终 git log 五批链为准 | 本报告+6张部署后截图（chore，postdeploy） |

## 2. 部署产物

- H5 重建：pnpm run build:h5 RC0（DONE Build complete）。
- 主 bundle：index-BBtKYeLL.js（index.html:21 引用）。
- 主 bundle sha256：e195c1c2278d840f301fbdd6391124ff0a75dc65b3ae53587776b08eaa7c0d79。
- 后端 lyp-server-amd64 沿用不回退。

## 3. 三视口真浏览器复查（生产构建 http://127.0.0.1:5201）

- 真浏览器：agent-browser 0.41.1 真 Chromium，三视口 414×896 / 820×1180 / 1440×900。
- 静态服务：python -m http.server 5201 on dist/build/h5。
- init-script 注入合成 config/占位令牌 r99-qa-placeholder-token（零真值），使 link 自然渲染；XHR+fetch 双 patch 记录请求。
- 探针：data/pd-probe.js（linkCount/role/tabIndex/iw/cw/sw/hOverflow/isLogin）、data/pd-hit.js（CTA_HIT/CTA_NO）、data/pd-focus.js（outlineStyle/Width/Color/isLink）、data/pd-enter-observer.js（单行IIFE监听keydown记__qaKeys）、data/pd-actions.js。
- 驱动：data/pd-driver.sh —— 三视口 × bag/me，open --init-script → sleep5 → 重设视口 → probe → observer → AX → screenshot → 14次Tab循环，命中CTA即取焦点环并按Enter。

| 视口 | 页面 | linkCount | role/tabIndex | iw/cw/sw | hOverflow | isLogin | AX button | Tab 命中 | 焦点环 | Enter 送达 |
|---|---|---|---|---|---|---|---|---|---|---|
| 414×896 | bag | 3 | 全 button/0 | 414/414/414 | false | false | ✓ | 命中 link1/link2 | solid rgb(255,107,53) | isLink=true |
| 414×896 | me | 2 | 全 button/0 | 414/414/414 | false | false | ✓ | 命中 改名/复制 | solid rgb(255,107,53) | isLink=true |
| 820×1180 | bag | 4 | 契约3处 button/0 | 820/820/820 | false | false | ✓ | 命中 link1/link2 | solid rgb(255,107,53) | isLink=true |
| 820×1180 | me | 1 | button/0 | 820/820/820 | false | false | ✓ | 命中 复制 | solid rgb(255,107,53) | isLink=true |
| 1440×900 | bag | 4 | 契约3处 button/0 | 1440/1440/1440 | false | false | ✓ | 命中 link1/link2 | solid rgb(255,107,53) | isLink=true |
| 1440×900 | me | 1 | button/0 | 1440/1440/1440 | false | false | ✓ | 命中 复制 | solid rgb(255,107,53) | isLink=true |

- CTA_HIT 累计 27 次（14次Tab循环中重复命中同一link），6组全部至少1次命中；AX button 节点含去关卡页/保存当前构筑/点这里填名字保存一套/改名/复制/去专精页/‹ 返回。
- 焦点环全为 outlineStyle=solid / outlineColor=rgb(255, 107, 53)（tokens.css:35 --accent:#ff6b35），isLink=true。
- 6组 isLogin 全 false、hOverflow 全 false、iw=cw=sw=视口宽，零横向溢出。

## 4. 部署后截图表（落盘实物 sha256）

| 文件 | sha256 |
|---|---|
| pd99-414-bag.png | 05a686ccbf0fa695f7aabf9ac3ecee6189c4d25df0b029daf203ea4fadc25f1c |
| pd99-414-me.png | 6a5d339a0948218986f096ec3baff45913a695e3c4dabbeb476d8e2f67943163 |
| pd99-820-bag.png | f6ed59a5f304eb1cc8d7ba1368a09bb95275fc778d3e2e4db07fea6f8d6207e1 |
| pd99-820-me.png | 7a41f6dd1ab900e8137bb35aa80aca4ab6cddd0930a0ad85de8e9c9b684aca0e |
| pd99-1440-bag.png | 70007fa2f21352b57afe057f94d3a6f6bae3d73cab054ddcc6fbd355a6dae02d |
| pd99-1440-me.png | 235de984dc17b119fb34d7577bd14a5279721d464f8da9a613df2352d6eade0a |

- 6 张全互异 UNIQUE=6/6，且与 QA 6 张（r99-bag/me-390/768/1280）前缀全不同，无一张复用。
- 亲读 3 张均非登录页：pd99-414-bag.png（背包页，AI空态+去关卡页CTA+保存当前构筑/点这里填名字保存一套 link）；pd99-820-me.png（指挥官档案页，‹ 返回+去专精页+ID 0 复制 link）；pd99-1440-me.png（同页桌面布局）。其余 3 张仅列 sha 未逐张亲读。

## 5. 门禁与静态回归

- Go TestNoInfraIdentifiersInRepo：PASS 8.22s。
- Go TestNoRealSecretsInRepo：SKIP 非 PASS —— secrets_leak_test.go:182 逐字「读不到 .local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」」，如实标注。
- QA 库只读：users=828 / battle_records=104（docker exec lyp-qa-pg psql -U postgres -d laiyipao），未写库。
- 令牌三锚点在、业务源码零 --r99-、19 处契约静态断言在（承袭 t251/t252 门禁 G1-G7 全 RC0）。

## 6. 诚实边界

- 后端 8080 未起：数据来自 init-script 合成 config + 占位令牌，非真值；未连接真实后端。
- Enter 送达以 __qaKeys 中 isLink=true 为判据；uni.navigateTo / uni.setClipboardData 桥接的具体导航/剪贴值未在 h5 生产构建稳定捕获，故不对 Enter 后页面实际跳转/复制成功作 DOM 断言。
- .gitignore:639-640 的 R99 postdeploy 规则名写作 round99-a-link-keyboard-postdeploy（带 -a-），与本次实际目录 round99-link-keyboard-postdeploy（不带 -a-）不一致，未实际拦截本目录 data/。B5 通过精确 git add 仅提交本报告与 6 张截图，data/ 下探针脚本一律不入库，tracked 本轮 data=0，零真值。该规则名不一致为 low 级观察，提请队长后续修正，不阻断本次发布。
- 焦点环宽度探针归一时同为 solid 橙色（1px），与 QA 阶段 1px/3px 两种均为可见焦点环。

## 7. 判定

- 三视口 6 组 bag/me 部署后复查全绿：role=button / tabIndex=0 / AX button / 真 Tab 命中 / solid 橙色焦点环 / Enter isLink=true / 零横溢 / 非登录页。
- 6 图 UNIQUE=6/6 且全 ≠ QA 图，亲读 3 张坐实。
- 零真值入库、auth 路径 ABSENT、tracked 本轮 data=0。
- 结论：部署后复查 PASS，与 QA / 终审一致，同意本轮五批发布。