# R112 RED 证据摘要：R7 对比弹层键盘闭环契约

## 命令与结果

- 命令：cd laiyipao/miniapp && npx vitest run src/styles/r112R7CompareKeyboard.contract.test.ts --reporter=verbose
- 实测（两次一致）：exit=1，Test Files 1 failed (1)，Tests 12 failed | 6 passed (18)，v4.1.12。
- 首跑 Start at 20:45:30 / Duration 287ms；复跑 Start at 20:46:17 / Duration 327ms。
- 逐字日志：同目录 RED-raw.log（153 行，为复跑版本）。

## 12 处失败 = 12 个真实缺口（两页各 6，改前应为红）

| # | 测试 | 失败原因（逐字摘录） |
|---|---|---|
| 1 | skills G1 | 须有 role=dialog: 收到 <view v-if="r7CompareOpen" class="r7-compare-mask" @click="r7CompareOpen = false"> |
| 2 | skills G2 | r7CompareRef 须以 ref(null) 声明: expected false to be true |
| 3 | skills G3 | 须记录 document.activeElement 触发源 |
| 4 | skills G4 | closeR7Compare 必须存在: expected "" not to be "" |
| 5 | skills G5 | 模板不得残留内联关闭: expected 5 to be +0 |
| 6 | skills G6 | trapR7CompareTab 必须存在: expected "" not to be "" |
| 7 | equipment G1 | 须有 role=dialog（同上裸 view） |
| 8 | equipment G2 | r7CompareRef 须以 ref(null) 声明: expected false to be true |
| 9 | equipment G3 | 须记录 document.activeElement 触发源 |
| 10 | equipment G4 | closeR7Compare 必须存在 |
| 11 | equipment G5 | 模板不得残留内联关闭: expected 5 to be +0 |
| 12 | equipment G6 | trapR7CompareTab 必须存在 |

G5 的 5 处内联关闭 = 遮罩 @click + 取消 @click + 取消 @keydown.enter + 完成 @click + 完成 @keydown.enter，改后须全部改调 closeR7Compare（G5 断言模板零残留且至少 3 处引用 closeR7Compare）。

## 6 处通过 = 6 条不变量（改前应为绿，防实施回退）

- I1 两步确认：open 只设 r7CompareOpen=true 不写 persistLoadout/applyEquipmentImpact；confirm 才写入。
- I2 写入失败保留原配置 + 请重试：confirm 含 r7Applying/r7ApplyError/重试/未改动。
- I3 令牌三锚点（v1 / v3-r3 / r6）+ 两页零 --r112-。
- I4 R73 图例 / R103 时间线 mask 的 dialog/aria-modal/esc/tab 与两 trap 函数逐字不动。
- I5 r7ImpactApply 假守卫保留记录（仅查 .r7-compare-mask 类名），role=dialog/aria-modal/esc/tab 语义检查由本契约补齐。
- I6 mask v-if 与内层 @click.stop 保留。

## 复跑一致性

- 首跑 VITEST_RC=1，12 failed | 6 passed (18)，Duration 287ms。
- 复跑 VITEST_RC=1，12 failed | 6 passed (18)，Duration 327ms（RED-raw.log 留存本次）。
- 两跑失败集合完全相同，无 PARSE_ERROR：契约文件本身编译通过，所有失败均为断言失败（真实缺口）。

## 焦点边界（队长裁定，写入 GREEN/QA 口径）

- CTA 与弹层按钮均为 text（UNI-TEXT），H5 focus() 不生效（R103 已坐实）；焦点移入优先 focus 弹层根 view（UNI-VIEW，可真浏览器坐实），故 G3 锁定 r7CompareRef.value。
- 焦点归还目标 CTA 为 text，若落 BODY，按 R103 先例由 G4 静态守源码 closeR7Compare 含 focus 归还调用；QA 须如实披露，不冒充真浏览器 PASS。
- dialog 语义 / ESC 关闭 / Tab 陷阱属 DOM 层，GREEN 阶段必须真浏览器坐实，不以静态通过替代。

## 无自纠 / 无异常

- 契约一次写就、两次跑出一致预期 RED，无正则转义自纠、无 EOF、无解析错误。
- 假守卫 I5 按任务要求保留记录并由本契约补语义检查，不改 r7ImpactApply.contract.test.ts。
