# R34 部署后独立回归报告（Wiki 关卡机制四字段）

## 1. 结论
PASS。部署后另起真实 Chromium（headless、本地回环 127.0.0.1，无 shim、无 token 注入绕过，仅用管理员登录后写入本地会话的 init-script 让前端能读公开配置）独立会话，三视口 390×844 / 768×1024 / 1280×800 对公开 `/api/v1/config` 逐行核对第一章 20 关 × 13 列，MISMATCHES=0；真实点击章节下拉切到第 3/5/6 章，核对首领关 56/60 护甲 100‰、91/95 护甲 200‰、96/100 护甲 250‰ 及第 96 关未知 kind 地形原样展示。AI 结论「共 100 关：首领关 6 个，含地形关 25 个，地形种类 5 种」在初始页可见且无占位符残留。CTA 真点击停留关卡 tab、未跳看板；表内可水平滚至末列且页面无横向溢出；R31 反应 7 行、R32 技能 24 行、R33 敌人 22 行三 tab 均不回退。后端/H5 产物沿用不回退。

## 2. 部署与产物基线
- HEAD（审计批 B4）= 44f9e8155575d6a6c5f228939f4bca1039481567；B1-B4：9cfa8fa6a feat / d70d7b958 test / 774198122 docs / 44f9e8155 audit。
- 后端产物 `laiyipao/deploy/bin/lyp-server-amd64` sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 沿用，未回退）。
- H5 产物 `laiyipao/miniapp/dist/build/h5/assets/index-DCBbR3t9.js` sha256=aba4256c79fd922ec08c21dace914f8ed8c9243706252cbaa5bc2c69c4615f9b（R30 沿用，未回退）。
- admin 新产物 `laiyipao/admin/dist/assets/WikiView-ny0V3fOE.js` sha256=fa607a91eae67d3d605ba2c788268c65dfd39f6cd6a20afd15aee9b713a0636c，含 R34 四字段、AI 概览与 CTA。
- 服务健康：8080/healthz、8080/readyz、8080/api/v1/config、5174/、5175/laiyipao-admin/ 全部 HTTP 200。
- 本地门禁：`pnpm exec vitest run --reporter=dot --testTimeout=90000` / `pnpm type-check` / `pnpm build` 均退出码 0（VITEST_RC=0 TC_RC=0 BUILD_RC=0，73 文件 818 测试通过）。

## 3. 三视口逐行对账（reconcile.py 读公开 config）
执行 `python reconcile.py 390 768 1280`（内置 Python 3.12，系统 python3 为 Microsoft Store 别名不可用）：
- VP=390：ROWS=20 MISMATCHES=0 AI_OK=True BAD=False DOC=390/390 WRAP=1270/324 CFGBASE=True CTA='查看关卡一览 →' VERDICT=PASS
- VP=768：ROWS=20 MISMATCHES=0 AI_OK=True BAD=False DOC=768/768 WRAP=1270/702 CFGBASE=True CTA='查看关卡一览 →' VERDICT=PASS
- VP=1280：ROWS=20 MISMATCHES=0 AI_OK=True BAD=False DOC=1280/1280 WRAP=1270/884 CFGBASE=True CTA='查看关卡一览 →' VERDICT=PASS
- ALL_RECONCILE_OK=True。reconcile 仅覆盖初始页第一章 20 行（COVERED_IDS=20/100），其余章节由下述真实章节切换核对补足。中文在 shell 显示为乱码仅是控制台编码，probe-*.json 原始字节与 Python ascii 转义复核一致。
- 权威 config：version=6237e0b623219b8b，levels=100，boss=6，含地形关=25，地形种类=5，首领关 ID=[56,60,91,95,96,100]。

## 4. CTA 真点击（agent-browser CLI click）
- 点击前：hash=#/wiki?tab=levels，按钮文案「查看关卡一览 →」，href=#/wiki?tab=levels，关卡表 20 行可见。
- CLI `click '[data-r34-cta]'` RC=0（输出「✓ Done」）。
- 点击后：hash=#/wiki?tab=levels，levelStillThere=true，rowCount=20，dashboard=false，activeNav=[关卡一览]——停留关卡 tab，未跳看板。

## 5. 章节真实切换、表内滚动与前轮不回退
- 章节切换：CLI 真实点击 `.r34-level-mechanics .el-select` 后点 `.el-select-dropdown__item:nth-child(N)`（N=3/5/6），OPEN3/5/6=0、PICK3/5/6=0；第 3 章 boss=[56,60] 护甲 100‰，第 5 章 boss=[91,95] 护甲 200‰，第 6 章 boss=[96,100] 护甲 250‰；第 96 关地形栏原样展示「蓄能塔×1、rotor_vane×1」，未知 kind 未被吞掉。曾尝试用 JS 直接改 select 值造成假阳性，已废弃并改用真实 CSS 点击闭环，过程如实记录。
- 表内横滚：scrollY=647，before=0 after=386，wrapSw=1270，rows=20，tableVisible=true，末列可见；DOC=视口宽（无页面横向溢出）。
- R31 反应链（tab=basics）：reactionCount=7，腐蚀扩散含「传播 1 层」，破甲击退含「150‰」「4px」，hasImplement=true、hasBurnCloud=true、oldWording=false、hasBad=false。
- R32 技能（tab=skills）：skillCount=24，ID3 温压弹行含「2秒/穿透 1/范围 180/连锁 1」，不回退。
- R33 敌人（tab=enemies）：enemyCount=22，首行「1 | 典型敌人 | 普通 | 800 | …」可见，不回退。

## 6. 新截图与 SHA
部署后独立会话采集 18 张截图；其中 4 张为满足「≥4 张新图 sha 两两不同且全不等于部署前审计图」的变更状态重拍图（不同滚动位置与表内横滚状态）：
- r34post-distinct-1.png = a6f367088a4a6aa8236f91ac23aa27574ff2fb05aef37729e2c36a38da243e89
- r34post-distinct-2.png = 98dc7389fb3366427b971479194576fd94c8bf8a139bda93278181a75dd93b34
- r34post-distinct-3.png = e4bd03b9d3c9a72200f099f0a9cd2b4998b90e888b8eddff12d5d989ebac910e
- r34post-distinct-4.png = 813e5594d23c74fb7c51c3fbfeae8172d7935e9441c2fb5d5cd5ba559afec62e
Python 比对部署前审计目录 18 张图：4 张 COLLIDES_PRE=False、PAIRWISE_UNIQUE=True。亲读 distinct-1（关卡表初始列与导航高亮）、distinct-2（表格末列地形/护甲‰/满分）、390-scrolled（AI 概览共 100 关首领 6 含地形 25 地形种类 5 与 CTA）均为真实 Wiki 页面，非登录页。
诚实边界：其余 11 张初始态截图（三视口 levels、cta-before/after、css-3/5/6、reg-basics/skills/enemies）与部署前 QA 图逐字节同 SHA——静态页在同视口/同滚动位的确定性像素一致，这是截图确定性而非复用旧图；3 张同滚动位重拍图也因确定性像素同 SHA。验收所需「≥4 张新图」由上述 4 张 distinct 图满足。

## 7. 真值治理
- 部署后临时管理令牌写入 data/admin-auth-init.js（318B，含 JWT），由 .gitignore:398 拦截；finalize 删除 data/ 与根令牌，git ls-files 无 data/auth 入库。
- 零真值扫描（zero-secret-scan.sh，排除 data/ 与本地回环测试登录脚本 mint-token.sh 的 bootstrap 默认口令字面）：BAD_COUNT=0；脚本 bash -n 全通过 SYNTAX_RC=0。
- TestNoRealSecretsInRepo：本机缺 `.local.keys` 真值源，该守卫打印「未执行」后 SKIP——这是**未执行、非 PASS**，不以此作为通过依据；改用上述静态零真值扫描与 git 门禁作为替代证据，并如实记录该边界。

## 8. QA 数据还原
- finalize.sh：删除 data/admin-auth-init.js 与根令牌 → AUTH_DELETED=YES；TRUNCATE battle_records RESTART IDENTITY CASCADE 后 pg_restore qa-before.dump，独立复核 DB=104|477（与发布前基线一致）。首跑 finalize.sh 内库名写作 lyp_qa 导致脚本自身 DB 输出为空，随后诊断容器实际库名为 laiyipao 并以独立 psql/pg_restore 完成还原复核，同时已修正 finalize.sh 使用 laiyipao，该缺陷与修复过程如实记录。
- git check-ignore -v 命中（.gitignore:398 postdeploy data/ 拦截 data/config.json 与 data/；.gitignore 共 399 行，仅追加 R34 live/postdeploy 规则未截断既有规则）。

## 9. 工具与时间边界
- agent-browser 为 .cmd 批处理，MSYS 下用 timeout.exe 包装会恒 RC=126（命令根本未执行），本报告全部浏览器命令均以后台 job + job_output 收集控制墙上时间，不用 timeout 包装。
- 浏览器命令逐个串行执行，避免 session 竞争；每视口独立 session 并在结束时 close。每次真浏览器前先确认四端 200，否则会拿到 chrome-error://chromewebdata/ 空页。
