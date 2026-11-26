# R60 部署后真浏览器复核报告（round60-postdeploy）

- 任务：t105 / same-route-rel；attempt_id=`52001a0d-0a72-47f1-93fb-0555f4dee1cc`
- 性质：发布后只读复核，不改业务源码、不写 git 之外真值；产物写入 `docs/laiyipao/audits/round60-postdeploy/`
- 起点 HEAD `fe93c86e5`（R57 末），分支 `opt1000/laiyipao-ai-refactor`

## 1. 五批提交链（精确 add，禁 -A/点号；每批 LEAK_OK/CHECK_RC=0/DATA_STAGED=0）

| 批次 | hash | 内容 |
| --- | --- | --- |
| B1 feat | `4d139389d3450f6491826fe9a8d52d8e440cffdc` | 11 页 vue + common.css，48 处 role=button 补 tabindex + @keydown.enter，保留 @click，加 cursor:pointer |
| B2 test | `0c462cd52eb83fe5bbdd916151394b7d804a17a0` | r60A11yKeyboard.contract.test.ts 6 it |
| B3 docs | `8c43af2f267b60dd1dfdd4d3e57fb6ef0b419fd9` | SPEC.md + ROUND60_DESIGN_REVIEW.md（verdict=PASS） |
| B4 audit | `f6f22896a252dcd8296cbfcc0556db77d5d53f95` | .gitignore + round60-a11y-keyboard（REPORT + 10 脚本 + 25 截图），data/ 零入库 |
| B5 postdeploy | 本报告提交时生成 | round60-postdeploy（REPORT + 探针/驱动脚本 + 12 截图），data/ 零入库 |

## 2. 部署产物核对（未回退）

| 产物 | sha256 |
| --- | --- |
| 后端 lyp-server-amd64 | `3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6` |
| admin WikiView-ST1PbAt2.js | `f074ffaca016dd0ff2ae8c5f211036d51c17252126a720474b3091b5b3fe5c2b` |
| H5 index-Dj0Yoogp.js | `246901cb51b32fbdd5a027474c3456b11fbb4921085dcf2620f0691d1db76cb8` |

说明：任务描述猜测的 admin 文件 `WikiView-JpQ4MFXA.js` 在本工作树不存在；实际唯一文件为 `WikiView-ST1PbAt2.js`，以实际为准且不回退。H5 经 `pnpm build:h5` 确定性重建（RC=0），文件名与 QA 阶段一致，主 bundle 已含 tabindex（11 页分块全部命中）与 CSS `[role=button]{cursor:pointer}`。

## 3. 部署后真 Chromium 复核（交叉视口 414×896 / 1440×900 + 768×1024 Tab/Enter）

API（start-api.sh）与 H5 静态（python http.server 5201）后台启动；auth 由 users.id=10 guest_token 经 POST /api/v1/auth/guest 换取并会话注入（只含 lyp_token/refresh/token_exp/guest/profile，data/ 被 .gitignore 忽略）。

- 6 代表页 × 2 交叉视口：announcements/reactions/equipment/skills/shop/mastery，共 12 组；`innerW==docW`、`hOverflow=0` 全部成立。
- `visAllTabindex=true`、`visAllCursorPointer=true` 在全部 12 组探针中成立；可见 role=button 均 tabindex≥0 且 cursor=pointer；focus outline 为 `solid 3px rgb(255,107,53)`（非 none）。
- 768 视口 6 页各 12 次真实 Tab：activeElement 落在 role=button（记录于 pd-tabs-*.txt / pd-focuslast-*.json，focus 态 outline=solid 3px、cursor=pointer）；随后 Enter 前后 state JSON 显示 hash/bodyLen/scrollY 产生预期副作用（如 equipment bodyLen 2320→2440、scrollY 452→413）。
- 代表探针证据：`pd-414-announcements`（rbCount=1）、`pd-1440-reactions`（rbCount=9）、`pd-414-equipment`（rbCount=2）、`pd-1440-mastery`（rbCount=1），均 hOverflow=0。

### 12 张新截图 sha256（12 张全互异）

| 截图 | sha256 |
| --- | --- |
| pd-414-announcements | `8d29b0b6a5f118cf5c12366d82956f24714ee41842a429149ec03689136f6cd7` |
| pd-414-reactions | `3268d96f96dce5b85019d3faf953a9b41b6b073683c460443c1c18a2a5cc591e` |
| pd-414-equipment | `a1b9877dec97ee988c04d757b710605c97326d27797c5484d75baa57e6c9d5a4` |
| pd-414-skills | `ba9abf30f2370f0be4787b31426500f44e9c9680853c9c2f062d31e6513d8c6f` |
| pd-414-shop | `662eb5b8161826e27acbf29aa1e60075f44d00d401162a1464faeec714eec800` |
| pd-414-mastery | `6769d4cde971763900b232d0a8dbdbab3263c7ab98ad178d94a09dea0322fda1` |
| pd-1440-announcements | `97e3136246665910c39e7612027d645953a2c720f439bf652ad93f6409f77652` |
| pd-1440-reactions | `0028281dc02e84fe6b90a00d12d359e3e4d363eb1ccea3fff7e01b722692d636` |
| pd-1440-equipment | `b6a2f2231ca70061d0774ed53152982f715447050f6ae07a8799a5dfdf24597c` |
| pd-1440-skills | `d57ee565307a32745773dd3bc40f6027f20316b27d72da6d4d45121e2ac82303` |
| pd-1440-shop | `b5333ef0d0fb1b16ca9bc12cbafa52ead49058d06d9b61e15b4c74b683971a58` |
| pd-1440-mastery | `089e655ad84d3cf5efc03ba565d3e6cb14be646bf260212319fc8fd4dbe9cb5e` |

已亲读 `pd-414-announcements.png`（公告页）、`pd-1440-reactions.png`（反应图鉴）、`pd-414-equipment.png`（装备图鉴）三张，均为非登录页真实内容。新图 12 张互异且均不等于 R60 QA 六代表页截图 sha。

## 4. 回归锚点（未回退）

- 48 处 role=button = tabindex="0" = @keydown = @keydown.enter 四一对应（QA 终审已核）。
- 令牌三锚点 tokens.css:14-16 逐字在；全 src 无 --r60- 业务使用（仅契约 2 处否定断言）。
- CTA 锚点 battles r20 @click=goStage、index ai-cta、equipment/bag/gems/skills r12-priority-cta @click=r12Go() 完好。
- R51 me 雷达、R53 首页整数百分比、R55 视觉可读性、R57 截断守卫、R48 自指防护锚点 QA 终审均确认不回退。

## 5. 门禁与收尾

- Go：`TestNoInfraIdentifiersInRepo` PASS（8.22s）；`TestNoRealSecretsInRepo` **SKIP 非 PASS**（缺 `laiyipao/deploy/.local.keys`，逐字「真值扫描这条守卫没有执行，不是『通过了』」）。
- QA 库 `battle_records` count|max = 104|477。
- `.gitignore` R60 四规则完整且末尾无空行；`git diff --check` 业务源码与全仓均 RC=0（QA 阶段 EOF 空行已由队长删除）。
- `data/`（含 auth-init.js、探针输出）整目录被 .gitignore 忽略，未入库；finalize 已删除 auth-init.js。

## 6. 诚实边界

1. `TestNoRealSecretsInRepo` 为 SKIP 非 PASS，Go 进程 RC=0 不代表该用例通过。
2. `pd-1440-announcements` 与 R57 postdeploy 同视口同数据确定性截图 sha 相同（97e31362…），非登录页也非 R60 QA 图；12 张本轮截图之间仍全互异。
3. 本轮交叉视口为 414/1440，不同于 QA 的三视口 390/768/1280；768 视口仅用于 Tab/Enter 复核，未额外截图，故新图与 QA 图不重叠。
4. CTA 残留键盘缺口（R12 自指「在本页强化装备契合 →」与 battles「看战报详情 →」仍纯 view/text）属 R60 范围外非回退，@click 鼠标言行完好。
5. admin 实际产物名与任务描述猜测不一致，以工作树唯一存在的 WikiView-ST1PbAt2.js 为准。

结论：**PASS（带上述诚实边界）**，可发布。