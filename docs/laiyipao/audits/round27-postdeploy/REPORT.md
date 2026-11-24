# R27 线上回归报告：实时波次推进 HUD 发布后验证
任务：t211（R27 提交部署）。基线 HEAD=4f1a7045（R26）；四批本地提交 bee61666 / b0ad7737b / d9a0c4cea / 本报告。域名统一占位 <YOUR_TEST_DOMAIN>；真值零入库零回显。

> **证据口径补充说明（t213 追加，勿删原历史）：** 本报告第 5、6 节所列 r27-01…r27-05 五图（SHA 1747bcd7… / 0816cb98… / 73ca5f4c… / 2ab01640… / 06ea3c75…）为**部署前 QA 批 2 的截图**，仓库文件在 t211 部署后经 git checkout HEAD 恢复为入库版，故 SHA 与批 2 一致。t212 已在真实部署后从零重跑 E2E 并独立补采五张**部署后**截图 screenshots/postdeploy-r27-01-first-wave.png 至 05-settle.png（SHA b1f82c45… / b1dffd32… / a092e160… / 7e0cf3f7… / ad439fe5…），两套 SHA 完全不同。独立部署后证据以 EVIDENCE.md 与 postdeploy-r27-01…05 为准；第 5、6 节文字与 SHA 仅作部署前 QA 历史保留，部署后新图 SHA 不冒充旧图。

## 1. 部署产物与三端健康
- 后端：laiyipao/deploy/bin/lyp-server-amd64 sha256=3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（R21 产物沿用，R27 无后端业务改动）。
- H5：重建 laiyipao/miniapp/dist/build/h5/assets/index-BLjjQbiy.js sha256=08b98caf6335f5f3cd73c277f9402fb83ba4966932d57ab97129894e61f3e0c0；bundle 命中 r27-wave-progress / waveProgressHud / 波次推进（battle-B87B4c2K.css、engine.tGp3KrGv.js、pages-battle-battle.Lh-7ynyH.js）。
- admin：沿用 R20 index-CGo_zhD3.js sha256=d5e9c483992f2a370b5def55d7418da958b695deda4db9c3446e26bd04d58c70（R27 无 admin 改动）。
- 健康：/healthz=200、/readyz=200、H5(5174)=200、admin(5175/laiyipao-admin/)=200。

## 2. 本地门禁（发布前）
- 契约：battle_r27_wave_progress.contract.test.ts 1 file / 23 tests passed（exit 0）。
- 全量单测：pnpm test 209 passed | 1 skipped files，1923 passed | 2 skipped（exit 0）。
- 类型检查：pnpm type-check 与 pnpm type-check:test 均 exit 0。
- H5 构建：pnpm build:h5 DONE Build complete（exit 0）。
- Go 真值守卫：go test ./internal/domain -run TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo -count=1 ok（exit 0）。

## 3. 线上真浏览器 L5 通关与四时点 DOM
- agent-browser 无头 Chromium 390×844（非物理真机）经本地回环访问 <YOUR_TEST_DOMAIN>，无 shim、无产品代码补丁。
- 真实点击选卡通关：78 帧至 SETTLED，CARD_CLICKS=4、SKIPS=0；settle status=200、battle_id=478；结算页回放哈希 afc9d55842120e60（可验真）。HUD 未额外触发 hash 端点（hashResponses=[]）。
- 四时点截图判定：首波 index=0 本波 1/3 · 33.3%；中途 index 内进度 MID；清波 100.0%；换到下一波本波 0/5 · 0.0%。fill 宽度与 DOM 百分比一致；所有 progText 严格 x.x%。
- 权威波次总数 L5 W0=3/W1=5/W2=6/W3=8/W4=14 与 spawns 逐字段一致（审计 REPORT §1）。

## 4. R27 核心语义与 AI 判据
- 只读派生：engine.waveProgressHud() 只读 spawns 总数、场上存活、待出场，返回新对象；不写 step/record/score/replay_hash，settle 与回放哈希不受污染。
- 空态逐字「本波暂无敌人进度数据」由契约补证（L5 每波均有敌人，浏览器侧不可达，不伪造空态截图）。
- AI 四判据齐：①可见「AI · 波次推进」②真实数据结论逐帧随 DOM 变化 ③依据行逐字「依据：引擎 waveProgressHud() 只读快照（当前波 spawns 总数 − 场上存活 − 待出场）；只读派生不进 replay_hash」④CTA 逐字「继续清理场上敌人 →」。
- 诚实红线逐字「辅助进度提示非权威战斗判定，已处理含击败与漏怪（漏怪已扣防线）；权威以 replay_hash 为准」。hasUndefined/hasNaN=false。

## 5. 真浏览器证据与探针
- 5 张截图 390×844 亲读非登录页：r27-01 首波 1/3·33.3% + AI 解读，r27-02 中途，r27-03 清波 100%，r27-04 下一波归零，r27-05 通关结算（hash afc9d558…）。本轮亲读 r27-01 与 r27-05，均见 HUD/结算有效内容。
- 探针：consoleErrors=0、apiFailures=0、failedRequests=0；pageErrors=67 逐条均为 Chrome Autoplay 限制噪声「The play() request was interrupted by a call to pause()」与 R24/R25/R26 同口径，过滤后真实错误 peReal=0。
- R24 元素层、R26 下一波简报与 R27 同屏共存，无横向溢出，hud-row 保留 flex-wrap:wrap，未回退 R9 窄屏布局。

## 6. 截图 SHA256（入库版，5 图各异）
- r27-01-first-wave.png = 1747bcd7584476e3eb50d581e639123fb04a217e7eebe8006cb8b8efe213297e
- r27-02-mid-wave.png = 0816cb98df703f09834950cded2b5b0ba1bb784e83b78efef0492479a5d45d23
- r27-03-clear-100.png = 73ca5f4c22c0bb2449df10559b6b8f81f9c54257b10c413dc5febd4c04e50746
- r27-04-next-wave.png = 2ab016400876e1d009fc26014daa42c9b48ad2d146e6a60594e77f40e2f68611
- r27-05-settle.png = 06ea3c7589e133f816e80654f639fd7128128a556899d2b8a6e3aecf29d8979c
- E2E 重拍后已 git checkout HEAD 恢复为入库版，SHA 与批2提交一致，保相邻提交稳定。

## 7. 零真值、清理与结论
- 本报告及四批提交零真值：域名 <YOUR_TEST_DOMAIN> 占位，JWT/口令/带口令连接串/个人路径/真实 IP 全部 0 命中；auth-init.js（真 token）已删除，仅留占位模板；data/ 由 .gitignore:331-332 拦截，git check-ignore 确认。
- QA 库已用 pg_restore --clean 还原，battle_records=104（E2E 新增战报 id=478 已清除）。
- 结论：R27 线上回归六项门禁全部通过——三端健康、H5 含 R27 产物、L5 真浏览器通关四时点 DOM、权威 settle/hash 不污染、AI 四判据与诚实红线、探针真实错误 0、截图各异亲读、零真值。可发布。