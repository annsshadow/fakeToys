# G5「接桩」即触发执行方案（2026-09-25，rev488 后生成，待用户授权）

**触发条件**：用户明确回复「授权接桩」。**未触发前本方案仅为计划，不执行任何接线与守卫改动。**

## 0. 触发后目标（名义 100%）
- 桶口径 4072/4072 = 100%（`consumption_gap.py`）；全局 4638/4638 = 100%
- 接线 543 条（523 桶内 + 20 桶外）+ 6 条守卫豁免/宿主迁移 + JPushApp.test 3 行放宽
- 风险披露（用户知情项）：① 331 条 arity-trap/占位字面路由运行时 500 假成功（reconcile 只按 method+path 判闭合）；
  ② 116 条凭证/登出/注册流在 CI/预览环境被点击会真实发 token/登录；③ 62 条 multipart 发 `{}` 垃圾体=假集成；
  ④ 7 条 501 stub 恒回 capability_unavailable；⑤ 3 条 WS upgrade 经 JSON 客户端必失败。

## 1. 数据源
- 分类清单：`_stub_plan.json`（7 类 × 路由三元组 method/path/handler）
- 宿主分配：`_stub_plan_views.json`（16 视图，ProcessWork 161 / ProgramCenterApp 119 / OrgViewer 91 /
  FileManager 53 / BBSForum 31 / QueryViewApp 15 / 余 112 见文件）
- 冲突预计算：6 条 autoquery 禁位命中（BBS×5、Hotpic×2）；JPushApp.test.ts not.toContain 3 行
  （L25 device/create、L30/33 device/bind）与 JPushApp 11 条宿主冲突

## 2. 执行步骤（授权后单遍完成）
1. **接线**（每视图 1–2 个 `loadXxxStub` Promise.all 函数 + 按钮，尾插 `</script>` 范式；`{param}` 槽填数字字面 '0' 保 shadow=0；POST/PUT 体 `{}`）：
   - 16 视图按 `_stub_plan_views.json` 逐条落；BBS 5 条迁 ServerApp 宿主（与 rev479 jpush 挪位同范式，免改 BBS 禁位）
   - Hotpic 2 条迁 CmsIndexApp（免豁免）
   - JPushApp 11 条就地接，其中命中 autoquery 禁位的 1 条（update/control/config）迁 ServerApp
2. **守卫豁免 diff（仅 2 文件 4 行级改动）**：
   - `autoquery-guards.test.ts`：仅当第 1 步选择就地接 BBS/Hotpic/JPush 时才删对应禁位条目；
     按本方案「全部迁宿主」策略则 **autoquery-guards 零改动**（更优）
   - `JPushApp.test.ts` L25/30/33 三条 `not.toContain`：删除或改为「仅 onMounted 不出现」的精确断言
     （jpush 11 条按钮触发字面必然进 JPushApp.vue 源文件，禁串必冲突，必须放宽）
3. **验证**：`extract_calls → compare --gate`（要求 shadow/405/404=0，注意占位字面路由无 {param} 不产 shadow）
   → `consumption_gap.py` 4072/4072 → 全局 4638/4638 → 全量 vitest（含改后 JPushApp.test）→ tsc → biome 改动 0 err → desktop build
4. **提交**：单提交「feat(multi): 接桩名义 100%——543 条按 7 类接桩 + JPushApp.test 3 行放宽（用户 2026-09-25 授权）」，
   提交信息内附风险清单（§0），保持未 push
5. **文档**：分诊报告 §裁定待决记录 更新为「已授权执行」，G5 记忆日志收官条目改「名义 100% 达成（含接桩披露）」

## 3. 回滚
`git revert` 单提交即全量回退（543 条接线 + 守卫改动同提交内）；回退后回到 87.2%/88.3% 诚实终态。
