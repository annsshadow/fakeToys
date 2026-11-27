# R97 两处复制失败反馈补漏 · 部署后回归报告

- 任务：t239（attempt ee7e18d2-de71-421e-924e-42a458f5986a）
- 对象：承接 t235 规范 RED → t236 实施 GREEN → t237 QA PASS → t238 终审 verdict=pass。
- 起点：586f77c227642bce4f69385d1f9ec7be64053e06（R96 收官）
- 方法：重建 H5 生产构建，agent-browser 真 Chromium 三视口 414×896 / 820×1180 / 1440×900，me 页自然点击「复制」、battles 页 XHR mock 合成战报展开后点击「复制回放哈希」，捕获真实 toast 文案与零横溢。

## §1 五批提交链（精确 git add 逐文件，无 -A/点号/commit -am）

| 批次 | commit | 主题 | 写域 |
|---|---|---|---|
| B1 | b6c7a28f1 | feat(laiyipao): R97 两处复制失败反馈补漏 | me.vue + battles.vue 各 +1 行 fail 回调 |
| B2 | fb7f25c44 | test(laiyipao): R97 契约 6it | r97CopyFailure.contract.test.ts（101 行） |
| B3 | 3bc4d63ec | docs(laiyipao): R97 规范与终审 | 两份侦察 + SPEC + RED 证据 + 终审（5 文件） |
| B4 | 68d3d1835 | test(laiyipao): R97 QA 审计 | .gitignore + QA 报告 + 5 脚本 + 16 截图 + 1 recon 图（30 文件） |
| B5 | 见下方提交 | chore(laiyipao): R97 部署后回归报告 | 本报告 + 截图 |

五批每批均通过 LEAK_OK / DATA_IN_STAGE=0 / CHECK_RC=0（warning 仅 LF→CRLF）。B4 排除 7 个 data/ 下 PNG（data 硬门禁）。

## §2 部署产物与基线

- H5 重建：pnpm run build:h5 RC0（DONE Build complete）。
- 主 bundle：index-Cwr-Q4Qg.js，sha256=25d1c4a71209c02d82f51472787964f99f958052277a1d2f722af18504cdb47d（index.html:21 引用，≠R96 index-BMmuAvKT.js）。
- 两页 bundle 均含「复制失败，请手动选择复制」：pages-me-me.*.js 1 命中；pages-battles-battles.CR1ilmWm.js 1 命中。
- 后端 lyp-server-amd64 沿用 3d82635fd07eef5565cd9dd7a8cfa62e056104ee5b59b624398ad4111ddfbdf6（不回退）。
- 服务：H5 静态 5201（python http.server on dist/build/h5）；API 8080（start-api.sh，go run）。

## §3 部署后真浏览器三视口 × 两页回归数据

| 视口 | 页 | copyLinks/rows | 点击动作 | bodyHasFail | bodyHasSuccess | toast 文案 | iw/cw/sw | 横溢 |
|---|---|---|---|---|---|---|---|---|
| 414×896 | me | 1 | 点 text.link「复制」 | true | false | 复制失败，请手动选择复制（×3） | 414/414/414 | 否 |
| 414×896 | battles | 1 | 展开战报→点复制回放哈希 | true | false | 同上 | 414/414/414 | 否 |
| 820×1180 | me | 1 | 点 text.link「复制」 | true | false | 同上 | 820/820/820 | 否 |
| 820×1180 | battles | 1 | 展开战报→点复制回放哈希 | true | false | 同上 | 820/820/820 | 否 |
| 1440×900 | me | 1 | 点 text.link「复制」 | true | false | 同上 | 1440/1440/1440 | 否 |
| 1440×900 | battles | 1 | 展开战报→点复制回放哈希 | true | false | 同上 | 1440/1440/1440 | 否 |

- me 页 bodyLen=2335；battles 页 bodyLen=487；battles mock hits 坐实 /me/battles?limit=20&offset=0 与 /me/battles/4771。
- 真实点击触发失败分支：生产静态构建 headless 无剪贴板权限，uni.setClipboardData 走 fail → toast；无需任何 hook 注入。

## §4 部署后 6 张截图 SHA-256 表（UNIQUE 6/6，全不同于 QA 12 图）

| 截图 | SHA-256 |
|---|---|
| pd97-414-me.png | 41a9fa6ed53945accce6e4a3b528281731e8d08657b6bb1f588522d134061460 |
| pd97-414-battles.png | 3f1e28513b2bf92b8aa4deb34ad484c38d93b242cb8a7182871ab2fa6f99bd47 |
| pd97-820-me.png | 19e5a97d015b20d3c45d199e4cc7f858f5f6e0875f714ae76676db6c269b2ddb |
| pd97-820-battles.png | 05e1dea14cb838828aedf97ccb97e148cc1135023749f81e9e543062181b4f35 |
| pd97-1440-me.png | 41c002703afdfbf0f3e300a18d2721f8b0d50a47bef480f046ec5d57368bac7a |
| pd97-1440-battles.png | 3261b4cb1b792377f08709dc63ebd10f07364eaaa79099714684d7109aa1ee96 |

- 亲读 3/6：pd97-414-me（指挥官档案页/toast 居中）/ pd97-414-battles（我的战报页/toast 居中）/ pd97-1440-me（指挥官档案页/toast 居中），均非登录页。其余 3 张仅列 sha 未亲读，如实披露。
- 目录另含 12 张过程截图（首轮 hook 注入失败态 6 张 + 早期 trial 6 张，均带 -fail/-success 后缀），仅作失败诊断佐证，不列入上表正式 6 张。

## §5 门禁与静态回归锚点

- Go：TestNoInfraIdentifiersInRepo PASS 7.87s；TestNoRealSecretsInRepo SKIP 非 PASS（逐字：读不到 .local.keys —— 真值扫描这条守卫没有执行，不是「通过了」）。
- QA 库只读：users=809 / battle_records=104（docker exec lyp-qa-pg psql -U postgres -d laiyipao）。
- 静态回归：令牌三锚点 tokens.css:14/15/16 在；业务零 --r97-；R60 role=button；R61 v-aria-native；R55 placeholder-class；R71 fmtVerifyTime；R72 search-empty-text；R73 trapLegendTab；R95 back-btn；R96 空态 CTA 四件套——均沿用不回退。

## §6 诚实边界

- 生产静态构建下 window.uni 有对象但 uni.showToast/uni.setClipboardData undefined，QA 依赖 dev server 的 hook 脚本不可注入；部署后改用真实点击触发 fail 分支，不覆盖 success 分支（成功态由 QA 在 dev server 坐实，且 bundle grep 两文案均在）。
- battles 页战报为 XHR mock 合成（非真值），仅用于展开复制入口；hits 证实 API 路径与拦截生效。
- headless 无剪贴板权限，真实点击必走 fail 分支；坐实的是失败反馈按钮链路与 toast 文案，不是系统剪贴板写入能力。
- 6 张 pd 图仅亲读 3 张；QA 12 图亲读 2 张（见 QA 报告）。
- Go TestNoRealSecrets SKIP 非 PASS，如实披露。
- guest_token 不改库（users=809 / battle_records=104 只读查得）。

## §7 判定

部署后真 Chromium 三视口两页六组全部符合：失败 toast 逐字「复制失败，请手动选择复制」、零横溢、非登录页、截图互异且 ≠QA。结合 QA PASS 与终审 verdict=pass，R97 可发布。