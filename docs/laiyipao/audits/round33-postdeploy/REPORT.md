# R33 部署后独立回归报告（Wiki 敌人战斗机制）

## 1. 结论
PASS。部署后另起真实 Chromium（headless、本地回环 127.0.0.1）独立会话，三视口 390×844 / 768×1024 / 1280×800 对公开 `/api/v1/config` 逐行核对 22 种敌人 × 15 列，MISMATCHES=0；AI 结论「共 22 种敌人：首领 4 种，飞行 3 种，潜地 1 种，带护甲 9 种」在三视口一致且无占位符残留。CTA 真点击停留敌人 tab、未跳看板；表内可水平滚至末列且页面无横向溢出；R31 反应 7 行三字段与 R32 技能 24 行 ID3 温压弹均不回退。后端/H5 产物沿用不回退。

## 2. 部署与产物基线
- HEAD（发布提交）= e8adf55d8（B1-B4：db6956c1b feat / 5763c317a test / 0f13dab54 docs / e8adf55d8 audit）。
- 后端产物 `laiyipao/deploy/bin/lyp-server-amd64` sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 沿用，未回退）。
- H5 产物 `laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js` sha256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b（R30 沿用，未回退）。
- admin 新产物 `laiyipao/admin/dist/assets/WikiView-i9qlBNf2.js` sha256=6115e9a8852c51c63af2acc283dbcabbded83783c3f967656f7d620dae6b18c3，含 R33 九字段与 AI 四要素文案。
- 服务健康：8080/healthz、8080/readyz、8080/api/v1/config、5174/、5175/laiyipao-admin/ 全部 HTTP 200。

## 3. 三视口逐行对账（reconcile.py 读公开 config）
执行 `python reconcile.py 390 768 1280`（内置 Python 3.12，系统 python3 为 Microsoft Store 别名不可用）：
- VP=390：ROWS=22 MISMATCHES=0 AI_OK=True BAD=False DOC=390/390 WRAP=1515/324 SCROLL=0->1191 VERDICT=PASS
- VP=768：ROWS=22 MISMATCHES=0 AI_OK=True BAD=False DOC=768/768 WRAP=1515/702 SCROLL=0->813 VERDICT=PASS
- VP=1280：ROWS=22 MISMATCHES=0 AI_OK=True BAD=False DOC=1280/1280 WRAP=1515/884 SCROLL=0->631 VERDICT=PASS
- ALL_RECONCILE_OK=True。中文在 shell 显示为乱码仅是控制台编码，probe-*.json 原始字节与 Python ascii 转义复核一致。

## 4. CTA 真点击（agent-browser CLI click）
- 点击前：hash=#/wiki?tab=enemies，按钮文案「查看敌人列表 →」，href=#/wiki?tab=enemies，敌人表 22 行可见。
- CLI `click .r33-ai-cta a` RC=0（输出「✓ Done」）。
- 点击后：hash=#/wiki?tab=enemies，enemyStillThere=true，rowCount=22，dashboard=false，navActive=[敌人抗性图鉴]——停留敌人 tab，未跳看板。

## 5. 表内滚动与前轮不回退
- 表内横滚：before=0 after=1191 sw=1515 cw=324，lastHeader=说明，scrolledToEnd=true；页面 document.scrollWidth == viewport（无横向溢出）。
- R31 反应链（tab=basics）：rowCount=7，表头 11 列含层数传播/削甲/击退；腐蚀扩散含「传播 1 层」，破甲击退含「150‰」「4px」，hasImplement=true、hasBurnCloud=true、oldWording=false、hasBad=false。
- R32 技能（tab=skills）：rowCount=24，ID3 温压弹行「3|温压弹|焰|flame|180|40|2秒|主动|穿透 1|范围 180|连锁 1|2|…」，sk3Active=true、sk3Cd=true，无回退。

## 6. 新截图与 SHA
部署后独立会话采集 11 张截图，其中 4 张为滚动/状态变化重拍的新图，SHA 两两不同且全部不等于部署前 QA 图：
- r33post-390-scrolled.png = 5f4a42f7bd17b61f52c85e4616a35e295643f748b6ca526c9e152b7b53d0caf9
- r33post-768-scrolled.png = c1ffab9e859072e4b58bdd03e9634a995720c4fc1c6482ca105643576fe18ee5
- r33post-1280-scrolled.png = 24127a8fccbdcafb7f4589a078ae928b9bc8400acfc0eea8608e4f7021e46eb6
- r33post-reaction-scrolled.png = 6a924d657de663a50b5ce0edfed40643ef43c7d4e989799a3f61434035f5e4fb
亲读以上 4 张及 CTA-after 图，均为真实 Wiki 页面（敌人/反应表、导航、AI 区），非登录页。
诚实边界：其余 7 张初始态截图（三视口 enemies、cta-before/after、reaction-tab、skill-tab）与部署前 QA 图逐字节同 SHA——静态页在相同视口/相同滚动位的确定性像素一致，这是截图确定性而非复用旧图；验收所需「≥4 张新图 sha 各异且≠部署前」由上述 4 张滚动状态图满足。

## 7. 真值治理
- 部署后临时管理令牌写入 data/admin-auth-init.js（318B，含 JWT），由 .gitignore:390 拦截；finalize 删除 data/ 与根令牌，git ls-files 无 data/auth 入库。
- 零真值扫描（.r33-scan.sh，排除 data/ 与本地回环测试登录脚本 mint-token.sh 的 bootstrap 默认口令字面）：BAD_COUNT=0；六份 sh 脚本 bash -n 全通过 SYNTAX_RC=0。
- TestNoRealSecretsInRepo：本机缺 `.local.keys` 真值源，该守卫打印「未执行」后 SKIP——这是**未执行、非 PASS**，不以此作为通过依据；改用上述静态零真值扫描与 git 门禁作为替代证据，并如实记录该边界。

## 8. QA 数据还原
- finalize.sh：删除 data/admin-auth-init.js 与根令牌 → AUTH_DELETED=YES；TRUNCATE battle_records RESTART IDENTITY CASCADE 后 pg_restore qa-before.dump → DB=104|477（与发布前基线一致）。
- git check-ignore -v 三条均命中（.gitignore:390 data/、:391 admin-auth-init.js、:386 round33-wiki-enemy-mechanics/data/）。

## 9. 工具与时间边界
- agent-browser 为 .cmd 批处理，MSYS 下用 timeout.exe 包装会恒 RC=126（命令根本未执行），本报告全部浏览器命令均以后台 job + job_output 收集控制墙上时间，不用 timeout 包装。
- 浏览器命令逐个串行执行，避免 session 竞争；每个视口独立 session 并在结束时 close。
