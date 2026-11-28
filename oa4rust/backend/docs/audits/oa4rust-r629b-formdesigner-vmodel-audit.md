# R629b 审计：FormDesigner 双新增面板 v-model 写丢弃（只读甄别）

## 结论
判级 P3。两处新增辅助面板（访问控制规则、数据源）的输入 ref 被 v-model 写入后，
新增按钮不读该 ref，而取数组尾部旧值提交：用户输入被丢弃；两数组又不进入 saveForm
序列化，刷新即丢失。属半成品辅助面板可用性缺陷：不涉越权、无跨用户读写、无数据破坏，
主流程（字段设计/保存表单）不受影响；无同族 P2 先例可援引，故按 P3 据实判级。

## 真父与写域
- 真父 HEAD：26a2c002b2ac824ec773658d6d5c6321f4141e7a（写域工作树干净）
- 零源码改；唯一写域=本文；临时探针 oa4rust/tests/r629b_probe.test.ts 用后即删

## 锚点（FormDesigner.vue）
- :1628-1632 newAccessRule = ref({fieldKey:'',role:'admin',action:'show'})
- :1634 newDataSource = ref({fieldKey:'',url:'',method:'GET'})
- v-model：:189 .fieldKey / :192 .role / :195 .action；:215 .fieldKey / :218 .url / :219 .method
- 新增按钮：:198 addAccessRule(accessRules[尾部]?.fieldKey||'', ...role||'', ...action||'')
  :222 addDataSource(dataSources[尾部]?.fieldKey||'', ...url||'', ...method||'GET','','')
  :400 另一入口 addDataSource() 零参
- 处理器：:1777-1778 addAccessRule 仅 push；:1787-1788 addDataSource 仅 push（均不清空 ref）
- :1783-1786 getAccessAction 定义后全仓库零调用
- :1089-1131 saveForm 的 serializeFormDefinition({name,application,fields,base,desktop,mobile,
  actions,events,validation}) 不含 accessRules/dataSources
- 全文件 newAccessRule/newDataSource 各 4 命中（3 v-model + 1 声明），无 .value 读取点

## 路由与消费链
- main.ts:218-221 path:'form-designer' 可达；:185 showAccessControl / :211 showDataSourcePanel 入口真存在
- 链路：用户输入 → v-model 写 ref → 点击新增 → 处理器收数组尾部旧值 → push → 数组未被 saveForm 序列化 → 刷新丢失

## vitest 真测（vitest+jsdom，10/10 通过，EXIT=0）
探针 oa4rust/tests/r629b_probe.test.ts；日志 oa4rust/test-results/r629b-probe.log
- T1 两 ref + 六 v-model 存在
- T2 新增按钮取数组尾部、源码无 addAccessRule(newAccessRule...) / addDataSource(newDataSource...)
- T3 new Function 运行时求值：首次点击 passed1=['','','']、passed2=['','','GET','','']；
  即使填入 user-field/editor/hide 与 /api/user/POST，处理器收到的仍是空串/默认
- T4 add 处理器仅 push、不清空 new 表单
- T5 两 ref 各 4 命中、无 .value 消费
- T6 getAccessAction 仅定义（1 命中）
- T7 saveForm 序列化不含两字段
- T8 兄弟范本成立
- T9 路由与面板入口可达
- T10 附注传参矛盾坐实

## 兄弟正确范本（对照）
- 同文件 :2324-2335 addCrossFieldCalc()：读 newCalcFormula.value / newCalcTarget.value /
  newCalcSource.value 提交后清空——正确读取 ref 并复位
- BBSForum.vue:632-645 createTopic()：提交 {...newTopic.value} 后重置——正确范式

## 三分据实
- 可达性：路由与面板入口真实存在，用户可触发（非死代码）→ P3 可用性缺陷
- 无安全：不涉越权、无跨用户读写、无数据破坏；输入仅被丢弃而非误写他人
- 半成品辅助面板：accessRules/dataSources 未纳入持久化契约，属未完成功能，主流程不受影响

## 附注（不单列候选）
:229 removeDataSource(di) 传索引 vs :407 removeDataSource(ds.id) 传 id，而 :1790-1792
一律 splice(idx,1)；两面板传参口径矛盾，按 id 调用会删错行。属另一辅助面板缺陷，仅记录不单列。

## 台账
docs/ops/current-state.md 无 R629b 条目（仅命中 R164 百分比格式化等历史项），本轮线索未登记。

## SHA-256（分列，均不含本文自身）
- runnerSha（meta 紧凑 JSON）：3b344bcc031c44a7338ef3689329bb8463b1bf6b6e316d30bc991acc914fe2ae
- 源文件 FormDesigner.vue：7bb613705e5ccb0f59c043f0c7b0721b2554b0ca22690015f02a1c1a111e5efc
- main.ts：e0019e870c2a6b47ed9c3505e8acdfbefff3807ddfe04bb8bc25d76ba02ea3f0
- BBSForum.vue：d1e9fe62ebf6874a89a6feeaef923b187db2831b232f480ba044b4f00b29fbd6
- 探针日志 r629b-probe.log：e1d368d45020d11fc39af1f90cd3493d141168b44958e202de8fc6df154569c0
- 文档 SHA-256：提交时计算，见任务回执（不在本文自引用）

## 披露
- 未跑浏览器 GUI：真实点击链路以 new Function 运行时求值 + 源码契约替代
- 探针为临时文件，验证后删除，不入库
- 无真值（无域名/IP/口令/个人信息）
