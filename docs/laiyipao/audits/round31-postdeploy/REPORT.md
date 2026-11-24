# R31 部署后独立回归报告（Wiki 反应机制字段）

## 1. 结论

PASS。部署后另起独立真实 Chromium 会话（agent-browser 0.41.1，无共享部署前会话），在 1440×900 / 375×390 / 390×844 三视口对公开 /api/v1/config 反应字段逐行对账，7 反应 × 3 字段（层数传播 / 削甲 / 击退）全部一致；表格可横向滚到末列，页面零横向溢出；深链 / 搜索 / burn_cloud 未实现文案未回退。四张部署后新截图 SHA 两两不同且不同于部署前五图。

## 2. 环境与产物

- 起点 HEAD=9f874bdf9eeee0548c06b741d2aa7a4845260844；提交批次 B1=8962cf19f79babfff9923001b5428b4f36bd2623 / B2=5b36ab66cfe56f66873c755840aa7e3a7ffaa244 / B3=4575cd5601f3b84fc9d4079376e1f5e82d9019c1 / B4=1c0dc6799e36ff4f5479c4c7331dc826377e431a。
- 三端健康：/healthz=200、/readyz=200、H5 根=200、admin 根=200（均 127.0.0.1 回环）。
- 产物 SHA-256：后端 lyp-server-amd64=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 沿用，未回退）；H5 index-DCBbR3t9.js=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b（R30 沿用，未回退）；admin index-Cu8EeCJa.js=59e77e97261d6a3db5d9be6b8bb7b85596944c00f1246f332bb89111033a68ac；admin WikiView-DQo7myEL.js=e7bf328f677cbc76222fd6cc755219299eba78daa0a986c7fc314b6a1e345aba。
- WikiView 新产物 grep 计数：spread_stacks=1、armor_shred_permille=1、knockback=1、层数传播=3、削甲=2、击退=2。
- 公开 /api/v1/config version=6237e0b623219b8b，反应条目 7 条；快照存于 data/config.json（被 .gitignore 排除）。

## 3. 本地门禁（如实记录）

- 全量 vitest：默认 testTimeout=20000 时 tests/bootstrap.test.ts「挂载后 #app 渲染登录页」在并发负载下 20s 超时（1 failed / 780 passed）；隔离单独跑 1.6s PASS；第三次以 --testTimeout=60000 全量 69 files / 781 tests 全 PASS。属既有并发毛刺，非 R31 引入。
- admin pnpm type-check（vue-tsc）exit=0；pnpm build exit=0；Go TestNoInfraIdentifiersInRepo PASS；Go TestNoRealSecretsInRepo 因本机缺少 .local.keys 打印「真值扫描这条守卫没有执行，不是通过了」后 SKIP——如实标注，未冒充 PASS；以 OPS 静态真值扫描（R31 目录零命中）替代。
- 三份审计脚本 bash -n 通过；git diff --cached --check 通过（B4 曾因 REPORT.md 末尾空行 exit=2，已删至单个 EOF 换行后 commit）。

## 4. 三视口逐行对账

- 对账脚本 data/reconcile-7x3.py 以公开 config 的 spread_stacks / armor_shred_permille / knockback 数值格式化为「传播 N 层 / N‰ / Npx」，与 DOM 表头「层数传播 / 削甲 / 击退」列逐行比对。
- desktop 1440×900：CHECKED=7、OVERFLOW=0、SCROLL={before:0, after:206, max:206}、BAD=[]、OK=true。
- mobile375 375×390：CHECKED=7、OVERFLOW=0、SCROLL={before:0, after:781, max:781}、BAD=[]、OK=true。
- mobile390 390×844：CHECKED=7、OVERFLOW=0、SCROLL={before:0, after:766, max:766}、BAD=[]、OK=true。
- FULL_7X3_RECONCILE_OK=true。三视口 page.scrollWidth=clientWidth=innerWidth，badToken=false。
- 关键行 DOM：腐蚀扩散=「传播 1 层」；破甲击退=「150‰」+「4px」。末列滚动探针：found=true、before=0、after=766、max=766、lastRight=357、wrapRight=357。

## 5. 深链 / 搜索 / 文案

- 深链 #/wiki 加载：visibleSections=["5 元素"]、focusRows=0、tableRows=124、docOverflowX=0。
- 搜索框焦点探针（mobile390 底部滚动后）：focused=true。
- 文案保留：hasImplement=true（层数传播已实现）、hasBurnCloud=true（burn_cloud 仍未实现）、oldWording=false（旧「没有特殊效果的只剩两条」未回退）、has150permille=true、has15percent=false、badToken=false。

## 6. 截图 SHA-256（四张部署后新图）

- r31post-desktop-wiki.png=780ec68c89284a08f0d60df4dbf81c2fb8cd82b5068dcb6182675a4a35c06bc6（1440×900）
- r31post-mobile375-wiki.png=e7e9cfb42557d10bc83a4e3d1f06ef4a21701fd7688e5bd69178795e3a7997b5（375×390）
- r31post-mobile390-wiki.png=296b31a699304f571935ae27c1a11aadff50287d2af10ec0ed26f6461b88589f（390×844）
- r31post-04-deeplink.png=53508a9bd9971f6e3dcbb0b29e0de74f8f1eed9d1550d57fd87b03456d61ea4d
- 四图 SHA 两两不同，且全部不同于部署前 QA 五图（r31a-desktop=9237e65d…、r31a-mobile375=de75ce6a…、r31a-mobile390=e5f10e4d…、r31a-04-deeplink=209f8af5…、r31a-05-search-focus=ec8a0d6f…）。首轮 E2E 截图与 QA 图逐字节相同（静态页面未产生新像素），已通过滚动/表格归位/搜索焦点等真实状态切换重拍四图，过程如实保留。

## 7. 真值清理与局限

- admin-auth-init.js 为本地令牌注入临时文件，已被 .gitignore 排除；收尾删除。data/ 下 config.json、probe*.json、config 快照等均被 .gitignore 排除，零入库。
- QA 库收尾 pg_restore qa-before.dump 还原 battle_records=104 / max_id=477（与部署前一致）。
- 局限：视口由浏览器模拟，非物理真机；屏幕截图 SHA 受渲染环境影响，仅用于区分会话与状态，不作为产品判据；TestNoRealSecretsInRepo 本机 SKIP 已如实标注。

## 8. 第六批更正：.gitignore 截断回归与修复

- 第五批 f8d00139 追加 R31 postdeploy 忽略规则时，误将根 .gitignore 从父提交 367 行截断为 12 行，丢失 .env、*.local.keys、.agents/、node_modules/ 等通用 secrets/构建规则及 R15–R30 历史审计真值规则。
- 产品功能、三视口对账证据与部署证据不受影响（产物哈希未变，无需重新部署）；但零真值防线出现回归，必须如实记录。
- 新增 laiyipao/admin/tests/GitignoreSafety.test.ts：读取根 .gitignore 并断言行数不少于 360、通用规则与 R15/R30/R31 审计真值规则齐备。在截断版本上运行 3/3 RED（缺 .env 与 R15 data 规则等），恢复后 3/3 GREEN。
- 修复：从父提交 f8d00139^ 恢复完整 .gitignore（367 行），再追加 R31 postdeploy 的 data/ 与 admin-auth-init.js 两条规则，合计 371 行；git check-ignore 对 R15/R30/R31 临时 data 与 auth-init 路径全部命中。
- 第六批仅提交 .gitignore、防回归测试与本报告；本报告不含任何真值。TestNoRealSecretsInRepo 本机 SKIP 的口径不变。
