# R94 DefensesView 空态 CTA 一一对应 —— 部署后回归报告

- 任务：t220（kind=work，attempt=1，attempt_id=d2d5048f-21d5-42cb-963c-be93dc9bb65a）
- 执行：same-route-rel（R36 发布路由；本轮五批发布+部署后回归）
- 起点 HEAD：b4bedf83944d41ce142cd2dfd5191ef516610540（R93 收官）
- 报告日期：2026-11-27
- 性质：部署后真浏览器回归，接入已重建 admin 前端产物，验证空态 reason 与 CTA 一一对应在生产构建下依然成立。

## 一、五批发布链表

| 批次 | commit | 类型 | 写域 |
|---|---|---|---|
| B1 | `560162263` | feat | `laiyipao/admin/src/views/DefensesView.vue` +10/-1（新增 resetFilters + CTA 清空筛选） |
| B2 | `1e14fbe26` | test | `laiyipao/admin/tests/R94DefenseEmptyCta.contract.test.ts` 67 行 6it |
| B3 | `64d79ea20` | docs | `docs/laiyipao/specs/round94-defense-empty-cta/SPEC.md` + `docs/laiyipao/reviews/ROUND94_DESIGN_REVIEW.md`（151 行） |
| B4 | `bd1a2a9b2` | test | `.gitignore` + `round94-defense-empty-cta/REPORT.md` + 7 脚本 + 6 截图（15 文件 240+） |
| B5 | 本报告提交 | chore | `round94-defense-empty-cta-postdeploy/REPORT.md` + `pd-probe.js` + `pd-driver.sh` + 6 截图 |

每批提交前执行泄露守卫（扫描带口令连接串 / 私钥头 / 授权头 / Cookie 头等模式）、`data/` 暂存数=0、`git diff --cached --check` RC0；AUTH 四路径全程 ABSENT。B4 因 `.gitignore:612` 将 `round94-defense-empty-cta/data/` 整目录忽略，按「禁 git add -f 真值」约束不提交 data/ 下任何文本与 mock JS，仅提交未忽略的报告/脚本/截图。

## 二、产物 sha256

重建命令：`cd laiyipao/admin && pnpm run build:only`，RC0（✓ built in 965ms）。

| 产物 | sha256 |
|---|---|
| `dist/assets/index-CHxFzEy9.js`（主 bundle，index.html 引用） | `06bb3f7e82c4d392d099bbbf1e455fda2903f37eb6ddc53191277f01be36fcd6` |
| `dist/assets/DefensesView-BGi9RnI0.js`（本轮改动视图） | `93b872c28907318b30f01c5dfa1d6409a104f1c631bae97be15ef86cc59c5ebb` |
| `dist/assets/PageSkeleton-DwLo_tPX.js` | `d8ac38b1b920bd2e9a993213891d1300db8aaa6892785660e96961aa299e08f3` |
| 后端 `laiyipao/deploy/bin/lyp-server-amd64`（沿用不回退） | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` |

## 三、部署后真浏览器三视口回归

- 服务：静态服务 5175（`pd-serve.py`，base `/laiyipao-admin/`），后端未起，init-script 注入占位令牌 `r94-qa-placeholder` + mock fetch（带筛选返回空表，无筛选返回 3 行合成防线）。
- 操作：真实 `fill` 搜索框 `NO_SUCH_zzz` + `press Enter` 造空态 → 探针 `pd-r94-V-before` + 截图 → 真实 `click ".empty-state .empty-state-action"` → 探针 `pd-r94-V-after` + 截图。

| 视口 | 阶段 | keyword | 空态 | reason | CTA | 表格行 | resultCount | defenses URL 序列（末条） | hOverflow | isLogin |
|---|---|---|---|---|---|---|---|---|---|---|
| 414×896 | before | NO_SUCH_zzz | true | 当前筛选条件下没有防线；可清空筛选或刷新后重试。 | 清空筛选 | 0 | 共 0 条防线 | ...&keyword=NO_SUCH_zzz | false | false |
| 414×896 | after | "" | false | — | — | 3 | 共 3 条防线 | /admin/defenses?limit=50&offset=0（裸） | false | false |
| 820×1180 | before | NO_SUCH_zzz | true | 同上逐字 | 清空筛选 | 0 | 共 0 条防线 | ...&keyword=NO_SUCH_zzz | false | false |
| 820×1180 | after | "" | false | — | — | 3 | 共 3 条防线 | 裸请求 | false | false |
| 1440×900 | before | NO_SUCH_zzz | true | 同上逐字 | 清空筛选 | 0 | 共 0 条防线 | ...&keyword=NO_SUCH_zzz | false | false |
| 1440×900 | after | "" | false | — | — | 3 | 共 3 条防线 | 裸请求 | false | false |

三视口 before/after 的 `innerW==clientW==scrollW`（414/820/1440），`hOverflow=false`，`isLogin=false`。after 的 fetch URL 序列明确显示：`?limit=50&offset=0` → `?limit=50&offset=0&keyword=NO_SUCH_zzz` → `?limit=50&offset=0`（裸），坐实 CTA 真实清空筛选并重载，恢复 3 行防线。

## 四、部署后截图 sha256（6 张全互异 UNIQUE=6/6，全不同于 QA 六图）

| 截图 | sha256 |
|---|---|
| pd-r94-414-before.png | `69af6e8515e52263779311a83190c34b61024629cee591dca0438efded5d3379` |
| pd-r94-414-after.png | `df2396ae4d1fa1e2a8ab14b607e9c556ab8a0c76200199559c9073282a4919bd` |
| pd-r94-820-before.png | `c9efd7f708629c4fe7856b31625b0b69a001f7f4febf25359a9ca0984ff57c50` |
| pd-r94-820-after.png | `f91510b4892a790d6548843f13c952c5704bbdaad9062c9afa45ae7fe306f4d2` |
| pd-r94-1440-before.png | `b1c1b4e06afead77fbc5f60352ac1e1fe49df7f4a05b377d066c0867bc4999a1` |
| pd-r94-1440-after.png | `47b09501f7d74f6d37fd2497839f15fd6afad75acd53d133cb65f03598ebbf0a` |

QA 六图基线（全不同）：390-before=c65aeebc / 390-after=2563f674 / 768-before=b109ad1a / 768-after=9d7891c0 / 1280-before=936347bc / 1280-after=33cb4639。

**亲读 3 张**：`pd-r94-414-before.png`（414×896，防线值守页，AI 数据不足卡 + KPI 全 0 + 搜索框 NO_SUCH_zzz + 空态卡，非登录页）、`pd-r94-820-after.png`（820×1180，防线值守页恢复态，KPI 3/0/3 + 搜索框已清空 + 3 行防线 QAplayer1/2/3，非登录页）、`pd-r94-1440-after.png`（1440×900，同页恢复态，KPI 3 + 3 行完整防线列表 + AI 洞察，非登录页）。其余 3 张以 DOM 探针 + sha256 为证。

## 五、静态回归锚点

- 令牌三锚点 `laiyipao/admin/src/styles/tokens.css:12/13/14`：`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 逐字在。
- 业务源码 `laiyipao/admin/src` 全量 grep `--r94-` 零命中（ZERO）。
- `resetFilters()` 范式 5 视图齐在：Battles:135 / Defenses:219（本轮新增）/ Levels:56 / Skills:21 / Users:27，前序四视图未回退。
- `git diff --check` 与 `git diff @{u} HEAD --check` RC0。

## 六、门禁

| 门禁 | 结果 | RC |
|---|---|---|
| R94 契约 6it（QA §5 / 终审 §5 独立复跑） | 6/6 passed | 0 |
| admin 全量 vitest（QA / 终审） | 83 files / 915 tests passed | 0 |
| vue-tsc type-check | 通过 | 0 |
| pnpm run build:only（本轮重建） | ✓ built in 965ms | 0 |
| Go TestNoInfraIdentifiersInRepo（本轮实跑） | PASS 8.22s | 0 |
| Go TestNoRealSecretsInRepo（本轮实跑） | **SKIP 非 PASS**（读不到 `.local.keys` —— 真值扫描这条守卫**没有执行**，不是「通过了」） | 0 |
| git diff --check | 通过 | 0 |

`TestNoRealSecretsInRepo` 逐字输出：`secrets_leak_test.go:182: 读不到 D:\WORKSPACE\fakeToys\.agents\worktrees\dsh-d0bc4721-3248\laiyipao\deploy\.local.keys —— 真值扫描这条守卫**没有执行**，不是「通过了」`。本报告如实标注为 SKIP，不把它计入「通过」。

## 七、诚实边界

- 后端未起，全部数据由 init-script mock（合成 3 行防线、占位令牌），坐实的是 CTA 行为链而非真实后端业务数据；令牌仅内存注入，零真值文件。
- 6 张部署后截图仅亲读 3 张，其余 3 张以 DOM 探针 + sha256 为证。
- QA 库只读核实：users 表 477 存在、104 无记录；battle_records 中 477 有 1 条历史记录、104 无记录；本轮未写库。
- `qaCleared` 字段恒 true 系 mock 首次裸请求即置位，不作判据；判据以 fetch URL 序列与 DOM 实测为准。
- 本轮无 Go 真值扫描 PASS 证据（TestNoRealSecrets SKIP），按 AGENTS.md「Fail loud」如实披露。

## 八、结论

五批发布全部完成，部署后三视口真浏览器回归全绿：空态 reason 承诺「清空筛选」与 CTA 行为一一对应，点击后 keyword 清空、请求 URL 无筛选参数、表格恢复 3 行、零横溢、非登录页。产物 sha、截图 sha、Go 门禁（含 SKIP 如实标注）与 hygiene 均已取证。**可发布**。
