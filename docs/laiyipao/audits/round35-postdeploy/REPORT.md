# R35 部署后独立回归报告（Wiki 宝石图鉴八宝石五品质与品质搜索）

## 1. 结论
PASS。部署后另起真实 Chromium（headless、本地回环 127.0.0.1，管理员会话仅通过本地 init-script 注入以读取公开配置）独立会话，三视口 390×844 / 768×1024 / 1280×800 对公开 `/api/v1/config` 逐行核对 8 宝石 × 5 品质，MISMATCHES=0；AI 概览「共 8 种宝石，5 档品质，词条 1–3 条」可见，千分号标注为初始系数不做百分比换算，诚实文案「词条随机，具体数值以游戏内为准」在位。决定性真实 CLI 搜索 gold 命中 focus=quality:gold 且品质表 gold 行 DOM 深蓝高亮；锋锐石 focus=gem:1 不回退。CTA 真点击停留 gems tab 未跳看板；R31 反应 7 / R32 技能 24 / R33 敌人 22 / R34 关卡 20 四 tab 均不回退，表内可滚动且页面无横向溢出。后端/H5 产物沿用不回退。

## 2. 部署与产物基线
- HEAD（审计批 B4）= 8379af4c2；B1-B4：3c06b3f87 feat / ae20129c5 test / 7ee27df3f docs / 8379af4c2 audit。
- 后端产物 `laiyipao/deploy/bin/lyp-server-amd64` sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 沿用，未回退）。
- H5 产物 `laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js` sha256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b（R30 沿用，未回退）。
- admin 新产物 `laiyipao/admin/dist/assets/WikiView-BUbjxf0V.js` sha256=05004d5849134327bd6243b26474ef65cc436fb2667208538aa4a9e53d1812b2，含 R35 宝石 tab、品质表与 quality 品质名搜索逻辑（grep 到 `quality:` 搜索分支与「查看宝石图鉴」CTA 文案）。admin 开发服务走 Vite HMR（src/main.ts 实时编译），浏览器实测页面即 R35 代码；dist 产物作为构建证据。
- 服务健康：8080/healthz、8080/readyz、8080/api/v1/config、5174/、5175/laiyipao-admin/ 全部 HTTP 200。
- 本地门禁：`pnpm exec vitest run --reporter=dot --testTimeout=90000` / `pnpm type-check` / `pnpm build` 均退出码 0（VITEST_RC=0 TC_RC=0 BUILD_RC=0，74 文件 827 测试通过）。
- Go 守卫 `go test ./... -run TestNoInfraIdentifiersInRepo` RC=0；`git diff --check` RC=0。

## 3. 三视口逐行对账（reconcile.py 读公开 config）
执行 e2e-viewports.sh（390 390×844 / 768 768×1024 / 1280 1280×800）后 `python reconcile.py`（内置 Python 3.12，系统 python3 为 Store 别名不可用）：
- 三视口 gem.headers=[ID,名称,属性,说明]，8 宝石行：1 锋锐石/攻击、2 元素石/元素系数、3 锐曜石/暴击率、4 共辉石/反应倍率、5 容量石/元素层数上限、6 散热石/热量上限、7 整垒石/护甲、8 技能石/技能伤害。
- 品质 5 行：gray 1 条 1000‰、blue 1 条 1200‰、purple 2 条 1450‰、red 2 条 1700‰、gold 3 条 2000‰；系数以千分号展示为初始系数，不冒充百分比。
- AI 期望 `共 8 种宝石，5 档品质，词条 1–3 条`；badText（MinValue/MaxValue/SeedGemAffixes/数字%）False；DOC=视口宽（无页面横溢出）。
- ALL_RECONCILE_OK=True，RECONCILE_EXIT=0，三视口 VERDICT=PASS。中文在 shell 显示乱码仅控制台编码，probe JSON 原始字节与 Python ascii 复核一致。

## 4. 决定性真实 CLI 搜索（品质名命中与高亮）
- gold：CLI `click input[type=search]` → `keyboard type gold` → `press Enter`（非 DOM eval / e.click 作最终证据）；hash=#/wiki?tab=gems&focus=quality:gold，focusAttr=quality:gold，focusRows=['gold 3 条 2000‰']，goldHasFocus=true，inputValue=gold，badText=false。亲读 r35post-search-gold.png 确认品质表 gold 行深蓝底色高亮。
- 锋锐石：同样 CLI 搜索，focus=gem:1，fengHasFocus=true，goldHasFocus=false——宝石名搜索不回退。

## 5. CTA 真点击与前轮不回退
- CTA 文案「查看宝石图鉴 →」href=#/wiki?tab=gems；CLI `click '[data-r35-cta]'` RC=0；点击后 hash=#/wiki?tab=gems，gemStillThere=true，dashboard=false，activeNav=[宝石图鉴]——停留 gems tab 未跳看板。
- 五 tab 回归（tab=basics/skills/enemies/levels/gems）：reactionCount=7 且 corrosionSpread/armorShred/armorKnock 均在，skillCount=24，enemyCount=22，levelCount=20；docW=innerW=1258（页面无横向溢出），bad=false，REG_DONE=YES。R31/R32/R33/R34 四 tab 不回退。

## 6. 新截图与 SHA
部署后独立会话采集 12 张截图。满足「≥4 张新图 sha 两两不同且全不等于部署前审计图」的图（节选）：
- r35post-05-cta-after.png = b3bb3feb64ea77e7bb2d6b892b17ccc9751794dcebd83d4e93aeff3f7d46b2aa
- r35post-1280-gems.png = 26a28d0ca181e0ff48fe8368edab3c2827a4a00e54b956f06464e9c70a0aa99f
- r35post-768-gems.png = 2da79af6cb75f2edbec2c51800034db0229d752544a31e5e5cf31d7faec45677
- r35post-reg-basics.png = 0db7d33d45814700f87eb84020a3d6b4ea1155262aa6fc3a3e2416583f2c2aa2
- r35post-reg-enemies.png = e901c522564d3168d920ac3d935add7d4bde8be363db9fa1479f84182a5b11c7
- r35post-reg-gems.png = 2500550050f930e3f8254a5cc7548d779cf766e98493cd5c2d2b3242a24bef60
- r35post-reg-levels.png = b8f706af0a4cafcc81955e6e1382e9a6e30aa40280c8f83a51466e5f0cab3b7a
- r35post-reg-skills.png = 889f28fc9041f987d49e95445883d2b99a1174a364438bab79d923fbd821e7dd
共 8 张唯一且与部署前 round35-wiki-gem-mechanics 审计图 SHA 无交集。亲读 1280-gems（宝石表 8 行 + 品质表 5 行 + AI 概览，非登录页）、search-gold（gold 品质行深蓝高亮 2000‰）、cta-after（宝石 tab 高亮停留 gems）三图确认真实页面。
诚实边界：r35post-search-gold.png = 9997a214… 与部署前 r35p-search-gold.png、r35post-search-feng.png = fde56332… 与部署前 r35p-search-feng.png / r35-search-锋锐石.png 为同 SHA——静态页在同视口/同滚动位的确定性像素一致（agent-browser 固定 1258×588 捕获）；这是截图确定性而非复用旧图。r35post-390-gems 与 r35post-04-cta-before 同 SHA（43febd20…）亦为同像。验收所需「≥4 张新图」由上述 8 张唯一图满足。

## 7. 真值治理
- 部署后临时管理令牌写入 data/admin-auth-init.js（bootstrap 默认口令登录，JWT 229B），由 .gitignore:406 拦截；finalize 删除 data/ 与根令牌，git status 无 data/auth 入库。
- 令牌 mint 根因：attempt2 草稿 mint-token.sh 的 password 字段是脱敏占位符字面量 `<REDACTED>`，非真实口令；服务以 bootstrap 默认口令（config.go 公开默认值）启动，替换为该公开默认值后 mint 成功 TOKEN_LEN=229。占位符修复不涉及任何真实 secret 入库。
- 零真值扫描（zero-secret-scan.sh，排除 data/ 与本地回环测试登录脚本）：BAD_COUNT=0；脚本 bash -n 全通过 SYNTAX_RC=0。
- TestNoRealSecretsInRepo：本机缺 `.local.keys` 真值源，该守卫打印「未执行」后 SKIP——**这是未执行、非 PASS**，不以此作为通过依据；改用静态零真值扫描与 git 门禁作为替代证据。

## 8. QA 数据还原
- finalize.sh：删除 postdeploy/data/ 与根令牌 → AUTH_DELETED=YES；TRUNCATE battle_records RESTART IDENTITY CASCADE（CASCADE 至 replay_verifications）后 pg_restore qa-before.dump；独立复核 DB=104|477（count=104、max id=477，与发布前基线一致）。库名用 laiyipao（容器 POSTGRES_DB），非历史误写的 lyp_qa。
- git check-ignore -v 命中 .gitignore:406 拦截 `round35-postdeploy/data/`；.gitignore 共 407 行（相对 R34 399 行仅追加 R35 live/postdeploy data 与 auth 8 行），未截断既有规则。

## 9. 工具与时间边界
- agent-browser 为 .cmd 批处理，MSYS 下用 timeout.exe 包装恒 RC=126（命令根本未执行）；全部浏览器命令均以后台 job + job_output 收集控制墙上时间，不用 timeout 包装。
- 浏览器命令逐个串行执行，避免 session 竞争；每视口独立 session 并在结束时 close。每次真浏览器前先确认五端 200，否则会拿到 chrome-error://chromewebdata/ 空页。
- 系统 python3 为 Microsoft Store 别名不可用，统一使用内置 Python 3.12（路径由本环境变量 `PY` 注入，脚本内以 `${PY:-python}` 占位）；本机路径不入库。
