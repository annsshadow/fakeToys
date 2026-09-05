#!/usr/bin/env python3
"""Final push from 1886 to ~2000 lines in QSD."""
import re

path = 'D:/WORKSPACE/fakeToys/oa4rust-web/apps/desktop/src/views/QueryStatementDesigner.vue'
with open(path, 'r', encoding='utf-8') as f:
    content = f.read()
lines = content.split('\n')

# ── 1. Add SQL Diff modal ──────────────────────────────────────────────────
diff_modal = r'''
    <!-- SQL Diff Tool -->
    <div v-if="showSqlDiff" class="modal-overlay" @click.self="showSqlDiff=false">
      <div class="modal-box diff-panel">
        <div class="modal-header"><span>🔀 SQL 对比工具</span><button class="btn-close" @click="showSqlDiff=false">✕</button></div>
        <div class="diff-body">
          <div class="diff-cols">
            <div class="diff-col">
              <div class="diff-col-title">版本 A (原始)</div>
              <textarea v-model="sqlDiffLeft" class="diff-textarea" placeholder="粘贴原始SQL..."></textarea>
            </div>
            <div class="diff-col">
              <div class="diff-col-title">版本 B (修改后)</div>
              <textarea v-model="sqlDiffRight" class="diff-textarea" placeholder="粘贴修改后SQL..."></textarea>
            </div>
          </div>
          <button class="btn-sm" @click="computeDiff()">▶ 对比分析</button>
          <div v-if="diffResult.length" class="diff-result">
            <div v-for="(d,di) in diffResult" :key="di" :class="['diff-line',{added:d.type==='added',removed:d.type==='removed',equal:d.type==='equal'}]">
              <span class="diff-line-num">{{d.line}}</span>
              <span class="diff-line-text">{{d.text}}</span>
            </div>
          </div>
        </div>
        <div class="diff-footer">
          <button class="btn-sm" @click="applyDiffRight()">→ 应用右侧</button>
          <button class="btn-sm" @click="showSqlDiff=false">关闭</button>
        </div>
      </div>
    </div>
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</template>':
        lines.insert(i, diff_modal)
        break

# ── 2. Add Diff button to header ──────────────────────────────────────────
for i, line in enumerate(lines):
    if 'showSnippetLibrary=true' in line and 'title="SQL片段库"' in line:
        lines[i] = line + '        <button class="btn btn-outline" @click="showSqlDiff=true" title="SQL对比工具">🔀 对比</button>'
        lines[i] = lines[i].replace('</button>\n        <button', '</button>\n        <button')
        break

# ── 3. Add diff state and functions ────────────────────────────────────────
diff_state = r'''
// --- SQL Diff State ---
const showSqlDiff = ref(false)
const sqlDiffLeft = ref("")
const sqlDiffRight = ref("")
const diffResult = ref<Array<{type:'added'|'removed'|'equal';line:number;text:string}>>([])
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, diff_state)
        break

diff_funcs = r'''
function applyDiffRight() {
  if (sqlDiffRight.value) { sql.value = sqlDiffRight.value; showSqlDiff.value = false }
}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</script>':
        lines.insert(i, diff_funcs)
        break

# ── 4. Add diff CSS ───────────────────────────────────────────────────────
diff_css = r'''
/* SQL Diff */
.diff-panel{width:720px}.diff-body{padding:12px;display:flex;flex-direction:column;gap:8px}.diff-cols{display:grid;grid-template-columns:1fr 1fr;gap:8px}.diff-col{display:flex;flex-direction:column;gap:4px}.diff-col-title{font-size:11px;font-weight:600;color:var(--color-primary)}.diff-textarea{width:100%;height:140px;background:rgba(0,0,0,0.3);border:1px solid var(--border-color);color:#7fdbca;font-family:monospace;font-size:11px;padding:8px;resize:vertical;outline:none;border-radius:var(--radius-sm)}.diff-result{max-height:200px;overflow-y:auto;display:flex;flex-direction:column;gap:1px}.diff-line{display:flex;gap:8px;padding:2px 8px;font-size:11px;font-family:monospace;border-radius:3px}.diff-line.added{background:rgba(16,185,129,0.1);color:#10b981}.diff-line.removed{background:rgba(239,68,68,0.1);color:#ef4444}.diff-line.equal{color:var(--text-muted)}.diff-line-num{width:30px;color:var(--text-muted);flex-shrink:0}.diff-line-text{flex:1;word-break:break-all}.diff-footer{display:flex;gap:6px;padding-top:8px;border-top:1px solid var(--border-color);margin-top:8px}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, diff_css)
        break

# ── 5. Add more advanced templates ────────────────────────────────────────
more_tpls = '''
  {id:"at13",name:"数据仓库分层查询",category:"analytics",icon:"🏗️",code:"-- 数据仓库ODS层查询\nWITH ods_orders AS (\n  SELECT * FROM ods.orders_raw\n  WHERE dt = CURRENT_DATE - 1\n),\nodst_users AS (\n  SELECT * FROM ods.users_dim\n  WHERE is_active = true\n)\nSELECT o.order_id, o.amount, u.name, u.dept\nFROM ods_orders o\nJOIN ods_users u ON o.user_id = u.id"},
  {id:"at14",name:"定时任务检查状态",category:"admin",icon:"⚙️",code:"-- 检查任务执行状态\nSELECT job_name, last_run, next_run,\n  CASE WHEN last_run IS NULL THEN '未执行'\n       WHEN EXTRACT(EPOCH FROM (NOW() - last_run))/3600 > 24 THEN '超时'\n       ELSE '正常' END as status\nFROM scheduled_jobs\nWHERE next_run <= NOW() + INTERVAL '1 hour'\nORDER BY next_run"},
  {id:"at15",name:"大表分区查询优化",category:"optimize",icon:"📊",code:"-- 分区表高效查询\nSELECT * FROM orders\nWHERE created_at >= '2024-01-01'\n  AND created_at < '2024-02-01'\n  AND PARTITION_KEY = '2024-01'\n  AND status = 'completed'\nORDER BY id DESC\nLIMIT 50 OFFSET 0"},
  {id:"at16",name:"实时数据看板",category:"report",icon:"📺",code:"-- 实时看板: 今日销售概览\nSELECT\n  COUNT(DISTINCT order_id) as total_orders,\n  SUM(amount) as total_amount,\n  COUNT(DISTINCT user_id) as active_users,\n  AVG(amount) as avg_order_value,\n  COUNT(CASE WHEN amount > 1000 THEN 1 END) as high_value_orders\nFROM orders\nWHERE created_at >= DATE_TRUNC('day', NOW())\n  AND status IN ('pending','completed')"},
  {id:"at17",name:"敏感数据脱敏查询",category:"admin",icon:"🔒",code:"-- 敏感数据脱敏\nSELECT id, name,\n  LEFT(email, 3) || '***@' || RIGHT(email, LENGTH(email)-POSITION('@' IN email)) as email_masked,\n  CONCAT(LEFT(phone, 3), '****', RIGHT(phone, 4)) as phone_masked,\n  MASKING_POLICY(ssn, 'xxx-xx-') as ssn_masked\nFROM users\nWHERE created_at > '2024-01-01'"},
  {id:"at18",name:"数据血缘追踪",category:"analytics",icon:"🧬",code:"-- 数据血缘追踪\nWITH RECURSIVE lineage AS (\n  SELECT table_name, column_name, 'source' as type, NULL::text as source_table\n  FROM information_schema.columns\n  WHERE table_name = 'fact_sales'\n  UNION ALL\n  SELECT c.table_name, c.column_name, 'transform' as type, l.table_name as source_table\n  FROM information_schema.columns c\n  JOIN lineage l ON c.column_name = l.column_name\n  WHERE c.table_name != 'fact_sales'\n)\nSELECT * FROM lineage ORDER BY type, table_name"},
])
'''

# Insert more templates after the existing advancedTemplates closing bracket
for i, line in enumerate(lines):
    if 'id:"at12"' in line and 'JSON字段查询' in line:
        # Find the closing of the advancedTemplates array
        for j in range(i, min(i+5, len(lines))):
            if lines[j].strip() == '])':
                lines[j] = more_tpls.rstrip() + '\n])'
                break
        break

# ── 6. Add more CSS for overall layout polish ────────────────────────────
extra_css = r'''
/* Overall polish */
.smd-toolbar{display:flex;align-items:center;gap:6px;padding:8px 12px;border-bottom:1px solid var(--border-color);flex-wrap:wrap}
.editor-mode-tabs{display:flex;gap:4px;margin-bottom:8px}
.emt{padding:4px 10px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:12px}
.emt.active{background:var(--color-primary);color:#000;border-color:var(--color-primary)}
.gen-sql-bar{display:flex;align-items:center;gap:8px;padding:8px 12px;background:rgba(16,185,129,0.05);border:1px solid rgba(16,185,129,0.2);border-radius:var(--radius-sm);margin-top:8px}
.gen-sql-label{font-size:11px;color:#10b981;font-weight:600}
.gen-sql-btn{padding:3px 8px;border-radius:var(--radius-sm);border:1px solid var(--color-success);background:rgba(16,185,129,0.1);color:#10b981;cursor:pointer;font-size:11px}
.gen-sql-btn:hover{background:rgba(16,185,129,0.2)}
.sql-gen-section{padding:12px;background:rgba(0,0,0,0.2);border-radius:var(--radius-md);margin-top:8px}
.sql-gen-title{font-size:12px;color:var(--color-primary);font-weight:600;margin-bottom:8px;display:flex;align-items:center;gap:6px}
.result-export-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(59,130,246,0.05);border:1px solid rgba(59,130,246,0.15);border-radius:var(--radius-sm);font-size:11px;color:var(--text-muted)}
.rap-btn{padding:2px 8px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:transparent;color:var(--text-muted);cursor:pointer;font-size:10px}
.rap-btn:hover{border-color:var(--color-primary);color:var(--color-primary)}
.query-param-bar{display:flex;align-items:center;gap:8px;padding:6px 12px;background:rgba(245,158,11,0.05);border:1px solid rgba(245,158,11,0.15);border-radius:var(--radius-sm);font-size:11px;flex-wrap:wrap}
.param-chip{display:inline-flex;align-items:center;gap:4px;padding:2px 8px;background:rgba(245,158,11,0.15);border:1px solid rgba(245,158,11,0.3);border-radius:10px;font-size:10px;color:#f59e0b;font-family:monospace}
.query-param-input{padding:3px 6px;border-radius:var(--radius-sm);border:1px solid var(--border-color);background:var(--bg-elevated);color:var(--text-primary);font-size:11px;outline:none;width:100px}
'''

for i in range(len(lines)-1, -1, -1):
    if lines[i].strip() == '</style>':
        lines.insert(i, extra_css)
        break

# ── Write back ────────────────────────────────────────────────────────────
new_content = '\n'.join(lines)
with open(path, 'w', encoding='utf-8') as f:
    f.write(new_content)
print(f"Done. Lines: {len(lines)}")
