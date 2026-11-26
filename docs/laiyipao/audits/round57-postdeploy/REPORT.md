# R57 部署后真浏览器复核报告

- 任务：t96 / attempt 7ada0191-72b8-4dfd-a8a1-24ce63dba242
- 工作树：dsh-d0bc4721-3248，分支 opt1000/laiyipao-ai-refactor
- 依赖：实施两文件 feat 374dacd86 / 契约 b48823eac / SPEC+终审 6050adf6c / QA审计 4d75d667a
- 结论：**PASS（带诚实边界）**。部署后 768 主门禁双候选修复在重建 H5 上复现，R12/R14/R48/R51/R53/R55 回归锚点无回退。

---

## 1. 部署产物基线（均未回退）

| 产物 | sha256 | 说明 |
|------|--------|------|
| lyp-server-amd64 | 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6 | 沿用 R21 后端基线，未改 |
| WikiView-ST1PbAt2.js | f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b | 沿用 R47 admin 基线，未改 |
| index-BzpPnYGI.js | 3dc4dfaaadb4480f0e9ad2baef53cbfeb4749932c7d072f25ff54319e530be15 | 主 JS 内容 hash 不变：本轮是纯 CSS 修复 |

本轮在 laiyipao/miniapp 下 `pnpm build:h5` RC=0。修复落在 CSS 产物：
- `announcements-BiBaUt9U.css`（sha 48aa81c9…）`.ann-title{…min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}`；
- `index-BMHvomjF.css`（index.html 实际引用）`.ph{color:var(--dim);min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}`。
主 JS bundle 因无逻辑改动 hash 不变属预期；bundle 内无 `--r57-` 业务使用。

## 2. 真浏览器复核（Chromium，API 8080 + H5 静态 5201）

### 2.1 768×1024 主门禁

- 公告页：innerW=docW=768，hOverflow=0（QA 基线 795/27 的溢出消失）；`.ann-title` sw506→cw151 带 ellipsis/nowrap/hidden/min-width:0 收缩；`.ann-date` right=610 不越界；日期文本「2026-11-24 01:10」与「最新」徽标（detail 探针 .ann-new display=block/visibility=visible/right=229）均在；hardTruncCount=0。
- 敌人图鉴：innerW=docW=768，hOverflow=0；占位 sw466>cw409 但四守卫齐全 → ellipsisOKCount=1、hardTruncCount=0；色 rgb(150,160,173) 对比约 6:1≥4.5；placeholder-class=ph 与文案「搜索敌人名称 / 类别（如 Boss、…」尾部省略号如预期。

### 2.2 交叉视口新图（414×896 / 820×1180 / 1440×900）

三视口 × announcements/enemies 共 6 组探针 + 截图；各视口 hOverflow=0、hardTruncCount=0，截图 sha 互异且均不等于 QA 六图。连同 768 两张共 8 张新图，均亲读确认为非登录页。

### 2.3 回归锚点（768）

- R12/R48：equipment 页「装备契合 0/18（0%，权重 21%）：补满可涨约 378 分，优先补它」与「在本页强化装备契合 →」自指 CTA 均在。
- R14：reactions 页 7 处 `.r14-advice-empty-cta`「去打一场战斗 →」white-space=nowrap，未竖排。
- R51：me 页 `.r51-radar` viewBox 结构 rings=3/axes=5，诚实口径「非权威战斗数据；权威以 replay_hash 为准」在。
- R53：index 页结论「装备契合仅 0%（权重 21%）：优先补它，收益 378」无长浮点（hasLongFloat=false）。
- 令牌三锚点与无 `--r57-` 业务使用由 QA 契约与终审已覆盖，本轮未触碰。

## 3. 截图 SHA-256（8 张互异，≠ QA 六图）

| 截图 | 视口 | 字节 | sha256 |
|------|------|------|--------|
| pd-768-announcements-announcements.png | 768×1024 | 251267 | 506038cca7088dcc695efb6b45ab7005bdb44fd9be4564e34abc223f45233523 |
| pd-768-enemies-enemies.png | 768×1024 | 241362 | 1b3d3f13cda02714d83fec849666fca1354cf9e67554c4910a4285a0c5b5e40f |
| pd-414-announcements-announcements.png | 414×896 | 121146 | f23c37db2a831388fd5306d49c15086c0c8ff048d79d7f69d9c32b3a5b8b9ef7 |
| pd-414-enemies-enemies.png | 414×896 | 103806 | dabd8bb6493b83c8e9c23040df14ca5e935a9a0cf1d092259ef1c1355d6cfe45 |
| pd-820-announcements-announcements.png | 820×1180 | 281383 | c1378715d0d30da391682a7e8270e1f848a66ef0e4c8016e3d8584a87a3e1185 |
| pd-820-enemies-enemies.png | 820×1180 | 277737 | 8555dc33af5d760f9a265c5edcf882dd16137ba3ce290bf3615ee960ea06c9b2 |
| pd-1440-announcements-announcements.png | 1440×900 | 146195 | 97e3136246665910c39e7612027d645953a2c720f439bf652ad93f6409f77652 |
| pd-1440-enemies-enemies.png | 1440×900 | 137510 | ccfb91c9f307c245aa662738ca7394901e9d7d1ad79cb3bd193312af62543575 |

## 4. 门禁与收尾

- Go：`TestNoInfraIdentifiersInRepo` PASS(5.07s)；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（缺 .local.keys，原文「真值扫描这条守卫没有执行，不是『通过了』」）。
- QA 库 battle_records：104|477（与基线一致）。data/ 与 auth-init.js 未入库；finalize 清理鉴权临时文件。
- 五批：B1 feat 374dacd86 / B2 test b48823eac / B3 docs 6050adf6c / B4 audit 4d75d667a / B5 postdeploy（本报告），每批 LEAK=0、diff --check=0、data/ 无入库。

## 5. 诚实边界

1. TestNoRealSecrets 因缺 .local.keys SKIP 非 PASS，未伪装通过。
2. 主 JS bundle hash 不变是纯 CSS 修复的预期结果，非未重建；生效证据是 CSS 产物四守卫与本页探针。
3. 首版 PD 探针的「最新」徽标选择器写错导致 annLatest=0；已用 detail 探针（.ann-new）复核徽标 display=block/visibility=visible/right=229，澄清为探针口径问题，非功能缺失。
4. index 页 768 出现 hOverflow=8，源是新生成 guest token 的访客昵称 `.res-row/.res-pill`（尾部🔑1 emoji）文本溢出，属环境数据差异，不在 R57 两文件修复范围内；同页面 R53 结论与长浮点断言均正常。
5. 回归用 user10 真实存档（装备契合 0%）而非 mock 的 44%，结论数值不同属数据差异非功能回退。
